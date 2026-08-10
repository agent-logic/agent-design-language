#!/usr/bin/env ruby
# frozen_string_literal: true

require "yaml"

root = File.expand_path("../../../..", __dir__)
milestone = File.join(root, "docs/milestones/v0.92.1")

required = %w[
  README.md
  VISION_v0.92.1.md
  DESIGN_v0.92.1.md
  DECISIONS_v0.92.1.md
  WBS_v0.92.1.md
  SPRINT_v0.92.1.md
  WP_ISSUE_WAVE_v0.92.1.yaml
  DEMO_MATRIX_v0.92.1.md
  MILESTONE_CHECKLIST_v0.92.1.md
  RELEASE_PLAN_v0.92.1.md
  RELEASE_NOTES_v0.92.1.md
  QUALITY_GATE_v0.92.1.md
  FEATURE_PROOF_COVERAGE_v0.92.1.md
  WP_EXECUTION_READINESS_v0.92.1.md
  ADR_PLAN_v0.92.1.md
  NEXT_MILESTONE_HANDOFF_v0.92.1.md
  DISTRIBUTED_TEST_PLAN_CONSULTATION.md
  features/README.md
  features/CORPORATE_AND_IP_TRANSFER_v0.92.1.md
  features/CSDLC_V3_v0.92.1.md
  features/DISTRIBUTED_MULTI_AGENT_RUNTIME_QUALIFICATION_v0.92.1.md
  sources/CORPORATE_INFRASTRUCTURE_CONSOLIDATION_SOURCE.md
]

missing = required.reject { |relative| File.file?(File.join(milestone, relative)) }
abort("missing required v0.92.1 files: #{missing.join(', ')}") unless missing.empty?

wave_path = File.join(milestone, "WP_ISSUE_WAVE_v0.92.1.yaml")
wave = YAML.safe_load(File.read(wave_path), permitted_classes: [], aliases: false)
abort("issue wave must be a mapping") unless wave.is_a?(Hash)

packages = wave.fetch("work_packages")
externals = wave.fetch("external_dependencies")
abort("work_packages must be an array") unless packages.is_a?(Array)
abort("external_dependencies must be an array") unless externals.is_a?(Array)

expected_ids =
  (1..8).map { |n| format("CORP-%02d", n) } +
  ((1..9).to_a + (12..16).to_a).map { |n| format("V3-%02d", n) } +
  %w[V3-D11 V3-10A V3-10B V3-11A V3-11B V3-R01] +
  (1..7).map { |n| format("DRT-%02d", n) } +
  (1..3).map { |n| format("INT-%02d", n) }
actual_ids = packages.map { |entry| entry.fetch("id") }
abort("duplicate work-package ids") unless actual_ids.uniq.length == actual_ids.length
missing_ids = expected_ids - actual_ids
unexpected_ids = actual_ids - expected_ids
abort("issue inventory mismatch; missing=#{missing_ids.join(',')} unexpected=#{unexpected_ids.join(',')}") unless missing_ids.empty? && unexpected_ids.empty?

known_ids = actual_ids + externals.map { |entry| entry.fetch("id") }
unknown_dependencies = packages.flat_map do |entry|
  entry.fetch("depends_on").reject { |dependency| known_ids.include?(dependency) }
end.uniq
abort("unknown dependencies: #{unknown_dependencies.join(', ')}") unless unknown_dependencies.empty?

by_id = packages.each_with_object({}) { |entry, memo| memo[entry.fetch("id")] = entry }
critical_dependencies = {
  "V3-08" => %w[V3-06 V3-07 V3-D11],
  "V3-R01" => %w[V3-16],
  "DRT-03" => %w[DRT-01 DRT-02 RUNTIME-142],
  "DRT-04" => %w[DRT-03],
  "INT-01" => %w[CORP-08 V3-16 DRT-07]
}
critical_dependencies.each do |id, expected|
  actual = by_id.fetch(id).fetch("depends_on")
  abort("#{id} dependency mismatch: #{actual.inspect}") unless actual.sort == expected.sort
end
abort("V3-R01 must remain deferred") unless by_id.fetch("V3-R01")["status"] == "deferred"

runtime_input = externals.find { |entry| entry["id"] == "RUNTIME-142" }
abort("RUNTIME-142 must require terminal issue #142") unless runtime_input && runtime_input["issue"] == 142 && runtime_input["required_state"] == "terminal"

visiting = {}
visited = {}
walk = lambda do |id|
  return if visited[id]
  abort("dependency cycle at #{id}") if visiting[id]
  visiting[id] = true
  by_id.fetch(id).fetch("depends_on").each { |dependency| walk.call(dependency) if by_id.key?(dependency) }
  visiting.delete(id)
  visited[id] = true
end
actual_ids.each { |id| walk.call(id) }

cross_lane_edges = packages.flat_map do |entry|
  entry.fetch("depends_on").select do |dependency|
    by_id.key?(dependency) && by_id.fetch(dependency).fetch("lane") != entry.fetch("lane")
  end.map { |dependency| [entry.fetch("id"), dependency] }
end
expected_cross_lane_edges = [["INT-01", "CORP-08"], ["INT-01", "V3-16"], ["INT-01", "DRT-07"]]
abort("unexpected cross-lane dependencies: #{cross_lane_edges.inspect}") unless cross_lane_edges.sort == expected_cross_lane_edges.sort

source_sha = wave.fetch("source_revisions").fetch("csdlc_v3_pr_77_merge")
abort("unexpected C-SDLC v3 source SHA") unless source_sha == "413fa9b8588dd25be3785cfe111c4f1df3af36eb"
unless system("git", "merge-base", "--is-ancestor", source_sha, "origin/main", chdir: root, out: File::NULL, err: File::NULL)
  abort("C-SDLC v3 source SHA is not ancestral to origin/main")
end

serialized = required.map do |relative|
  path = File.join(milestone, relative)
  next nil unless File.file?(path)
  File.read(path)
end.compact.join("\n")

required_terms = [
  "corporate",
  "intellectual property",
  "C-SDLC v3",
  "distributed multi-agent Runtime",
  "413fa9b8588dd25be3785cfe111c4f1df3af36eb",
  "#142",
  "V3-R01",
  "Decision 11",
  "counsel",
  "three voters",
  "Route53",
  "ACM",
  "SES",
  "qualification-only",
  "3 -> 2 -> 1",
  "AWS-only continuity"
]
missing_terms = required_terms.reject { |term| serialized.include?(term) }
abort("missing required package terms: #{missing_terms.join(', ')}") unless missing_terms.empty?

forbidden = ["release approved", "transfer complete", "C-SDLC v3 complete", "Runtime qualification passed"]
violations = forbidden.select { |term| serialized.downcase.include?(term.downcase) }
abort("unsupported completion claims: #{violations.join(', ')}") unless violations.empty?

puts "PASS: v0.92.1 package contract"
