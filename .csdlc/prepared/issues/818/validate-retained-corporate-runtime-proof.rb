#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

DENOMINATOR = "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json"
SOURCE = "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/retained-corporate-runtime.json"
PLAN = ENV.fetch("ISSUE818_PLAN", ".csdlc/prepared/issues/818/retained-corporate-runtime-resolution-plan.json")
RECEIPT = ENV.fetch("ISSUE818_RECEIPT", ".csdlc/evidence/818/retained-corporate-runtime/reconciliation.json")
EXPECTED_CANDIDATE = "add8f48867e55be9256244767758abba6f348091"

def assert(condition, message)
  raise message unless condition
end

def git(*args)
  stdout, status = Open3.capture2e("git", *args)
  raise "git failure: #{stdout}" unless status.success?

  stdout.strip
end

def git_bytes(*args)
  stdout, stderr, status = Open3.capture3("git", *args)
  raise "git failure: #{stderr}" unless status.success?

  stdout
end

plan = JSON.parse(File.read(PLAN))
receipt = JSON.parse(File.read(RECEIPT))
denominator_bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{DENOMINATOR}")
source_bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{SOURCE}")
assert(File.binread(DENOMINATOR) == denominator_bytes, "working denominator differs from exact candidate")
assert(File.binread(SOURCE) == source_bytes, "working source mapping differs from exact candidate")

denominator = JSON.parse(denominator_bytes).fetch("remediation_rows").select do |row|
  row.fetch("mapping_file") == "retained-corporate-runtime.json" && row.fetch("row_id").start_with?("CORP-")
end
source_rows = JSON.parse(source_bytes).fetch("rows").to_h { |row| [row.fetch("row_id"), row] }
denominator_by_id = denominator.to_h { |row| [row.fetch("row_id"), row] }
expected_ids = denominator.map { |row| row.fetch("row_id") }.sort
plan_ids = plan.fetch("rows").map { |row| row.fetch("row_id") }
receipt_ids = receipt.fetch("rows").map { |row| row.fetch("row_id") }

assert(plan.fetch("denominator") == DENOMINATOR, "plan denominator pointer drift")
assert(plan.fetch("source_mapping") == SOURCE, "plan source pointer drift")
assert(plan.fetch("candidate") == EXPECTED_CANDIDATE, "plan candidate drift")
assert(expected_ids.length == 17, "denominator must contain exactly 17 corporate rows")
assert(expected_ids.all? { |id| id.start_with?("CORP-") }, "sibling bucket entered denominator")
assert(plan_ids.length == plan_ids.uniq.length && plan_ids.sort == expected_ids, "plan must consume all 17 rows exactly once")
assert(receipt_ids.length == receipt_ids.uniq.length && receipt_ids.sort == expected_ids, "receipt must consume all 17 rows exactly once")
assert(plan.fetch("row_count") == 17 && receipt.fetch("row_count") == 17, "row count drift")
assert(plan.fetch("candidate_execution_count") == 0, "unsupported candidate execution claim")
assert(receipt.fetch("candidate_execution_passed") == 0, "unsupported execution pass")
assert(receipt.fetch("governed_disposition_proposals") == 17, "proposal count drift")
assert(receipt.fetch("operator_approval_pending") == 17, "operator-review denominator drift")
assert(receipt.fetch("unclassified") == 0, "receipt has unclassified rows")
assert(receipt.fetch("release_ready") == false, "release may not be ready before operator approval")
assert(receipt.fetch("candidate") == EXPECTED_CANDIDATE, "receipt candidate drift")
assert(git("cat-file", "-t", EXPECTED_CANDIDATE) == "commit", "candidate unavailable")

receipt_by_id = receipt.fetch("rows").to_h { |row| [row.fetch("row_id"), row] }
plan.fetch("rows").each do |row|
  source = source_rows.fetch(row.fetch("row_id"))
  denom = denominator_by_id.fetch(row.fetch("row_id"))
  assert(source.fetch("proposed_result") == "non_proving", "source is not non-proving for #{row.fetch('row_id')}")
  assert(row.fetch("criterion_id") == denom.fetch("criterion_id"), "criterion identity drift")
  assert(row.fetch("criterion_text") == source.fetch("text"), "criterion text drift")
  assert(row.fetch("criterion_text_digest") == denom.fetch("criterion_text_digest"), "criterion digest source drift")
  assert(Digest::SHA256.hexdigest(row.fetch("criterion_text")) == row.fetch("criterion_text_digest"), "criterion digest drift")
  assert(row.fetch("owner") == denom.fetch("owner"), "owner drift")
  assert(row.fetch("source_assessment") == source.fetch("proposed_result"), "source assessment drift")
  assert(row.fetch("source_rationale") == source.fetch("rationale"), "source rationale drift")
  assert(row.fetch("source_remaining_action") == source.fetch("remaining_action"), "remaining action drift")
  expected_paths = ([source.fetch("source_path")] + source.fetch("evidence_refs").reject { |ref| ref.start_with?("http://", "https://") }).uniq.sort
  expected_refs = source.fetch("evidence_refs").select { |ref| ref.start_with?("http://", "https://") }.sort
  assert(row.fetch("source_repository_paths") == expected_paths, "source path drift")
  assert(row.fetch("reference_only_evidence") == expected_refs, "reference-only evidence drift")

  resolution = row.fetch("resolution")
  assert(resolution.fetch("type") == "governed_disposition_proposal", "unknown resolution type")
  assert(resolution.fetch("behavioral_pass_claim") == false, "proposal may not claim behavioral pass")
  assert(resolution.fetch("approval_state") == "pending_operator_review", "plan fabricates approval")
  assert(resolution.fetch("proposed_disposition") == "remove_from_v0.92.1_retained_release_gate", "disposition action drift")
  expected_scope = "Remove only #{row.fetch('row_id')} (#{row.fetch('criterion_text_digest')}) from the v0.92.1 D520-RET-001 corporate/Runtime retained release gate; no product behavior, private approval, execution, or provider state is claimed implemented or removed."
  assert(resolution.fetch("removal_scope") == expected_scope, "removal scope drift")
  assert(resolution.fetch("replacement_text").nil?, "implicit replacement text denied")
  expected_boundary = "The source assessment is non-proving. Public repository artifacts and issue links are context only; no private approval, instrument execution, provider readback, or behavioral pass is inferred."
  assert(resolution.fetch("source_proof_boundary") == expected_boundary, "proof boundary drift")
  expected_digest = Digest::SHA256.hexdigest([
    row.fetch("row_id"), row.fetch("criterion_text_digest"), resolution.fetch("proposed_disposition"),
    expected_scope, row.fetch("source_rationale"), row.fetch("source_remaining_action"), expected_boundary
  ].join("\0"))
  assert(resolution.fetch("proposal_digest") == expected_digest, "proposal digest drift")
  assert(resolution.fetch("operator_review_target").include?("exact removal proposal and digest"), "operator-review target is not exact")
  assert(resolution.fetch("operator_review_effect").include?("before merge it remains pending and release-blocking"), "pre-merge authority boundary missing")
  assert(resolution.fetch("preexisting_evidence_context").include?("not treated as execution proof"), "source reference was promoted to proof")

  observed = receipt_by_id.fetch(row.fetch("row_id"))
  expected_observed = row.merge(
    "resolution" => resolution.merge("status" => "pending_operator_review", "candidate" => EXPECTED_CANDIDATE)
  )
  observed_without_evidence = observed.reject { |key, _| key == "candidate_evidence" }
  assert(observed_without_evidence == expected_observed, "receipt plan drift for #{row.fetch('row_id')}")
  evidence = observed.fetch("candidate_evidence")
  assert(evidence.map { |entry| entry.fetch("path") }.sort == expected_paths, "candidate evidence path drift")
  evidence.each do |entry|
    assert(entry.fetch("status") == "present", "candidate evidence missing")
    assert(entry.fetch("proof_role") == "candidate_bound_context_not_behavioral_proof", "evidence proof role drift")
    candidate_bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{entry.fetch('path')}")
    assert(Digest::SHA256.hexdigest(candidate_bytes) == entry.fetch("sha256"), "candidate evidence digest drift")
    assert(git("rev-parse", "#{EXPECTED_CANDIDATE}:#{entry.fetch('path')}") == entry.fetch("git_blob"), "candidate evidence blob drift")
  end
end

expected_non_claims = [
  "No source reference, owner, issue closure, or PR closure is counted as execution proof.",
  "No private instrument, approval, signature, credential, or provider state was requested or inspected.",
  "No governed removal proposal is approved before operator review of the closing PR."
]
assert(receipt.fetch("non_claims") == expected_non_claims, "non-claim boundary drift")
assert(plan.fetch("rows_digest") == Digest::SHA256.hexdigest(expected_ids.join("\0")), "row digest drift")
puts "PASS issue #818 corporate/Runtime packet: 17/17 unique, 17 exact removals pending operator review, 0 behavioral passes, 0 unclassified, release_ready=false"
