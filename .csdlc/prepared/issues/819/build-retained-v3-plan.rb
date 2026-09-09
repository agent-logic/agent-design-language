#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

DENOMINATOR = "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json"
SOURCE = "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/retained-v3.json"
OUTPUT = ".csdlc/prepared/issues/819/retained-v3-resolution-plan.json"

AMENDMENTS = {
  "output-filter-removal" => %w[
    V3-A:retained-161-ac-6 V3-A:retained-161-ac-15
    V3-B:retained-164-ac-5 V3-B:retained-164-ac-6
  ],
  "bounded-filesystem-state" => %w[
    V3-A:retained-161-ac-11 V3-A:retained-161-ac-12
    V3-A:retained-161-ac-13 V3-A:retained-161-ac-14
  ],
  "accepted-operational-construction" => %w[
    V3-A:retained-162-ac-6 V3-A:retained-162-ac-7
    V3-B:retained-165-ac-2
  ],
  "synchronous-per-invocation-services" => %w[
    V3-B:retained-165-ac-1 V3-B:retained-165-ac-3
    V3-B:retained-165-ac-4 V3-B:retained-165-ac-5
    V3-B:retained-165-ac-6 V3-B:retained-165-ac-7
    V3-B:retained-165-ac-8 V3-B:retained-165-ac-9
    V3-B:retained-165-ac-10 V3-E:retained-174-ac-1
    V3-E:retained-174-ac-2 V3-E:retained-174-ac-3
    V3-E:retained-177-ac-5 V3-E:retained-177-ac-6
    V3-E:retained-177-ac-7 V3-E:retained-177-ac-8
    V3-E:retained-177-ac-9
  ],
  "projection-state-shape" => %w[
    V3-B:retained-167-ac-1 V3-B:retained-167-ac-5
    V3-C:retained-169-ac-1 V3-C:retained-169-ac-3
  ],
  "supported-platform-scope" => %w[
    V3-A:retained-163-ac-1 V3-A:retained-163-ac-2
    V3-C:retained-169-ac-8 V3-C:retained-169-ac-9
  ],
  "dependency-policy-separation" => %w[V3-B:retained-164-ac-8],
  "process-output-shape" => %w[V3-E:retained-174-ac-5]
}.freeze

amendment_by_row = {}
AMENDMENTS.each { |kind, ids| ids.each { |id| raise "duplicate amendment #{id}" if amendment_by_row[id]; amendment_by_row[id] = kind } }
denominator = JSON.parse(File.read(DENOMINATOR)).fetch("remediation_rows").select { |row| row.fetch("mapping_file") == "retained-v3.json" }
source_rows = JSON.parse(File.read(SOURCE)).fetch("rows").to_h { |row| [row.fetch("row_id"), row] }

rows = denominator.map do |denom|
  source = source_rows.fetch(denom.fetch("row_id"))
  resolution = if source.fetch("proposed_result") == "proven"
    {
      "type" => "candidate_bound_execution",
      "behavioral_pass_claim" => true,
      "command_id" => "csdlc-v3-all-tests",
      "criterion_specific_basis" => source.fetch("rationale"),
      "source_proof_boundary" => source.fetch("proof_boundary"),
      "execution_join" => "Complete C-SDLC v3 suite executed at the exact #520 candidate; candidate evidence paths are byte-bound below."
    }
  else
    kind = amendment_by_row[source.fetch("row_id")] || "unproven-#{source.fetch('root_cause')}"
    disposition = "remove_from_v0.92.1_retained_release_gate"
    removal_scope = "Remove only #{source.fetch('row_id')} (#{denom.fetch('criterion_text_digest')}) from the v0.92.1 D520-RET-001 retained-v3 release gate; no product behavior is claimed implemented or removed."
    proposal_digest = Digest::SHA256.hexdigest([
      source.fetch("row_id"), denom.fetch("criterion_text_digest"), disposition,
      removal_scope, source.fetch("rationale"), source.fetch("proof_boundary")
    ].join("\0"))
    {
      "type" => "governed_disposition_proposal",
      "category" => kind,
      "proposed_disposition" => disposition,
      "removal_scope" => removal_scope,
      "replacement_text" => nil,
      "proposal_digest" => proposal_digest,
      "behavioral_pass_claim" => false,
      "approval_state" => "pending_operator_review",
      "criterion_specific_basis" => source.fetch("rationale"),
      "source_proof_boundary" => source.fetch("proof_boundary"),
      "operator_review_target" => "The closing PR for issue #819 must explicitly approve this exact removal proposal and digest before release admission.",
      "operator_review_effect" => "Merging the closing PR for issue #819 approves only this exact proposal_digest; before merge it remains pending and release-blocking.",
      "preexisting_cutover_context" => "PR #591 selected the current operational implementation, but is not treated as criterion-specific approval for this disposition."
    }
  end
  {
    "row_id" => source.fetch("row_id"),
    "criterion_id" => denom.fetch("criterion_id"),
    "criterion_text" => source.fetch("criterion_text"),
    "criterion_text_digest" => denom.fetch("criterion_text_digest"),
    "owner" => denom.fetch("owner"),
    "source_assessment" => source.fetch("proposed_result"),
    "root_cause" => source.fetch("root_cause"),
    "source_evidence_paths" => source.fetch("evidence").map { |entry| entry.fetch("path") }.uniq.sort,
    "resolution" => resolution
  }
end

unknown_amendments = amendment_by_row.keys - rows.map { |row| row.fetch("row_id") }
raise "unknown amendments: #{unknown_amendments.join(', ')}" unless unknown_amendments.empty?
raise "expected 152 rows, got #{rows.length}" unless rows.length == 152

document = {
  "schema" => "adl.v0921.issue819.retained_v3_resolution_plan.v1",
  "issue" => 819,
  "parent_issue" => 522,
  "finding" => "D520-RET-001",
  "denominator" => DENOMINATOR,
  "source_mapping" => SOURCE,
  "row_count" => rows.length,
  "execution_count" => rows.count { |row| row.dig("resolution", "type") == "candidate_bound_execution" },
  "disposition_proposal_count" => rows.count { |row| row.dig("resolution", "type") == "governed_disposition_proposal" },
  "rows_digest" => Digest::SHA256.hexdigest(rows.map { |row| row.fetch("row_id") }.sort.join("\0")),
  "rows" => rows
}
File.write(OUTPUT, JSON.pretty_generate(document) + "\n")
puts "wrote #{OUTPUT}: #{document['execution_count']} execution, #{document['disposition_proposal_count']} governed removal proposals"
