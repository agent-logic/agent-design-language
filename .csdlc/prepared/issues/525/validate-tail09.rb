#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
require "digest"
ROOT = File.expand_path("../../../..", __dir__)
REPORT = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-09/planning-review.json")
REQUIRED_CHECKS = %w[scope dependencies single_result_units deferrals closeout_order operator_gates unsupported_claims].freeze
SEVERITIES = %w[P0 P1 P2 P3].freeze
DISPOSITIONS = %w[open fixed deferred accepted_nonblocking].freeze

def blob_digest(revision, path)
  bytes = IO.popen(["git", "-C", ROOT, "show", "#{revision}:#{path}"], err: File::NULL, &:read)
  return nil unless $CHILD_STATUS.success?
  Digest::SHA256.hexdigest(bytes)
end

def errors_for(report, expected_paths)
  errors = []
  revision = report["reviewed_revision"].to_s
  predecessor = report.fetch("predecessor", {})
  errors << "invalid reviewed revision" unless revision.match?(/\A[0-9a-f]{40}\z/)
  errors << "wrong #524 predecessor" unless predecessor["issue"] == 524 && predecessor["reviewed"] == true && predecessor["merged"] == true
  errors << "invalid #524 merge" unless predecessor["merge_sha"].to_s.match?(/\A[0-9a-f]{40}\z/)
  errors << "review must be independent and read-only" unless report["independent"] == true && report["read_only"] == true
  errors << "review status invalid" unless report["status"] == "final"
  errors << "review denominator mismatch" unless report["reviewed_paths"] == expected_paths
  digests = report.fetch("path_sha256", {})
  errors << "digest denominator mismatch" unless digests.keys.sort == expected_paths
  errors << "invalid content digest" unless digests.values.all? { |value| value.to_s.match?(/\A[0-9a-f]{64}\z/) }
  checks = report.fetch("semantic_checks", {})
  errors << "semantic check denominator incomplete" unless checks.keys.sort == REQUIRED_CHECKS.sort && checks.values.all? { |value| value == "reviewed" }
  validators = report.fetch("validator_results", [])
  errors << "validator evidence absent or failed" if validators.empty? || validators.any? { |row| row["status"] != "passed" || !row["argv"].is_a?(Array) || row["argv"].empty? || !row["evidence_path"].to_s.start_with?("docs/milestones/v0.92.1/evidence/release/tail-09/") || !row["evidence_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/) }
  findings = report.fetch("findings", [])
  findings.each_with_index do |row, index|
    errors << "finding #{index + 1} invalid severity" unless SEVERITIES.include?(row["severity"])
    errors << "finding #{index + 1} invalid owner" unless row["owner"].to_s.match?(/\A(?:issue:#(?:523|524)|operator|deferred:[A-Z0-9-]+)\z/)
    errors << "finding #{index + 1} invalid disposition" unless DISPOSITIONS.include?(row["disposition"])
    errors << "finding #{index + 1} incomplete" unless %w[id evidence].all? { |key| !row[key].to_s.strip.empty? }
  end
  unresolved = findings.count { |row| row["disposition"] == "open" && %w[P0 P1].include?(row["severity"]) }
  errors << "unresolved blocker count mismatch" unless report["unresolved_release_blockers"] == unresolved
  errors << "invalid review outcome" unless %w[pass changes_required].include?(report["outcome"])
  errors << "pass with open finding" if report["outcome"] == "pass" && findings.any? { |row| row["disposition"] == "open" }
  if report["outcome"] == "changes_required"
    route = report.fetch("remediation_route", {})
    target = route["target_issue"]
    errors << "invalid remediation target" unless [523, 524].include?(target)
    expected_reopen = [".adl/bin/csdlc-v2/csdlc-github-issue", "run", "--request", ".csdlc/evidence/525/reopen-#{target}.json"]
    expected_bind = [".adl/bin/csdlc-v2/csdlc-bind", "--root", ".", "--request", ".csdlc/evidence/525/rebind-#{target}.json"]
    errors << "typed reopen route missing" unless route["reopen_argv"] == expected_reopen
    errors << "typed rebind route missing" unless route["rebind_argv"] == expected_bind
    errors << "fresh #525 rereview missing" unless route["rereview_issue"] == 525 && route["requires_fresh_revision"] == true
  end
  errors
end

expected_paths = Dir.glob(File.join(ROOT, "docs/milestones/v0.92.2/**/*")).select { |p| File.file?(p) }.map { |p| p.delete_prefix("#{ROOT}/") }.sort
expected_paths << "docs/planning/ADL_FEATURE_LIST.md"
expected_paths.sort!

if ARGV == ["--negative"]
  revision = IO.popen(["git", "-C", ROOT, "rev-parse", "HEAD"], &:read).strip
  revision = "a" * 40 unless revision.match?(/\A[0-9a-f]{40}\z/)
  base = {"reviewed_revision" => revision, "predecessor" => {"issue" => 524, "reviewed" => true, "merged" => true, "merge_sha" => "b" * 40},
          "independent" => true, "read_only" => true, "status" => "final", "reviewed_paths" => expected_paths,
          "path_sha256" => expected_paths.to_h { |path| [path, "c" * 64] }, "semantic_checks" => REQUIRED_CHECKS.to_h { |key| [key, "reviewed"] },
          "validator_results" => [{"argv" => ["ruby", "validator.rb"], "status" => "passed", "evidence_path" => "docs/milestones/v0.92.1/evidence/release/tail-09/validator.log", "evidence_sha256" => "d" * 64}],
          "findings" => [], "unresolved_release_blockers" => 0, "outcome" => "pass"}
  mutations = [base.merge("predecessor" => base["predecessor"].merge("merged" => false)), base.merge("reviewed_paths" => []),
               base.merge("path_sha256" => {}), base.merge("semantic_checks" => {}), base.merge("validator_results" => []),
               base.merge("status" => "superseded"), base.merge("validator_results" => [base["validator_results"].first.merge("status" => "failed")]),
               base.merge("findings" => [{"id" => "F1", "severity" => "critical", "evidence" => "x", "owner" => "nobody", "disposition" => "ignored"}]),
               base.merge("outcome" => "changes_required", "findings" => [{"id" => "F1", "severity" => "P1", "evidence" => "x", "owner" => "issue:#523", "disposition" => "open"}], "unresolved_release_blockers" => 1)]
  abort "negative mutation escaped" unless mutations.all? { |row| !errors_for(row, expected_paths).empty? }
  puts "issue 525 negative contract passed (#{mutations.length} mutations)"
  exit 0
end

abort "missing planning review report" unless File.file?(REPORT) && !File.zero?(REPORT)
report = JSON.parse(File.read(REPORT))
revision = report["reviewed_revision"].to_s
expected_paths = IO.popen(["git", "-C", ROOT, "ls-tree", "-r", "--name-only", revision, "--", "docs/milestones/v0.92.2", "docs/planning/ADL_FEATURE_LIST.md"], err: File::NULL, &:read).lines.map(&:strip).reject(&:empty?).sort
errors = errors_for(report, expected_paths)
merge_sha = report.dig("predecessor", "merge_sha")
errors << "reviewed revision does not resolve" unless revision && system("git", "-C", ROOT, "cat-file", "-e", "#{revision}^{commit}", out: File::NULL, err: File::NULL)
errors << "#524 merge is not ancestral to reviewed revision" unless merge_sha && revision && system("git", "-C", ROOT, "merge-base", "--is-ancestor", merge_sha, revision, out: File::NULL, err: File::NULL)
report.fetch("path_sha256", {}).each do |path, digest|
  errors << "reviewed blob digest mismatch: #{path}" unless blob_digest(revision, path) == digest
  errors << "reviewed content is not current: #{path}" unless File.file?(File.join(ROOT, path)) && Digest::SHA256.file(File.join(ROOT, path)).hexdigest == digest
end
report.fetch("validator_results", []).each do |row|
  path = row["evidence_path"].to_s
  full = File.join(ROOT, path)
  errors << "validator evidence missing or empty: #{path}" unless File.file?(full) && !File.zero?(full)
  errors << "validator evidence digest mismatch: #{path}" unless File.file?(full) && Digest::SHA256.file(full).hexdigest == row["evidence_sha256"]
end
status = IO.popen(["git", "-C", ROOT, "status", "--porcelain"], &:read)
errors << "review validation requires a clean exact tree" unless status.empty?
abort errors.join("\n") unless errors.empty?
puts "issue 525 exact-revision review passed (#{report.fetch('findings').length} findings)"
