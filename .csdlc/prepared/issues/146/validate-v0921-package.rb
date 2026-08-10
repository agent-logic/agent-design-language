#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
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
  WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
  WP_LIVE_ISSUE_MAP_v0.92.1.yaml
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

spec_path = File.join(milestone, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
spec_doc = YAML.safe_load(File.read(spec_path), permitted_classes: [], aliases: false)
map_path = File.join(milestone, "WP_LIVE_ISSUE_MAP_v0.92.1.yaml")
map_doc = YAML.safe_load(File.read(map_path), permitted_classes: [], aliases: false)

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

umbrella_ids = %w[CORP-U V3-U DRT-U INT-U]
actual_umbrellas = wave.fetch("umbrellas").map { |entry| entry.fetch("id") }
abort("umbrella inventory mismatch") unless actual_umbrellas.sort == umbrella_ids.sort

specs = spec_doc.fetch("work_packages")
abort("execution specification denominator mismatch") unless specs.size == expected_ids.size
spec_by_id = specs.to_h { |entry| [entry.fetch("id"), entry] }
abort("execution specification ids mismatch") unless spec_by_id.keys.sort == expected_ids.sort

live = map_doc.fetch("issues")
expected_live_ids = umbrella_ids + expected_ids
abort("live issue map denominator mismatch") unless live.keys.sort == expected_live_ids.sort
live_numbers = live.values.map { |entry| entry.fetch("number") }
abort("duplicate live issue numbers") unless live_numbers.uniq.size == live_numbers.size
abort("invalid live issue number") if live_numbers.any? { |number| number <= 0 }

wave.fetch("umbrellas").each do |entry|
  mapped = live.fetch(entry.fetch("id"))
  abort("umbrella live issue mismatch for #{entry.fetch('id')}") unless entry.fetch("issue") == mapped.fetch("number") && entry.fetch("url") == mapped.fetch("url")
end

required_spec_arrays = %w[deliverables acceptance_criteria non_goals owned_paths pvf_lanes authority_boundary risks stop_conditions review_prompts source_refs]
packages.each do |entry|
  id = entry.fetch("id")
  mapped = live.fetch(id)
  spec = spec_by_id.fetch(id)
  abort("wave live issue mismatch for #{id}") unless entry.fetch("issue") == mapped.fetch("number") && entry.fetch("url") == mapped.fetch("url")
  abort("spec live issue mismatch for #{id}") unless spec.fetch("github_issue") == mapped.fetch("number") && spec.fetch("github_url") == mapped.fetch("url")
  %w[title objective scope validation_proof proof_summary].each do |field|
    abort("#{id} missing #{field}") if spec.fetch(field).to_s.strip.empty?
  end
  required_spec_arrays.each do |field|
    values = spec.fetch(field)
    abort("#{id} missing #{field}") unless values.is_a?(Array) && !values.empty? && values.none? { |value| value.respond_to?(:empty?) && value.empty? }
  end
  abort("#{id} acceptance too small") if spec.fetch("acceptance_criteria").size < 4
  spec.fetch("pvf_lanes").each do |lane|
    %w[lane proof_role resource_profile budget_seconds budget_tokens argv_template].each { |field| abort("#{id} PVF missing #{field}") if lane[field].nil? }
    abort("#{id} PVF command empty") unless lane.fetch("argv_template").is_a?(Array) && !lane.fetch("argv_template").empty?
    abort("#{id} PVF budget invalid") unless lane.fetch("budget_seconds") > 0 && lane.fetch("budget_tokens") > 0
  end

  issue = mapped.fetch("number")
  issue_root = File.join(root, ".csdlc/issues/#{issue}")
  prepared_root = File.join(root, ".csdlc/prepared/issues/#{issue}")
  abort("#{id} missing issue record") unless File.file?(File.join(issue_root, "index.json"))
  cards = %w[sip stp spp vpp srp sor]
  cards.each do |card|
    abort("#{id} missing #{card} card") unless File.file?(File.join(issue_root, "cards/#{card}.md")) && File.file?(File.join(issue_root, "cards/#{card}.values.json"))
  end
  %w[design.md diagram.mmd validate-outcome.rb].each do |artifact|
    abort("#{id} missing #{artifact}") unless File.file?(File.join(prepared_root, artifact))
  end
  design = File.read(File.join(prepared_root, "design.md"))
  %w[Objective Scope Dependencies Deliverables Owned\ Paths Acceptance\ Criteria PVF\ Lanes Validation\ Proof Authority\ Boundary Non-goals Risks Stop\ Conditions Review\ Prompts Source\ Authority].each do |heading|
    abort("#{id} design missing #{heading.tr('\\', '')}") unless design.include?("## #{heading.tr('\\', '')}")
  end
  index = JSON.parse(File.read(File.join(issue_root, "index.json")))
  abort("#{id} record identity mismatch") unless index.fetch("issue") == issue && index.fetch("repository") == "agent-logic/agent-design-language"
  approval = index.dig("design_review", "approved")
  abort("#{id} design is not independently approved") unless approval.is_a?(Hash) && approval.fetch("reviewer") == "subagent:019fed4b-5813-7f33-b7db-a3e87741bfc1"
  stp = JSON.parse(File.read(File.join(issue_root, "cards/stp.values.json")))
  spp = JSON.parse(File.read(File.join(issue_root, "cards/spp.values.json")))
  vpp = JSON.parse(File.read(File.join(issue_root, "cards/vpp.values.json")))
  abort("#{id} acceptance/card mismatch") unless stp.dig("content", "values", "acceptance_criteria") == spec.fetch("acceptance_criteria")
  affected = spp.dig("content", "values", "affected_areas") || []
  lifecycle_paths = [".csdlc/issues/#{issue}/**", ".csdlc/prepared/issues/#{issue}/**", ".csdlc/evidence/#{issue}/**"]
  expected_paths = spec.fetch("owned_paths") + lifecycle_paths
  abort("#{id} owned paths/card mismatch") unless expected_paths.all? { |path| affected.include?(path) }
  expected_lanes = spec.fetch("pvf_lanes").map { |lane| lane.fetch("lane") }
  actual_lanes = (vpp.dig("content", "values", "lanes") || []).map { |lane| lane.fetch("lane") }
  abort("#{id} PVF/card mismatch") unless actual_lanes == expected_lanes
end

expected_live_ids.each do |id|
  issue = live.fetch(id).fetch("number")
  issue_root = File.join(root, ".csdlc/issues/#{issue}")
  prepared_root = File.join(root, ".csdlc/prepared/issues/#{issue}")
  index_path = File.join(issue_root, "index.json")
  abort("#{id} missing issue record") unless File.file?(index_path)
  %w[sip stp spp vpp srp sor].each do |card|
    abort("#{id} missing #{card} card") unless File.file?(File.join(issue_root, "cards/#{card}.md")) && File.file?(File.join(issue_root, "cards/#{card}.values.json"))
  end
  %w[design.md diagram.mmd validate-outcome.rb].each do |artifact|
    abort("#{id} missing #{artifact}") unless File.file?(File.join(prepared_root, artifact))
  end
  index = JSON.parse(File.read(index_path))
  abort("#{id} record identity mismatch") unless index.fetch("issue") == issue && index.fetch("repository") == "agent-logic/agent-design-language"
  approval = index.dig("design_review", "approved")
  abort("#{id} design is not independently approved") unless approval.is_a?(Hash) && approval.fetch("reviewer") == "subagent:019fed4b-5813-7f33-b7db-a3e87741bfc1"
  affected = JSON.parse(File.read(File.join(issue_root, "cards/spp.values.json"))).dig("content", "values", "affected_areas") || []
  abort("#{id} contains duplicate owned paths") unless affected.uniq == affected
end

plan_fingerprints = {}
validator_fingerprints = {}
diagram_fingerprints = {}
expected_live_ids.each do |id|
  issue = live.fetch(id).fetch("number")
  issue_root = File.join(root, ".csdlc/issues/#{issue}")
  prepared_root = File.join(root, ".csdlc/prepared/issues/#{issue}")
  plan = JSON.parse(File.read(File.join(issue_root, "cards/spp.values.json"))).dig("content", "values")
  actions = plan.fetch("steps").map { |step| step.fetch("action") }
  abort("#{id} plan has fewer than four issue-specific steps") if actions.size < 4
  old_generic_actions = [
    "Confirm dependencies, design, exact owned paths, and issue-local validator contract.",
    "Implement the declared deliverables within the approved authority boundary.",
    "Run every declared PVF lane and retain exact producer-derived evidence.",
    "Complete independent exact-head review, remediate findings, and publish the issue PR."
  ]
  abort("#{id} retains the generic four-step plan") if actions == old_generic_actions
  joined_actions = actions.join(" ").downcase
  abort("#{id} plan omits proof production") unless joined_actions.match?(/proof|prove|receipt|evidence|validat|artifact|test|fixture/)
  abort("#{id} plan omits failure or cleanup handling") unless joined_actions.match?(/fail|stop|rollback|clean|close|discard|recovery|revert|cancel|terminate/)
  plan_digest = Digest::SHA256.hexdigest(actions.join("\n"))
  abort("#{id} duplicates plan actions from #{plan_fingerprints[plan_digest]}") if plan_fingerprints.key?(plan_digest)
  plan_fingerprints[plan_digest] = id

  validator_path = File.join(prepared_root, "validate-outcome.rb")
  validator = File.read(validator_path)
  abort("#{id} validator trusts asserted pass flags") if validator.include?('["passed"]') || validator.include?("passed: true")
  %w[Digest::SHA256 File.file?].each do |required_token|
    abort("#{id} validator omits #{required_token}") unless validator.include?(required_token)
  end
  validator_digest = Digest::SHA256.hexdigest(validator)
  abort("#{id} duplicates outcome validator from #{validator_fingerprints[validator_digest]}") if validator_fingerprints.key?(validator_digest)
  validator_fingerprints[validator_digest] = id

  diagram = File.read(File.join(prepared_root, "diagram.mmd"))
  abort("#{id} retains the generic dependency skeleton") if diagram.include?('WORK["') && diagram.include?('PROOF["Producer-derived proof"]')
  abort("#{id} diagram is too small to express its issue design") if diagram.lines.count < 8
  diagram_digest = Digest::SHA256.hexdigest(diagram)
  abort("#{id} duplicates diagram from #{diagram_fingerprints[diagram_digest]}") if diagram_fingerprints.key?(diagram_digest)
  diagram_fingerprints[diagram_digest] = id
end

drt03_diagram = File.read(File.join(root, ".csdlc/prepared/issues/183/diagram.mmd")).downcase
%w[voter agent shepherd observatory lease quorum snapshot restart clean].each do |term|
  abort("DRT-03 diagram omits #{term}") unless drt03_diagram.include?(term)
end

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

decision_path = File.join(milestone, "DECISIONS_v0.92.1.md")
decision_text = File.read(decision_path)
architecture_decisions = [
  ["V3-D01", "Approve the shared v3 product and command contract."],
  ["V3-D02", "Approve the Rust construction-spike measurements and pass/fail thresholds."],
  ["V3-D03", "Approve one binary and one operator skill."],
  ["V3-D04", "Approve the `App` dependency-container boundary."],
  ["V3-D05", "Approve `state.json` as the sole typed aggregate and commit point."],
  ["V3-D06", "Approve direct flags plus optional typed `--input`."],
  ["V3-D07", "Approve branch/worktree topology rather than claims and heartbeat authority."],
  ["V3-D08", "Approve explicit foreground `pr watch` with structured cancellation."],
  ["V3-D09", "Approve no initial extension system beyond repository-declared PVF runners."],
  ["V3-D10", "Decide whether `finish` can ever own an explicitly authorized merge."],
  ["V3-D11", "Approve the per-platform commit matrix and whether Windows mutation support ships initially or remains fail-closed read-only pending equivalent proof."]
]
architecture_decisions.each do |id, wording|
  abort("missing architecture decision #{id}") unless decision_text.include?("| #{id} | #{wording} |")
end
abort("architecture decision denominator mismatch") unless decision_text.scan(/^\| V3-D\d{2} \|/).size == 11

readme = File.read(File.join(milestone, "README.md"))
abort("README falsely treats #142 as terminal") if readme.include?("Runtime qualification source: terminal issue `#142`")
abort("README missing nonterminal #142 gate") unless readme.include?("does not treat `#142` as terminal")

puts "PASS: v0.92.1 package contract"
