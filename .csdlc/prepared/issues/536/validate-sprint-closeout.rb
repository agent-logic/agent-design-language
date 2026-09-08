#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../..", __dir__)
evidence_path = File.join(root, "docs/milestones/v0.92.1/evidence/sprint-8/SPRINT_8_CLOSEOUT_READINESS.md")
evidence = File.read(evidence_path)

children = [51, 261, 262, 263, 264, 342, 511, 512]
children.each { |issue| abort "missing child ##{issue}" unless evidence.include?("| ##{issue} |") }
abort "missing #671 residual routing" unless evidence.include?("follow-up #671")
abort "missing no-PR disposition truth" unless evidence.scan("operator-approved no-PR disposition").length == 2

merges = %w[
  7f04298f87e2bde5b90eb174d9d7758067d348d0
  6e01e2bbe40915e814e43e54f84dfb61c84601e3
  e13b5db0b49f9bc6772ca2765634a852abdf1ed2
  bdbf8aa32620da0e277bf3e2ed5f272354021744
  b381edce8020567c4ac5af03f5df062157f55a16
  af5f8036ab7a0619751ab55fa9bd4891f377cd9d
]
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
