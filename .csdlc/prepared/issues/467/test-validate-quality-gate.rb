#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "pathname"

ROOT_FOR_TEST = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
VALIDATOR = ROOT_FOR_TEST / ".csdlc/prepared/issues/467/validate-quality-gate.rb"
WORK = ROOT_FOR_TEST / ".csdlc/evidence/467/adversarial-fixtures"
load VALIDATOR

def copy(value)
  Marshal.load(Marshal.dump(value))
end

def errors_for(name, matrix)
  path = WORK / "#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  validate_matrix(path, canonical: false).last
end

def reject!(name, matrix, expected)
  errors = errors_for(name, matrix)
  raise "#{name} unexpectedly passed" if errors.empty?
  raise "#{name} did not report #{expected}: #{errors.inspect}" unless errors.any? { |error| error.include?(expected) }
end

def rebind!(delivery)
  unsigned = delivery.reject { |field, _| field == "binding_sha256" }
  delivery["binding_sha256"] = Digest::SHA256.hexdigest(JSON.generate(unsigned))
end

FileUtils.rm_rf(WORK)
FileUtils.mkdir_p(WORK)
base = JSON.parse(MATRIX.read)
raise "base fixture failed" unless errors_for("base", base).empty?

tampered = copy(base)
tampered["rows"].shift
reject!("missing-row", tampered, "denominator_missing")

tampered = copy(base)
tampered["rows"] << copy(tampered["rows"].first)
reject!("duplicate-row", tampered, "denominator_duplicate")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
row["disposition"] = "blocked"
reject!("accepted-suppressed", tampered, "accepted_mapping_missing")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
row["evidence"]["deliveries"].first["merge_sha"] = "0" * 40
rebind!(row["evidence"]["deliveries"].first)
reject!("stale-merge-head", tampered, "delivery_identity_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
delivery = row["evidence"]["deliveries"].first
delivery["review"]["reviewed_head"] = "0" * 40
rebind!(delivery)
reject!("stale-non-ancestral-review", tampered, "review_head_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
delivery = row["evidence"]["deliveries"].first
delivery["terminal"]["terminal_digest"] = "malformed"
rebind!(delivery)
reject!("malformed-terminal-digest", tampered, "terminal_authority_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| Array(item.dig("evidence", "deliveries")).any? { |delivery| delivery.dig("checks", "policy") == "required_aggregate_checks" } }
delivery = row["evidence"]["deliveries"].find { |item| item.dig("checks", "policy") == "required_aggregate_checks" }
delivery["checks"]["successful"].first["id"] = -1
rebind!(delivery)
reject!("fabricated-check-run", tampered, "check_receipt_identity_invalid")

tampered = copy(base)
row = tampered["rows"].find { |item| Array(item.dig("evidence", "deliveries")).any? { |delivery| delivery.dig("checks", "policy") == "required_aggregate_checks" } }
delivery = row["evidence"]["deliveries"].find { |item| item.dig("checks", "policy") == "required_aggregate_checks" }
delivery["checks"]["policy"] = "ruleset-bypass"
rebind!(delivery)
reject!("substituted-check-policy", tampered, "check_policy_substituted")

tampered = copy(base)
accepted = tampered["rows"].select { |item| item["disposition"] == "accepted" }
accepted.first["evidence"]["deliveries"] = copy(accepted.last["evidence"]["deliveries"])
reject!("cross-row-substitution", tampered, "delivery_count_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
delivery = row["evidence"]["deliveries"].first
delivery["pr_repository"] = "attacker/substitute"
rebind!(delivery)
reject!("cross-repository-substitution", tampered, "delivery_identity_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
delivery = row["evidence"]["deliveries"].first
delivery["proofs"].first["sha256"] = "0" * 64
rebind!(delivery)
reject!("fabricated-accepted-proof", tampered, "proof_content_tampered")

tampered = copy(base)
row = tampered["rows"].find { |item| Array(item.dig("evidence", "deliveries")).any? { |delivery| Array(delivery["proofs"]).any? { |proof| proof["json_schema"] } } }
delivery = row["evidence"]["deliveries"].find { |item| Array(item["proofs"]).any? { |proof| proof["json_schema"] } }
delivery["proofs"].find { |proof| proof["json_schema"] }["json_schema"] = "forged.schema.v1"
rebind!(delivery)
reject!("evidence-schema-tamper", tampered, "proof_schema_tampered")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "scoped_out" }
row["evidence"]["scope"]["target"] = "invented"
reject!("scope-tamper", tampered, "scope_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["id"] == "feature:OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92" }
row["evidence"]["scope_authority"].first["sha256"] = "0" * 64
reject!("scope-authority-tamper", tampered, "scope_authority_tampered")

tampered = copy(base)
row = tampered["rows"].find { |item| item["id"] == "critical:AEE-018" }
row["source_status"] = "blocked_with_evidence"
reject!("implemented-status-tamper", tampered, "source_status_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["id"] == "critical:AEE-020" }
row["disposition"] = "accepted"
reject!("circular-tail-tamper", tampered, "scope_mismatch")

tampered = copy(base)
tampered["rows"].each { |row| row["disposition"] = "blocked" }
reject!("vacuous-all-blocked", tampered, "vacuous_all_blocked")

FileUtils.rm_rf(WORK)
puts JSON.generate(schema: "adl.v0.92.quality_gate_adversarial_suite.v4", status: "passed", cases: 17, accepted: 30, scoped_out: 3, blocked: 0)
