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

def git(*argv)
  stdout, stderr, status = Open3.capture3("git", "-C", ROOT.to_s, *argv)
  raise stderr unless status.success?
  stdout
end

def invoke(path, env = {})
  Open3.capture3(env, "ruby", VALIDATOR.to_s, "matrix", "--matrix", path.to_s, chdir: ROOT.to_s)
end

def expect_failure(name, matrix, expected)
  path = WORK / "#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  stdout, stderr, status = invoke(path)
  raise "#{name} unexpectedly passed" if status.success?
  raise "#{name} did not prove #{expected}: #{stdout} #{stderr}" unless stderr.include?(expected)
end

def expect_observation_failure(name, payload, expected)
  path = WORK / "observation-#{name}.json"
  path.write(JSON.pretty_generate(payload) + "\n")
  stdout, stderr, status = Open3.capture3("ruby", VALIDATOR.to_s, "observation", "--input", path.to_s, chdir: ROOT.to_s)
  raise "observation #{name} unexpectedly passed" if status.success?
  raise "observation #{name} did not prove #{expected}: #{stdout} #{stderr}" unless stderr.include?(expected)
end

def clone(value)
  Marshal.load(Marshal.dump(value))
end

def retain(name, payload)
  path = WORK / name
  path.write(JSON.pretty_generate(payload) + "\n")
  { "path" => path.relative_path_from(ROOT).to_s, "sha256" => Digest::SHA256.file(path).hexdigest }
end

def blob_ref(commit, path)
  bytes = git("show", "#{commit}:#{path}")
  { "path" => path, "sha256" => Digest::SHA256.hexdigest(bytes) }
end

FileUtils.rm_rf(WORK)
FileUtils.mkdir_p(WORK)
base = JSON.parse(MATRIX.read)
stdout, stderr, status = invoke(MATRIX)
raise "blocked matrix failed: #{stdout} #{stderr}" unless status.success?

tampered = clone(base); tampered["rows"].shift
expect_failure("missing-row", tampered, "denominator_missing")
tampered = clone(base); tampered["rows"] << clone(tampered["rows"].first)
expect_failure("duplicate-row", tampered, "denominator_duplicate")
tampered = clone(base); tampered["rows"] << { "id" => "feature:INVENTED", "kind" => "feature", "source" => "invented", "owner" => "none", "disposition" => "blocked", "claim_boundary" => "none", "blockers" => ["invented"] }
expect_failure("extra-row", tampered, "denominator_extra")
tampered = clone(base); tampered["evaluation_base_sha"] = "0" * 40
expect_failure("stale-head", tampered, "evaluation_base_not_ancestral")
tampered = clone(base); tampered["rows"].first["disposition"] = "planned"
expect_failure("planned-disposition", tampered, "disposition_invalid")
tampered = clone(base); tampered["rows"].first["blockers"] = []
expect_failure("blockerless-row", tampered, "blocked_without_reason")
tampered = clone(base); row = tampered["rows"].first; row["disposition"] = "accepted"; row["blockers"] = []; row["evidence"] = { "authority_kind" => "self_asserted_json" }
expect_failure("self-attested-accepted", tampered, "repository_missing")

# Canonical positive control: terminal #451 / merged PR #459. This consumes the
# stable C-SDLC owner, exact reviewed/PR/merge Git identities, typed issue index
# and SOR, retained reviewed evidence, live GitHub closing linkage/check runs,
# and the active main-protection ruleset. No fake executable or response hook is
# available to the production validator.
issue = 451
pr = 459
reviewed_head = "3c612a0c302d1a34562b9e0c160b12aca91222e3"
pr_head = "414777b543bf5df295a41eacc9c4fd19735c413b"
merge_sha = "e926e3bca0ab1981d77b4658d2feb4059bdf33a6"
common = Pathname.new(git("rev-parse", "--git-common-dir").strip).realpath
bin_dir = common.parent / ".adl/bin/csdlc-v2"
terminal_stdout, terminal_stderr, terminal_status = Open3.capture3((bin_dir / "csdlc-finish").to_s, "--root", ROOT.to_s, "--validate-cached-issue", issue.to_s)
raise "canonical terminal unavailable: #{terminal_stderr}" unless terminal_status.success?
terminal_receipt = JSON.parse(terminal_stdout)
terminal = terminal_receipt.fetch("terminal")
terminal_ref = retain("terminal-451.json", terminal_receipt)

proof_specs = {
  "positive" => [".csdlc/evidence/451/production_birthday_kernel.log", 1],
  "negative" => [".csdlc/evidence/451/retained_evidence_contract.log", 3],
  "integration" => [".csdlc/evidence/451/production_birthday_resident_path.log", 2],
  "platform" => [".csdlc/evidence/451/runtime_feature_wiring_audit.log", 4]
}
proofs = proof_specs.to_h do |klass, (path, index)|
  [klass, blob_ref(pr_head, path).merge("validation_index" => index)]
end

accepted = clone(base)
row = accepted["rows"].first
row["disposition"] = "accepted"
row["blockers"] = []
row["evidence"] = {
  "authority_kind" => "canonical_observation", "repository" => "agent-logic/agent-design-language", "issue" => issue,
  "implementation_paths" => ["adl/src/production_birthday.rs"], "reviewed_head" => reviewed_head,
  "pr_head" => pr_head, "pull_request" => pr, "merge_sha" => merge_sha,
  "positive" => proofs["positive"], "negative" => proofs["negative"], "integration" => proofs["integration"], "platform" => proofs["platform"],
  "typed_terminal" => { "generation" => terminal["canonical_generation"], "digest" => terminal["canonical_digest"], "cache" => terminal_ref },
  "review_artifact" => blob_ref(pr_head, ".csdlc/issues/451/index.json"),
  "required_checks" => ["adl-ci", "adl-coverage"]
}
accepted_path = WORK / "canonical-accepted.json"
accepted_path.write(JSON.pretty_generate(accepted) + "\n")
accepted_stdout, accepted_stderr, accepted_status = invoke(accepted_path, { "CSDLC_V2_BIN_DIR" => "/nonexistent", "QUALITY_GATE_GH_BIN" => "/nonexistent" })
raise "canonical accepted control failed: #{accepted_stdout} #{accepted_stderr}" unless accepted_status.success?

cases = {
  "cross-repository-substitution" => ["repository_mismatch", ->(m) { m["rows"].first["evidence"]["repository"] = "danielbaustin/agent-design-language" }],
  "stale-reviewed-head" => ["review_revision_invalid", ->(m) { m["rows"].first["evidence"]["reviewed_head"] = git("rev-parse", "#{reviewed_head}^").strip }],
  "non-ancestral-pr-head" => ["reviewed_head_not_in_pr_head", ->(m) { m["rows"].first["evidence"]["pr_head"] = git("rev-parse", "#{reviewed_head}^").strip }],
  "wrong-merge" => ["typed_terminal:merge_sha_mismatch", ->(m) { m["rows"].first["evidence"]["merge_sha"] = reviewed_head }],
  "self-selected-checks" => ["required_checks_not_canonical", ->(m) { m["rows"].first["evidence"]["required_checks"] = ["adl-ci"] }],
  "terminal-generation" => ["typed_terminal:canonical_generation_mismatch", ->(m) { m["rows"].first["evidence"]["typed_terminal"]["generation"] += 1 }],
  "terminal-digest" => ["typed_terminal:canonical_digest_mismatch", ->(m) { m["rows"].first["evidence"]["typed_terminal"]["digest"] = "0" * 64 }],
  "malformed-terminal-cache" => ["typed_terminal:issue_mismatch", ->(m) { m["rows"].first["evidence"]["typed_terminal"]["cache"] = retain("malformed-cache.json", { "canonical_match" => false, "terminal" => {} }) }],
  "terminal-cache-digest" => ["typed_terminal_cache:digest_mismatch", ->(m) { m["rows"].first["evidence"]["typed_terminal"]["cache"]["sha256"] = "0" * 64 }],
  "missing-platform-proof" => ["platform_missing", ->(m) { m["rows"].first["evidence"]["platform"] = {} }],
  "review-digest" => ["review_artifact:candidate_digest_mismatch", ->(m) { m["rows"].first["evidence"]["review_artifact"]["sha256"] = "0" * 64 }],
  "review-content" => ["review_artifact_path_mismatch", ->(m) { m["rows"].first["evidence"]["review_artifact"] = blob_ref(pr_head, ".csdlc/issues/451/cards/sor.values.json") }],
  "implementation-path" => ["git_identity_unresolvable", ->(m) { m["rows"].first["evidence"]["implementation_paths"] = ["adl/src/not-real.rs"] }],
  "closing-link" => ["github_closing_link_missing", ->(m) { m["rows"].first["evidence"]["issue"] = 450 }],
  "wrong-pr" => ["github_pr_head_mismatch", ->(m) { m["rows"].first["evidence"]["pull_request"] = 458 }],
  "positive-proof-digest" => ["positive:candidate_digest_mismatch", ->(m) { m["rows"].first["evidence"]["positive"]["sha256"] = "0" * 64 }],
  "negative-proof-semantic" => ["negative:evidence_ref_mismatch", ->(m) { m["rows"].first["evidence"]["negative"]["validation_index"] = 1 }],
  "integration-proof-digest" => ["integration:candidate_digest_mismatch", ->(m) { m["rows"].first["evidence"]["integration"]["sha256"] = "0" * 64 }],
  "platform-proof-semantic" => ["platform:evidence_ref_mismatch", ->(m) { m["rows"].first["evidence"]["platform"]["validation_index"] = 1 }],
  "fixture-authority" => ["prohibited_authority:fixture", ->(m) { m["rows"].first["evidence"]["authority_kind"] = "fixture" }],
  "receipt-only-authority" => ["prohibited_authority:receipt_only", ->(m) { m["rows"].first["evidence"]["authority_kind"] = "receipt_only" }],
  "demo-authority" => ["prohibited_authority:demo", ->(m) { m["rows"].first["evidence"]["authority_kind"] = "demo" }],
  "synthetic-authority" => ["prohibited_authority:synthetic", ->(m) { m["rows"].first["evidence"]["authority_kind"] = "synthetic" }],
  "substituted-provider-authority" => ["prohibited_authority:substituted_provider", ->(m) { m["rows"].first["evidence"]["authority_kind"] = "substituted_provider" }]
}
cases.each do |name, (expected, mutate)|
  tampered = clone(accepted)
  mutate.call(tampered)
  expect_failure(name, tampered, expected)
end

observation = {
  "evidence" => { "issue" => issue, "pr_head" => pr_head, "merge_sha" => merge_sha, "required_checks" => ["adl-ci", "adl-coverage"] },
  "pull" => { "state" => "MERGED", "merged" => true, "baseRefName" => "main", "headRefOid" => pr_head,
              "mergeCommit" => { "oid" => merge_sha },
              "closingIssuesReferences" => { "nodes" => [{ "number" => issue, "repository" => { "nameWithOwner" => "agent-logic/agent-design-language" } }] } },
  "checks" => { "check_runs" => [{ "name" => "adl-ci", "conclusion" => "success" }, { "name" => "adl-coverage", "conclusion" => "success" }] },
  "ruleset" => { "name" => "main-protection", "enforcement" => "active", "target" => "branch",
                 "conditions" => { "ref_name" => { "include" => ["~DEFAULT_BRANCH"] } },
                 "rules" => [{ "type" => "required_status_checks", "parameters" => { "required_status_checks" => [{ "context" => "adl-ci" }, { "context" => "adl-coverage" }] } }] }
}
path = WORK / "observation-valid.json"
path.write(JSON.pretty_generate(observation) + "\n")
observation_stdout, observation_stderr, observation_status = Open3.capture3("ruby", VALIDATOR.to_s, "observation", "--input", path.to_s, chdir: ROOT.to_s)
raise "observation control failed: #{observation_stdout} #{observation_stderr}" unless observation_status.success?

tampered = clone(observation); tampered["checks"]["check_runs"].find { |item| item["name"] == "adl-coverage" }["conclusion"] = "failure"
expect_observation_failure("failed-required-check", tampered, "required_check_not_successful:adl-coverage")
tampered = clone(observation); tampered["pull"]["baseRefName"] = "feature"
expect_observation_failure("wrong-base-branch", tampered, "github_base_branch_mismatch")
tampered = clone(observation); tampered["pull"]["closingIssuesReferences"]["nodes"] = []
expect_observation_failure("missing-closing-link", tampered, "github_closing_link_missing")
tampered = clone(observation); tampered["ruleset"]["enforcement"] = "disabled"
expect_observation_failure("inactive-ruleset", tampered, "ruleset_authority_invalid")
tampered = clone(observation); tampered["ruleset"]["rules"].first["parameters"]["required_status_checks"].pop
expect_observation_failure("ruleset-check-omission", tampered, "required_checks_not_canonical")

FileUtils.rm_rf(WORK)
puts JSON.generate(schema: "adl.v0.92.quality_gate_negative_suite.v2", status: "passed", cases: 36,
                   canonical_control: { issue: issue, pull_request: pr, reviewed_head: reviewed_head, pr_head: pr_head, merge_sha: merge_sha },
                   authority_substitution_ignored: true)
