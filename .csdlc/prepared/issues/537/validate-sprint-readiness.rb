#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "yaml"

root = File.expand_path("../../../..", __dir__)
packet_path = File.join(root, ".csdlc/prepared/issues/537/sprint-execution-packet.yaml")
abort "missing Sprint 9 execution packet" unless File.file?(packet_path)

packet = YAML.safe_load(File.read(packet_path), permitted_classes: [], aliases: false)
expected = [515, 516, 517, 518, 519]

abort "wrong sprint issue" unless packet.fetch("sprint_issue") == 537
abort "wrong execution mode" unless packet.fetch("execution_mode") == "sequential"
abort "wrong ordered membership" unless packet.fetch("ordered_issue_numbers") == expected
abort "wrong membership version" unless packet.fetch("current_membership_version") == 4

expected.each do |issue|
  issue_root = File.join(root, ".csdlc/issues/#{issue}")
  abort "missing issue #{issue} index" unless File.file?(File.join(issue_root, "index.json"))
  %w[sip stp spp vpp srp sor].each do |card|
    abort "missing issue #{issue} #{card} values" unless File.file?(File.join(issue_root, "cards/#{card}.values.json"))
    abort "missing issue #{issue} #{card} projection" unless File.file?(File.join(issue_root, "cards/#{card}.md"))
  end
end

gates = packet.fetch("serial_gates").join("\n")
abort "missing issue 514 predecessor gate" unless gates.include?("issue 514 before issue 515")
abort "missing release-tail admission gate" unless gates.include?("all named issue 516 roots before issue 516")
abort "missing quality gate" unless gates.include?("issue 516 before issue 517")
abort "missing docs gate" unless gates.include?("issue 517 before issue 518")
abort "missing publication-candidate gate" unless gates.include?("issue 518 before issue 519")

non_goals = packet.fetch("non_goals").join("\n").downcase
%w[merge tag release].each do |term|
  abort "issue 519 must exclude #{term}" unless non_goals.include?(term)
end

puts JSON.generate({
  schema: "adl.sprint9.readiness_contract.v1",
  status: "passed",
  sprint_issue: 537,
  membership_version: 4,
  ordered_issue_numbers: expected,
  packet_sha256: Digest::SHA256.file(packet_path).hexdigest
})
