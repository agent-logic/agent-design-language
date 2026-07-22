#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ISSUES = [5498, 5349].freeze

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort("command failed: #{argv.join(' ')}\n#{stderr}") unless status.success?
  stdout.strip
end

root = Pathname.new(capture!("git", "rev-parse", "--show-toplevel"))
common = Pathname.new(capture!("git", "rev-parse", "--path-format=absolute", "--git-common-dir"))

failures = ISSUES.map do |issue|
  receipt_path = common.join("csdlc-v2", "closeout", "#{issue}.json")
  next "##{issue}: missing retained closeout receipt" unless receipt_path.file?

  receipt = JSON.parse(receipt_path.read)
  record = receipt["record"] || receipt
  phase = record["phase"]
  terminal = record["terminal"] || {}
  disposition = terminal["disposition"]
  merged_sha = terminal["observed_sha"]
  claim = record["claim"]

  next "##{issue}: receipt phase is #{phase.inspect}, expected closed_out" unless phase == "closed_out"
  next "##{issue}: active claim remains in terminal receipt" unless claim.nil?
  next "##{issue}: terminal disposition is #{disposition.inspect}, expected merged" unless disposition == "merged"
  next "##{issue}: terminal merged SHA is absent" if merged_sha.to_s.empty?

  _out, _err, status = Open3.capture3("git", "merge-base", "--is-ancestor", merged_sha, "HEAD", chdir: root.to_s)
  next "##{issue}: merged SHA #{merged_sha} is not ancestral to HEAD" unless status.success?

  nil
end.compact

if failures.empty?
  puts JSON.generate(status: "ready", issues: ISSUES, final_wp09_gate: 5349)
  exit 0
end

puts JSON.generate(status: "waiting", issues: ISSUES, final_wp09_gate: 5349, blockers: failures)
exit 3
