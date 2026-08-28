#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

ROOT = File.expand_path("../../../..", __dir__)

def read(path)
  File.read(File.join(ROOT, path))
end

def json(path)
  JSON.parse(read(path))
end

def assert(condition, message)
  raise message unless condition
end

issue = json(".csdlc/issues/505/index.json")
stp = read(".csdlc/issues/505/cards/stp.md")
sip = read(".csdlc/issues/505/cards/sip.md")
spp = json(".csdlc/issues/505/cards/spp.values.json")
vpp = json(".csdlc/issues/505/cards/vpp.values.json")
srp = json(".csdlc/issues/505/cards/srp.values.json")
design = read(".csdlc/prepared/issues/505/design.md")
diagram = read(".csdlc/prepared/issues/505/diagram.mmd")
packet_text = [stp, sip, design, diagram].join("\n")

assert(issue["issue"] == 505, "wrong issue")
assert(issue["repository"] == "agent-logic/agent-design-language", "wrong repository")
assert(issue["phase"] == "initialized", "pre-bind validator expects initialized #505")
assert(issue["branch"].nil? && issue["worktree"].nil?, "pre-bind #505 must remain unbound")

[
  "Requirements #179 and #180 are mapped",
  "v2-v3 parity is measured",
  "Canary rollback is exercised",
  "Cutover and retirement require operator approval"
].each { |text| assert(stp.include?(text), "missing acceptance text: #{text}") }

deps = spp.dig("content", "values", "steps").to_s + stp + design
assert(deps.include?("#504"), "missing #504 dependency")
assert(deps.include?("terminal") && deps.include?("ancestral"), "missing terminal/ancestral dependency language")
assert(packet_text.include?("Closes #505"), "missing visible future closing-linkage requirement")
assert(packet_text.include?("C-SDLC v2 remains") || packet_text.include?("v2 remains"), "missing v2 live-authority boundary")
assert(packet_text.include?("operator approval"), "missing explicit operator approval gate")
assert(packet_text.include?("No silent v2 retirement") || packet_text.include?("Silent v2 retirement"), "missing no-silent-retirement boundary")
assert(diagram.include?("Rollback exercise"), "missing rollback diagram node")
assert(diagram.include?("Observation evidence"), "missing observation diagram node")

lanes = vpp.dig("content", "values", "lanes")
assert(lanes.is_a?(Array) && lanes.length == 1, "initialized #505 should expose exactly one executable pre-bind lane")
lane = lanes.first
assert(lane["lane"] == "prebind-v3-f-preparation", "unexpected pre-bind lane")
assert(lane["argv"] == ["ruby", ".csdlc/prepared/issues/505/validate-authority-transition-prep.rb"], "pre-bind lane must target this validator")
assert(lane["defer_reason"].nil?, "pre-bind validator must be executable, not deferred")

review_prompts = srp.dig("content", "values", "review_prompts") || []
combined = review_prompts.join("\n") + "\n" + packet_text
["#504", "#179", "#180", "Closes #505"].each do |needle|
  assert(combined.include?(needle), "missing review/planning prompt marker #{needle}")
end

puts JSON.generate(
  {
    schema: "adl.csdlc_v3.issue505.prebind_validation.v1",
    status: "pass",
    issue: 505,
    phase: issue["phase"],
    checked: [
      "acceptance_denominator",
      "504_terminal_dependency",
      "v2_live_authority_boundary",
      "no_silent_v2_retirement",
      "operator_approval_gate",
      "future_closing_linkage",
      "single_executable_prebind_lane"
    ]
  }
)
