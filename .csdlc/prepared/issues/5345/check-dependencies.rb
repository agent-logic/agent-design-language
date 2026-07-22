#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ISSUES = [5339, 5338, 5340, 5342, 5341, 5349].freeze
ROOT = File.expand_path("../../../..", __dir__)

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort("command failed: #{argv.join(' ')}\n#{stderr}") unless status.success?
  stdout.strip
end

common_dir = File.expand_path(capture!("git", "rev-parse", "--git-common-dir"))
head = capture!("git", "rev-parse", "HEAD")

ISSUES.each do |issue|
  path = File.join(common_dir, "csdlc-v2", "closeout", "#{issue}.json")
  abort("missing retained closeout receipt for ##{issue}: #{path}") unless File.file?(path)

  receipt = JSON.parse(File.read(path))
  record = receipt.fetch("record")
  phase = record["phase"]
  publication = record.fetch("publication")
  readiness = record.fetch("readiness")
  terminal = record.fetch("terminal")
  disposition = terminal["disposition"]
  merged_sha = terminal["observed_sha"]
  tracked_path = File.join(ROOT, ".csdlc", "issues", issue.to_s, "index.json")

  abort("##{issue} receipt identity mismatch") unless receipt["issue"] == issue && receipt["repository"] == "danielbaustin/agent-design-language"
  abort("##{issue} receipt phase is #{phase.inspect}, expected closed_out") unless phase == "closed_out"
  abort("##{issue} retained record still has a claim") unless record["claim"].nil?
  abort("##{issue} disposition is #{disposition.inspect}, expected merged") unless disposition == "merged"
  abort("##{issue} publication does not retain merged GitHub truth") unless publication["issue"] == issue && publication["observed_state"] == "merged" && publication["pull_request"].is_a?(Integer)
  abort("##{issue} terminal publication identity differs") unless terminal["pull_request"] == publication["pull_request"] && terminal["observed_state"] == "merged"
  abort("##{issue} receipt has no merged SHA") unless merged_sha.is_a?(String) && merged_sha.match?(/\A[0-9a-f]{40}\z/)
  abort("##{issue} readiness does not bind the merged head") unless readiness["ready"] == true && readiness["head_sha"] == merged_sha
  required = readiness.fetch("checks").select { |check| check["requirement"] == "required" }
  abort("##{issue} has no required-check evidence") if required.empty?
  abort("##{issue} required checks are not all successful") unless required.all? { |check| check["conclusion"] == "success" }
  abort("##{issue} current tree lacks the terminal typed projection") unless File.file?(tracked_path)
  tracked = JSON.parse(File.read(tracked_path))
  abort("##{issue} retained receipt and current typed projection differ") unless tracked == record

  system("git", "merge-base", "--is-ancestor", merged_sha, head)
  abort("##{issue} merged SHA #{merged_sha} is not ancestral to HEAD #{head}") unless $?.success?
end

puts JSON.generate(
  schema: "adl.v0918.wp10_dependency_gate.v1",
  status: "pass",
  head: head,
  dependencies: ISSUES
)
