#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
ROOT = File.expand_path("../../../..", __dir__)
REPORT = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-09/planning-review.json")
REQUIRED_CHECKS = %w[scope dependencies single_result_units deferrals closeout_order operator_gates unsupported_claims].freeze

def report_errors(report, expected_paths)
  errors = []
  revision = report["revision"].to_s
  errors << "invalid exact revision" unless revision.match?(/\A[0-9a-f]{40}\z/)
  errors << "checkout dirt state absent" unless [true, false].include?(report["checkout_clean"])
  errors << "review must be independent and read-only" unless report["independent"] == true && report["read_only"] == true
  errors << "review denominator mismatch" unless report["reviewed_paths"] == expected_paths
  checks = report.fetch("semantic_checks", {})
  errors << "semantic check denominator incomplete" unless REQUIRED_CHECKS.all? { |key| checks[key] == "reviewed" }
  validators = report.fetch("validator_results", [])
  errors << "validator evidence absent" if validators.empty? || validators.any? { |row| row["argv"].to_s.empty? || row["status"].to_s.empty? }
  findings = report.fetch("findings", [])
  findings.each_with_index do |row, index|
    required = %w[id severity evidence owner disposition]
    errors << "finding #{index + 1} incomplete" unless required.all? { |key| !row[key].to_s.strip.empty? }
  end
  unresolved = findings.count { |row| %w[open accepted_unfixed].include?(row["disposition"]) && %w[P0 P1].include?(row["severity"]) }
  errors << "unresolved blocker count mismatch" unless report["unresolved_release_blockers"] == unresolved
  outcome = report["outcome"]
  errors << "invalid review outcome" unless %w[pass changes_required].include?(outcome)
  errors << "pass with unresolved blocker" if outcome == "pass" && unresolved.positive?
  errors
end

expected_paths = Dir.glob(File.join(ROOT, "docs/milestones/v0.92.2/**/*")).select { |p| File.file?(p) }.map { |p| p.delete_prefix("#{ROOT}/") }.sort
expected_paths << "docs/planning/ADL_FEATURE_LIST.md"
expected_paths.sort!

if ARGV == ["--negative"]
  good = {"revision" => "a" * 40, "checkout_clean" => true, "independent" => true, "read_only" => true,
          "reviewed_paths" => expected_paths, "semantic_checks" => REQUIRED_CHECKS.to_h { |k| [k, "reviewed"] },
          "validator_results" => [{"argv" => "ruby validator.rb", "status" => "passed"}], "findings" => [],
          "unresolved_release_blockers" => 0, "outcome" => "pass"}
  mutations = [good.merge("revision" => "main"), good.merge("reviewed_paths" => []), good.merge("semantic_checks" => {}),
               good.merge("validator_results" => []), good.merge("independent" => false),
               good.merge("findings" => [{"id" => "F1", "severity" => "P1", "evidence" => "x", "owner" => "#523", "disposition" => "open"}], "unresolved_release_blockers" => 0)]
  abort "negative mutation escaped" unless mutations.all? { |row| !report_errors(row, expected_paths).empty? }
  puts "issue 525 negative contract passed (#{mutations.length} mutations)"
  exit 0
end

abort "missing planning review report" unless File.file?(REPORT) && !File.zero?(REPORT)
report = JSON.parse(File.read(REPORT))
errors = report_errors(report, expected_paths)
unless report["revision"].to_s.empty?
  errors << "review revision does not resolve" unless system("git", "cat-file", "-e", "#{report['revision']}^{commit}", out: File::NULL, err: File::NULL)
end
abort errors.join("\n") unless errors.empty?
abort "planning package semantics failed" unless system("ruby", ".csdlc/prepared/issues/523/validate-tail07.rb")
puts "issue 525 exact-revision review semantics passed (#{report.fetch('findings').length} findings)"
