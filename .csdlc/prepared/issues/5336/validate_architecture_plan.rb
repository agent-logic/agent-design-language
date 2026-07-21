#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "yaml"

root = File.expand_path("../../../..", __dir__)
milestone = File.join(root, "docs/milestones/v0.91.8")
baseline = JSON.parse(File.read(File.join(milestone, "baseline_and_ownership_v0.91.8.json")))
parity = JSON.parse(File.read(File.join(milestone, "runtime_v3_functional_parity_plan_v0.91.8.json")))
wave = YAML.safe_load(File.read(File.join(milestone, "WP_ISSUE_WAVE_v0.91.8.yaml")), aliases: true)

def assert!(condition, message)
  abort(message) unless condition
end

assert!(baseline["schema"] == "adl.v0.91.8.baseline_and_ownership.v1", "baseline schema drift")
owners = baseline.fetch("source_roots").select { |entry| entry["owner"] == "runtime_v3_canonical" }
assert!(owners.map { |entry| entry["path"] } == ["adl-runtime-kernel/src"], "canonical Runtime v3 owner drift")

budget = baseline.fetch("runtime_owner_report")
expected_budget = {
  "physical_lines" => 12_209,
  "challenge_target" => 10_000,
  "reviewed_target" => 12_000,
  "exception_lines" => 209,
  "exception_ceiling" => 20_000,
  "test_count" => 189,
  "test_ceiling_exclusive" => 1_000,
  "disposition" => "reviewed_exception_required"
}
assert!(expected_budget.all? { |key, value| budget[key] == value }, "baseline Runtime budget drift")
assert!(budget["test_count"] < budget["test_ceiling_exclusive"], "Runtime test ceiling failed")

assert!(parity["schema"] == "adl.runtime_v3.functional_parity_plan.v1", "parity schema drift")
assert!(parity["canonical_runtime"] == "adl-runtime-kernel", "parity canonical runtime drift")
parity_budget = expected_budget.reject { |key, _| key == "disposition" }
parity_budget["runtime_physical_lines"] = parity_budget.delete("physical_lines")
assert!(parity["baseline"].slice(*parity_budget.keys) == parity_budget, "parity budget drift")
assert!(parity["no_aws"] == true, "AWS must remain unauthorized")
assert!(parity["cutover_authorized"] == false, "cutover became authorized")
assert!(parity["deletion_authorized"] == false, "deletion became authorized")
assert!(parity.fetch("lanes").map { |lane| lane["id"] }.sort == %w[A B C D], "parity lane set drift")
assert!(parity.fetch("proof_groups").map { |group| group["id"] } == (1..10).to_a, "ten proof groups are incomplete")
assert!(parity.fetch("feature_dispositions").sort == %w[blocker boundary_or_non_claim deferred_with_owner live_runtime_v3 owned_outside_runtime].sort, "feature dispositions drift")

cutover = parity.fetch("serial_gates").find { |gate| gate["gate"] == "cutover" }
assert!(cutover && cutover.fetch("depends_on").include?(5361), "cutover must depend on Runtime acceptance #5361")
wp12 = wave.fetch("work_packages").find { |entry| entry["wp"] == "WP-12" }
assert!(wp12 && wp12.fetch("depends_on").include?("#5361"), "WP-12 must depend on Runtime acceptance #5361")

packages = wave.fetch("work_packages").to_h { |entry| [entry.fetch("wp"), entry] }
visiting = {}
visited = {}
visit = lambda do |wp|
  return if visited[wp]
  abort("dependency cycle at #{wp}") if visiting[wp]

  visiting[wp] = true
  packages.fetch(wp).fetch("depends_on", []).grep(/^WP-/).each { |dependency| visit.call(dependency) }
  visiting.delete(wp)
  visited[wp] = true
end
packages.each_key { |wp| visit.call(wp) }

puts "runtime_v3_architecture_plan=pass proof_groups=10 cutover_dependency=5361"
