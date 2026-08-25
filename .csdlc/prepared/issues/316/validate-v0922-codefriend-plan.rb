#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"
require "yaml"

root = Pathname(__dir__).join("../../../..").realpath
milestone = root.join("docs/milestones/v0.92.2")
errors = []

required = %w[
  README.md VISION_v0.92.2.md DESIGN_v0.92.2.md DECISIONS_v0.92.2.md
  WBS_v0.92.2.md SPRINT_v0.92.2.md WP_ISSUE_WAVE_v0.92.2.yaml
  WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml PLANNED_ISSUE_CATALOG_v0.92.2.md
  CANONICAL_DOC_INVENTORY_v0.92.2.md DEMO_MATRIX_v0.92.2.md
  MILESTONE_CHECKLIST_v0.92.2.md RELEASE_PLAN_v0.92.2.md
  RELEASE_NOTES_v0.92.2.md QUALITY_GATE_v0.92.2.md
  FEATURE_PROOF_COVERAGE_v0.92.2.md WP_EXECUTION_READINESS_v0.92.2.md
  ADR_PLAN_v0.92.2.md NEXT_MILESTONE_HANDOFF_v0.92.2.md features/README.md
  features/PRODUCT_SHELL_AND_OPERATOR_CONTROLS_v0.92.2.md
  features/PORTABLE_ADAPTER_V2_v0.92.2.md features/EVIDENCE_CORE_v0.92.2.md
  features/ARCHITECTURE_COGNITION_v0.92.2.md
  features/EXECUTABLE_GOVERNANCE_v0.92.2.md
  features/MULTI_PERSPECTIVE_REVIEW_v0.92.2.md
  features/LONGITUDINAL_REVIEW_MEMORY_v0.92.2.md
  features/GOVERNED_PUBLICATION_v0.92.2.md
  features/BETA1_QUALIFICATION_v0.92.2.md
]
required.each { |path| errors << "missing:#{path}" unless milestone.join(path).file? }

wave = YAML.safe_load(milestone.join("WP_ISSUE_WAVE_v0.92.2.yaml").read, aliases: false)
spec = YAML.safe_load(milestone.join("WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml").read, aliases: false)
rows = wave.fetch("work_packages")
ids = rows.map { |row| row.fetch("id") }
expected = %w[
  WP-01 CF-SHELL CF-ADAPTER CF-EVIDENCE CF-COG CF-GOV CF-REVIEW CF-MEMORY
  CF-UX CF-PROOF CF-INTEGRATE TAIL-01 TAIL-02 TAIL-03 TAIL-04 TAIL-05
  TAIL-06 TAIL-07 TAIL-08 TAIL-09 TAIL-10
]
errors << "planned id denominator mismatch" unless ids == expected
errors << "duplicate planned ids" unless ids.uniq.length == ids.length
errors << "issue numbers allocated" unless rows.all? { |row| row["issue"].nil? }

rows.each do |row|
  Array(row["depends_on"]).each do |dep|
    errors << "unknown dependency:#{row['id']}:#{dep}" unless ids.include?(dep)
  end
end

tail = (1..10).map { |n| "TAIL-%02d" % n }
errors << "canonical release tail mismatch" unless wave.fetch("canonical_release_tail") == tail
errors << "execution release tail mismatch" unless spec.fetch("release_tail").fetch("order") == tail
spec_ids = spec.fetch("specifications").map { |row| row.fetch("id") }
errors << "execution specification mismatch" unless spec_ids == expected

deferred = wave.fetch("deferred_tracks")
required_deferred = %w[jira linear slack broad_workspace autonomous_mutation public_customer_scale security_tournaments ate mlx_metal oci_model_packaging optional_openrewrite runtime_v4]
errors << "deferred track mismatch" unless deferred.sort == required_deferred.sort

files = milestone.glob("**/*").select(&:file?)
text = files.map(&:binread).join("\n")
errors << "tracked local-path dependency" if text.include?(".adl/")
errors << "Google Drive runtime dependency" if text.match?(%r{https?://(?:docs|drive)\.google\.com})

markers = [
  "product shell", "Adapter v2", "evidence", "architecture cognition",
  "fitness", "correctness", "security", "adversarial", "constitutional",
  "publication", "longitudinal", "Markdown", "HTML", "PDF", "external",
  "Jira", "ATE", "MLX", "OCI"
]
markers.each { |marker| errors << "missing Beta 1 marker:#{marker}" unless text.downcase.include?(marker.downcase) }

files.select { |path| path.extname == ".md" }.each do |path|
  path.read.scan(/\[[^\]]+\]\(([^)]+)\)/).flatten.each do |target|
    next if target.start_with?("http://", "https://", "#", "mailto:")
    clean = target.split("#", 2).first
    next if clean.empty?
    errors << "broken link:#{path.relative_path_from(root)}:#{target}" unless path.dirname.join(clean).cleanpath.exist?
  end
end

if errors.empty?
  puts JSON.generate(schema: "adl.v0922.codefriend-plan-validation.v1", result: "passed",
                     planned_ids: ids.length, feature_docs: files.count { |path| path.dirname.basename.to_s == "features" && path.basename.to_s != "README.md" },
                     release_tail: tail.length, deferred_tracks: deferred.length)
else
  warn errors.join("\n")
  exit 1
end
