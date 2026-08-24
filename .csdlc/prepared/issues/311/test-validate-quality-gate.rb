#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"

TEST_ROOT = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
VALIDATOR = TEST_ROOT / ".csdlc/prepared/issues/311/validate-quality-gate.rb"
MATRIX = TEST_ROOT / "docs/reviews/v0.92/quality-gate-311/feature-completion-matrix.json"
WORK = TEST_ROOT / ".csdlc/evidence/311/negative-fixtures"
load VALIDATOR

def git(*argv)
  stdout, stderr, status = Open3.capture3("git", "-C", TEST_ROOT.to_s, *argv)
  raise stderr unless status.success?
  stdout
end

def invoke(path, env = {})
  fixture = <<~'RUBY'
    require "json"
    require "pathname"
    load ARGV.shift
    _matrix, errors = validate_matrix(Pathname.new(ARGV.shift), canonical: false)
    if errors.empty?
      puts JSON.generate(schema: "adl.v0.92.quality_gate_fixture_validation.v1", status: "passed")
    else
      warn JSON.generate(schema: "adl.v0.92.quality_gate_fixture_validation.v1", status: "failed", errors: errors)
      exit 1
    end
  RUBY
  Open3.capture3(env, "ruby", "-e", fixture, VALIDATOR.to_s, path.to_s, chdir: TEST_ROOT.to_s)
end

def expect_failure(name, matrix, expected)
  path = WORK / "#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  stdout, stderr, status = invoke(path)
  raise "#{name} unexpectedly passed" if status.success?
  raise "#{name} did not prove #{expected}: #{stdout} #{stderr}" unless stderr.include?(expected)
end

def expect_success(name, matrix, env = {})
  path = WORK / "#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  stdout, stderr, status = invoke(path, env)
  raise "#{name} failed: #{stdout} #{stderr}" unless status.success?
end

def expect_canonical_failure(name, matrix, expected)
  path = WORK / "canonical-#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  _validated, errors = validate_matrix(path, canonical: true)
  raise "canonical #{name} unexpectedly passed" if errors.empty?
  raise "canonical #{name} did not prove #{expected}: #{errors.inspect}" unless errors.any? { |error| error.include?(expected) }
end

def clone(value)
  Marshal.load(Marshal.dump(value))
end

def bind_row_contract(row, reviewed_head, validations)
  source_bytes = git("show", "#{reviewed_head}:#{row.fetch('source')}")
  bindings = %w[positive negative integration platform].map do |proof_class|
    proof = row.fetch("evidence").fetch(proof_class)
    lane = validations.fetch(proof.fetch("validation_index"))
    {
      "class" => proof_class, "path" => proof["path"], "sha256" => proof["sha256"],
      "validation_index" => proof["validation_index"], "command" => lane["command"],
      "purpose" => lane["purpose"], "evidence_ref" => lane["evidence_ref"],
      "semantic_observation" => proof_semantic_observation(row.fetch("id"), proof_class,
        git("show", "#{row.fetch('evidence').fetch('pr_head')}:#{proof.fetch('path')}"), row.fetch("evidence"))
    }
  end
  row["evidence"]["row_contract"] = {
    "schema" => "adl.v0.92.quality_gate_row_contract.v1", "row_id" => row["id"],
    "owner" => row["owner"], "source_path" => row["source"],
    "source_sha256" => Digest::SHA256.hexdigest(source_bytes), "issue" => row["evidence"]["issue"],
    "implementation_paths" => row["evidence"]["implementation_paths"].sort,
    "proof_binding_sha256" => Digest::SHA256.hexdigest(JSON.generate(bindings))
  }
end

def accepted_row(matrix)
  matrix.fetch("rows").find { |row| row["disposition"] == "accepted" }
end

def retain(name, payload)
  path = WORK / name
  path.write(JSON.pretty_generate(payload) + "\n")
  { "path" => path.relative_path_from(TEST_ROOT).to_s, "sha256" => Digest::SHA256.file(path).hexdigest }
end

def blob_ref(commit, path)
  bytes = git("show", "#{commit}:#{path}")
  { "path" => path, "sha256" => Digest::SHA256.hexdigest(bytes) }
end

FileUtils.rm_rf(WORK)
FileUtils.mkdir_p(WORK)
base = JSON.parse(MATRIX.read)
blocked_control = WORK / "blocked-control.json"
blocked_control.write(JSON.pretty_generate(base) + "\n")
stdout, stderr, status = invoke(blocked_control)
raise "blocked matrix failed: #{stdout} #{stderr}" unless status.success?

expect_success("git-dir-substitution", base, { "GIT_DIR" => "/nonexistent/substitute.git" })
expect_success("git-work-tree-substitution", base, { "GIT_WORK_TREE" => "/nonexistent/substitute" })
expect_success("git-object-substitution", base, { "GIT_OBJECT_DIRECTORY" => "/nonexistent/objects", "GIT_ALTERNATE_OBJECT_DIRECTORIES" => "/nonexistent/alternate" })
expect_success("git-config-substitution", base, { "GIT_CONFIG_COUNT" => "1", "GIT_CONFIG_KEY_0" => "remote.origin.url", "GIT_CONFIG_VALUE_0" => "https://github.com/attacker/substitute" })
shim_dir = WORK / "path-shim"
shim_dir.mkpath
shim = shim_dir / "git"
shim.write("#!/bin/sh\nexit 91\n")
shim.chmod(0o755)
expect_success("path-git-shim", base, { "PATH" => "#{shim_dir}:#{ENV.fetch('PATH')}" })

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
terminal_stdout, terminal_stderr, terminal_status = Open3.capture3((bin_dir / "csdlc-finish").to_s, "--root", TEST_ROOT.to_s, "--validate-cached-issue", issue.to_s)
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
row = accepted["rows"].find { |item| item["id"] == "feature:FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92" }
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
sor = JSON.parse(git("show", "#{pr_head}:.csdlc/issues/451/cards/sor.values.json"))
validations = sor.dig("content", "values", "actual_validation")
bind_row_contract(row, reviewed_head, validations)
accepted_path = WORK / "canonical-accepted.json"
accepted_path.write(JSON.pretty_generate(accepted) + "\n")
accepted_stdout, accepted_stderr, accepted_status = invoke(accepted_path, { "CSDLC_V2_BIN_DIR" => "/nonexistent", "QUALITY_GATE_GH_BIN" => "/nonexistent" })
raise "canonical accepted control failed: #{accepted_stdout} #{accepted_stderr}" unless accepted_status.success?
hostile_git_env = {
  "GIT_DIR" => "/nonexistent/substitute.git", "GIT_WORK_TREE" => "/nonexistent/substitute",
  "GIT_OBJECT_DIRECTORY" => "/nonexistent/objects", "GIT_ALTERNATE_OBJECT_DIRECTORIES" => "/nonexistent/alternate",
  "GIT_CONFIG_COUNT" => "1", "GIT_CONFIG_KEY_0" => "remote.origin.url", "GIT_CONFIG_VALUE_0" => "https://github.com/attacker/substitute",
  "PATH" => "#{shim_dir}:#{ENV.fetch('PATH')}",
  "HTTP_PROXY" => "http://127.0.0.1:9", "HTTPS_PROXY" => "http://127.0.0.1:9",
  "ALL_PROXY" => "http://127.0.0.1:9", "SSL_CERT_FILE" => "/nonexistent/hostile-ca.pem",
  "SSL_CERT_DIR" => "/nonexistent/hostile-ca-dir"
}
expect_success("accepted-authority-environment-isolated", accepted, hostile_git_env)

saved_authority_env = %w[HTTP_PROXY HTTPS_PROXY ALL_PROXY SSL_CERT_FILE SSL_CERT_DIR].to_h { |key| [key, ENV[key]] }
begin
  ENV.update("HTTP_PROXY" => "http://127.0.0.1:9", "HTTPS_PROXY" => "http://127.0.0.1:9",
             "ALL_PROXY" => "http://127.0.0.1:9", "SSL_CERT_FILE" => "/nonexistent/hostile-ca.pem",
             "SSL_CERT_DIR" => "/nonexistent/hostile-ca-dir")
  authority_http = github_http(URI("https://api.github.com/repos/#{REPOSITORY}"))
  raise "github authority inherited proxy" if authority_http.proxy?
  raise "github authority did not require peer verification" unless authority_http.verify_mode == OpenSSL::SSL::VERIFY_PEER
  raise "github authority did not pin a system trust store" unless authority_http.cert_store.is_a?(OpenSSL::X509::Store)
ensure
  saved_authority_env.each { |key, value| value.nil? ? ENV.delete(key) : ENV[key] = value }
end

tampered = clone(base); tampered["candidate_source_sha"] = reviewed_head; tampered["candidate_source_tree"] = git("rev-parse", "#{reviewed_head}^{tree}").strip
expect_canonical_failure("alternate-ancestral-candidate", tampered, "candidate_source_sha_mismatch")
tampered = clone(base); tampered["candidate_source_sha"] = git("rev-parse", "HEAD").strip; tampered["candidate_source_tree"] = git("rev-parse", "HEAD^{tree}").strip
expect_canonical_failure("later-nominated-candidate", tampered, "candidate_source_sha_mismatch")
tampered = clone(base); tampered["candidate_source_tree"] = "0" * 40
expect_canonical_failure("candidate-tree-mismatch", tampered, "candidate_source_tree_mismatch")

tampered = clone(accepted)
birthday = tampered["rows"].find { |item| item["id"] == "feature:FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92" }
unrelated = tampered["rows"].find { |item| item["id"] == "feature:ACP_COGNITIVE_PROFILES_v0.92" }
unrelated["disposition"] = "accepted"
unrelated["blockers"] = []
unrelated["evidence"] = clone(birthday["evidence"])
birthday["disposition"] = "blocked"
birthday["blockers"] = ["accepted_evidence_packet_missing"]
birthday["evidence"] = {}
expect_failure("unrelated-row-evidence", tampered, "source_outside_review_scope")

tampered = clone(accepted); accepted_row(tampered)["owner"] = "WP-INVENTED"
expect_failure("owner-mismatch", tampered, "owner_mismatch")
tampered = clone(accepted); accepted_row(tampered)["evidence"]["row_contract"]["source_sha256"] = "0" * 64
expect_failure("reviewed-source-contract", tampered, "row_contract_mismatch")
tampered = clone(accepted); accepted_row(tampered)["evidence"]["implementation_paths"] = ["adl/src/long_lived_agent.rs"]
expect_failure("implementation-contract", tampered, "row_contract_unresolvable")
tampered = clone(accepted); accepted_row(tampered)["evidence"]["positive"] = clone(accepted_row(tampered)["evidence"]["integration"])
expect_failure("proof-content-contract", tampered, "row_contract_unresolvable")
tampered = clone(accepted)
tampered_positive = blob_ref(pr_head, ".csdlc/evidence/451/diff_hygiene.log").merge("validation_index" => 0)
accepted_row(tampered)["evidence"]["positive"] = tampered_positive
expect_failure("nonsemantic-positive-proof", tampered, "positive:semantic_proof_invalid")
tampered = clone(accepted)
duplicate = tampered["rows"].find { |item| item["id"] == "critical:AEE-008" }
duplicate["disposition"] = "accepted"; duplicate["blockers"] = []; duplicate["evidence"] = clone(accepted_row(tampered)["evidence"])
expect_failure("duplicate-accepted-packet", tampered, "accepted_packet_semantics_unresolvable")

cases = {
  "cross-repository-substitution" => ["repository_mismatch", ->(m) { accepted_row(m)["evidence"]["repository"] = "danielbaustin/agent-design-language" }],
  "stale-reviewed-head" => ["review_revision_invalid", ->(m) { accepted_row(m)["evidence"]["reviewed_head"] = git("rev-parse", "#{reviewed_head}^").strip }],
  "non-ancestral-pr-head" => ["reviewed_head_not_in_pr_head", ->(m) { accepted_row(m)["evidence"]["pr_head"] = git("rev-parse", "#{reviewed_head}^").strip }],
  "wrong-merge" => ["typed_terminal:merge_sha_mismatch", ->(m) { accepted_row(m)["evidence"]["merge_sha"] = reviewed_head }],
  "self-selected-checks" => ["required_checks_not_canonical", ->(m) { accepted_row(m)["evidence"]["required_checks"] = ["adl-ci"] }],
  "terminal-generation" => ["typed_terminal:canonical_generation_mismatch", ->(m) { accepted_row(m)["evidence"]["typed_terminal"]["generation"] += 1 }],
  "terminal-digest" => ["typed_terminal:canonical_digest_mismatch", ->(m) { accepted_row(m)["evidence"]["typed_terminal"]["digest"] = "0" * 64 }],
  "malformed-terminal-cache" => ["typed_terminal:issue_mismatch", ->(m) { accepted_row(m)["evidence"]["typed_terminal"]["cache"] = retain("malformed-cache.json", { "canonical_match" => false, "terminal" => {} }) }],
  "terminal-cache-digest" => ["typed_terminal_cache:digest_mismatch", ->(m) { accepted_row(m)["evidence"]["typed_terminal"]["cache"]["sha256"] = "0" * 64 }],
  "missing-platform-proof" => ["platform_missing", ->(m) { accepted_row(m)["evidence"]["platform"] = {} }],
  "review-digest" => ["review_artifact:candidate_digest_mismatch", ->(m) { accepted_row(m)["evidence"]["review_artifact"]["sha256"] = "0" * 64 }],
  "review-content" => ["review_artifact_path_mismatch", ->(m) { accepted_row(m)["evidence"]["review_artifact"] = blob_ref(pr_head, ".csdlc/issues/451/cards/sor.values.json") }],
  "implementation-path" => ["git_identity_unresolvable", ->(m) { accepted_row(m)["evidence"]["implementation_paths"] = ["adl/src/not-real.rs"] }],
  "closing-link" => ["github_closing_link_missing", ->(m) { accepted_row(m)["evidence"]["issue"] = 450 }],
  "wrong-pr" => ["github_pr_head_mismatch", ->(m) { accepted_row(m)["evidence"]["pull_request"] = 458 }],
  "positive-proof-digest" => ["positive:candidate_digest_mismatch", ->(m) { accepted_row(m)["evidence"]["positive"]["sha256"] = "0" * 64 }],
  "negative-proof-semantic" => ["negative:evidence_ref_mismatch", ->(m) { accepted_row(m)["evidence"]["negative"]["validation_index"] = 1 }],
  "integration-proof-digest" => ["integration:candidate_digest_mismatch", ->(m) { accepted_row(m)["evidence"]["integration"]["sha256"] = "0" * 64 }],
  "platform-proof-semantic" => ["platform:evidence_ref_mismatch", ->(m) { accepted_row(m)["evidence"]["platform"]["validation_index"] = 1 }],
  "fixture-authority" => ["prohibited_authority:fixture", ->(m) { accepted_row(m)["evidence"]["authority_kind"] = "fixture" }],
  "receipt-only-authority" => ["prohibited_authority:receipt_only", ->(m) { accepted_row(m)["evidence"]["authority_kind"] = "receipt_only" }],
  "demo-authority" => ["prohibited_authority:demo", ->(m) { accepted_row(m)["evidence"]["authority_kind"] = "demo" }],
  "synthetic-authority" => ["prohibited_authority:synthetic", ->(m) { accepted_row(m)["evidence"]["authority_kind"] = "synthetic" }],
  "substituted-provider-authority" => ["prohibited_authority:substituted_provider", ->(m) { accepted_row(m)["evidence"]["authority_kind"] = "substituted_provider" }]
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
  "issue" => { "number" => issue, "state" => "closed", "state_reason" => "completed" },
  "checks" => { "check_runs" => [{ "id" => 10, "name" => "adl-ci", "conclusion" => "success", "completed_at" => "2026-08-24T10:00:00Z", "app" => { "id" => 15_368 } }, { "id" => 11, "name" => "adl-coverage", "conclusion" => "success", "completed_at" => "2026-08-24T10:00:00Z", "app" => { "id" => 15_368 } }] },
  "rulesets" => [{ "name" => "main-protection", "enforcement" => "active", "target" => "branch",
                 "conditions" => { "ref_name" => { "include" => ["~DEFAULT_BRANCH"] } },
                 "rules" => [{ "type" => "required_status_checks", "parameters" => { "required_status_checks" => [{ "context" => "adl-ci", "integration_id" => 15_368 }, { "context" => "adl-coverage", "integration_id" => 15_368 }] } }] }]
}

def validate_observation(payload)
  errors = []
  validate_live_authority(payload.fetch("evidence"), payload.fetch("pull"), payload.fetch("issue"), payload.fetch("checks"), payload.fetch("rulesets"), "observation", errors)
  errors
end

def expect_live_failure(name, payload, expected)
  errors = validate_observation(payload)
  raise "observation #{name} unexpectedly passed" if errors.empty?
  raise "observation #{name} did not prove #{expected}: #{errors.inspect}" unless errors.any? { |error| error.include?(expected) }
end

raise "observation control failed: #{validate_observation(observation).inspect}" unless validate_observation(observation).empty?

tampered = clone(observation); tampered["checks"]["check_runs"].find { |item| item["name"] == "adl-coverage" }["conclusion"] = "failure"
expect_live_failure("failed-required-check", tampered, "required_check_not_successful:adl-coverage")
tampered = clone(observation); tampered["pull"]["baseRefName"] = "feature"
expect_live_failure("wrong-base-branch", tampered, "github_base_branch_mismatch")
tampered = clone(observation); tampered["pull"]["closingIssuesReferences"]["nodes"] = []
expect_live_failure("missing-closing-link", tampered, "github_closing_link_missing")
tampered = clone(observation); tampered["rulesets"].first["enforcement"] = "disabled"
expect_live_failure("inactive-ruleset", tampered, "ruleset_authority_invalid")
tampered = clone(observation); tampered["rulesets"].first["rules"].first["parameters"]["required_status_checks"].pop
expect_live_failure("ruleset-check-omission", tampered, "required_checks_not_canonical")
tampered = clone(observation); tampered["issue"]["state"] = "open"; tampered["issue"]["state_reason"] = "reopened"
expect_live_failure("reopened-issue", tampered, "github_issue_not_closed")
tampered = clone(observation); tampered["checks"]["check_runs"].find { |item| item["name"] == "adl-ci" }["app"]["id"] = 1
expect_live_failure("wrong-check-app", tampered, "required_check_not_successful:adl-ci")
tampered = clone(observation); tampered["checks"]["check_runs"] << { "id" => 99, "name" => "adl-ci", "conclusion" => "failure", "completed_at" => "2026-08-24T10:01:00Z", "app" => { "id" => 15_368 } }
expect_live_failure("newer-failed-duplicate", tampered, "required_check_not_successful:adl-ci")
tampered = clone(observation); tampered["checks"]["check_runs"] << { "id" => 99, "name" => "adl-ci", "conclusion" => "success", "completed_at" => "2026-08-24T10:00:00Z", "app" => { "id" => 15_368 } }
expect_live_failure("tied-successes", tampered, "required_check_latest_ambiguous:adl-ci")
tampered = clone(observation); tampered["checks"]["check_runs"].find { |item| item["name"] == "adl-ci" }["conclusion"] = "failure"; tampered["checks"]["check_runs"] << { "id" => 99, "name" => "adl-ci", "conclusion" => "failure", "completed_at" => "2026-08-24T10:00:00Z", "app" => { "id" => 15_368 } }
expect_live_failure("tied-failures", tampered, "required_check_latest_ambiguous:adl-ci")
tampered = clone(observation); tampered["checks"]["check_runs"] << { "id" => 99, "name" => "adl-ci", "conclusion" => "failure", "completed_at" => "2026-08-24T10:00:00Z", "app" => { "id" => 15_368 } }
expect_live_failure("tied-mixed", tampered, "required_check_latest_ambiguous:adl-ci")
tampered = clone(observation); tampered["rulesets"].first["rules"].first["parameters"]["required_status_checks"].first.delete("integration_id")
expect_live_failure("missing-check-integration", tampered, "required_check_integration_missing:adl-ci")
tampered = clone(observation)
tampered["rulesets"] << { "name" => "release-safety", "enforcement" => "active", "target" => "branch",
                          "conditions" => { "ref_name" => { "include" => ["refs/heads/main"] } },
                          "rules" => [{ "type" => "required_status_checks", "parameters" => { "required_status_checks" => [{ "context" => "release-proof", "integration_id" => 15_368 }] } }] }
expect_live_failure("second-applicable-ruleset", tampered, "required_checks_not_canonical")
tampered = clone(observation)
tampered["rulesets"] << { "name" => "wildcard-safety", "enforcement" => "active", "target" => "branch",
                          "conditions" => { "ref_name" => { "include" => ["refs/heads/ma*"] } },
                          "rules" => [{ "type" => "required_status_checks", "parameters" => { "required_status_checks" => [{ "context" => "wildcard-proof", "integration_id" => 15_368 }] } }] }
expect_live_failure("wildcard-applicable-ruleset", tampered, "required_checks_not_canonical")
tampered = clone(observation)
tampered["rulesets"] << { "name" => "excluded-wildcard", "enforcement" => "active", "target" => "branch",
                          "conditions" => { "ref_name" => { "include" => ["refs/heads/**"], "exclude" => ["refs/heads/main"] } },
                          "rules" => [{ "type" => "required_status_checks", "parameters" => { "required_status_checks" => [{ "context" => "excluded-proof", "integration_id" => 15_368 }] } }] }
raise "excluded ruleset control failed: #{validate_observation(tampered).inspect}" unless validate_observation(tampered).empty?
tampered = clone(observation); tampered["rulesets"].first["conditions"]["ref_name"]["include"] = ["~UNSUPPORTED"]
expect_live_failure("unsupported-ruleset-pattern", tampered, "ruleset_ref_pattern_unsupported")

wp21a_errors = []
validate_wp21a_observation({ "merge_sha" => "unused" }, reviewed_head, reviewed_head, [], wp21a_errors)
raise "main-only ancestry negative did not fail closed: #{wp21a_errors.inspect}" unless wp21a_errors.include?("wp21a_merge_not_on_live_main")
wp21a_errors = []
validate_wp21a_observation({ "merge_sha" => "unused" }, merge_sha, merge_sha,
                           ["worktree /renamed/path\nHEAD #{WP21A_HEAD}\nbranch refs/heads/codex/310-rust-refactoring\n"], wp21a_errors)
raise "renamed worktree negative did not fail closed: #{wp21a_errors.inspect}" unless wp21a_errors.include?("wp21a_worktree_not_cleaned")

wp21a_issue = { "number" => 310, "state" => "closed", "state_reason" => "completed" }
wp21a_pull = {
  "number" => 465, "state" => "MERGED", "merged" => true, "baseRefName" => "main",
  "headRefOid" => WP21A_HEAD, "mergeCommit" => { "oid" => WP21A_MERGE },
  "closingIssuesReferences" => { "nodes" => [{ "number" => 310, "repository" => { "nameWithOwner" => REPOSITORY } }] }
}
raise "wp21a live authority control failed" unless (control = []; validate_wp21a_live_authority(wp21a_issue, wp21a_pull, control); control.empty?)
{
  "reopened" => [->(issue_payload, _pull) { issue_payload["state"] = "open" }, "wp21a_live_issue_not_closed"],
  "wrong-pr" => [->(_issue_payload, pull_payload) { pull_payload["number"] = 466 }, "wp21a_live_pr_mismatch"],
  "unmerged-pr" => [->(_issue_payload, pull_payload) { pull_payload["merged"] = false }, "wp21a_live_pr_not_merged"],
  "wrong-base" => [->(_issue_payload, pull_payload) { pull_payload["baseRefName"] = "feature" }, "wp21a_live_pr_base_mismatch"],
  "wrong-head" => [->(_issue_payload, pull_payload) { pull_payload["headRefOid"] = reviewed_head }, "wp21a_live_pr_head_mismatch"],
  "wrong-merge" => [->(_issue_payload, pull_payload) { pull_payload["mergeCommit"]["oid"] = reviewed_head }, "wp21a_live_pr_merge_mismatch"],
  "missing-link" => [->(_issue_payload, pull_payload) { pull_payload["closingIssuesReferences"]["nodes"] = [] }, "wp21a_live_closing_link_missing"]
}.each do |name, (mutate, expected)|
  issue_payload = clone(wp21a_issue); pull_payload = clone(wp21a_pull); mutate.call(issue_payload, pull_payload)
  errors = []; validate_wp21a_live_authority(issue_payload, pull_payload, errors)
  raise "wp21a #{name} did not fail closed: #{errors.inspect}" unless errors.include?(expected)
end

graphql_pages = [
  { "data" => { "repository" => { "pullRequest" => wp21a_pull.merge("closingIssuesReferences" => {
      "nodes" => Array.new(100) { |index| { "number" => 1_000 + index, "repository" => { "nameWithOwner" => REPOSITORY } } },
      "pageInfo" => { "hasNextPage" => true, "endCursor" => "page-2" }
  }) } } },
  { "data" => { "repository" => { "pullRequest" => wp21a_pull.merge("closingIssuesReferences" => {
      "nodes" => wp21a_pull.dig("closingIssuesReferences", "nodes"),
      "pageInfo" => { "hasNextPage" => false, "endCursor" => nil }
  }) } } }
]
observed_cursors = []
requester = lambda do |_path, method:, body:|
  raise "pagination did not use GraphQL POST" unless method == :post
  observed_cursors << body.fetch(:variables).fetch(:cursor)
  graphql_pages.shift
end
paginated_pull = github_pull_with_closing_issues(465, requester: requester)
raise "closing linkage pagination incomplete" unless paginated_pull.dig("closingIssuesReferences", "nodes").length == 101
raise "closing linkage cursor sequence invalid" unless observed_cursors == [nil, "page-2"]
pagination_errors = []; validate_wp21a_live_authority(wp21a_issue, paginated_pull, pagination_errors)
raise "closing linkage after page 100 was not recognized: #{pagination_errors.inspect}" unless pagination_errors.empty?

FileUtils.rm_rf(WORK)
puts JSON.generate(schema: "adl.v0.92.quality_gate_negative_suite.v2", status: "passed", cases: 76,
                   canonical_control: { issue: issue, pull_request: pr, reviewed_head: reviewed_head, pr_head: pr_head, merge_sha: merge_sha },
                   candidate_source_sha: CANDIDATE_SOURCE_SHA, candidate_source_tree: CANDIDATE_SOURCE_TREE,
                   authority_substitution_ignored: true)
