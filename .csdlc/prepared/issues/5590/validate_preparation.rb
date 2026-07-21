#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

issue = 5590
kinds = %w[sip stp spp vpp srp sor]
cards = kinds.to_h do |kind|
  path = ".csdlc/issues/#{issue}/cards/#{kind}.values.json"
  [kind, JSON.parse(File.read(path)).fetch("content").fetch("values")]
end

abort "expected six cards" unless cards.length == 6
generations = kinds.map do |kind|
  JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{kind}.values.json")).dig("identity", "generation")
end
abort "card generation mismatch" unless generations.uniq.length == 1

expected = (1..8).map { |number| "AC-#{number}" }
acceptance = cards.fetch("stp").fetch("acceptance_criteria").map { |value| value[/AC-\d+/] }.uniq.sort
step_coverage = cards.fetch("spp").fetch("steps").flat_map { |step| step.fetch("acceptance_ids") }.uniq.sort
lane_coverage = cards.fetch("vpp").fetch("lanes").flat_map { |lane| lane.fetch("acceptance_ids") }.uniq.sort
abort "acceptance set incomplete" unless acceptance == expected
abort "SPP coverage incomplete" unless step_coverage == expected
abort "VPP coverage incomplete" unless lane_coverage == expected
abort "deferred validation lane" if cards.fetch("vpp").fetch("lanes").any? { |lane| lane["defer_reason"] }

claim = JSON.parse(File.read(".csdlc/issues/#{issue}/index.json")).fetch("claim")
expected_paths = [
  ".csdlc/evidence/5590",
  ".csdlc/issues/5590",
  ".csdlc/locks/5590.lock",
  ".csdlc/prepared/issues/5590"
]
abort "claim is not preparation-only" unless claim.fetch("protected_paths").sort == expected_paths.sort
abort "claim purpose omits implementation gate" unless claim.fetch("purpose").include?("without product edits")

design = File.read(".csdlc/prepared/issues/5590/design.md")
diagram = File.read(".csdlc/prepared/issues/5590/diagram.mmd")
matrix = File.read(".csdlc/prepared/issues/5590/security-acceptance-matrix.md")
%w[HTTPS WebSocket Observatory guardian Vector rollback 20997 Runtime\ v2].each do |term|
  normalized = term.tr("\\", "")
  abort "design missing #{normalized}" unless design.include?(normalized)
end
abort "diagram incomplete" unless %w[guardian https websocket observatory vector rollback].all? { |term| diagram.downcase.include?(term) }
abort "matrix incomplete" unless expected.all? { |id| matrix.include?(id) }

forbidden = %w[adl-runtime adl-runtime-kernel infra/runtime-v3 demos/v0.91.7/html-observatory]
abort "product path protected prematurely" if claim.fetch("protected_paths").any? { |path| forbidden.include?(path) }

puts "cards=6 generation=#{generations.first} acceptance=8 steps=complete lanes=complete deferrals=0 claim=preparation-only"
