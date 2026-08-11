#!/usr/bin/env ruby

require "json"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE_ROOT = File.join(ROOT, ".csdlc/issues/112")
PREP_ROOT = File.join(ROOT, ".csdlc/prepared/issues/112")
CARD_KINDS = %w[sip stp spp vpp srp sor].freeze

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
fail!("missing #83 serial gate") unless dependencies.any? { |value| value.include?("#83") && value.include?("Hard serial gate") }
fail!("missing #111 serial gate") unless dependencies.any? { |value| value.include?("#111") && value.include?("Hard serial gate") }

vpp = JSON.parse(File.read(File.join(ISSUE_ROOT, "cards/vpp.values.json")))
lanes = vpp.dig("content", "values", "lanes") || []
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
  schema: "adl.csdlc.issue_112_preparation_validation.v2",
  issue: 112,
  outcome: "pass",
  phase: index.fetch("phase"),
  generation: index.fetch("generation"),
  cards: CARD_KINDS,
  dependency_gates: [83, 111],
  planned_lane: requested_lane,
  product_code_changed: false
)
