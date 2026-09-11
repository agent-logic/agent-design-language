#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"

ROOT = File.expand_path("../../../..", __dir__)
HANDOFF = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_HANDOFF.md")
REPORT = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_REPORT.md")
EXPECTED = "9c7e57d412d61898bd44ab00d53e31afbb779e5c"

def validate(text, expected)
  failures = []
  failures << "candidate must be 40 lowercase hexadecimal characters" unless expected.match?(/\A[0-9a-f]{40}\z/)
  failures << "candidate missing from handoff" unless text.include?(expected)
  failures << "candidate must be named explicitly" unless text.include?("Candidate commit: \`#{expected}\`")
  failures << "clean detached checkout requirement missing" unless text.match?(/detached review\s+checkout must be clean/)
  failures << "candidate drift rule missing" unless text.include?("candidate-drift result")
  failures << "#522 closure gate missing" unless text.include?("Issue #522 must remain open")
  failures << "mutable-target prohibition missing" unless text.include?("Do not infer the target")
  failures
end

def validate_report_binding(assignment_sha, report_sha)
  return [] if assignment_sha == report_sha

  ["assignment/report candidate SHA mismatch"]
end

def report_candidate_shas(text)
  text.scan(/^Candidate commit:\s*`?([0-9a-f]{40})`?\s*$/).flatten
end

abort("handoff missing") unless File.file?(HANDOFF)
text = File.read(HANDOFF)
failures = validate(text, EXPECTED)

_, status = Open3.capture2e("git", "-C", ROOT, "cat-file", "-e", "#{EXPECTED}^{commit}")
failures << "candidate commit is not available locally" unless status.success?

negative_cases = {
  "missing" => text.gsub(EXPECTED, ""),
  "malformed" => text.gsub(EXPECTED, "not-a-sha"),
  "substituted" => text.gsub(EXPECTED, "1" * 40),
  "mutable" => text.gsub("Do not infer the target", "Infer the target")
}
negative_cases.each do |name, candidate_text|
  abort("negative case #{name} was accepted") if validate(candidate_text, EXPECTED).empty?
end

abort("matching assignment/report SHA was rejected") unless validate_report_binding(EXPECTED, EXPECTED).empty?
abort("drifting assignment/report SHA was accepted") if validate_report_binding(EXPECTED, "2" * 40).empty?
parser_positive = "Evidence commit: #{'2' * 40}\nCandidate commit: #{EXPECTED}\n"
abort("explicit candidate parser selected unrelated SHA") unless report_candidate_shas(parser_positive) == [EXPECTED]
parser_conflict = "Candidate commit: #{EXPECTED}\nCandidate commit: #{'2' * 40}\n"
abort("conflicting candidate fields were accepted") unless report_candidate_shas(parser_conflict).length != 1

if File.file?(REPORT)
  report_text = File.read(REPORT)
  report_shas = report_candidate_shas(report_text)
  failures << "returned report must contain exactly one explicit Candidate commit field" unless report_shas.length == 1
  failures.concat(validate_report_binding(EXPECTED, report_shas.first)) if report_shas.length == 1
elsif ENV["CSDLC_REQUIRE_EXTERNAL_REPORT"] == "1"
  failures << "required external review report is missing"
end

if failures.empty?
  phase = File.file?(REPORT) ? "handoff and report bound" : "phase-one handoff; report pending"
  puts "PASS: #{phase}; 7 negative cases rejected"
else
  abort(failures.join("\n"))
end
