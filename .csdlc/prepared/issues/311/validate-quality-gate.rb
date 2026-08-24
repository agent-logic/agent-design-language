#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "net/http"
require "openssl"
require "open3"
require "pathname"
require "time"
require "uri"

ROOT = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
FEATURE_INDEX = ROOT / "docs/milestones/v0.92/features/README.md"
COVERAGE = ROOT / "docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"
DEFAULT_MATRIX = ROOT / "docs/reviews/v0.92/quality-gate-311/feature-completion-matrix.json"
DEFAULT_GATE = ROOT / "docs/reviews/v0.92/quality-gate-311/quality-gate-record.json"
DEFAULT_REPORT = ROOT / "docs/reviews/v0.92/quality-gate-311/blocker-report.md"

ALLOWED_DISPOSITIONS = %w[accepted blocked].freeze
PROHIBITED_AUTHORITY = %w[fixture receipt_only demo synthetic substituted_provider self_asserted_json].freeze
REQUIRED_ACCEPTED_EVIDENCE = %w[
  repository issue implementation_paths reviewed_head pr_head pull_request merge_sha positive negative integration
  platform typed_terminal review_artifact required_checks row_contract
].freeze
REPOSITORY = "agent-logic/agent-design-language"
PROOF_CLASSES = %w[positive negative integration platform].freeze
CANONICAL_ROW_PROFILES = {
  "feature:FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92" => {
    "issue" => 451,
    "implementation_paths" => ["adl/src/production_birthday.rs"],
    "proof_paths" => {
      "positive" => ".csdlc/evidence/451/production_birthday_kernel.log",
      "negative" => ".csdlc/evidence/451/retained_evidence_contract.log",
      "integration" => ".csdlc/evidence/451/production_birthday_resident_path.log",
      "platform" => ".csdlc/evidence/451/runtime_feature_wiring_audit.log"
    },
    "positive_tests" => { "passed" => 5, "target" => "production_birthday" },
    "integration_tests" => { "passed" => 2, "target" => "production_birthday_runtime" },
    "evidence_contract" => ".csdlc/evidence/451/production-birthday-evidence.json",
    "audit_contract" => ".csdlc/evidence/451/runtime-feature-wiring-audit.json",
    "audit_features" => %w[birthday_decision birth_witness memory_palace acc_tool_authority]
  }
}.freeze
WP21A_HEAD = "ca78a65a1390f2bc088f8cf20018670d06e87068"
WP21A_MERGE = "a06c34774ad88ea8c56a00533f0fcef810fa7441"
WP21A_TERMINAL_DIGEST = "4080b704ac5123e9aaa3d877095603fcf1db48c5d0953d0ab476724ff30d11d2"
WP21A_RECEIPT_DIGEST = "3db375f9d4e27c0d62f7ed1e7506d2ea816e170b72726ed878084520366a9bde"
CANDIDATE_SOURCE_SHA = "9b43fc535e864155b7c97b0e1b266c0787875bde"
CANDIDATE_SOURCE_TREE = "181093683ad06a62f5b6fc2469791f685cc11ce3"

def run_git(*argv)
  stdout, stderr, status = Open3.capture3(git_environment, "/usr/bin/git", "-C", ROOT.to_s, *argv)
  raise "git #{argv.join(' ')} failed: #{stderr.strip}" unless status.success?

  stdout.strip
end

def run_git_bytes(*argv)
  stdout, stderr, status = Open3.capture3(git_environment, "/usr/bin/git", "-C", ROOT.to_s, *argv)
  raise "git #{argv.join(' ')} failed: #{stderr.strip}" unless status.success?
  stdout
end

def git_environment
  ENV.keys.grep(/\AGIT_/).to_h { |key| [key, nil] }.merge(
    "PATH" => "/usr/bin:/bin", "GIT_CONFIG_NOSYSTEM" => "1", "GIT_CONFIG_GLOBAL" => "/dev/null"
  )
end

def git_ancestor?(ancestor, descendant)
  _stdout, _stderr, status = Open3.capture3(git_environment, "/usr/bin/git", "-C", ROOT.to_s,
                                             "merge-base", "--is-ancestor", ancestor, descendant)
  status.success?
end

def validate_repository_identity(errors)
  remote = run_git("remote", "get-url", "origin")
  normalized = remote.sub(/\.git\z/, "").sub(%r{\Agit@github\.com:}, "https://github.com/")
  errors << "repository_identity_mismatch" unless normalized == "https://github.com/#{REPOSITORY}"
end

def denominator
  feature_text = FEATURE_INDEX.read
  feature_section = feature_text.split("## Feature Documents", 2).fetch(1).split("## WP Coverage Map", 2).first
  feature_paths = feature_section.scan(/\]\(([^)]+\.md)\)/).flatten
  coverage_text = COVERAGE.read
  features = feature_paths.map do |relative|
    path = "docs/milestones/v0.92/features/#{relative}"
    owner_line = feature_text.lines.find { |line| line.start_with?("|") && line.include?("](#{relative})") }
    owner = owner_line ? owner_line.split("|").map(&:strip)[1] : "unmapped feature owner"
    { "id" => "feature:#{File.basename(relative, '.md')}", "kind" => "feature", "source" => path,
      "owner" => owner, "source_status" => "feature_contract" }
  end

  critical_rows = coverage_text.lines.each_with_object([]) do |line, rows|
    next unless line.start_with?("|")
    cells = line.split("|").map(&:strip)
    next unless cells.length >= 6 && cells[5]&.match?(/^AEE-\d{3}$/)

    rows << { "id" => "critical:#{cells[5]}", "kind" => "critical_path", "source" => COVERAGE.relative_path_from(ROOT).to_s,
              "owner" => cells[2], "source_status" => cells[4], "outcome" => cells[1] }
  end
  critical = critical_rows.uniq { |row| row["id"] }
  rows = features + critical
  raise "denominator contains duplicate ids" unless rows.map { |row| row["id"] }.uniq.length == rows.length
  raise "feature denominator must contain 13 rows" unless features.length == 13
  raise "critical-path denominator must contain 20 rows" unless critical.length == 20

  rows
end

def duplicated_critical_ids
  ids = COVERAGE.read.lines.each_with_object([]) do |line, values|
    next unless line.start_with?("|")
    cells = line.split("|").map(&:strip)
    values << cells[5] if cells.length >= 6 && cells[5]&.match?(/^AEE-\d{3}$/)
  end
  counts = ids.each_with_object(Hash.new(0)) { |id, memo| memo[id] += 1 }
  counts.select { |_id, count| count > 1 }.keys.map { |id| "critical:#{id}" }
end

def sha256(path)
  Digest::SHA256.file(path).hexdigest
end

def sha256_bytes(value)
  Digest::SHA256.hexdigest(value)
end

def validate_hex(value, length, label, errors)
  errors << "#{label}:invalid" unless value.is_a?(String) && value.match?(/\A[0-9a-f]{#{length}}\z/)
end

def retained_json(value, label, errors)
  unless value.is_a?(Hash) && value.keys.sort == %w[path sha256]
    errors << "#{label}:reference_invalid"
    return nil
  end
  relative = value["path"]
  unless relative.is_a?(String) && !relative.start_with?("/") && !Pathname.new(relative).each_filename.include?("..")
    errors << "#{label}:path_invalid"
    return nil
  end
  path = ROOT / relative
  unless path.file?
    errors << "#{label}:missing"
    return nil
  end
  validate_hex(value["sha256"], 64, "#{label}:sha256", errors)
  errors << "#{label}:digest_mismatch" unless value["sha256"] == sha256(path)
  JSON.parse(path.read)
rescue JSON::ParserError
  errors << "#{label}:json_invalid"
  nil
end

def safe_relative(value, label, errors)
  unless value.is_a?(String) && !value.start_with?("/") && !Pathname.new(value).each_filename.include?("..")
    errors << "#{label}:path_invalid"
    return nil
  end
  value
end

def retained_blob(value, label, commit, errors)
  unless value.is_a?(Hash) && value.keys.sort == %w[path sha256]
    errors << "#{label}:reference_invalid"
    return nil
  end
  relative = safe_relative(value["path"], label, errors)
  return nil unless relative
  validate_hex(value["sha256"], 64, "#{label}:sha256", errors)
  begin
    candidate_bytes = run_git_bytes("show", "#{commit}:#{relative}")
    retained_bytes = run_git_bytes("show", "HEAD:#{relative}")
    errors << "#{label}:candidate_digest_mismatch" unless sha256_bytes(candidate_bytes) == value["sha256"]
    errors << "#{label}:retained_digest_mismatch" unless sha256_bytes(retained_bytes) == value["sha256"]
    candidate_bytes
  rescue StandardError
    errors << "#{label}:blob_unresolvable"
    nil
  end
end

def stable_bin_dir
  common = Pathname.new(run_git("rev-parse", "--git-common-dir")).realpath
  common.parent / ".adl/bin/csdlc-v2"
end

def canonical_terminal(issue, errors)
  bin_dir = stable_bin_dir
  installer = bin_dir / "csdlc-install"
  owner = bin_dir / "csdlc-finish"
  unless installer.file? && owner.file?
    errors << "typed_terminal_owner_missing"
    return nil
  end
  resolved, resolved_status = Open3.capture2e(git_environment, installer.to_s, "resolve", "--repo", ROOT.to_s, "--issue", issue.to_s)
  unless resolved_status.success? && resolved.strip == '"v2"'
    errors << "typed_terminal_owner_not_v2"
    return nil
  end
  stdout, owner_status = Open3.capture2e(git_environment, owner.to_s, "--root", ROOT.to_s, "--validate-cached-issue", issue.to_s)
  unless owner_status.success?
    errors << "typed_terminal_validation_failed"
    return nil
  end
  receipt = JSON.parse(stdout)
  unless receipt["schema"] == "csdlc.derived_terminal_validation.v1" && receipt["canonical_match"] == true
    errors << "typed_terminal_noncanonical"
    return nil
  end
  receipt["terminal"]
rescue JSON::ParserError
  errors << "typed_terminal_output_invalid"
  nil
end

def recordless_terminal(issue, errors)
  path = Pathname.new(run_git("rev-parse", "--git-common-dir")).realpath / "csdlc-v2/closeout/#{issue}.json"
  unless path.file?
    errors << "recordless_terminal_missing:#{issue}"
    return nil
  end
  receipt = JSON.parse(path.read)
  terminal = receipt["terminal"]
  unless receipt["schema"] == "csdlc.recordless_terminal_receipt.v1" && receipt["issue"] == issue &&
      receipt["repository"] == REPOSITORY && receipt["source_projection_at_pr_head"] == false &&
      receipt["local_projection_present"] == false && terminal.is_a?(Hash) &&
      terminal["source"] == "live_github_recordless_closeout" && terminal["issue"] == issue &&
      terminal["repository"] == REPOSITORY && terminal["issue_state"] == "closed_by_merged_pr" &&
      terminal["disposition"] == "merged"
    errors << "recordless_terminal_invalid:#{issue}"
    return nil
  end
  errors << "recordless_terminal_receipt_digest_mismatch:#{issue}" unless receipt["digest"] == WP21A_RECEIPT_DIGEST
  %w[head_sha merge_sha].each { |key| validate_hex(terminal[key], 40, "recordless_terminal:#{issue}:#{key}", errors) }
  validate_hex(terminal["digest"], 64, "recordless_terminal:#{issue}:digest", errors)
  begin
    run_git("cat-file", "-e", "#{terminal['head_sha']}^{commit}")
    run_git("cat-file", "-e", "#{terminal['merge_sha']}^{commit}")
    head_tree = run_git("rev-parse", "#{terminal['head_sha']}^{tree}")
    merge_tree = run_git("rev-parse", "#{terminal['merge_sha']}^{tree}")
    errors << "recordless_terminal:#{issue}:merged_tree_mismatch" unless head_tree == merge_tree
    errors << "recordless_terminal:#{issue}:merge_not_ancestral" unless git_ancestor?(terminal["merge_sha"], "HEAD")
  rescue StandardError
    errors << "recordless_terminal:#{issue}:git_identity_unresolvable"
  end
  terminal
rescue JSON::ParserError
  errors << "recordless_terminal_invalid_json:#{issue}"
  nil
end

def validate_wp21a_prerequisite(errors)
  terminal = recordless_terminal(310, errors)
  return unless terminal

  errors << "wp21a_pull_request_mismatch" unless terminal["pull_request"] == 465
  errors << "wp21a_head_mismatch" unless terminal["head_sha"] == WP21A_HEAD
  errors << "wp21a_merge_mismatch" unless terminal["merge_sha"] == WP21A_MERGE
  errors << "wp21a_terminal_digest_mismatch" unless terminal["digest"] == WP21A_TERMINAL_DIGEST
  issue_payload = github_request("/repos/#{REPOSITORY}/issues/310")
  pull = github_pull_with_closing_issues(465)
  validate_wp21a_live_authority(issue_payload, pull, errors)
  main = github_request("/repos/#{REPOSITORY}/branches/main").dig("commit", "sha")
  origin_main = run_git("rev-parse", "refs/remotes/origin/main")
  worktrees = run_git("worktree", "list", "--porcelain").split("\n\n")
  validate_wp21a_observation(terminal, main, origin_main, worktrees, errors)
end

def validate_wp21a_live_authority(issue_payload, pull, errors)
  errors << "wp21a_live_issue_mismatch" unless issue_payload["number"] == 310
  errors << "wp21a_live_issue_not_closed" unless issue_payload["state"] == "closed" && issue_payload["state_reason"] == "completed"
  errors << "wp21a_live_pr_mismatch" unless pull["number"] == 465
  errors << "wp21a_live_pr_not_merged" unless pull["state"] == "MERGED" && pull["merged"] == true
  errors << "wp21a_live_pr_base_mismatch" unless pull["baseRefName"] == "main"
  errors << "wp21a_live_pr_head_mismatch" unless pull["headRefOid"] == WP21A_HEAD
  errors << "wp21a_live_pr_merge_mismatch" unless pull.dig("mergeCommit", "oid") == WP21A_MERGE
  closing = Array(pull.dig("closingIssuesReferences", "nodes"))
  linked = closing.any? { |item| item["number"] == 310 && item.dig("repository", "nameWithOwner") == REPOSITORY }
  errors << "wp21a_live_closing_link_missing" unless linked
end

def validate_wp21a_observation(terminal, main, origin_main, worktrees, errors)
  validate_hex(main, 40, "wp21a_live_main_sha", errors)
  begin
    run_git("cat-file", "-e", "#{main}^{commit}")
    errors << "wp21a_merge_not_on_live_main" unless git_ancestor?(WP21A_MERGE, main.to_s)
    errors << "wp21a_origin_main_drift" unless origin_main == main
  rescue StandardError
    errors << "wp21a_live_main_unresolvable"
  end
  registered = Array(worktrees).any? do |entry|
    entry.lines.any? { |line| line.chomp == "branch refs/heads/codex/310-rust-refactoring" }
  end
  errors << "wp21a_worktree_not_cleaned" if registered
end

def github_token
  path = Pathname.new(ENV.fetch("ADL_GITHUB_TOKEN_FILE", File.join(Dir.home, "keys/github.token")))
  raise "github_token_path_invalid" unless path.absolute? && path.file? && !path.symlink?
  raise "github_token_permissions_invalid" unless (path.stat.mode & 0o077).zero?
  token = path.read.strip
  raise "github_token_empty" if token.empty?
  token
end

def github_request(path, method: :get, body: nil)
  uri = URI("https://api.github.com#{path}")
  request = method == :post ? Net::HTTP::Post.new(uri) : Net::HTTP::Get.new(uri)
  request["Authorization"] = "Bearer #{github_token}"
  request["Accept"] = "application/vnd.github+json"
  request["X-GitHub-Api-Version"] = "2022-11-28"
  request["User-Agent"] = "adl-wp22-quality-gate"
  if body
    request["Content-Type"] = "application/json"
    request.body = JSON.generate(body)
  end
  http = github_http(uri)
  response = http.start { |connection| connection.request(request) }
  raise "github_http_#{response.code}" unless response.is_a?(Net::HTTPSuccess)
  JSON.parse(response.body)
end

def github_http(uri)
  # Do not inherit proxy routing or caller-selected TLS trust. GitHub authority
  # is observed directly and verified against the OpenSSL installation's fixed
  # system trust locations.
  http = Net::HTTP.new(uri.hostname, uri.port, nil)
  http.use_ssl = true
  http.open_timeout = 10
  http.read_timeout = 30
  http.verify_mode = OpenSSL::SSL::VERIFY_PEER
  store = OpenSSL::X509::Store.new
  cert_file = OpenSSL::X509::DEFAULT_CERT_FILE
  cert_dir = OpenSSL::X509::DEFAULT_CERT_DIR
  store.add_file(cert_file) if File.file?(cert_file)
  store.add_path(cert_dir) if File.directory?(cert_dir)
  raise "github_system_trust_unavailable" unless File.file?(cert_file) || File.directory?(cert_dir)
  http.cert_store = store
  http
end

def github_paginated(path, collection: nil)
  separator = path.include?("?") ? "&" : "?"
  page = 1
  values = []
  loop do
    payload = github_request("#{path}#{separator}per_page=100&page=#{page}")
    batch = collection ? Array(payload[collection]) : Array(payload)
    values.concat(batch)
    break if batch.length < 100
    page += 1
  end
  values
end

def github_pull_with_closing_issues(pr, requester: method(:github_request))
  nodes = []
  cursor = nil
  pull = nil
  loop do
    query = <<~GRAPHQL
      query($owner:String!, $name:String!, $number:Int!, $cursor:String) {
        repository(owner:$owner, name:$name) {
          pullRequest(number:$number) {
            number state merged baseRefName headRefOid mergeCommit { oid }
            closingIssuesReferences(first:100, after:$cursor) {
              nodes { number repository { nameWithOwner } }
              pageInfo { hasNextPage endCursor }
            }
          }
        }
      }
    GRAPHQL
    graph = requester.call("/graphql", method: :post, body: {
      query: query, variables: { owner: "agent-logic", name: "agent-design-language", number: pr, cursor: cursor }
    })
    raise "github_graphql_errors" unless Array(graph["errors"]).empty?
    page_pull = graph.dig("data", "repository", "pullRequest")
    raise "github_pull_request_missing" unless page_pull.is_a?(Hash)
    pull ||= page_pull.reject { |key, _value| key == "closingIssuesReferences" }
    connection = page_pull.fetch("closingIssuesReferences")
    nodes.concat(Array(connection["nodes"]))
    page_info = connection.fetch("pageInfo")
    break unless page_info["hasNextPage"] == true
    next_cursor = page_info["endCursor"]
    raise "github_closing_link_cursor_invalid" unless next_cursor.is_a?(String) && !next_cursor.empty? && next_cursor != cursor
    cursor = next_cursor
  end
  pull.merge("closingIssuesReferences" => { "nodes" => nodes })
end

def live_github(issue, pr, pr_head, errors)
  pull = github_pull_with_closing_issues(pr)
  issue_payload = github_request("/repos/#{REPOSITORY}/issues/#{issue}")
  checks = { "check_runs" => github_paginated("/repos/#{REPOSITORY}/commits/#{pr_head}/check-runs?filter=all", collection: "check_runs") }
  summaries = github_paginated("/repos/#{REPOSITORY}/rulesets")
  active = summaries.select { |item| item["enforcement"] == "active" && item["target"] == "branch" }
  raise "github_active_branch_rulesets_missing" if active.empty?
  rulesets = active.map { |item| github_request("/repos/#{REPOSITORY}/rulesets/#{item.fetch('id')}") }
  [pull, issue_payload, checks, rulesets]
rescue StandardError => error
  errors << "github_observation_failed:#{error.message}"
  nil
end

def ref_pattern_matches?(pattern, ref, default_ref, errors)
  return true if pattern == "~ALL"
  return ref == default_ref if pattern == "~DEFAULT_BRANCH"
  unless pattern.is_a?(String) && pattern.start_with?("refs/heads/")
    errors << "ruleset_ref_pattern_unsupported:#{pattern}"
    return false
  end
  File.fnmatch?(pattern, ref, File::FNM_PATHNAME | File::FNM_EXTGLOB)
end

def ruleset_applies_to_branch?(ruleset, base_branch, errors)
  includes = ruleset.dig("conditions", "ref_name", "include")
  excludes = Array(ruleset.dig("conditions", "ref_name", "exclude"))
  ref = "refs/heads/#{base_branch}"
  included = Array(includes).any? { |item| ref_pattern_matches?(item, ref, "refs/heads/main", errors) }
  excluded = excludes.any? { |item| ref_pattern_matches?(item, ref, "refs/heads/main", errors) }
  included && !excluded
end

def required_checks_from_rulesets(rulesets, base_branch, errors)
  applicable = Array(rulesets).select do |ruleset|
    ruleset["enforcement"] == "active" && ruleset["target"] == "branch" && ruleset_applies_to_branch?(ruleset, base_branch, errors)
  end
  errors << "ruleset_authority_invalid" if applicable.empty?
  observed = {}
  applicable.each do |ruleset|
    Array(ruleset["rules"]).select { |item| item["type"] == "required_status_checks" }.each do |rule|
      Array(rule.dig("parameters", "required_status_checks")).each do |item|
        context = item["context"]
        integration_id = item["integration_id"]
        next unless context.is_a?(String) && !context.empty?
        errors << "required_check_integration_missing:#{context}" unless integration_id.is_a?(Integer) && integration_id.positive?
        if observed.key?(context) && observed[context] != integration_id
          errors << "required_check_integration_ambiguous:#{context}"
        end
        observed[context] = integration_id
      end
    end
  end
  checks = observed.map { |context, integration_id| { "context" => context, "integration_id" => integration_id } }.sort_by { |item| item["context"] }
  errors << "canonical_required_checks_empty" if checks.empty?
  checks
end

def row_contract_text(row, source_bytes)
  return source_bytes if row["kind"] == "feature"

  source_bytes.lines.find { |line| line.include?("| #{row.fetch('id').delete_prefix('critical:')} |") }.to_s
end

def profile_for(row_id, evidence)
  profile = CANONICAL_ROW_PROFILES[row_id]
  raise "canonical_row_profile_missing" unless profile && profile["issue"] == evidence["issue"] &&
    profile["implementation_paths"] == evidence["implementation_paths"].sort
  profile
end

def proof_semantic_observation(row_id, proof_class, bytes, evidence)
  profile = profile_for(row_id, evidence)
  raise "proof_path_not_profiled" unless evidence.dig(proof_class, "path") == profile.dig("proof_paths", proof_class)
  cargo = bytes.match(/test result: ok\.\s+(\d+) passed;\s+0 failed;/)
  json = begin
    JSON.parse(bytes.strip)
  rescue JSON::ParserError
    nil
  end
  case proof_class
  when "positive", "integration"
    expected = profile["#{proof_class}_tests"]
    raise "non_proving_test_denominator" unless cargo && cargo[1].to_i == expected["passed"] &&
      bytes.include?("Running tests/#{expected['target']}.rs")
    { "kind" => "cargo_test", "passed" => cargo[1].to_i, "failed" => 0, "target" => expected["target"],
      "row_id" => row_id, "issue" => evidence["issue"], "reviewed_head" => evidence["reviewed_head"] }
  when "negative"
    raise "negative_validator_result_invalid" unless json.is_a?(Hash) && json["issue"] == evidence["issue"] &&
      json["result"] == "passed" && json["schema"].to_s.end_with?("_evidence_result.v1")
    contract = JSON.parse(run_git_bytes("show", "#{evidence['pr_head']}:#{profile['evidence_contract']}"))
    required = %w[full_input_replay_denial cross_binding_denial]
    raise "negative_contract_invalid" unless contract["issue"] == evidence["issue"] &&
      required.all? { |key| contract.dig("proof", key) == true } && git_ancestor?(contract["source_revision"], evidence["reviewed_head"])
    { "kind" => "negative_validator", "schema" => json["schema"], "issue" => evidence["issue"],
      "claims" => required, "source_revision" => contract["source_revision"], "row_id" => row_id, "result" => "passed" }
  when "platform"
    raise "platform_audit_result_invalid" unless json.is_a?(Hash) && json["issue"] == evidence["issue"] &&
      json["result"] == "passed" && json["rows"].is_a?(Integer) && json["rows"].positive? &&
      git_ancestor?(json["source_revision"], evidence["reviewed_head"])
    audit = JSON.parse(run_git_bytes("show", "#{evidence['pr_head']}:#{profile['audit_contract']}"))
    live = Array(audit["rows"]).select { |row| profile["audit_features"].include?(row["feature"]) && row["disposition"] == "live" }
    raise "platform_audit_contract_invalid" unless audit["issue"] == evidence["issue"] && live.map { |row| row["feature"] }.sort == profile["audit_features"].sort
    { "kind" => "platform_audit", "schema" => json["schema"], "issue" => evidence["issue"],
      "rows" => json["rows"], "features" => profile["audit_features"], "source_revision" => json["source_revision"],
      "row_id" => row_id, "result" => "passed" }
  else
    raise "unknown_proof_class"
  end
end

def proof_binding_digest(row_id, evidence, validations)
  bindings = PROOF_CLASSES.map do |proof_class|
    proof = evidence.fetch(proof_class)
    lane = validations.fetch(proof.fetch("validation_index"))
    {
      "class" => proof_class, "path" => proof["path"], "sha256" => proof["sha256"],
      "validation_index" => proof["validation_index"], "command" => lane["command"],
      "purpose" => lane["purpose"], "evidence_ref" => lane["evidence_ref"],
      "semantic_observation" => proof_semantic_observation(row_id, proof_class,
        run_git_bytes("show", "#{evidence.fetch('pr_head')}:#{proof.fetch('path')}"), evidence)
    }
  end
  sha256_bytes(JSON.generate(bindings))
end

def validate_row_binding(row, evidence, review_scope, validations, errors)
  row_id = row["id"]
  errors << "#{row_id}:source_outside_review_scope" unless reviewed_scope_includes?(review_scope, row["source"])
  Array(evidence["implementation_paths"]).each do |path|
    errors << "#{row_id}:implementation_path_outside_review_scope:#{path}" unless reviewed_scope_includes?(review_scope, path)
  end
  source_bytes = run_git_bytes("show", "#{evidence['reviewed_head']}:#{row['source']}")
  contract_text = row_contract_text(row, source_bytes)
  errors << "#{row_id}:contract_issue_binding_missing" unless contract_text.match?(/(?:issue\s+`?#|`#|\s#)#{evidence['issue']}(?!\d)/i)
  contract = evidence["row_contract"]
  expected = {
    "schema" => "adl.v0.92.quality_gate_row_contract.v1", "row_id" => row_id,
    "owner" => row["owner"], "source_path" => row["source"],
    "source_sha256" => sha256_bytes(source_bytes), "issue" => evidence["issue"],
    "implementation_paths" => evidence["implementation_paths"].sort,
    "proof_binding_sha256" => proof_binding_digest(row_id, evidence, validations)
  }
  errors << "#{row_id}:row_contract_mismatch" unless contract == expected
rescue StandardError
  errors << "#{row_id}:row_contract_unresolvable"
end

def reviewed_scope_includes?(scope, path)
  Array(scope).any? { |entry| entry == path || path.start_with?("#{entry}/") }
end

def validate_proof(proof_ref, proof_class, pr_head, review_scope, validations, row_id, evidence, errors)
  unless proof_ref.is_a?(Hash) && proof_ref.keys.sort == %w[path sha256 validation_index]
    errors << "#{row_id}:#{proof_class}:reference_invalid"
    return
  end
  bytes = retained_blob({ "path" => proof_ref["path"], "sha256" => proof_ref["sha256"] }, "#{row_id}:#{proof_class}", pr_head, errors)
  return unless bytes
  errors << "#{row_id}:#{proof_class}:outside_review_scope" unless reviewed_scope_includes?(review_scope, proof_ref["path"])
  index = proof_ref["validation_index"]
  unless index.is_a?(Integer) && index >= 0 && index < validations.length
    errors << "#{row_id}:#{proof_class}:validation_index_invalid"
    return
  end
  lane = validations[index]
  errors << "#{row_id}:#{proof_class}:lane_not_passed" unless lane["outcome"] == "passed"
  errors << "#{row_id}:#{proof_class}:command_missing" unless lane["command"].is_a?(Array) && !lane["command"].empty?
  evidence_name = lane["evidence_ref"]
  errors << "#{row_id}:#{proof_class}:evidence_ref_mismatch" unless evidence_name.is_a?(String) && File.basename(proof_ref["path"]) == File.basename(evidence_name)
  begin
    proof_semantic_observation(row_id, proof_class, bytes, evidence)
  rescue StandardError
    # The exact issue-aware semantic check is performed as part of the row
    # contract binding. This catch retains a class-local error if raw content
    # cannot satisfy even the selected proof shape.
    errors << "#{row_id}:#{proof_class}:semantic_proof_invalid"
  end
end

def validate_live_authority(evidence, pull, issue_payload, check_payload, rulesets, row_id, errors)
  errors << "#{row_id}:github_pr_not_merged" unless pull["state"] == "MERGED" && pull["merged"] == true
  errors << "#{row_id}:github_base_branch_mismatch" unless pull["baseRefName"] == "main"
  errors << "#{row_id}:github_pr_head_mismatch" unless pull["headRefOid"] == evidence["pr_head"]
  errors << "#{row_id}:github_merge_sha_mismatch" unless pull.dig("mergeCommit", "oid") == evidence["merge_sha"]
  closing = Array(pull.dig("closingIssuesReferences", "nodes"))
  linked = closing.any? { |item| item["number"] == evidence["issue"] && item.dig("repository", "nameWithOwner") == REPOSITORY }
  errors << "#{row_id}:github_closing_link_missing" unless linked
  errors << "#{row_id}:github_issue_not_closed" unless issue_payload["number"] == evidence["issue"] &&
    issue_payload["state"] == "closed" && issue_payload["state_reason"] == "completed"
  canonical_checks = required_checks_from_rulesets(rulesets, pull["baseRefName"], errors)
  canonical_names = canonical_checks.map { |item| item["context"] }
  errors << "#{row_id}:required_checks_not_canonical" unless Array(evidence["required_checks"]).sort == canonical_names.sort
  canonical_checks.each do |required|
    matching = Array(check_payload["check_runs"]).select { |item| item["name"] == required["context"] }
    authorized = matching.select { |item| item.dig("app", "id") == required["integration_id"] }
    authorized.each { |item| errors << "#{row_id}:required_check_timestamp_missing:#{required['context']}" unless item["completed_at"].is_a?(String) }
    latest_time = authorized.map { |item| Time.iso8601(item["completed_at"]) rescue nil }.compact.max
    latest_items = authorized.select { |item| (Time.iso8601(item["completed_at"]) rescue nil) == latest_time }
    errors << "#{row_id}:required_check_latest_ambiguous:#{required['context']}" unless latest_items.length == 1
    latest = latest_items.max_by { |item| item["id"].to_i }
    accepted = latest && latest["conclusion"] == "success"
    errors << "#{row_id}:required_check_not_successful:#{required['context']}" unless accepted
  end
end

def validate_accepted(row, errors, accepted_packets)
  evidence = row["evidence"]
  unless evidence.is_a?(Hash)
    errors << "#{row['id']}:accepted_evidence_missing"
    return
  end
  REQUIRED_ACCEPTED_EVIDENCE.each do |key|
    value = evidence[key]
    errors << "#{row['id']}:#{key}_missing" if value.nil? || value == "" || value == [] || value == {}
  end
  return unless errors.none? { |error| error.start_with?("#{row['id']}:") }

  validate_hex(evidence["reviewed_head"], 40, "#{row['id']}:reviewed_head", errors)
  validate_hex(evidence["pr_head"], 40, "#{row['id']}:pr_head", errors)
  validate_hex(evidence["merge_sha"], 40, "#{row['id']}:merge_sha", errors)
  errors << "#{row['id']}:repository_mismatch" unless evidence["repository"] == REPOSITORY
  errors << "#{row['id']}:issue_invalid" unless evidence["issue"].is_a?(Integer) && evidence["issue"].positive?
  errors << "#{row['id']}:pull_request_invalid" unless evidence["pull_request"].is_a?(Integer) && evidence["pull_request"].positive?
  unless evidence["implementation_paths"].is_a?(Array) && !evidence["implementation_paths"].empty? &&
      evidence["implementation_paths"].all? { |path| path.is_a?(String) && !path.start_with?("/") && !Pathname.new(path).each_filename.include?("..") }
    errors << "#{row['id']}:implementation_paths_invalid"
  end
  terminal = evidence["typed_terminal"]
  unless terminal.is_a?(Hash) && terminal.keys.sort == %w[cache digest generation] && terminal["generation"].is_a?(Integer) && terminal["generation"].positive?
    errors << "#{row['id']}:typed_terminal_generation_invalid"
  end
  validate_hex(terminal.is_a?(Hash) ? terminal["digest"] : nil, 64, "#{row['id']}:typed_terminal_digest", errors)
  source_kind = evidence["authority_kind"]
  errors << "#{row['id']}:prohibited_authority:#{source_kind}" if PROHIBITED_AUTHORITY.include?(source_kind)
  errors << "#{row['id']}:canonical_authority_missing" unless source_kind == "canonical_observation"

  return unless %w[reviewed_head pr_head merge_sha].all? { |key| evidence[key].to_s.match?(/\A[0-9a-f]{40}\z/) }

  begin
    run_git("cat-file", "-e", "#{evidence['reviewed_head']}^{commit}")
    run_git("cat-file", "-e", "#{evidence['pr_head']}^{commit}")
    run_git("cat-file", "-e", "#{evidence['merge_sha']}^{commit}")
    evidence["implementation_paths"].each { |path| run_git("cat-file", "-e", "#{evidence['reviewed_head']}:#{path}") }
    errors << "#{row['id']}:reviewed_head_not_in_pr_head" unless git_ancestor?(evidence["reviewed_head"], evidence["pr_head"])
    tree_equal = run_git("rev-parse", "#{evidence['pr_head']}^{tree}") == run_git("rev-parse", "#{evidence['merge_sha']}^{tree}")
    errors << "#{row['id']}:pr_head_not_merged" unless git_ancestor?(evidence["pr_head"], evidence["merge_sha"]) || tree_equal
    errors << "#{row['id']}:merge_not_ancestral" unless git_ancestor?(evidence["merge_sha"], "HEAD")
  rescue StandardError
    errors << "#{row['id']}:git_identity_unresolvable"
  end

  terminal_cache = terminal.is_a?(Hash) ? retained_json(terminal["cache"], "#{row['id']}:typed_terminal_cache", errors) : nil
  canonical = evidence["issue"].is_a?(Integer) ? canonical_terminal(evidence["issue"], errors) : nil
  if terminal_cache && canonical
    expected_terminal = { "repository" => REPOSITORY, "issue" => evidence["issue"], "issue_state" => "closed_by_merged_pr",
        "pull_request" => evidence["pull_request"], "head_sha" => evidence["pr_head"], "merge_sha" => evidence["merge_sha"],
        "canonical_generation" => terminal["generation"], "canonical_digest" => terminal["digest"] }
    expected_terminal.each do |key, expected|
      errors << "#{row['id']}:typed_terminal:#{key}_mismatch" unless canonical[key] == expected && terminal_cache.dig("terminal", key) == expected
    end
    errors << "#{row['id']}:typed_terminal_cache_noncanonical" unless terminal_cache["canonical_match"] == true
  end

  review_bytes = retained_blob(evidence["review_artifact"], "#{row['id']}:review_artifact", evidence["pr_head"], errors)
  review_index = review_bytes ? JSON.parse(review_bytes) : nil
  review = review_index&.dig("review")
  expected_review_path = ".csdlc/issues/#{evidence['issue']}/index.json"
  errors << "#{row['id']}:review_artifact_path_mismatch" unless evidence.dig("review_artifact", "path") == expected_review_path
  if review
    errors << "#{row['id']}:review_index_issue_mismatch" unless review_index["issue"] == evidence["issue"] && review_index["repository"] == REPOSITORY
    errors << "#{row['id']}:review_index_terminal_digest_mismatch" unless review_index["generation"] == terminal["generation"] && review_index["digest"] == terminal["digest"]
    errors << "#{row['id']}:review_incomplete" unless review["completed"] == true
    errors << "#{row['id']}:review_findings_present" unless Array(review["findings"]).empty?
    revision_match = review["reviewed_revision"].to_s.match(/\Agit-blake3:([0-9a-f]{40}):([0-9a-f]{64})\z/)
    errors << "#{row['id']}:review_revision_invalid" unless revision_match && revision_match[1] == evidence["reviewed_head"]
    assignment = review_index["review_assignment"]
    errors << "#{row['id']}:review_assignment_mismatch" unless assignment.is_a?(Hash) &&
      assignment["reviewer"] == review["reviewer"] && assignment["revision"] == review["reviewed_revision"] &&
      assignment["scope"] == review["scope"]
    metadata_paths = run_git("diff", "--name-only", evidence["reviewed_head"], evidence["pr_head"]).lines.map(&:strip).reject(&:empty?)
    allowed_prefix = ".csdlc/issues/#{evidence['issue']}/"
    errors << "#{row['id']}:post_review_product_drift" unless metadata_paths.all? { |path| path.start_with?(allowed_prefix) }
  else
    errors << "#{row['id']}:review_authority_missing"
  end
  sor_bytes = begin
    run_git_bytes("show", "#{evidence['pr_head']}:.csdlc/issues/#{evidence['issue']}/cards/sor.values.json")
  rescue StandardError
    errors << "#{row['id']}:typed_sor_missing"
    nil
  end
  validations = sor_bytes ? Array(JSON.parse(sor_bytes).dig("content", "values", "actual_validation")) : []
  review_scope = review ? review["scope"] : []
  validate_row_binding(row, evidence, review_scope, validations, errors)
  PROOF_CLASSES.each { |proof_class| validate_proof(evidence[proof_class], proof_class, evidence["pr_head"], review_scope, validations, row["id"], evidence, errors) }
  proof_paths = PROOF_CLASSES.map { |proof_class| evidence.dig(proof_class, "path") }
  errors << "#{row['id']}:proof_paths_not_distinct" unless proof_paths.compact.uniq.length == PROOF_CLASSES.length
  begin
    packet_identity = sha256_bytes(JSON.generate({ "issue" => evidence["issue"], "pr_head" => evidence["pr_head"],
      "proofs" => PROOF_CLASSES.map { |proof_class| [evidence[proof_class]["sha256"], proof_semantic_observation(row["id"], proof_class,
        run_git_bytes("show", "#{evidence['pr_head']}:#{evidence[proof_class]['path']}"), evidence)] } }))
    prior = accepted_packets[packet_identity]
    errors << "#{row['id']}:accepted_packet_reused_from:#{prior}" if prior && prior != row["id"]
    accepted_packets[packet_identity] = row["id"]
  rescue StandardError
    errors << "#{row['id']}:accepted_packet_semantics_unresolvable"
  end

  live = evidence["pull_request"].is_a?(Integer) ? live_github(evidence["issue"], evidence["pull_request"], evidence["pr_head"], errors) : nil
  if live
    pull, issue_payload, check_payload, ruleset = live
    validate_live_authority(evidence, pull, issue_payload, check_payload, ruleset, row["id"], errors)
  end
rescue JSON::ParserError
  errors << "#{row['id']}:typed_artifact_json_invalid"
end

def validate_complete_packet(matrix, errors)
  gate = JSON.parse(DEFAULT_GATE.read)
  receipt_path = ROOT / ".csdlc/evidence/311/validation.json"
  receipt = JSON.parse(receipt_path.read)
  report = DEFAULT_REPORT.read
  rows = matrix.fetch("rows")
  accepted = rows.count { |row| row["disposition"] == "accepted" }
  blocked_rows = rows.select { |row| row["disposition"] == "blocked" }
  result = blocked_rows.empty? ? "passed" : "blocked"
  unlock = result == "passed"
  candidate = matrix["candidate_source_sha"]
  candidate_tree = matrix["candidate_source_tree"]
  errors << "packet:candidate_source_sha_mismatch" unless candidate == CANDIDATE_SOURCE_SHA
  errors << "packet:candidate_source_tree_mismatch" unless candidate_tree == CANDIDATE_SOURCE_TREE
  validate_hex(candidate, 40, "packet:candidate_source_sha", errors)
  validate_hex(candidate_tree, 40, "packet:candidate_source_tree", errors)
  begin
    errors << "packet:candidate_source_not_ancestral" unless git_ancestor?(candidate, "HEAD")
    errors << "packet:candidate_source_tree_mismatch" unless run_git("rev-parse", "#{candidate}^{tree}") == candidate_tree
    allowed_after_candidate = [
      ".csdlc/prepared/issues/311/validate-quality-gate.rb", ".csdlc/prepared/issues/311/test-validate-quality-gate.rb",
      ".csdlc/evidence/311/quality-negative-suite.log", ".csdlc/evidence/311/semantic-quality-matrix.log",
      ".csdlc/evidence/311/validation.json", "docs/reviews/v0.92/quality-gate-311/feature-completion-matrix.json",
      "docs/reviews/v0.92/quality-gate-311/quality-gate-record.json"
    ]
    post_candidate = run_git("diff", "--name-only", candidate, "HEAD").lines.map(&:strip).reject(&:empty?)
    unexpected_after_candidate = post_candidate.reject do |path|
      allowed_after_candidate.include?(path) || path.start_with?(".csdlc/issues/311/")
    end
    errors << "packet:post_candidate_scope_mismatch" unless unexpected_after_candidate.empty?
    dirty = run_git("status", "--porcelain=v1").lines.map(&:strip).reject do |line|
      line.end_with?(".csdlc/locks/311.lock") || line.match?(/\.csdlc\/issues\/311\//)
    end
    errors << "packet:candidate_worktree_dirty" unless dirty.empty?
  rescue StandardError
    errors << "packet:candidate_identity_unresolvable"
  end

  gate_expected = {
    "schema" => "adl.v0.92.quality_gate_record.v1", "issue" => 311,
    "evaluation_base_sha" => matrix["evaluation_base_sha"], "candidate_source_sha" => candidate,
    "candidate_source_tree" => candidate_tree, "matrix_sha256" => sha256(DEFAULT_MATRIX),
    "validator_sha256" => sha256(Pathname.new(__FILE__)), "feature_rows" => 13,
    "critical_path_rows" => 20, "accepted_rows" => accepted,
    "blocked_rows" => blocked_rows.length, "result" => result, "downstream_unlock" => unlock
  }
  gate_expected.each { |key, value| errors << "packet:gate:#{key}_mismatch" unless gate[key] == value }

  receipt_expected = {
    "schema" => "adl.v0.92.quality_gate_validation_receipt.v1", "issue" => 311,
    "evaluation_base_sha" => matrix["evaluation_base_sha"], "candidate_source_sha" => candidate,
    "candidate_source_tree" => candidate_tree, "structural_validation" => "passed",
    "quality_gate_result" => result, "downstream_unlock" => unlock
  }
  receipt_expected.each { |key, value| errors << "packet:receipt:#{key}_mismatch" unless receipt[key] == value }
  dependency_expected = {
    "issue_310" => "closed_by_merged_pr_465", "head_sha" => WP21A_HEAD, "merge_sha" => WP21A_MERGE,
    "terminal_source" => "live_github_recordless_closeout", "terminal_digest" => WP21A_TERMINAL_DIGEST,
    "receipt_digest" => WP21A_RECEIPT_DIGEST, "cleanup" => "complete", "release_credit" => false
  }
  errors << "packet:receipt:dependency_observation_mismatch" unless receipt["dependency_observation"] == dependency_expected
  denominator_expected = { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33,
                           "accepted_rows" => accepted, "blocked_rows" => blocked_rows.length }
  errors << "packet:receipt:denominator_mismatch" unless receipt["denominator"] == denominator_expected

  lanes = Array(receipt["lanes"])
  lane_names = lanes.map { |lane| lane["name"] }
  errors << "packet:receipt:duplicate_lane" unless lane_names.uniq.length == lane_names.length
  lane_by_name = lanes.to_h { |lane| [lane["name"], lane] }
  expected_lanes = %w[semantic-quality-matrix quality-negative-suite docs-schema-diff]
  errors << "packet:receipt:lane_set_mismatch" unless lane_by_name.keys.sort == expected_lanes.sort
  expected_lanes.each do |name|
    lane = lane_by_name[name]
    next errors << "packet:lane:#{name}:missing" unless lane
    errors << "packet:lane:#{name}:result_mismatch" unless lane["result"] == "passed"
    path = safe_relative(lane["log"], "packet:lane:#{name}", errors)
    errors << "packet:lane:#{name}:log_missing" unless path && (ROOT / path).file?
    errors << "packet:lane:#{name}:log_digest_mismatch" unless path && lane["sha256"] == sha256(ROOT / path)
  end
  errors << "packet:lane:semantic:gate_result_mismatch" unless lane_by_name.dig("semantic-quality-matrix", "gate_result") == result
  errors << "packet:lane:negative:cases_mismatch" unless lane_by_name.dig("quality-negative-suite", "cases") == 65

  semantic_log = JSON.parse((ROOT / lane_by_name.dig("semantic-quality-matrix", "log")).read)
  negative_log = JSON.parse((ROOT / lane_by_name.dig("quality-negative-suite", "log")).read)
  errors << "packet:semantic_log_mismatch" unless semantic_log["status"] == "passed" && semantic_log["rows"] == 33 && semantic_log["blocked_rows"] == blocked_rows.length && semantic_log["gate_result"] == result && semantic_log["candidate_source_sha"] == candidate && semantic_log["candidate_source_tree"] == candidate_tree
  errors << "packet:negative_log_mismatch" unless negative_log["schema"] == "adl.v0.92.quality_gate_negative_suite.v2" && negative_log["status"] == "passed" && negative_log["cases"] == 65 && negative_log["authority_substitution_ignored"] == true && negative_log["candidate_source_sha"] == candidate && negative_log["candidate_source_tree"] == candidate_tree
  diff_log = ROOT / lane_by_name.dig("docs-schema-diff", "log")
  errors << "packet:diff_log_not_clean" unless diff_log.read.empty?

  artifacts = receipt["artifacts"]
  expected_artifacts = {
    "validator_sha256" => sha256(Pathname.new(__FILE__)),
    "negative_suite_sha256" => sha256(ROOT / ".csdlc/prepared/issues/311/test-validate-quality-gate.rb"),
    "matrix_sha256" => sha256(DEFAULT_MATRIX), "gate_record_sha256" => sha256(DEFAULT_GATE),
    "blocker_report_sha256" => sha256(DEFAULT_REPORT),
    "semantic_log_sha256" => sha256(ROOT / ".csdlc/evidence/311/semantic-quality-matrix.log"),
    "negative_log_sha256" => sha256(ROOT / ".csdlc/evidence/311/quality-negative-suite.log"),
    "diff_log_sha256" => sha256(ROOT / ".csdlc/evidence/311/docs-schema-diff.log")
  }
  errors << "packet:receipt:artifact_set_mismatch" unless artifacts.is_a?(Hash) && artifacts.keys.sort == expected_artifacts.keys.sort
  expected_artifacts.each { |key, value| errors << "packet:receipt:#{key}_mismatch" unless artifacts&.[](key) == value }

  errors << "packet:report:result_mismatch" unless report.include?("Result: **#{result.upcase}**")
  blocked_rows.each do |row|
    expected_line = "- `#{row['id']}` — #{row['blockers'].join(', ')}"
    errors << "packet:report:row_mismatch:#{row['id']}" unless report.lines.count { |line| line.chomp == expected_line } == 1
  end
rescue JSON::ParserError, KeyError, Errno::ENOENT => error
  errors << "packet:invalid:#{error.class}"
end

def validate_matrix(path, canonical: true)
  matrix = JSON.parse(path.read)
  errors = []
  validate_repository_identity(errors)
  errors << "schema_invalid" unless matrix["schema"] == "adl.v0.92.quality_gate_matrix.v1"
  errors << "milestone_invalid" unless matrix["milestone"] == "v0.92"
  errors << "issue_invalid" unless matrix["issue"] == 311
  errors << "denominator_object_invalid" unless matrix["denominator"] == { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33 }
  validate_wp21a_prerequisite(errors) if canonical
  evaluation_base = matrix["evaluation_base_sha"]
  validate_hex(evaluation_base, 40, "evaluation_base_sha", errors)
  errors << "evaluation_base_not_wp21a_merge" unless evaluation_base == WP21A_MERGE
  if evaluation_base.to_s.match?(/\A[0-9a-f]{40}\z/)
    errors << "evaluation_base_not_ancestral" unless git_ancestor?(evaluation_base, "HEAD")
  end

  expected = denominator
  rows = matrix["rows"]
  unless rows.is_a?(Array)
    return [matrix, ["rows_missing"]]
  end
  expected_ids = expected.map { |row| row["id"] }
  observed_ids = rows.map { |row| row["id"] }
  errors << "denominator_missing:#{(expected_ids - observed_ids).join(',')}" unless (expected_ids - observed_ids).empty?
  errors << "denominator_extra:#{(observed_ids - expected_ids).join(',')}" unless (observed_ids - expected_ids).empty?
  counts = observed_ids.each_with_object(Hash.new(0)) { |id, memo| memo[id] += 1 }
  duplicates = counts.select { |_id, count| count > 1 }.keys
  errors << "denominator_duplicate:#{duplicates.join(',')}" unless duplicates.empty?

  expected_by_id = expected.to_h { |row| [row["id"], row] }
  accepted_packets = {}
  rows.each do |row|
    id = row["id"]
    next unless expected_by_id.key?(id)
    errors << "#{id}:kind_mismatch" unless row["kind"] == expected_by_id[id]["kind"]
    errors << "#{id}:source_mismatch" unless row["source"] == expected_by_id[id]["source"]
    errors << "#{id}:source_status_mismatch" unless row["source_status"] == expected_by_id[id]["source_status"]
    errors << "#{id}:owner_mismatch" unless row["owner"] == expected_by_id[id]["owner"]
    errors << "#{id}:claim_boundary_missing" unless row["claim_boundary"].is_a?(String) && !row["claim_boundary"].strip.empty?
    disposition = row["disposition"]
    errors << "#{id}:disposition_invalid" unless ALLOWED_DISPOSITIONS.include?(disposition)
    if disposition == "accepted"
      errors << "#{id}:accepted_has_blockers" unless Array(row["blockers"]).empty?
      validate_accepted(row, errors, accepted_packets)
    elsif disposition == "blocked"
      errors << "#{id}:blocked_without_reason" if Array(row["blockers"]).empty?
    end
  end
  validate_complete_packet(matrix, errors) if canonical
  [matrix, errors]
end

def build_blocked_matrix
  evaluation_base = run_git("merge-base", "origin/main", "HEAD")
  candidate_source_sha = CANDIDATE_SOURCE_SHA
  candidate_source_tree = CANDIDATE_SOURCE_TREE
  rows = denominator.map do |entry|
    blockers = ["accepted_evidence_packet_missing"]
    if entry["kind"] == "feature"
      blockers << "feature_contract_has_no_accepted_evidence_binding"
    elsif entry["source_status"] != "accepted"
      blockers << "coverage_status_not_accepted:#{entry['source_status']}"
    end
    blockers << "duplicate_source_coverage_row" if duplicated_critical_ids.include?(entry["id"])
    entry.merge(
      "disposition" => "blocked",
      "claim_boundary" => "No release credit until exact canonical evidence satisfies every accepted-row field.",
      "blockers" => blockers,
      "evidence" => {}
    )
  end
  {
    "schema" => "adl.v0.92.quality_gate_matrix.v1",
    "milestone" => "v0.92",
    "issue" => 311,
    "evaluation_base_sha" => evaluation_base,
    "candidate_source_sha" => candidate_source_sha,
    "candidate_source_tree" => candidate_source_tree,
    "denominator" => { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33 },
    "rows" => rows
  }
end

def write_generated_packet
  DEFAULT_MATRIX.dirname.mkpath
  matrix = build_blocked_matrix
  DEFAULT_MATRIX.write(JSON.pretty_generate(matrix) + "\n")
  matrix_digest = sha256(DEFAULT_MATRIX)
  gate = {
    "schema" => "adl.v0.92.quality_gate_record.v1",
    "issue" => 311,
    "evaluation_base_sha" => matrix["evaluation_base_sha"],
    "candidate_source_sha" => matrix["candidate_source_sha"],
    "candidate_source_tree" => matrix["candidate_source_tree"],
    "matrix_sha256" => matrix_digest,
    "validator_sha256" => sha256(Pathname.new(__FILE__)),
    "feature_rows" => 13,
    "critical_path_rows" => 20,
    "accepted_rows" => 0,
    "blocked_rows" => 33,
    "result" => "blocked",
    "downstream_unlock" => false,
    "non_claim" => "WP-22 structural execution completed; v0.92 release quality did not pass."
  }
  DEFAULT_GATE.write(JSON.pretty_generate(gate) + "\n")
  report = [
    "# v0.92 WP-22 Blocker Report",
    "",
    "Result: **BLOCKED**",
    "",
    "The exact denominator contains 13 feature rows and 20 supporting critical-path rows. None currently has a complete canonical accepted-evidence packet, so no row receives release credit.",
    "",
    "## Blocking Rule",
    "",
    "Every row remains blocked until it binds exact implementation paths, reviewed head, closing PR and merge, positive and negative validation, integration and platform proof, canonical typed terminal evidence, required checks, and a digest-bound review artifact.",
    "",
    "## Findings",
    ""
  ]
  matrix["rows"].each { |row| report << "- `#{row['id']}` — #{row['blockers'].join(', ')}" }
  report.concat(["", "## Downstream", "", "WP-23, WP-25, and the release tail remain blocked. This report does not waive, repair, or silently defer any missing feature evidence.", ""])
  DEFAULT_REPORT.write(report.join("\n"))
end

if __FILE__ == $PROGRAM_NAME
  command = ARGV.shift || "matrix"
  case command
  when "generate"
    write_generated_packet
    puts JSON.generate(schema: "adl.v0.92.quality_gate_generation.v1", status: "generated", rows: 33)
  when "matrix"
    unless ARGV.empty?
      warn "canonical matrix validation accepts no alternate path"
      exit 2
    end
    matrix, errors = validate_matrix(DEFAULT_MATRIX, canonical: true)
    if errors.empty?
      blocked = matrix.fetch("rows").count { |row| row["disposition"] == "blocked" }
      puts JSON.generate(schema: "adl.v0.92.quality_gate_validation.v1", status: "passed", rows: matrix["rows"].length,
                         blocked_rows: blocked, gate_result: blocked.zero? ? "passed" : "blocked",
                         candidate_source_sha: matrix["candidate_source_sha"], candidate_source_tree: matrix["candidate_source_tree"])
    else
      warn JSON.generate(schema: "adl.v0.92.quality_gate_validation.v1", status: "failed", errors: errors)
      exit 1
    end
  else
    warn "usage: validate-quality-gate.rb generate|matrix"
    exit 2
  end
end
