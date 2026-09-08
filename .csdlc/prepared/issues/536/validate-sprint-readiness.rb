#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "yaml"
require "digest"

root = File.expand_path("../../../..", __dir__)
packet_path = File.join(root, ".csdlc/prepared/issues/536/sprint-execution-packet.yaml")
packet = YAML.safe_load(File.read(packet_path), permitted_classes: [], aliases: false)

expected = [261, 342, 262, 263, 264, 511, 512, 51]
abort "wrong sprint issue" unless packet.fetch("sprint_issue") == 536
abort "wrong execution mode" unless packet.fetch("execution_mode") == "hybrid"
abort "wrong ordered membership" unless packet.fetch("ordered_issue_numbers") == expected
abort "wrong membership version" unless packet.fetch("current_membership_version") == 5

expected.each do |issue|
  issue_root = File.join(root, ".csdlc/issues/#{issue}")
  abort "missing issue #{issue} index" unless File.file?(File.join(issue_root, "index.json"))
  cards = %w[sip stp spp vpp srp sor]
  cards.each do |card|
    abort "missing issue #{issue} #{card} values" unless File.file?(File.join(issue_root, "cards/#{card}.values.json"))
    abort "missing issue #{issue} #{card} projection" unless File.file?(File.join(issue_root, "cards/#{card}.md"))
  end
end

serial = packet.fetch("serial_gates").join("\n")
abort "missing podcast identity gate" unless serial.include?("261 before issue 342")
abort "missing episode package gate" unless serial.include?("261 and 342 before issue 262")
abort "missing provider authorization gate" unless serial.include?("explicit provider-specific operator authorization")
abort "missing Observatory convergence gate" unless serial.include?("issue 511 before issue 512")
abort "issue 84 must not gate issue 512" if serial.match?(/84.*before issue 512/)

blocked = packet.fetch("candidate_parallel_lanes").select { |lane| lane.fetch("classification") == "blocked_until_dependency" }
abort "issue 512 must be blocked only on issue 511" unless blocked.any? { |lane| lane.fetch("issues") == [512] && lane.fetch("dependency_gates") == ["issue 511 reviewed terminal"] }
abort "issue 264 must be blocked" unless blocked.any? { |lane| lane.fetch("issues") == [264] && lane.fetch("dependency_gates").join(" ").include?("operator authorization") }

scope = packet.fetch("safe_parallel_lanes").flat_map { |lane| lane.fetch("issues") }
abort "opening lanes must contain 261 and 511" unless scope.include?(261) && scope.include?(511)

puts JSON.generate({
  schema: "adl.sprint8.readiness_contract.v1",
  status: "passed",
  sprint_issue: 536,
  membership_version: 5,
  ordered_issue_numbers: expected,
  packet_sha256: Digest::SHA256.file(packet_path).hexdigest,
  known_blockers: [
    { issue: 264, blocks: [264], disposition: "operator_authorization_required" }
  ]
})
