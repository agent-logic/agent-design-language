#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../..", __dir__)
evidence_path = File.join(root, "docs/milestones/v0.92.1/evidence/sprint-8/SPRINT_8_CLOSEOUT_READINESS.md")
evidence = File.read(evidence_path)

rows = evidence.lines.map do |line|
  match = line.match(/^\| #(\d+) \| (.+) \| (.+) \|$/)
  [Integer(match[1]), match[2], match[3]] if match
end.compact
expected = {
  51 => ["Closed as the podcast coordination parent.", "operator-approved no-PR disposition"],
  261 => ["Closed and merged after independent review with no findings.", "PR #611", "3067d90cc54e94d40d6a988672b09314e9ab272b", "78af9095a16886f8c0876e139113620dca806984", "3 passed, 14 skipped, 0 failed", "7f04298f87e2bde5b90eb174d9d7758067d348d0"],
  262 => ["Closed and merged after independent review with no findings.", "PR #618", "572fe15253b5c91d36bbfd24c63a2f4692451c8e", "842b7d57d09b66a6f9ea9a43a4e5d02be43aa3bb", "9 passed, 8 skipped, 0 failed", "6e01e2bbe40915e814e43e54f84dfb61c84601e3"],
  263 => ["Closed and merged after independent review with no findings.", "PR #626", "ee61ef40d7e7862b172e848a4f89eca52977715c", "a77e8e6c8e6e3c59330a5c15ce45924985735b7c", "3 passed, 14 skipped, 0 failed", "e13b5db0b49f9bc6772ca2765634a852abdf1ed2"],
  264 => ["Closed and merged after independent review with no findings.", "PR #649", "9c944965116eccf989b50198f1f13b7daf2da9a4", "a285a690b86f95f6fc3ea3dd150ded76b32c469b", "3 passed, 14 skipped, 0 failed", "bdbf8aa32620da0e277bf3e2ed5f272354021744"],
  342 => ["Closed and merged after independent review with no findings.", "PR #586", "176b7b4e4250766562c3e911eb28fb02bd15bf7e", "822c81e9d0ad15e479960de542d419e64c80e1f9", "4 passed, 12 skipped, 0 failed", "b381edce8020567c4ac5af03f5df062157f55a16"],
  511 => ["Closed as absorbed into #512", "operator-approved no-PR disposition"],
  512 => ["Closed and merged after independent review of the substantive Observatory implementation.", "PR #719", "a57e551a14089a6ed53de05f7ff88041880d175e", "e8a0e0b9bb6a18687a5a2dc9e85b8130bbb182b4", "9 checks passed, 8 policy-declared lanes skipped, 0 failed", "af5f8036ab7a0619751ab55fa9bd4891f377cd9d"]
}
abort "wrong child row set" unless rows.map(&:first).sort == expected.keys.sort
children = expected.keys
rows.each do |issue, disposition, integration|
  combined = "#{disposition} #{integration}"
  expected.fetch(issue).each { |fragment| abort "wrong mapping for ##{issue}: #{fragment}" unless combined.include?(fragment) }
end
abort "missing #51 residual routing" unless rows.assoc(51).join(" ").include?("#671")

merges = expected.values.flatten.grep(/\A[0-9a-f]{40}\z/).select do |sha|
  evidence.include?("merge `#{sha}`")
end
abort "wrong merge denominator" unless merges.length == 6
merges.each do |sha|
  system("git", "-C", root, "merge-base", "--is-ancestor", sha, "origin/main") || abort("merge not ancestral: #{sha}")
end

puts JSON.generate(
  schema: "adl.sprint8.closeout_contract.v1",
  status: "passed",
  sprint_issue: 536,
  children: children,
  ancestral_merges: merges.length,
  residual_issue: 671
)
