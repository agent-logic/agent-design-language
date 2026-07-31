#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

issue = 5591
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

acceptance = cards.fetch("stp").fetch("acceptance_criteria").map { |value| value[/AC-\d+/] }.uniq.sort
expected = (1..8).map { |number| "AC-#{number}" }
abort "acceptance set incomplete" unless acceptance == expected

step_coverage = cards.fetch("spp").fetch("steps").flat_map { |step| step.fetch("acceptance_ids") }.uniq.sort
lane_coverage = cards.fetch("vpp").fetch("lanes").flat_map { |lane| lane.fetch("acceptance_ids") }.uniq.sort
abort "SPP coverage incomplete" unless step_coverage == expected
abort "VPP coverage incomplete" unless lane_coverage == expected
abort "deferred validation lane" if cards.fetch("vpp").fetch("lanes").any? { |lane| lane["defer_reason"] }

constraints = cards.fetch("sip").fetch("operator_constraints").join("\n")
dependencies = cards.fetch("stp").fetch("dependencies").join("\n")
review_scope = cards.fetch("srp").fetch("review_scope")
exact_head = "8fa1bfe66e677ed3ae160b3fee81d204d4211a37"
abort "stacked head missing from constraints" unless constraints.include?(exact_head)
abort "stacked head missing from dependencies" unless dependencies.include?(exact_head)
abort "merge publication gate missing" unless dependencies.include?("publication, merge, integrated readiness")
abort "full review scope missing" unless review_scope.include?("AC-1 through AC-8") && review_scope.include?("positive and negative")

design = File.read(".csdlc/prepared/issues/5591/design.md")
diagram = File.read(".csdlc/prepared/issues/5591/diagram.mmd")
%w[guardian checkpoint replay resume pressure TLS Observatory Runtime\ v2].each do |term|
  normalized = term.tr("\\", "")
  abort "design missing #{normalized}" unless design.include?(normalized)
end
diagram_lower = diagram.downcase
abort "diagram omits exact outcome" unless %w[guardian checkpoint replay observatory].all? { |term| diagram_lower.include?(term) }

puts "cards=6 generation=#{generations.first} acceptance=8 spp=complete vpp=complete deferrals=0 base=#{exact_head}"
