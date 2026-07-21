#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ISSUES = [5499, 5349].freeze
REPOSITORY = "danielbaustin/agent-design-language"

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort("command failed: #{argv.join(' ')}\n#{stderr}") unless status.success?
  stdout.strip
end

root = Pathname.new(capture!("git", "rev-parse", "--show-toplevel"))
common = Pathname.new(capture!("git", "rev-parse", "--path-format=absolute", "--git-common-dir"))
path_inventory = JSON.parse(root.join(".csdlc", "prepared", "issues", "5498", "planned-path-sets.json").read)
confirmations = path_inventory.fetch("confirmations")

failures = ISSUES.map do |issue|
  receipt_path = common.join("csdlc-v2", "closeout", "#{issue}.json")
  next "##{issue}: missing retained closeout receipt" unless receipt_path.file?

  receipt = JSON.parse(receipt_path.read)
  record = receipt["record"] || {}
  phase = receipt.dig("record", "phase") || receipt["phase"]
  terminal = receipt.dig("record", "terminal") || receipt["terminal"] || {}
  disposition = terminal["disposition"] || receipt.dig("terminal", "disposition")
  merged_sha = terminal["observed_sha"] || receipt.dig("terminal", "observed_sha")

  expected_ref = "csdlc-v2/closeout/#{issue}.json"
  next "##{issue}: wrong receipt schema" unless receipt["schema"] == "csdlc.terminal_receipt.v1"
  next "##{issue}: receipt issue identity mismatch" unless receipt["issue"] == issue && record["issue"] == issue
  next "##{issue}: receipt repository mismatch" unless receipt["repository"] == REPOSITORY && record["repository"] == REPOSITORY
  next "##{issue}: receipt reference mismatch" unless receipt["receipt_ref"] == expected_ref && terminal["receipt_path"] == expected_ref
  receipt_init = receipt["initialization_digest"].to_s
  record_init = record["initialization_digest"].to_s
  next "##{issue}: initialization identity is absent" if receipt_init.empty? || record_init.empty?
  next "##{issue}: initialization identity mismatch" unless receipt_init == record_init
  next "##{issue}: terminal record digest is absent" if record["digest"].to_s.empty?
  next "##{issue}: terminal claim was not released" unless record.key?("claim") && record["claim"].nil?
  next "##{issue}: receipt phase is #{phase.inspect}, expected closed_out" unless phase == "closed_out"
  next "##{issue}: terminal disposition is #{disposition.inspect}, expected merged" unless disposition == "merged"
  next "##{issue}: terminal merged SHA is absent" if merged_sha.to_s.empty?

  _out, _err, status = Open3.capture3("git", "merge-base", "--is-ancestor", merged_sha, "HEAD", chdir: root.to_s)
  next "##{issue}: merged SHA #{merged_sha} is not ancestral to HEAD" unless status.success?

  nil
end.compact

confirmations.each do |issue, state|
  failures << "##{issue}: adjacent planned-path owner confirmation is #{state.inspect}, expected confirmed" unless state == "confirmed"
end

if failures.empty?
  puts JSON.generate(status: "ready", issues: ISSUES, conductor_gate: 5499, interface_gate: 5349, path_confirmations: confirmations)
  exit 0
end

puts JSON.generate(status: "waiting", issues: ISSUES, conductor_gate: 5499, interface_gate: 5349, path_confirmations: confirmations, blockers: failures)
exit 3
