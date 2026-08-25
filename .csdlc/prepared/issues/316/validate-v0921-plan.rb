#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"
require "yaml"

root = Pathname(__dir__).join("../../../..").realpath
milestone = root.join("docs/milestones/v0.92.1")
errors = []

required = %w[
  README.md VISION_v0.92.1.md DESIGN_v0.92.1.md DECISIONS_v0.92.1.md
  WBS_v0.92.1.md SPRINT_v0.92.1.md WP_ISSUE_WAVE_v0.92.1.yaml
  WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml PLANNED_ISSUE_CATALOG_v0.92.1.md
  CANONICAL_DOC_INVENTORY_v0.92.1.md DEMO_MATRIX_v0.92.1.md
  MILESTONE_CHECKLIST_v0.92.1.md RELEASE_PLAN_v0.92.1.md
  RELEASE_NOTES_v0.92.1.md QUALITY_GATE_v0.92.1.md
  FEATURE_PROOF_COVERAGE_v0.92.1.md WP_EXECUTION_READINESS_v0.92.1.md
  ADR_PLAN_v0.92.1.md NEXT_MILESTONE_HANDOFF_v0.92.1.md features/README.md
  features/RUNTIME_V2_V3_DECOUPLING_v0.92.1.md
  features/PROVIDER_INFERENCE_PROFILES_v0.92.1.md
  features/GCP_SIX_RESIDENT_QUALIFICATION_v0.92.1.md
  features/CODEFRIEND_BETA1_HANDOFF_v0.92.1.md
]
required.each { |path| errors << "missing:#{path}" unless milestone.join(path).file? }

wave = YAML.safe_load(milestone.join("WP_ISSUE_WAVE_v0.92.1.yaml").read, aliases: false)
spec = YAML.safe_load(milestone.join("WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml").read, aliases: false)

flatten = lambda do |rows|
  rows.flat_map { |row| [row] + flatten.call(row.fetch("packages", [])) }
end
rows = flatten.call(wave.fetch("work_packages"))
ids = rows.map { |row| row.fetch("id") }
errors << "duplicate planned ids" unless ids.uniq.length == ids.length

expected = %w[
  REP-01 WP-01 CORP-01 CORP-A CORP-B CORP-C CORP-D V3-01 V3-A V3-B V3-C
  V3-D V3-E V3-F DRT-01 DRT-A DRT-B DRT-C POD-01 DEC-01 PROV-01 PROV-A
  PROV-B DRT-D HOT-01 OBS-01 OBS-A OBS-B INT-01 TAIL-01 TAIL-02 TAIL-03
  TAIL-04 TAIL-05 TAIL-06 TAIL-07 TAIL-08 TAIL-09 TAIL-10
]
errors << "planned id denominator mismatch" unless ids.sort == expected.sort

known_refs = ids.to_h { |id| [id, true] }
known_refs.merge!(%w[issue-84 issue-345].to_h { |id| [id, true] })
rows.each do |row|
  Array(row["depends_on"]).each do |dep|
    errors << "unknown dependency:#{row['id']}:#{dep}" unless known_refs[dep]
  end
end

spec_ids = spec.fetch("issue_specifications").map { |row| row.fetch("id") }
creation_ids = rows.select { |row| row["creation_owner"] == "WP-01" }.map { |row| row.fetch("id") }
errors << "execution specification denominator mismatch" unless spec_ids.sort == creation_ids.sort

tail = rows.select { |row| row.fetch("id").start_with?("TAIL-") }
errors << "release tail order mismatch" unless tail.map { |row| row.fetch("id") } == (1..10).map { |n| "TAIL-%02d" % n }

text = milestone.glob("**/*").select(&:file?).map(&:binread).join("\n")
errors << "tracked local-path dependency" if text.include?(".adl/")
%w[#84 #122 #251 #345 #457 #188 #189 #190 v0.92.2 Runtime\ v2 Runtime\ v3].each do |marker|
  errors << "missing routing marker:#{marker}" unless text.include?(marker.gsub("\\ ", " "))
end

if errors.empty?
  puts JSON.generate(schema: "adl.v0921.plan-validation.v1", result: "passed", planned_ids: ids.length,
                     creation_slots: creation_ids.length, release_tail: tail.length)
else
  warn errors.join("\n")
  exit 1
end
