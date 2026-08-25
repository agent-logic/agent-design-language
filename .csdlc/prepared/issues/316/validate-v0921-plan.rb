#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
require "pathname"
require "yaml"

root = Pathname(__dir__).join("../../../..").realpath
milestone = root.join("docs/milestones/v0.92.1")
errors = []

ledger_path = root.join(".csdlc/evidence/316/source-disposition-ledger.json")
unless ledger_path.file?
  errors << "missing:source-disposition-ledger"
else
  ledger = JSON.parse(ledger_path.read)
  candidates = ledger.fetch("candidates")
  expected_candidate_ids = (1..17).map { |n| "TBD-%03d" % n } +
                           (1..16).map { |n| "CF-%03d" % n } +
                           %w[DRIVE-CF-001 DRIVE-CF-002 DRIVE-CF-003 DRIVE-CF-004 DRIVE-CF-005 DRIVE-ATE-001 GIT-001]
  candidate_ids = candidates.map { |row| row.fetch("candidate_id") }
  errors << "source candidate denominator mismatch" unless candidate_ids.sort == expected_candidate_ids.sort
  errors << "duplicate source candidate ids" unless candidate_ids.uniq.length == candidate_ids.length
  errors << "source candidate count mismatch" unless ledger.fetch("candidate_count") == candidates.length
  candidates.each do |row|
    errors << "local source dependency:#{row['candidate_id']}" if row.fetch("source_identity").include?(".adl/")
    errors << "missing disposition:#{row['candidate_id']}" if row.fetch("disposition").strip.empty?
    errors << "missing target:#{row['candidate_id']}" if row.fetch("target").strip.empty?
    errors << "missing reason:#{row['candidate_id']}" if row.fetch("reason").strip.empty?
    errors << "source became execution dependency:#{row['candidate_id']}" unless row.fetch("execution_dependency") == false
    digest = row["source_sha256"]
    if row.fetch("source_class").start_with?("local_")
      errors << "invalid local source digest:#{row['candidate_id']}" unless digest&.match?(/\A[0-9a-f]{64}\z/)
    end
    if row.fetch("source_class") == "google_drive_snapshot"
      errors << "invalid Drive snapshot digest:#{row['candidate_id']}" unless digest&.match?(/\A[0-9a-f]{64}\z/)
      errors << "missing Drive mirror:#{row['candidate_id']}" if row.fetch("mirror_identity").strip.empty?
      errors << "Drive snapshot identity unavailable without disposition:#{row['candidate_id']}" if row.fetch("drive_document_id_status").strip.empty?
    end
  end
end

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
  features/AWS_ACCOUNT_MOVE_IN_v0.92.1.md
  features/GCP_ACCOUNT_MOVE_IN_v0.92.1.md
  features/CROSS_CLOUD_TERRAFORM_CONVERSION_v0.92.1.md
  features/RUST_RESILIENCE_REFACTORING_v0.92.1.md
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

wp01 = rows.find { |row| row["id"] == "WP-01" }
errors << "concrete closed conductor assigned future work" unless wave["conductor_issue"].nil?
errors << "missing number-free conductor id" unless wave["conductor_id"] == "WP-01"
errors << "missing milestone-opening authority" unless wave["opening_authority"] == "milestone_operator_after_planning_merge"
errors << "missing operator-controlled opening trigger" unless wave["opening_trigger"] == "operator_declares_milestone_ready_after_planning_merge"
errors << "closed #431 must be provenance only" unless wave["legacy_planning_issue"] == 431
if wp01
  errors << "WP-01 must remain number-free before opening" unless wp01["issue"].nil?
  errors << "WP-01 lacks viable future creation owner" unless wp01["creation_owner"] == "milestone_operator_after_planning_merge"
  errors << "WP-01 legacy #431 mapping missing" unless wp01["legacy_issue"] == 431
  errors << "closed #431 assigned future conductor authority" unless wp01["legacy_issue_disposition"] == "closed_planning_provenance_only"
else
  errors << "missing WP-01 conductor row"
end

expected = %w[
  REP-01 WP-01 CORP-01 CORP-A CORP-B CORP-C CORP-D
  AWS-01 AWS-A AWS-B AWS-C AWS-D AWS-E AWS-F AWS-G
  GCP-01 GCP-A GCP-B GCP-C GCP-D GCP-E XCL-01 RUST-01
  V3-01 V3-A V3-B V3-C
  V3-D V3-E V3-F DRT-01 DRT-A DRT-B DRT-C POD-01 DEC-01 PROV-01 PROV-A
  PROV-B DRT-D HOT-01 OBS-01 OBS-A OBS-B INT-01 TAIL-01 TAIL-02 TAIL-03
  TAIL-04 TAIL-05 TAIL-06 TAIL-07 TAIL-08 TAIL-09 TAIL-10
]
errors << "planned id denominator mismatch" unless ids.sort == expected.sort

known_refs = ids.to_h { |id| [id, true] }
known_refs.merge!(%w[issue-51 issue-84 issue-122 issue-251 issue-256 issue-340 issue-345].to_h { |id| [id, true] })
rows.each do |row|
  Array(row["depends_on"]).each do |dep|
    errors << "unknown dependency:#{row['id']}:#{dep}" unless known_refs[dep]
  end
end

spec_ids = spec.fetch("issue_specifications").map { |row| row.fetch("id") }
creation_ids = rows.select { |row| row["creation_owner"] == "WP-01" }.map { |row| row.fetch("id") }
errors << "execution specification denominator mismatch" unless spec_ids.sort == (["WP-01"] + creation_ids).sort
errors << "WP-01 must not create itself" if creation_ids.include?("WP-01")
errors << "future creation wave has no children" if creation_ids.empty?

tail = rows.select { |row| row.fetch("id").start_with?("TAIL-") }
errors << "release tail order mismatch" unless tail.map { |row| row.fetch("id") } == (1..10).map { |n| "TAIL-%02d" % n }

text = milestone.glob("**/*").select(&:file?).map(&:binread).join("\n")
errors << "tracked local-path dependency" if text.include?(".adl/")
%w[#84 #122 #251 #345 #457 #188 #189 #190 v0.92.2 Runtime\ v2 Runtime\ v3].each do |marker|
  errors << "missing routing marker:#{marker}" unless text.include?(marker.gsub("\\ ", " "))
end

if errors.empty?
  puts JSON.generate(schema: "adl.v0921.plan-validation.v1", result: "passed", planned_ids: ids.length,
                     creation_slots: creation_ids.length, release_tail: tail.length,
                     source_candidates: ledger.fetch("candidate_count"),
                     source_ledger_sha256: Digest::SHA256.file(ledger_path).hexdigest)
else
  warn errors.join("\n")
  exit 1
end
