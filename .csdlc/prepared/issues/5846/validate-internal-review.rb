#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"; require "json"; require "open3"; require "pathname"
REPOSITORY = "agent-logic/agent-design-language"
PACKET_ROOT = "docs/reviews/v0.92/internal-review-5846"
ALLOWED_PREFIXES = ["#{PACKET_ROOT}/", "docs/milestones/v0.92/review/", ".csdlc/prepared/issues/313/", ".csdlc/prepared/issues/5846/"].freeze
PRIVATE_PATTERNS = [%r{/Users/}, %r{/Volumes/}, %r{/private/}, /AKIA[0-9A-Z]{16}/, /(?:api[_-]?key|token|secret)\s*[:=]\s*["']?[A-Za-z0-9_\-]{16,}/i].freeze
def fail!(message)
  abort("internal-review validation failed: #{message}")
end
def read_json!(path, label)
  fail!("missing #{label}: #{path}") unless File.file?(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  fail!("invalid #{label}: #{error.message}")
end
def run!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  fail!("command failed: #{argv.join(' ')}: #{stderr.strip}") unless status.success?
  stdout
end
def safe_path!(path)
  fail!("non-relative manifest path: #{path}") unless path.is_a?(String) && !path.empty? && !Pathname.new(path).absolute?
  fail!("manifest path traversal: #{path}") unless Pathname.new(path).cleanpath.to_s == path && !path.split("/").include?("..")
  fail!("manifest path outside review authority: #{path}") unless ALLOWED_PREFIXES.any? { |prefix| path.start_with?(prefix) }
  fail!("missing manifest object: #{path}") unless File.file?(path)
end
def repo_path!(path)
  fail!("non-relative evidence path: #{path}") unless path.is_a?(String) && !path.empty? && !Pathname.new(path).absolute?
  fail!("evidence path traversal: #{path}") unless Pathname.new(path).cleanpath.to_s == path && !path.split("/").include?("..")
  fail!("missing evidence path: #{path}") unless File.exist?(path)
end
require_meta_review = ARGV.delete("--require-meta-review")
fail!("unexpected arguments: #{ARGV.join(' ')}") if ARGV.length > 1
root = ARGV.fetch(0, PACKET_ROOT)
fail!("non-canonical packet root") unless root == PACKET_ROOT
manifest = read_json!(File.join(root, "packet-manifest.json"), "packet manifest")
findings = read_json!(File.join(root, "findings.json"), "findings")
live = read_json!(File.join(root, "LIVE_STATE.json"), "live state")
roster = %w[architecture code dependencies docs security tests lifecycle demos release_publication].sort
fail!("manifest schema mismatch") unless manifest["schema"] == "adl.internal_review.packet_manifest.v1"
target = manifest["target_sha"]
fail!("target SHA missing") unless target.is_a?(String) && target.match?(/\A[0-9a-f]{40}\z/)
run!("git", "cat-file", "-e", "#{target}^{commit}"); run!("git", "merge-base", "--is-ancestor", target, "HEAD")
fail!("frozen target no longer equals origin/main") unless run!("git", "rev-parse", "origin/main").strip == target
origin = run!("git", "remote", "get-url", "origin").strip
fail!("wrong repository") unless origin.match?(%r{(?:github\.com[:/])agent-logic/agent-design-language(?:\.git)?\z})
paths = manifest["paths"]
fail!("packet corpus missing") unless paths.is_a?(Array) && !paths.empty?
fail!("manifest path_count mismatch") unless manifest["path_count"] == paths.length
fail!("manifest paths not unique and sorted") unless paths == paths.uniq.sort
paths.each { |path| safe_path!(path) }
expected = Dir.glob(File.join(root, "**", "*")).select { |path| File.file?(path) } - [File.join(root, "packet-manifest.json"), File.join(root, "PACKET_MANIFEST.md")]
missing = expected.sort - paths
fail!("unmanifested packet files: #{missing.join(', ')}") unless missing.empty?
normalized = paths.map { |path| "#{path}\0#{Digest::SHA256.file(path).hexdigest}" }.join("\n")
fail!("packet digest mismatch") unless manifest["packet_sha256"] == Digest::SHA256.hexdigest(normalized)
run_manifest = read_json!(File.join(root, "run_manifest.json"), "run manifest")
fail!("run manifest target mismatch") unless run!("git", "rev-parse", run_manifest.fetch("repo_ref")).strip == target
paths.select { |path| path.start_with?("docs/") && %w[.md .json].include?(File.extname(path)) }.each do |path|
  body = File.read(path)
  PRIVATE_PATTERNS.each { |pattern| fail!("private or secret-shaped text in #{path}") if body.match?(pattern) }
end
fail!("live-state identity mismatch") unless live["schema"] == "adl.internal_review.live_state.v1" && live["repository"] == REPOSITORY && live["target_sha"] == target
issues = live.fetch("issues")
fail!("#307 sprint graph mismatch") unless issues.dig("307", "state") == "OPEN" && issues.dig("307", "graph_check") == "#312 -> #313 -> #314 -> #315"
fail!("#313 dependency truth mismatch") unless issues.dig("313", "state") == "OPEN" && issues.dig("313", "dependencies") == [312, 10] && issues.dig("313", "deferred_non_blocking") == [342]
fail!("#342 deferment mismatch") unless issues.dig("342", "state") == "OPEN" && issues.dig("342", "milestone") == "v0.92.1"
fail!("#315 WP-27 authority mismatch") unless issues.dig("315", "state") == "OPEN" && issues.dig("315", "work_package") == "WP-27" && issues.dig("315", "legacy_predecessor") == 5848
fail!("dependency worktree truth mismatch") unless live["dependency_worktrees_absent"] == [312, 10]
fail!("live GitHub/terminal verification failed") unless system("ruby", ".csdlc/prepared/issues/313/capture_internal_review_live_state.rb", "--verify")
common = run!("git", "rev-parse", "--git-common-dir").strip
[312, 10].each do |number|
  receipt = read_json!(File.join(common, "csdlc-v2", "derived-terminal", "#{number}.json"), "terminal receipt #{number}")
  captured = live.fetch("terminal_dependencies").fetch(number.to_s)
  fail!("terminal receipt mismatch for #{number}") unless receipt["issue"] == number && receipt["repository"] == REPOSITORY && receipt["disposition"] == "merged" && receipt["issue_state"] == "closed_by_merged_pr" && captured["merge_sha"] == receipt["merge_sha"] && captured["terminal_digest"] == receipt["digest"]
  run!("git", "merge-base", "--is-ancestor", receipt.fetch("merge_sha"), target)
end
worktrees = run!("git", "worktree", "list", "--porcelain")
fail!("dependency worktree still registered") if worktrees.match?(%r{(?:branch refs/heads/codex/(?:312|10)-|worktree .*/adl-issue-(?:312|10)-)})
reports = findings["specialists"]
fail!("specialist roster mismatch") unless reports.is_a?(Array) && reports.map { |row| row["lane"] }.sort == roster
reports.each do |row|
  %w[reviewer_identity report_path report_sha256 target_sha].each { |field| fail!("#{row['lane']} #{field} missing") unless row[field].is_a?(String) && !row[field].strip.empty? }
  fail!("specialist target mismatch") unless row["target_sha"] == target
  safe_path!(row["report_path"])
  fail!("specialist report digest mismatch") unless Digest::SHA256.file(row["report_path"]).hexdigest == row["report_sha256"]
  count = row["finding_count"]
  fail!("invalid finding count") unless count.is_a?(Integer) && count >= 0
  if count.zero?
    fail!("zero-finding rationale missing") if row["zero_findings_rationale"].to_s.strip.empty?
    fail!("zero-finding coverage missing") unless row["coverage_refs"].is_a?(Array) && !row["coverage_refs"].empty?
  end
end
rows = findings["findings"]
fail!("findings array missing") unless rows.is_a?(Array)
fail!("specialist counts do not reconcile") unless reports.sum { |row| row["finding_count"] } == rows.length
required = %w[id severity evidence invariant reproduction_or_proof_gap recommendation owner disposition source_lane]
ids = rows.map { |row| row["id"] }
fail!("duplicate finding IDs") unless ids.uniq.length == ids.length
rows.each do |row|
  fail!("bad finding") unless required.all? { |key| row[key].is_a?(String) && !row[key].strip.empty? }
  fail!("invalid severity") unless %w[P0 P1 P2 P3].include?(row["severity"])
  fail!("invalid disposition") unless %w[open disputed accepted_risk duplicate resolved].include?(row["disposition"])
  repo_path!(row["evidence"].split(":", 2).first)
  fail!("accepted risk lacks authority") if row["disposition"] == "accepted_risk" && row["authority"].to_s.strip.empty?
end
findings.fetch("duplicates", []).each { |row| fail!("duplicate references unknown finding") unless ids.include?(row["canonical_id"]) && row.fetch("duplicate_ids").all? { |id| ids.include?(id) } }
findings.fetch("disagreements", []).each do |row|
  fail!("disagreement finding missing") unless row.fetch("finding_ids").all? { |id| ids.include?(id) }
  fail!("disagreement rationale missing") if row["rationale"].to_s.strip.empty?
end
if require_meta_review
  fail!("Gemini receipt verification failed") unless system("python3", ".csdlc/prepared/issues/313/run_gemini_meta_review.py", "--verify-receipt")
  quality = read_json!(File.join(root, "quality-evaluation/review_quality_evaluation.json"), "quality evaluation")
  redaction = read_json!(File.join(root, "redaction-audit/redaction_report.json"), "redaction audit")
  fail!("review quality gate did not pass") unless quality["status"] == "pass" && quality["blocking_issues"] == [] && quality["warnings"] == []
  fail!("redaction audit did not pass") unless redaction["status"] == "pass" && redaction.dig("counts", "blocker") == 0 && redaction.dig("counts", "warning") == 0
end
suffix = require_meta_review ? ", provider provenance, quality, and redaction gates" : ""
puts "PASS: repository identity, exact target, dependencies, packet closure, nine specialist lanes, and findings truth#{suffix}"
