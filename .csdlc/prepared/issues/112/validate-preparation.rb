#!/usr/bin/env ruby
require "json"
ROOT = File.expand_path("../../../..", __dir__)
ISSUE_ROOT = File.join(ROOT, ".csdlc/issues/112")
PREP_ROOT = File.join(ROOT, ".csdlc/prepared/issues/112")
KINDS = %w[sip stp spp vpp srp sor].freeze
def fail!(message); warn(message); exit 1; end
index = JSON.parse(File.read(File.join(ISSUE_ROOT, "index.json")))
fail!("wrong issue") unless index.fetch("issue") == 112
fail!("wrong repository") unless index.fetch("repository") == "agent-logic/agent-design-language"
fail!("not ready") unless index.fetch("phase") == "ready"
fail!("design unapproved") unless index.dig("design_review", "approved")
KINDS.each { |kind| %W[#{kind}.md #{kind}.values.json].each { |name| fail!("missing #{name}") unless File.file?(File.join(ISSUE_ROOT, "cards", name)) } }
sip = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/sip.values.json")))
stp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/stp.values.json")))
expected_gate = ["Hard serial gate: #111 must be closed by a merged PR and ancestral to the execution base"]
fail!("serial gate differs from live #112 authority") unless stp.dig("content", "values", "dependencies") == expected_gate
transitions = index.fetch("transitions")
fail!("missing transition") if transitions.empty?
transitions.each_cons(2) { |left, right| fail!("discontinuous transitions") unless left.fetch("to") == right.fetch("from") }
fail!("transition phase drift") unless transitions.last.fetch("to") == index.fetch("phase")
fail!("stale dependency in transitions") if transitions.any? { |item| item.fetch("reason").match?(/#83|#113/) }
ready = transitions.select { |item| item.fetch("to") == "ready" }
fail!("expected one ready transition") unless ready.length == 1 && ready.first.fetch("reason").include?("#111")
vpp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/vpp.values.json")))
lanes = vpp.dig("content", "values", "lanes") || []
expected_lanes = %w[issue-112-preparation-hygiene layer8-authority-contract-plan layer8-runtime-api-integration-plan layer8-observatory-ui-plan]
fail!("lane drift") unless lanes.map { |lane| lane.fetch("lane") } == expected_lanes
fail!("not fail closed") unless vpp.dig("content", "values", "failure_policy").start_with?("Fail closed")
contracts = {
  "authority-contract" => ["cargo nextest run", "adl-runtime/Cargo.toml", "layer8_authority", "#111"],
  "runtime-api-integration" => ["cargo nextest run", "adl/Cargo.toml", "layer8_authority_runtime_api", "#111"],
  "observatory-ui" => ["real-browser", "validate_layer8_authority_observatory_ui.sh", "authorized", "refused", "#111"]
}
requested = ARGV.each_cons(2).find { |left, _| left == "--lane" }&.last
if requested
  lane = lanes.find { |candidate| candidate.fetch("lane") == "layer8-#{requested}-plan" }
  fail!("missing planned lane") unless lane
  contracts.fetch(requested).each { |fragment| fail!("lane omits #{fragment}") unless lane.fetch("proof_role").include?(fragment) }
end
spp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/spp.values.json")))
current_planning_text = JSON.generate([sip, stp, spp])
fail!("stale multi-gate wording") if current_planning_text.match?(/both serial gates|either (?:declared )?serial gate/i)
fail!("SIP omits sole #111 gate") unless current_planning_text.include?("sole #111 serial gate")
s4 = spp.dig("content", "values", "steps").find { |step| step.fetch("id") == "S4" }
%w[authority-contract real-browser].each { |fragment| fail!("S4 omits #{fragment}") unless s4.fetch("action").include?(fragment) }
fail!("S4 omits Runtime API") unless s4.fetch("action").include?("Runtime API")
srp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/srp.values.json")))
fail!("review overstates progress") unless srp.dig("content", "values", "review_result") == "pre_review"
sor = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/sor.values.json"))).dig("content", "values")
fail!("SOR overstates changes") unless sor.fetch("actual_changes") == []
fail!("SOR overstates publication") unless sor.fetch("publication_state") == "not_published" && sor.fetch("merge_state") == "not_merged"
%w[design.md diagram.mmd validate-preparation.rb].each { |name| fail!("missing #{name}") unless File.file?(File.join(PREP_ROOT, name)) }
puts JSON.generate(schema: "adl.csdlc.issue_112_preparation_validation.v4", issue: 112, outcome: "pass", phase: index.fetch("phase"), generation: index.fetch("generation"), dependency_gates: [111], planned_lane: requested, product_code_changed: false)
