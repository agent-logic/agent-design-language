#!/usr/bin/env ruby

require "json"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE_ROOT = File.join(ROOT, ".csdlc/issues/112")
PREP_ROOT = File.join(ROOT, ".csdlc/prepared/issues/112")
CARD_KINDS = %w[sip stp spp vpp srp sor].freeze
EXPECTED_DEPENDENCIES = [
  "Hard serial gate: #83 must be closed by a merged PR and ancestral to the execution base",
  "Hard serial gate: #111 must be closed by a merged PR and ancestral to the execution base"
].freeze
EXPECTED_LANES = %w[
  issue-112-preparation-hygiene
  layer8-authority-contract-plan
  layer8-runtime-api-integration-plan
  layer8-observatory-ui-plan
].freeze

def fail!(message)
  warn(message)
  exit 1
end

index = JSON.parse(File.read(File.join(ISSUE_ROOT, "index.json")))
fail!("wrong issue identity") unless index.fetch("issue") == 112
fail!("wrong repository identity") unless index.fetch("repository") == "agent-logic/agent-design-language"
fail!("unexpected lifecycle phase") unless index.fetch("phase") == "ready"
fail!("design is not approved") unless index.dig("design_review", "approved")

CARD_KINDS.each do |kind|
  %W[#{kind}.md #{kind}.values.json].each do |name|
    fail!("missing #{name}") unless File.file?(File.join(ISSUE_ROOT, "cards", name))
  end
end

stp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/stp.values.json")))
dependencies = stp.dig("content", "values", "dependencies") || []
fail!("serial gates differ from exact #83/#111 contract") unless dependencies == EXPECTED_DEPENDENCIES

ready_transitions = index.fetch("transitions").select { |transition| transition.fetch("to") == "ready" }
fail!("expected exactly one readiness transition") unless ready_transitions.length == 1
ready_reason = ready_transitions.first.fetch("reason")
fail!("readiness reason omits #83") unless ready_reason.include?("#83")
fail!("readiness reason omits #111") unless ready_reason.include?("#111")
fail!("readiness reason adds undeclared #113 gate") if ready_reason.include?("#113")

vpp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/vpp.values.json")))
lanes = vpp.dig("content", "values", "lanes") || []
fail!("preparation lanes differ from exact contract") unless lanes.map { |lane| lane.fetch("lane") } == EXPECTED_LANES
failure_policy = vpp.dig("content", "values", "failure_policy").to_s
fail!("failure policy is not fail closed") unless failure_policy.start_with?("Fail closed")

lane_contracts = {
  "authority-contract" => ["cargo nextest run", "adl-runtime/Cargo.toml", "layer8_authority", "#83", "#111"],
  "runtime-api-integration" => ["cargo nextest run", "adl/Cargo.toml", "layer8_authority_runtime_api", "#83", "#111"],
  "observatory-ui" => ["real-browser", "validate_layer8_authority_observatory_ui.sh", "authorized", "refused", "#83", "#111"]
}.freeze

requested_lane = ARGV.each_cons(2).find { |left, _right| left == "--lane" }&.last
if requested_lane
  expected_fragments = lane_contracts.fetch(requested_lane) { fail!("unknown planned lane #{requested_lane}") }
  planned_lane = lanes.find { |lane| lane.fetch("lane") == "layer8-#{requested_lane}-plan" }
  fail!("missing planned lane #{requested_lane}") unless planned_lane
  contract = planned_lane.fetch("proof_role")
  expected_fragments.each do |fragment|
    fail!("planned lane #{requested_lane} omits #{fragment}") unless contract.include?(fragment)
  end
end

spp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/spp.values.json")))
s4 = spp.dig("content", "values", "steps")&.find { |step| step.fetch("id") == "S4" }
fail!("missing SPP S4") unless s4
%w[authority-contract Runtime\ API real-browser].each do |fragment|
  fail!("SPP S4 omits #{fragment}") unless s4.fetch("action").include?(fragment.tr("\\", ""))
end

srp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/srp.values.json")))
fail!("review is not pre-review") unless srp.dig("content", "values", "review_result") == "pre_review"

sor = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/sor.values.json")))
sor_values = sor.dig("content", "values") || {}
fail!("SOR overstates execution") unless sor_values.fetch("actual_changes") == []
fail!("SOR overstates publication") unless sor_values.fetch("publication_state") == "not_published"
fail!("SOR overstates merge") unless sor_values.fetch("merge_state") == "not_merged"

%w[design.md diagram.mmd validate-preparation.rb].each do |name|
  fail!("missing preparation artifact #{name}") unless File.file?(File.join(PREP_ROOT, name))
end

puts JSON.generate(
  schema: "adl.csdlc.issue_112_preparation_validation.v3",
  issue: 112,
  outcome: "pass",
  phase: index.fetch("phase"),
  generation: index.fetch("generation"),
  cards: CARD_KINDS,
  dependency_gates: [83, 111],
  planned_lane: requested_lane,
  product_code_changed: false
)
