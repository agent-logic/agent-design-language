#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "English"

root = `git rev-parse --show-toplevel`.strip
abort("not in a Git worktree") unless $CHILD_STATUS.success?
common = `git rev-parse --path-format=absolute --git-common-dir`.strip
abort("cannot resolve Git common directory") unless $CHILD_STATUS.success?
receipt_path = File.join(common, "csdlc-v2/closeout/5339.json")
abort("BLOCKED: retained typed closeout receipt for #5339 is absent") unless File.file?(receipt_path)

receipt = JSON.parse(File.read(receipt_path))
record = receipt.fetch("record")
terminal = record.fetch("terminal")
abort("BLOCKED: #5339 retained typed phase is not closed_out") unless record.fetch("phase") == "closed_out"
abort("BLOCKED: #5339 terminal disposition is not merged") unless terminal.fetch("disposition") == "merged"
merged_sha = terminal.fetch("observed_sha")
abort("BLOCKED: #5339 merged SHA is absent") unless merged_sha.is_a?(String) && !merged_sha.empty?

system("git", "merge-base", "--is-ancestor", merged_sha, "HEAD", out: File::NULL, err: File::NULL)
abort("BLOCKED: #5338 branch does not contain #5339 merged revision #{merged_sha}") unless $CHILD_STATUS.success?

puts JSON.generate(schema: "adl.csdlc.dependency-gate.v1", dependency_issue: 5339, phase: "closed_out", disposition: "merged", merged_sha: merged_sha, outcome: "passed")
