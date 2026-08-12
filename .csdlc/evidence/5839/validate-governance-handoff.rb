#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

ROOT = File.expand_path("../../..", __dir__)
MAP = File.join(ROOT, "docs/milestones/v0.92/review/V092_TO_V093_GOVERNANCE_EVIDENCE_MAP.md")
ADR_PLAN = File.join(ROOT, "docs/milestones/v0.92/ADR_PLAN_v0.92.md")
HANDOFF = File.join(ROOT, "docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md")
V093_DECISIONS = File.join(ROOT, "docs/milestones/v0.93/DECISIONS_v0.93.md")
V093_PLAN = File.join(ROOT, "docs/milestones/v0.93/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md")

REQUIRED_PATHS = [
  MAP,
  ADR_PLAN,
  HANDOFF,
  V093_DECISIONS,
  V093_PLAN,
  File.join(ROOT, "docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md"),
  File.join(ROOT, "docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json"),
  File.join(ROOT, "docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md"),
  File.join(ROOT, ".csdlc/evidence/5835/rejected-transfer-matrix.json")
].freeze

FORBIDDEN_CLAIMS = [
  /v0\.93 governance is complete/i,
  /governance completion is (?:proved|complete|implemented|accepted)/i,
  /production citizenship is (?:proved|granted|complete|implemented|accepted)/i,
  /legal personhood is (?:proved|granted|complete|implemented|accepted)/i,
  /rights and duties are (?:proved|granted|complete|implemented|accepted)/i,
  /standing is (?:proved|granted|complete|implemented|accepted)/i,
  /polis authority is (?:proved|established|complete|implemented|accepted)/i,
  /\| ADR 0068 \|[^\n]+\| Accepted \|/i
].freeze

def fail_with(code, detail)
  warn JSON.generate(schema: "adl.wp19.validation.failure.v1", code: code, detail: detail)
  exit 1
end

missing = REQUIRED_PATHS.reject { |path| File.file?(path) }
fail_with("missing_required_paths", missing) unless missing.empty?

map = File.read(MAP)
adr = File.read(ADR_PLAN)
handoff = File.read(HANDOFF)
decisions = File.read(V093_DECISIONS)
plan = File.read(V093_PLAN)
all_text = [map, adr, handoff].join("\n")

if ARGV.include?("--negative")
  FORBIDDEN_CLAIMS.each do |pattern|
    fail_with("forbidden_governance_claim", pattern.source) if all_text.match?(pattern)
  end

  required_non_claims = [
    "does not grant citizenship",
    "does not grant standing",
    "does not assign rights or duties",
    "does not complete v0.93 governance",
    "does not accept ADR 0068",
    "blocked_with_evidence"
  ]
  missing_non_claims = required_non_claims.reject { |needle| all_text.include?(needle) }
  fail_with("missing_negative_boundary", missing_non_claims) unless missing_non_claims.empty?

  puts JSON.generate(schema: "adl.wp19.validation.v1", lane: "negative-governance", result: "passed")
  exit 0
end

unless decisions.include?("D-01") && decisions.include?("Accepted for planning")
  fail_with("missing_v093_accepted_planning_decision", V093_DECISIONS)
end

unless plan.include?("| v0.93 | Constitutional citizenship") && plan.include?("Feature And Idea Allocation")
  fail_with("missing_v093_allocation", V093_PLAN)
end

required_columns = [
  "v0.92 evidence source",
  "Accepted state or blocker",
  "Allowed v0.93 use",
  "Forbidden inference",
  "Redaction posture",
  "Unresolved decision",
  "Accepting consumer"
]
missing_columns = required_columns.reject { |column| map.include?(column) }
fail_with("missing_map_columns", missing_columns) unless missing_columns.empty?

required_rows = [
  "Birthday review packet",
  "Birthday evidence manifest",
  "Cross-polis continuity transfer",
  "Rejected continuity-transfer matrix",
  "Runtime first-birthday demo",
  "v0.93 governance allocation"
]
missing_rows = required_rows.reject { |row| map.include?(row) }
fail_with("missing_required_rows", missing_rows) unless missing_rows.empty?

unless adr.include?("| ADR 0068 | Birthday-To-Governance Handoff Boundary | Proposed |")
  fail_with("adr_0068_not_proposed", ADR_PLAN)
end

unless handoff.include?("V092_TO_V093_GOVERNANCE_EVIDENCE_MAP.md")
  fail_with("handoff_missing_map_reference", HANDOFF)
end

puts JSON.generate(schema: "adl.wp19.validation.v1", lane: "map-completeness", result: "passed")
