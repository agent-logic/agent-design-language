#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
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
  WP_PREMATURE_ISSUE_RETIREMENT_v0.92.1.yaml
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

wave = YAML.safe_load(File.read(File.join(milestone, "WP_ISSUE_WAVE_v0.92.1.yaml")), permitted_classes: [], aliases: false)
spec_doc = YAML.safe_load(File.read(File.join(milestone, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")), permitted_classes: [], aliases: false)
retired = YAML.safe_load(File.read(File.join(milestone, "WP_PREMATURE_ISSUE_RETIREMENT_v0.92.1.yaml")), permitted_classes: [], aliases: false)

abort("issue wave must remain planning-only") unless wave["status"] == "planning_only"
abort("execution specs must remain planning-only") unless spec_doc["status"] == "planning_only"
abort("WP-01 must own issue creation") unless wave["opening_work_package"] == "WP-01" && wave["issue_creation_authority"] == "WP-01"
abort("specs must assign issue creation to WP-01") unless spec_doc["opening_work_package"] == "WP-01" && spec_doc["issue_creation_authority"] == "WP-01"

expected_ids =
  ["WP-01"] +
  (1..8).map { |n| format("CORP-%02d", n) } +
  ((1..9).to_a + (12..16).to_a).map { |n| format("V3-%02d", n) } +
  %w[V3-D11 V3-10A V3-10B V3-11A V3-11B V3-R01] +
  (1..7).map { |n| format("DRT-%02d", n) } +
  (1..6).map { |n| format("INT-%02d", n) }

packages = wave.fetch("work_packages")
actual_ids = packages.map { |entry| entry.fetch("id") }
abort("work-package inventory mismatch") unless actual_ids.sort == expected_ids.sort && actual_ids.uniq.size == actual_ids.size
abort("required child count mismatch") unless spec_doc["required_child_count"] == 42

umbrella_ids = %w[CORP-U V3-U DRT-U INT-U]
umbrellas = wave.fetch("umbrellas")
abort("umbrella inventory mismatch") unless umbrellas.map { |entry| entry.fetch("id") }.sort == umbrella_ids.sort

(packages + umbrellas).each do |entry|
  abort("#{entry.fetch('id')} contains a premature live issue number") if entry.key?("issue") || entry.key?("url")
  expected_status = entry["id"] == "V3-R01" ? "deferred" : entry["id"] == "WP-01" ? "planned_for_post_merge_creation" : "planned_for_wp01_creation"
  abort("#{entry.fetch('id')} planning status mismatch") unless entry["status"] == expected_status
end

specs = spec_doc.fetch("work_packages")
abort("execution specification denominator mismatch") unless specs.size == expected_ids.size
spec_by_id = specs.to_h { |entry| [entry.fetch("id"), entry] }
abort("execution specification ids mismatch") unless spec_by_id.keys.sort == expected_ids.sort

required_arrays = %w[deliverables acceptance_criteria non_goals owned_paths pvf_lanes authority_boundary risks stop_conditions review_prompts source_refs]
specs.each do |spec|
  id = spec.fetch("id")
  abort("#{id} must not have a live GitHub issue") unless spec["github_issue"].nil? && spec["github_url"].nil?
  expected_owner = id == "WP-01" ? "post-merge operator bootstrap" : "WP-01"
  abort("#{id} creation owner mismatch") unless spec["creation_owner"] == expected_owner
  expected_status = id == "V3-R01" ? "deferred" : id == "WP-01" ? "planned_for_post_merge_creation" : "planned_for_wp01_creation"
  abort("#{id} specification status mismatch") unless spec["status"] == expected_status
  %w[title objective scope validation_proof proof_summary].each do |field|
    abort("#{id} missing #{field}") if spec.fetch(field).to_s.strip.empty?
  end
  required_arrays.each do |field|
    values = spec.fetch(field)
    abort("#{id} missing #{field}") unless values.is_a?(Array) && !values.empty?
  end
  abort("#{id} acceptance too small") if spec.fetch("acceptance_criteria").size < 4
  spec.fetch("pvf_lanes").each do |lane|
    %w[lane proof_role resource_profile budget_seconds budget_tokens argv_template].each do |field|
      abort("#{id} PVF missing #{field}") if lane[field].nil?
    end
    abort("#{id} PVF command empty") unless lane.fetch("argv_template").is_a?(Array) && !lane.fetch("argv_template").empty?
    abort("#{id} PVF budget invalid") unless lane.fetch("budget_seconds") > 0 && lane.fetch("budget_tokens") > 0
  end
end

(149..190).each do |issue|
  abort("premature local issue record remains for ##{issue}") if Dir.exist?(File.join(root, ".csdlc/issues/#{issue}"))
  abort("premature prepared issue record remains for ##{issue}") if Dir.exist?(File.join(root, ".csdlc/prepared/issues/#{issue}"))
end
abort("premature Runtime proof stubs remain") if Dir.exist?(File.join(root, "adl/tools/v0921"))

archive = File.join(milestone, "planned-issue-packets")
archive_readme = File.join(archive, "README.md")
abort("preserved planning archive README missing") unless File.file?(archive_readme)
archive_text = File.read(archive_readme)
abort("archive authority boundary missing") unless archive_text.include?("planning inputs only") && archive_text.include?("do not represent open issues") && archive_text.include?("must not")
abort("preserved issue packet denominator mismatch") unless Dir.glob(File.join(archive, "issues", "*")).select { |path| File.directory?(path) }.size == 42
abort("preserved prepared packet denominator mismatch") unless Dir.glob(File.join(archive, "prepared/issues", "*")).select { |path| File.directory?(path) }.size == 42
abort("preserved proof-stub archive missing") unless Dir.exist?(File.join(archive, "proof-stubs/v0921"))

archive_manifest = JSON.parse(File.read(File.join(archive, "manifest.json")))
abort("archive manifest schema mismatch") unless archive_manifest["schema"] == "adl.milestone.planning_archive_manifest.v1"
abort("archive manifest authority mismatch") unless archive_manifest["authority"] == "non_authoritative_planning_input"
source_commit = archive_manifest["source_parent_commit"]
abort("archive source commit mismatch") unless source_commit == "6eeaca025c426ebdf28e09b9372ae4cce2db69e6"
archive_entries = archive_manifest.fetch("files")
abort("archive manifest denominator mismatch") unless archive_manifest["file_count"] == 721 && archive_entries.size == 721
manifest_paths = archive_entries.map { |entry| entry.fetch("archive_path") }
actual_paths = Dir.glob(File.join(archive, "{issues,prepared,proof-stubs}", "**", "*"))
  .select { |path| File.file?(path) }
  .map { |path| path.delete_prefix("#{archive}/") }
  .sort
abort("archive path inventory mismatch") unless manifest_paths.sort == actual_paths
abort("archive manifest contains duplicate paths") unless manifest_paths.uniq.size == manifest_paths.size
archive_entries.each do |entry|
  relative = entry.fetch("archive_path")
  source = entry.fetch("source_path")
  expected_source = if relative.start_with?("issues/", "prepared/")
                      ".csdlc/#{relative}"
                    elsif relative.start_with?("proof-stubs/v0921/")
                      "adl/tools/#{relative.delete_prefix("proof-stubs/")}"
                    end
  abort("archive source mapping mismatch for #{relative}") unless source == expected_source
  path = File.join(archive, relative)
  abort("archive byte count mismatch for #{relative}") unless File.size(path) == entry.fetch("bytes")
  abort("archive digest mismatch for #{relative}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
  source_bytes, source_error, source_status = Open3.capture3("git", "-C", root, "show", "#{source_commit}:#{source}")
  abort("archive source blob missing for #{source}: #{source_error.strip}") unless source_status.success?
  abort("archive source byte count mismatch for #{relative}") unless source_bytes.bytesize == entry.fetch("bytes")
  abort("archive source digest mismatch for #{relative}") unless Digest::SHA256.hexdigest(source_bytes) == entry.fetch("sha256")
end

retired_issues = retired.fetch("issues")
abort("retirement ledger denominator mismatch") unless retired["required_retired_count"] == 42 && retired_issues.size == 42
abort("retirement ledger numbers mismatch") unless retired_issues.values.map { |entry| entry.fetch("number") }.sort == (149..190).to_a
retired_issues.each do |id, entry|
  abort("#{id} is not closed") unless entry["state"] == "closed"
  abort("#{id} remains active authority") unless entry["active_authority"] == false
  abort("#{id} retirement disposition missing") unless entry["disposition"] == "premature_planning_issue_retired_without_execution"
  abort("#{id} replacement owner mismatch") unless entry["replacement_creation_owner"] == "WP-01"
end

known_ids = actual_ids + wave.fetch("external_dependencies").map { |entry| entry.fetch("id") }
unknown_dependencies = packages.flat_map do |entry|
  entry.fetch("depends_on").reject { |dependency| known_ids.include?(dependency) }
end.uniq
abort("unknown dependencies: #{unknown_dependencies.join(', ')}") unless unknown_dependencies.empty?

by_id = packages.to_h { |entry| [entry.fetch("id"), entry] }
critical_dependencies = {
  "V3-08" => %w[V3-06 V3-07 V3-D11],
  "V3-R01" => %w[V3-16],
  "DRT-03" => %w[DRT-01 DRT-02 RUNTIME-142],
  "INT-01" => %w[CORP-08 V3-16 DRT-07],
  "INT-02" => %w[INT-01],
  "INT-03" => %w[INT-02],
  "INT-04" => %w[INT-03],
  "INT-05" => %w[INT-02 INT-04],
  "INT-06" => %w[INT-05]
}
critical_dependencies.each do |id, expected|
  actual = by_id.fetch(id).fetch("depends_on")
  abort("#{id} dependency mismatch") unless actual.sort == expected.sort
end

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

expected_sequence = %w[
  milestone_planning
  wp01_issue_wave_opening
  parallel_lane_execution
  integrated_review_and_remediation
  release_qualification
  next_milestone_planning
  next_milestone_independent_review
  operator_authorized_release_ceremony
  terminal_milestone_closeout
]
abort("standard lifecycle sequence mismatch") unless wave["lifecycle_sequence"] == expected_sequence

decision_text = File.read(File.join(milestone, "DECISIONS_v0.92.1.md"))
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

decision_owners = Hash.new { |hash, key| hash[key] = [] }
spec_doc.fetch("work_packages").each do |entry|
  Array(entry["architecture_decisions"]).each { |decision| decision_owners[decision] << entry.fetch("id") }
end
expected_decision_owners = {
  "V3-D01" => %w[V3-01],
  "V3-D02" => %w[V3-02],
  "V3-D03" => %w[V3-03],
  "V3-D04" => %w[V3-04],
  "V3-D05" => %w[V3-06 V3-08],
  "V3-D06" => %w[V3-03 V3-10A V3-10B],
  "V3-D07" => %w[V3-10A],
  "V3-D08" => %w[V3-14],
  "V3-D09" => %w[V3-11A V3-11B],
  "V3-D10" => %w[V3-15],
  "V3-D11" => %w[V3-D11 V3-08]
}
abort("architecture decision identifier mismatch") unless decision_owners.keys.sort == expected_decision_owners.keys.sort
expected_decision_owners.each do |decision, expected|
  abort("#{decision} package mapping mismatch") unless decision_owners.fetch(decision).sort == expected.sort
end

readme = File.read(File.join(milestone, "README.md"))
abort("README missing planning-only posture") unless readme.include?("Planning-only package") && readme.include?("WP-01")
abort("README missing retired issue truth") unless readme.include?("#149-#190") && readme.include?("retired")
abort("README falsely treats #142 as terminal") if readme.include?("Runtime qualification source: terminal issue `#142`")
abort("README missing nonterminal #142 gate") unless readme.include?("does not treat `#142` as terminal")

puts "PASS: v0.92.1 planning-only package and standard lifecycle WBS"
