#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(ENV.fetch("QUALITY_GATE_ROOT", File.expand_path("../../../..", __dir__))).realpath
FEATURE_INDEX = ROOT / "docs/milestones/v0.92/features/README.md"
COVERAGE = ROOT / "docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"
DEFAULT_MATRIX = ROOT / "docs/reviews/v0.92/quality-gate-311/feature-completion-matrix.json"
DEFAULT_GATE = ROOT / "docs/reviews/v0.92/quality-gate-311/quality-gate-record.json"
DEFAULT_REPORT = ROOT / "docs/reviews/v0.92/quality-gate-311/blocker-report.md"

ALLOWED_DISPOSITIONS = %w[accepted blocked].freeze
PROHIBITED_AUTHORITY = %w[fixture receipt_only demo synthetic substituted_provider self_asserted_json].freeze
REQUIRED_ACCEPTED_EVIDENCE = %w[
  repository issue implementation_paths reviewed_head pull_request merge_sha positive negative integration
  platform typed_terminal review_artifact required_checks
].freeze
REPOSITORY = "agent-logic/agent-design-language"
PROOF_CLASSES = %w[positive negative integration platform].freeze
CANONICAL_REQUIRED_CHECKS = %w[
  adl-ci adl-coverage adl-coverage-hosted adl-coverage-runtime-hosted
  adl-coverage-workspace-hosted adl-tooling-contracts adl-rust-fmt-clippy
  adl-rust-tests adl-path-policy
].sort.freeze

def run_git(*argv)
  stdout, stderr, status = Open3.capture3("git", "-C", ROOT.to_s, *argv)
  raise "git #{argv.join(' ')} failed: #{stderr.strip}" unless status.success?

  stdout.strip
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

def canonical_terminal(issue, errors)
  bin_dir = ENV["CSDLC_V2_BIN_DIR"]
  unless bin_dir && Pathname.new(bin_dir).absolute?
    errors << "typed_terminal_owner_missing"
    return nil
  end
  installer = Pathname.new(bin_dir) / "csdlc-install"
  owner = Pathname.new(bin_dir) / "csdlc-finish"
  unless installer.file? && owner.file?
    errors << "typed_terminal_owner_missing"
    return nil
  end
  resolved, resolved_status = Open3.capture2e(installer.to_s, "resolve", "--repo", ROOT.to_s, "--issue", issue.to_s)
  unless resolved_status.success? && resolved.strip == '"v2"'
    errors << "typed_terminal_owner_not_v2"
    return nil
  end
  stdout, owner_status = Open3.capture2e(owner.to_s, "--root", ROOT.to_s, "--validate-cached-issue", issue.to_s)
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

def live_github(pr, errors)
  gh = ENV.fetch("QUALITY_GATE_GH_BIN", "gh")
  stdout, stderr, status = Open3.capture3(gh, "pr", "view", pr.to_s, "--repo", REPOSITORY,
    "--json", "number,state,headRefOid,mergeCommit,closingIssuesReferences,statusCheckRollup")
  unless status.success?
    errors << "github_observation_failed:#{stderr.strip}"
    return nil
  end
  JSON.parse(stdout)
rescue JSON::ParserError
  errors << "github_observation_invalid"
  nil
end

def validate_proof(proof_ref, proof_class, reviewed_head, row_id, errors)
  proof = retained_json(proof_ref, "#{row_id}:#{proof_class}", errors)
  return unless proof
  expected = { "schema" => "adl.v0.92.quality_gate_proof.v1", "class" => proof_class,
               "result" => "passed", "revision" => reviewed_head }
  expected.each { |key, value| errors << "#{row_id}:#{proof_class}:#{key}_mismatch" unless proof[key] == value }
end

def validate_accepted(row, errors)
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

  return unless evidence["reviewed_head"].to_s.match?(/\A[0-9a-f]{40}\z/) && evidence["merge_sha"].to_s.match?(/\A[0-9a-f]{40}\z/)

  begin
    run_git("cat-file", "-e", "#{evidence['reviewed_head']}^{commit}")
    run_git("cat-file", "-e", "#{evidence['merge_sha']}^{commit}")
    evidence["implementation_paths"].each { |path| run_git("cat-file", "-e", "#{evidence['reviewed_head']}:#{path}") }
    _stdout, _stderr, status = Open3.capture3("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", evidence["reviewed_head"], evidence["merge_sha"])
    errors << "#{row['id']}:reviewed_head_not_merged" unless status.success?
    _stdout, _stderr, status = Open3.capture3("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", evidence["merge_sha"], "HEAD")
    errors << "#{row['id']}:merge_not_ancestral" unless status.success?
  rescue StandardError
    errors << "#{row['id']}:git_identity_unresolvable"
  end

  terminal_cache = terminal.is_a?(Hash) ? retained_json(terminal["cache"], "#{row['id']}:typed_terminal_cache", errors) : nil
  canonical = evidence["issue"].is_a?(Integer) ? canonical_terminal(evidence["issue"], errors) : nil
  if terminal_cache && canonical
    %w[issue pull_request head_sha merge_sha canonical_generation canonical_digest].each do |key|
      expected = { "issue" => evidence["issue"], "pull_request" => evidence["pull_request"],
        "head_sha" => evidence["reviewed_head"], "merge_sha" => evidence["merge_sha"],
        "canonical_generation" => terminal["generation"], "canonical_digest" => terminal["digest"] }[key]
      errors << "#{row['id']}:typed_terminal:#{key}_mismatch" unless canonical[key] == expected && terminal_cache.dig("terminal", key) == expected
    end
    errors << "#{row['id']}:typed_terminal_cache_noncanonical" unless terminal_cache["canonical_match"] == true
  end

  review = retained_json(evidence["review_artifact"], "#{row['id']}:review_artifact", errors)
  if review
    expected = { "schema" => "adl.v0.92.quality_gate_review.v1", "result" => "passed",
      "repository" => REPOSITORY, "issue" => evidence["issue"], "pull_request" => evidence["pull_request"],
      "reviewed_head" => evidence["reviewed_head"], "findings" => [] }
    expected.each { |key, value| errors << "#{row['id']}:review_artifact:#{key}_mismatch" unless review[key] == value }
  end
  PROOF_CLASSES.each { |proof_class| validate_proof(evidence[proof_class], proof_class, evidence["reviewed_head"], row["id"], errors) }

  live = evidence["pull_request"].is_a?(Integer) ? live_github(evidence["pull_request"], errors) : nil
  if live
    errors << "#{row['id']}:github_pr_not_merged" unless live["state"] == "MERGED"
    errors << "#{row['id']}:github_reviewed_head_mismatch" unless live["headRefOid"] == evidence["reviewed_head"]
    errors << "#{row['id']}:github_merge_sha_mismatch" unless live.dig("mergeCommit", "oid") == evidence["merge_sha"]
    closing = Array(live["closingIssuesReferences"]).map { |item| item["number"] }
    errors << "#{row['id']}:github_closing_link_missing" unless closing.include?(evidence["issue"])
    checks = Array(live["statusCheckRollup"]).to_h { |item| [item["name"] || item["context"], item["conclusion"] || item["state"]] }
    errors << "#{row['id']}:required_checks_not_canonical" unless Array(evidence["required_checks"]).sort == CANONICAL_REQUIRED_CHECKS
    CANONICAL_REQUIRED_CHECKS.each { |name| errors << "#{row['id']}:required_check_not_successful:#{name}" unless checks[name] == "SUCCESS" }
  end
end

def validate_matrix(path)
  matrix = JSON.parse(path.read)
  errors = []
  errors << "schema_invalid" unless matrix["schema"] == "adl.v0.92.quality_gate_matrix.v1"
  errors << "milestone_invalid" unless matrix["milestone"] == "v0.92"
  evaluation_base = matrix["evaluation_base_sha"]
  validate_hex(evaluation_base, 40, "evaluation_base_sha", errors)
  if evaluation_base.to_s.match?(/\A[0-9a-f]{40}\z/)
    _stdout, _stderr, status = Open3.capture3("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", evaluation_base, "HEAD")
    errors << "evaluation_base_not_ancestral" unless status.success?
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
  rows.each do |row|
    id = row["id"]
    next unless expected_by_id.key?(id)
    errors << "#{id}:kind_mismatch" unless row["kind"] == expected_by_id[id]["kind"]
    errors << "#{id}:source_mismatch" unless row["source"] == expected_by_id[id]["source"]
    errors << "#{id}:source_status_mismatch" unless row["source_status"] == expected_by_id[id]["source_status"]
    errors << "#{id}:owner_missing" unless row["owner"].is_a?(String) && !row["owner"].strip.empty?
    errors << "#{id}:claim_boundary_missing" unless row["claim_boundary"].is_a?(String) && !row["claim_boundary"].strip.empty?
    disposition = row["disposition"]
    errors << "#{id}:disposition_invalid" unless ALLOWED_DISPOSITIONS.include?(disposition)
    if disposition == "accepted"
      errors << "#{id}:accepted_has_blockers" unless Array(row["blockers"]).empty?
      validate_accepted(row, errors)
    elsif disposition == "blocked"
      errors << "#{id}:blocked_without_reason" if Array(row["blockers"]).empty?
    end
  end
  [matrix, errors]
end

def build_blocked_matrix
  evaluation_base = run_git("merge-base", "origin/main", "HEAD")
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

command = ARGV.shift || "matrix"
matrix_arg = ARGV.each_cons(2).find { |left, _right| left == "--matrix" }&.last
case command
when "generate"
  write_generated_packet
  puts JSON.generate(schema: "adl.v0.92.quality_gate_generation.v1", status: "generated", rows: 33)
when "matrix"
  path = Pathname.new(matrix_arg || DEFAULT_MATRIX.to_s)
  matrix, errors = validate_matrix(path)
  if errors.empty?
    blocked = matrix.fetch("rows").count { |row| row["disposition"] == "blocked" }
    puts JSON.generate(schema: "adl.v0.92.quality_gate_validation.v1", status: "passed", rows: matrix["rows"].length, blocked_rows: blocked, gate_result: blocked.zero? ? "passed" : "blocked")
  else
    warn JSON.generate(schema: "adl.v0.92.quality_gate_validation.v1", status: "failed", errors: errors)
    exit 1
  end
else
  warn "usage: validate-quality-gate.rb generate|matrix [--matrix PATH]"
  exit 2
end
