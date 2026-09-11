#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

DENOMINATOR = "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json"
SOURCE = "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/retained-corporate-runtime.json"
OUTPUT = ".csdlc/prepared/issues/818/retained-corporate-runtime-resolution-plan.json"
EXPECTED_CANDIDATE = "add8f48867e55be9256244767758abba6f348091"

def candidate_json(path)
  bytes, stderr, status = Open3.capture3("git", "show", "#{EXPECTED_CANDIDATE}:#{path}")
  raise "cannot load canonical candidate path #{path}: #{stderr}" unless status.success?

  JSON.parse(bytes)
end

def category(row_id)
  case row_id
  when /retained-153-/ then "corporate-inventory-contract"
  when /retained-154-|retained-155-|retained-160-/ then "private-legal-and-use-rights-proof"
  when /retained-157-/ then "repository-migration-safeguards"
  when /retained-158-|retained-159-/ then "corporate-bootstrap-and-runbook-boundary"
  else raise "unclassified corporate row #{row_id}"
  end
end

def canonical(value)
  case value
  when Hash then value.keys.sort.to_h { |key| [key, canonical(value.fetch(key))] }
  when Array then value.map { |entry| canonical(entry) }
  else value
  end
end

denominator = candidate_json(DENOMINATOR).fetch("remediation_rows").select do |row|
  row.fetch("mapping_file") == "retained-corporate-runtime.json" && row.fetch("row_id").start_with?("CORP-")
end
source_rows = candidate_json(SOURCE).fetch("rows").to_h { |row| [row.fetch("row_id"), row] }

rows = denominator.map do |denom|
  source = source_rows.fetch(denom.fetch("row_id"))
  raise "#{source.fetch('row_id')} is not non-proving" unless source.fetch("proposed_result") == "non_proving"

  repository_paths = ([source.fetch("source_path")] + source.fetch("evidence_refs").reject { |ref| ref.start_with?("http://", "https://") }).uniq.sort
  reference_only = source.fetch("evidence_refs").select { |ref| ref.start_with?("http://", "https://") }.sort
  disposition = "remove_from_v0.92.1_retained_release_gate"
  removal_scope = "Remove only #{source.fetch('row_id')} (#{denom.fetch('criterion_text_digest')}) from the v0.92.1 D520-RET-001 corporate/Runtime retained release gate; no product behavior, private approval, execution, or provider state is claimed implemented or removed."
  proof_boundary = "The source assessment is non-proving. Public repository artifacts and issue links are context only; no private approval, instrument execution, provider readback, or behavioral pass is inferred."
  proposal = {
    "type" => "governed_disposition_proposal",
    "category" => category(source.fetch("row_id")),
    "proposed_disposition" => disposition,
    "removal_scope" => removal_scope,
    "replacement_text" => nil,
    "behavioral_pass_claim" => false,
    "approval_state" => "pending_operator_review",
    "criterion_specific_basis" => source.fetch("rationale"),
    "source_proof_boundary" => proof_boundary,
    "operator_review_target" => "The closing PR for issue #818 must explicitly approve this exact removal proposal and digest before release admission.",
    "operator_review_effect" => "Merging the closing PR for issue #818 approves only this exact proposal_digest; before merge it remains pending and release-blocking.",
    "preexisting_evidence_context" => "Source support, ownership, and issue or PR closure are not treated as execution proof or criterion-specific approval."
  }
  proposal["proposal_digest"] = Digest::SHA256.hexdigest(JSON.generate(canonical(proposal)))

  {
    "row_id" => source.fetch("row_id"),
    "criterion_id" => denom.fetch("criterion_id"),
    "criterion_text" => source.fetch("text"),
    "criterion_text_digest" => denom.fetch("criterion_text_digest"),
    "owner" => denom.fetch("owner"),
    "source_assessment" => source.fetch("proposed_result"),
    "source_rationale" => source.fetch("rationale"),
    "source_remaining_action" => source.fetch("remaining_action"),
    "source_repository_paths" => repository_paths,
    "reference_only_evidence" => reference_only,
    "resolution" => proposal
  }
end

raise "expected 17 rows, got #{rows.length}" unless rows.length == 17
raise "duplicate rows" unless rows.map { |row| row.fetch("row_id") }.uniq.length == 17

document = {
  "schema" => "adl.v0921.issue818.retained_corporate_runtime_resolution_plan.v1",
  "issue" => 818,
  "parent_issue" => 522,
  "finding" => "D520-RET-001",
  "candidate" => EXPECTED_CANDIDATE,
  "denominator" => DENOMINATOR,
  "source_mapping" => SOURCE,
  "row_count" => rows.length,
  "candidate_execution_count" => 0,
  "governed_disposition_proposal_count" => rows.length,
  "rows_digest" => Digest::SHA256.hexdigest(rows.map { |row| row.fetch("row_id") }.sort.join("\0")),
  "rows" => rows
}
File.write(OUTPUT, JSON.pretty_generate(document) + "\n")
puts "wrote #{OUTPUT}: 17 governed removal proposals, 0 behavioral pass claims"
