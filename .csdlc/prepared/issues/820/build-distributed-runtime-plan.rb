#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

DENOMINATOR = "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json"
SOURCE = "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/retained-corporate-runtime.json"
OUTPUT = ".csdlc/prepared/issues/820/distributed-runtime-resolution-plan.json"
EXPECTED_CANDIDATE = "fb6cbc7f619daa54f901fd2d12f480add682ace3"

def candidate_json(path)
  bytes, stderr, status = Open3.capture3("git", "show", "#{EXPECTED_CANDIDATE}:#{path}")
  raise "cannot load #{path} from candidate: #{stderr}" unless status.success?
  JSON.parse(bytes)
end

denominator = candidate_json(DENOMINATOR).fetch("remediation_rows").select { |row| row.fetch("row_id").start_with?("DRT-") }
source_rows = candidate_json(SOURCE).fetch("rows").to_h { |row| [row.fetch("row_id"), row] }

rows = denominator.map do |denom|
  source = source_rows.fetch(denom.fetch("row_id"))
  raise "denominator admitted a proving row: #{source.fetch('row_id')}" unless source.fetch("proposed_result") == "non_proving"

  disposition = "remove_from_v0.92.1_retained_release_gate"
  scope = "Remove only #{source.fetch('row_id')} (#{denom.fetch('criterion_text_digest')}) from the v0.92.1 D520-RET-001 distributed-Runtime retained gate; no product behavior is claimed implemented or removed."
  basis = source.fetch("rationale")
  boundary = "Candidate-bound qualification execution confirms the retained fixture contract only. #{source.fetch('remaining_action')} It does not prove live multi-node, cloud, partition, soak, security, replay, or cleanup behavior."
  approval_state = "pending_operator_review"
  resolution_type = "governed_disposition_proposal"
  category = "candidate_fixture_not_production_execution"
  replacement_text = nil
  behavioral_pass_claim = false
  operator_target = "Merging the closing PR for issue #820 approves only this exact proposal and digest."
  operator_effect = "Before merge this proposal remains pending and release-blocking."
  proposal_digest = Digest::SHA256.hexdigest([source.fetch("row_id"), denom.fetch("criterion_text_digest"), resolution_type, category, disposition, scope, replacement_text, behavioral_pass_claim, basis, boundary, approval_state, operator_target, operator_effect].join("\0"))

  {
    "row_id" => source.fetch("row_id"),
    "criterion_id" => denom.fetch("criterion_id"),
    "criterion_text" => source.fetch("text"),
    "criterion_text_digest" => denom.fetch("criterion_text_digest"),
    "owner" => denom.fetch("owner"),
    "source_assessment" => source.fetch("proposed_result"),
    "source_evidence_paths" => source.fetch("evidence_refs").uniq.sort,
    "resolution" => {
      "type" => resolution_type,
      "category" => category,
      "proposed_disposition" => disposition,
      "removal_scope" => scope,
      "replacement_text" => replacement_text,
      "proposal_digest" => proposal_digest,
      "behavioral_pass_claim" => behavioral_pass_claim,
      "approval_state" => approval_state,
      "criterion_specific_basis" => basis,
      "proof_boundary" => boundary,
      "operator_review_target" => operator_target,
      "operator_review_effect" => operator_effect
    }
  }
end

raise "expected exactly 25 DRT rows, got #{rows.length}" unless rows.length == 25
raise "duplicate DRT rows" unless rows.map { |row| row.fetch("row_id") }.uniq.length == 25

document = {
  "schema" => "adl.v0921.issue820.distributed_runtime_resolution_plan.v1",
  "issue" => 820,
  "parent_issue" => 522,
  "finding" => "D520-RET-001",
  "candidate" => EXPECTED_CANDIDATE,
  "denominator" => DENOMINATOR,
  "source_mapping" => SOURCE,
  "row_count" => rows.length,
  "candidate_execution_support_count" => rows.length,
  "disposition_proposal_count" => rows.length,
  "behavioral_pass_count" => 0,
  "rows_digest" => Digest::SHA256.hexdigest(rows.map { |row| row.fetch("row_id") }.sort.join("\0")),
  "rows" => rows
}
File.write(OUTPUT, JSON.pretty_generate(document) + "\n")
puts "wrote #{OUTPUT}: 25 governed disposition proposals, zero behavioral passes"
