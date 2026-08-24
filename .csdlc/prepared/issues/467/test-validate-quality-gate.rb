#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "pathname"

TEST_ROOT = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
VALIDATOR = TEST_ROOT / ".csdlc/prepared/issues/467/validate-quality-gate.rb"
WORK = TEST_ROOT / ".csdlc/evidence/467/adversarial-fixtures"
load VALIDATOR

def clone(value)
  Marshal.load(Marshal.dump(value))
end

def validate_fixture(name, matrix)
  path = WORK / "#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  validate_matrix(path, canonical: false).last
end

def expect_failure(name, matrix, expected)
  errors = validate_fixture(name, matrix)
  raise "#{name} unexpectedly passed" if errors.empty?
  raise "#{name} did not prove #{expected}: #{errors.inspect}" unless errors.any? { |error| error.include?(expected) }
end

def expect_success(name, matrix)
  errors = validate_fixture(name, matrix)
  raise "#{name} failed: #{errors.inspect}" unless errors.empty?
end

FileUtils.rm_rf(WORK)
FileUtils.mkdir_p(WORK)
base = build_matrix
expect_success("base-positive-controls", base)

tampered = clone(base)
tampered["rows"].shift
expect_failure("missing-row", tampered, "denominator_missing")

tampered = clone(base)
tampered["rows"] << clone(tampered["rows"].first)
expect_failure("duplicate-row", tampered, "denominator_duplicate")

tampered = clone(base)
tampered["rows"] << { "id" => "feature:INVENTED", "kind" => "feature", "source" => "invented", "owner" => "none", "source_status" => "feature_contract", "disposition" => "blocked", "discovery" => { "status" => "investigated" }, "blocker_kind" => "evidence_mapping_missing", "blockers" => ["invented"], "evidence" => {}, "claim_boundary" => "none" }
expect_failure("extra-row", tampered, "denominator_extra")

accepted = base["rows"].find { |row| row["disposition"] == "accepted" }
unprofiled = base["rows"].find { |row| row["id"] == "feature:ACP_COGNITIVE_PROFILES_v0.92" }

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == accepted["id"] }
row["disposition"] = "blocked"
row["blockers"] = ["required_proof_missing"]
row["blocker_kind"] = "required_proof_missing"
row["evidence"] = {}
expect_failure("suppressed-discoverable-evidence", tampered, "discoverable_evidence_suppressed")

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == unprofiled["id"] }
row["disposition"] = "accepted"
row["blockers"] = []
row["blocker_kind"] = nil
row["evidence"] = clone(accepted["evidence"])
expect_failure("fabricated-accepted-row", tampered, "accepted_without_canonical_profile")

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == accepted["id"] }
row["evidence"]["authority_kind"] = "self_asserted_json"
expect_failure("self-attested-accepted", tampered, "prohibited_authority:self_asserted_json")

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == accepted["id"] }
row["evidence"]["reviewed_head"] = "0" * 40
expect_failure("stale-reviewed-head", tampered, "reviewed_head_mismatch")

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == accepted["id"] }
row["evidence"]["merge_sha"] = row["evidence"]["reviewed_head"]
expect_failure("non-ancestral-merge", tampered, "merge_sha_mismatch")

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == accepted["id"] }
row["evidence"]["typed_terminal"]["cache"]["sha256"] = "0" * 64
expect_failure("malformed-terminal-evidence", tampered, "typed_terminal_cache:digest_mismatch")

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == accepted["id"] }
row["evidence"]["required_checks"] = ["adl-ci"]
expect_failure("check-ruleset-substitution", tampered, "required_checks_not_canonical")

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == accepted["id"] }
row["evidence"]["positive"]["sha256"] = "0" * 64
expect_failure("positive-proof-digest", tampered, "positive:digest_mismatch")

tampered = clone(base)
row = tampered["rows"].find { |item| item["id"] == unprofiled["id"] }
row["blockers"] = ["evidence_normalization_missing"]
expect_failure("normalization-only-product-blocker", tampered, "normalization_gap_not_concrete_blocker")

tampered = clone(base)
tampered["rows"].each do |row|
  row["disposition"] = "blocked"
  row["evidence"] = {}
  row["blocker_kind"] = "required_proof_missing"
  row["blockers"] = ["required_proof_missing"]
  row["discovery"] = { "status" => "uninvestigated" }
end
expect_failure("vacuous-all-blocked-publication", tampered, "discoverable_evidence_suppressed")
expect_failure("vacuous-all-blocked-publication", tampered, "vacuous_all_blocked_publication")

FileUtils.rm_rf(WORK)
puts JSON.generate(schema: "adl.v0.92.quality_gate_adversarial_suite.v2", status: "passed", cases: 13, positive_controls: base["rows"].count { |row| row["disposition"] == "accepted" })
