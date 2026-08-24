#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
VALIDATOR = ROOT / ".csdlc/prepared/issues/311/validate-quality-gate.rb"
MATRIX = ROOT / "docs/reviews/v0.92/quality-gate-311/feature-completion-matrix.json"
WORK = ROOT / ".csdlc/evidence/311/negative-fixtures"

def invoke(path, env = {})
  Open3.capture3(env, "ruby", VALIDATOR.to_s, "matrix", "--matrix", path.to_s, chdir: ROOT.to_s)
end

def expect_failure(name, matrix, env = {})
  path = WORK / "#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  _stdout, _stderr, status = invoke(path, env)
  raise "#{name} unexpectedly passed" if status.success?
end

def clone(value)
  Marshal.load(Marshal.dump(value))
end

def retain(name, payload)
  path = WORK / name
  path.write(JSON.pretty_generate(payload) + "\n")
  { "path" => path.relative_path_from(ROOT).to_s, "sha256" => Digest::SHA256.file(path).hexdigest }
end

FileUtils.rm_rf(WORK)
FileUtils.mkdir_p(WORK)
base = JSON.parse(MATRIX.read)
stdout, stderr, status = invoke(MATRIX)
raise "positive matrix failed: #{stdout} #{stderr}" unless status.success?

tampered = clone(base); tampered["rows"].shift
expect_failure("missing-row", tampered)
tampered = clone(base); tampered["rows"] << clone(tampered["rows"].first)
expect_failure("duplicate-row", tampered)
tampered = clone(base); tampered["rows"] << { "id" => "feature:INVENTED", "kind" => "feature", "source" => "invented", "owner" => "none", "disposition" => "blocked", "claim_boundary" => "none", "blockers" => ["invented"] }
expect_failure("extra-row", tampered)
tampered = clone(base); tampered["evaluation_base_sha"] = "0" * 40
expect_failure("stale-head", tampered)
tampered = clone(base); tampered["rows"].first["disposition"] = "planned"
expect_failure("planned-disposition", tampered)
tampered = clone(base); tampered["rows"].first["blockers"] = []
expect_failure("blockerless-row", tampered)
tampered = clone(base); row = tampered["rows"].first; row["disposition"] = "accepted"; row["blockers"] = []; row["evidence"] = { "authority_kind" => "self_asserted_json" }
expect_failure("self-attested-accepted", tampered)

head = `git -C #{ROOT} rev-parse HEAD`.strip
issue = 309
pr = 460
terminal_digest = "3" * 64
terminal_payload = { "issue" => issue, "pull_request" => pr, "head_sha" => head, "merge_sha" => head, "canonical_generation" => 1, "canonical_digest" => terminal_digest }
terminal_cache = retain("terminal.json", { "schema" => "csdlc.derived_terminal_validation.v1", "canonical_match" => true, "terminal" => terminal_payload })
review = retain("review.json", { "schema" => "adl.v0.92.quality_gate_review.v1", "result" => "passed", "repository" => "agent-logic/agent-design-language", "issue" => issue, "pull_request" => pr, "reviewed_head" => head, "findings" => [] })
proofs = %w[positive negative integration platform].to_h do |klass|
  [klass, retain("#{klass}.json", { "schema" => "adl.v0.92.quality_gate_proof.v1", "class" => klass, "result" => "passed", "revision" => head })]
end
gh_response = WORK / "gh.json"
required_checks = %w[adl-ci adl-coverage adl-coverage-hosted adl-coverage-runtime-hosted adl-coverage-workspace-hosted adl-tooling-contracts adl-rust-fmt-clippy adl-rust-tests adl-path-policy]
gh_response.write(JSON.generate({ "number" => pr, "state" => "MERGED", "headRefOid" => head, "mergeCommit" => { "oid" => head }, "closingIssuesReferences" => [{ "number" => issue }], "statusCheckRollup" => required_checks.map { |name| { "name" => name, "conclusion" => "SUCCESS" } } }))
terminal_response = WORK / "terminal-response.json"
terminal_response.write(JSON.generate({ "schema" => "csdlc.derived_terminal_validation.v1", "canonical_match" => true, "terminal" => terminal_payload }))
bin_dir = WORK / "bin"
FileUtils.mkdir_p(bin_dir)
(bin_dir / "csdlc-install").write("#!/bin/sh\nprintf '\"v2\"\\n'\n")
(bin_dir / "csdlc-finish").write("#!/bin/sh\ncat \"$QUALITY_GATE_TERMINAL_RESPONSE\"\n")
fake_gh = WORK / "gh"
fake_gh.write("#!/bin/sh\ncat \"$QUALITY_GATE_GH_RESPONSE\"\n")
[bin_dir / "csdlc-install", bin_dir / "csdlc-finish", fake_gh].each { |path| FileUtils.chmod(0o700, path) }
env = { "CSDLC_V2_BIN_DIR" => bin_dir.to_s, "QUALITY_GATE_GH_BIN" => fake_gh.to_s, "QUALITY_GATE_GH_RESPONSE" => gh_response.to_s, "QUALITY_GATE_TERMINAL_RESPONSE" => terminal_response.to_s }

accepted = clone(base)
row = accepted["rows"].first
row["disposition"] = "accepted"
row["blockers"] = []
row["evidence"] = {
  "authority_kind" => "canonical_observation", "repository" => "agent-logic/agent-design-language", "issue" => issue,
  "implementation_paths" => ["adl/src/lib.rs"], "reviewed_head" => head, "pull_request" => pr, "merge_sha" => head,
  "positive" => proofs["positive"], "negative" => proofs["negative"], "integration" => proofs["integration"], "platform" => proofs["platform"],
  "typed_terminal" => { "generation" => 1, "digest" => terminal_digest, "cache" => terminal_cache },
  "review_artifact" => review, "required_checks" => required_checks
}
accepted_path = WORK / "accepted.json"
accepted_path.write(JSON.pretty_generate(accepted) + "\n")
accepted_stdout, accepted_stderr, accepted_status = invoke(accepted_path, env)
raise "canonical accepted fixture failed: #{accepted_stdout} #{accepted_stderr}" unless accepted_status.success?

cases = {
  "cross-repository-substitution" => ->(m) { m["rows"].first["evidence"]["repository"] = "danielbaustin/agent-design-language" },
  "stale-reviewed-head" => ->(m) { m["rows"].first["evidence"]["reviewed_head"] = `git -C #{ROOT} rev-parse HEAD^`.strip },
  "non-ancestral-merge" => ->(m) { m["rows"].first["evidence"]["merge_sha"] = "1" * 40 },
  "fabricated-check" => ->(m) { m["rows"].first["evidence"]["required_checks"] = ["fabricated"] },
  "malformed-terminal-cache" => ->(m) { m["rows"].first["evidence"]["typed_terminal"]["cache"] = retain("malformed-cache.json", { "canonical_match" => false }) },
  "terminal-cache-digest-mismatch" => ->(m) { m["rows"].first["evidence"]["typed_terminal"]["cache"]["sha256"] = "0" * 64 },
  "missing-platform-proof" => ->(m) { m["rows"].first["evidence"]["platform"] = {} },
  "fixture-authority" => ->(m) { m["rows"].first["evidence"]["authority_kind"] = "fixture" },
  "receipt-only-authority" => ->(m) { m["rows"].first["evidence"]["authority_kind"] = "receipt_only" },
  "demo-authority" => ->(m) { m["rows"].first["evidence"]["authority_kind"] = "demo" },
  "synthetic-authority" => ->(m) { m["rows"].first["evidence"]["authority_kind"] = "synthetic" },
  "substituted-provider-authority" => ->(m) { m["rows"].first["evidence"]["authority_kind"] = "substituted_provider" },
  "review-artifact-digest-mismatch" => ->(m) { m["rows"].first["evidence"]["review_artifact"]["sha256"] = "0" * 64 },
  "implementation-path-missing-at-review" => ->(m) { m["rows"].first["evidence"]["implementation_paths"] = ["adl/src/not-real.rs"] }
}
cases.each do |name, mutate|
  tampered = clone(accepted)
  mutate.call(tampered)
  expect_failure(name, tampered, env)
end

FileUtils.rm_rf(WORK)
puts JSON.generate(schema: "adl.v0.92.quality_gate_negative_suite.v1", status: "passed", cases: 21)
