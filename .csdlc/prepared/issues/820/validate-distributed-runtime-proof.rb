#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

DENOMINATOR = "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json"
SOURCE = "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/retained-corporate-runtime.json"
PLAN = ENV.fetch("ISSUE820_PLAN", ".csdlc/prepared/issues/820/distributed-runtime-resolution-plan.json")
RECEIPT = ENV.fetch("ISSUE820_RECEIPT", ".csdlc/evidence/820/distributed-runtime/reconciliation.json")
EXPECTED_CANDIDATE = "fb6cbc7f619daa54f901fd2d12f480add682ace3"
EXPECTED_COMMANDS = {
  "distributed-contract" => ["cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_contract"],
  "distributed-failure-drt-c" => ["cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_failure_drt_c"]
}.freeze

def assert(value, message)
  raise message unless value
end
def git_bytes(*args)
  stdout, stderr, status = Open3.capture3("git", *args)
  raise "git failure: #{stderr}" unless status.success?
  stdout
end

def git(*args)
  stdout, stderr, status = Open3.capture3("git", *args)
  raise "git failure: #{stderr}" unless status.success?
  stdout.strip
end

plan = JSON.parse(File.read(PLAN))
receipt = JSON.parse(File.read(RECEIPT))
denominator_bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{DENOMINATOR}")
source_bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{SOURCE}")
assert(File.binread(DENOMINATOR) == denominator_bytes, "working denominator differs from candidate")
assert(File.binread(SOURCE) == source_bytes, "working source differs from candidate")
expected_rows = JSON.parse(denominator_bytes).fetch("remediation_rows").select { |row| row.fetch("row_id").start_with?("DRT-") }
expected_ids = expected_rows.map { |row| row.fetch("row_id") }.sort
source = JSON.parse(source_bytes).fetch("rows").to_h { |row| [row.fetch("row_id"), row] }
denominator = expected_rows.to_h { |row| [row.fetch("row_id"), row] }

assert(plan.fetch("schema") == "adl.v0921.issue820.distributed_runtime_resolution_plan.v1", "plan schema mismatch")
assert(plan.fetch("issue") == 820 && plan.fetch("parent_issue") == 522, "plan issue identity mismatch")
assert(plan.fetch("finding") == "D520-RET-001", "plan finding mismatch")
assert(plan.fetch("candidate") == EXPECTED_CANDIDATE, "plan candidate mismatch")
assert(plan.fetch("denominator") == DENOMINATOR && plan.fetch("source_mapping") == SOURCE, "plan source pointer mismatch")
assert(plan.fetch("row_count") == 25 && plan.fetch("disposition_proposal_count") == 25, "plan count mismatch")
assert(plan.fetch("candidate_execution_support_count") == 25 && plan.fetch("behavioral_pass_count") == 0, "plan proof count mismatch")
assert(receipt.fetch("schema") == "adl.v0921.issue820.distributed_runtime_reconciliation.v1", "receipt schema mismatch")
assert(receipt.fetch("issue") == 820 && receipt.fetch("parent_issue") == 522, "receipt issue identity mismatch")
assert(receipt.fetch("finding") == "D520-RET-001", "receipt finding mismatch")
assert(expected_ids.length == 25, "denominator must contain exactly 25 DRT rows")
assert(plan.fetch("rows").map { |row| row.fetch("row_id") }.sort == expected_ids, "plan denominator mismatch")
assert(receipt.fetch("rows").map { |row| row.fetch("row_id") }.sort == expected_ids, "receipt denominator mismatch")
assert(receipt.fetch("row_count") == 25, "receipt count mismatch")
assert(receipt.fetch("candidate") == EXPECTED_CANDIDATE, "candidate mismatch")
assert(receipt.fetch("behavioral_pass_count") == 0, "fixture proof may not become behavioral pass")
assert(receipt.fetch("operator_approval_pending") == 25, "all 25 rows must await operator review")
assert(receipt.fetch("unclassified") == 0, "unclassified rows remain")
assert(receipt.fetch("release_ready") == false, "release must remain blocked before approval")

plan_by_id = plan.fetch("rows").to_h { |row| [row.fetch("row_id"), row] }
receipt_by_id = receipt.fetch("rows").to_h { |row| [row.fetch("row_id"), row] }
expected_ids.each do |id|
  planned = plan_by_id.fetch(id)
  observed = receipt_by_id.fetch(id)
  original = source.fetch(id)
  denom = denominator.fetch(id)
  assert(planned.fetch("criterion_id") == denom.fetch("criterion_id"), "criterion id drift")
  assert(planned.fetch("criterion_text_digest") == denom.fetch("criterion_text_digest"), "denominator digest drift")
  assert(planned.fetch("owner") == denom.fetch("owner"), "owner drift")
  assert(planned.fetch("criterion_text") == original.fetch("text"), "criterion text drift")
  assert(planned.fetch("source_evidence_paths") == original.fetch("evidence_refs").uniq.sort, "source evidence drift")
  %w[criterion_id criterion_text criterion_text_digest owner source_assessment source_evidence_paths].each do |field|
    assert(observed.fetch(field) == planned.fetch(field), "receipt #{field} drift")
  end
  planned_resolution = planned.fetch("resolution")
  observed_resolution = observed.fetch("resolution").reject { |key, _| %w[status candidate candidate_fixture_execution].include?(key) }
  assert(observed_resolution == planned_resolution, "receipt resolution drift")
end

commands = receipt.fetch("commands").to_h { |command| [command.fetch("id"), command] }
assert(commands.keys.sort == EXPECTED_COMMANDS.keys.sort, "command denominator mismatch")
EXPECTED_COMMANDS.each do |id, argv|
  command = commands.fetch(id)
  assert(command.fetch("argv") == argv, "#{id} argv mismatch")
  assert(command.fetch("status") == "passed" && command.fetch("exit_code") == 0, "#{id} failed")
  log = ".csdlc/evidence/820/distributed-runtime/#{id}.log"
  assert(Digest::SHA256.file(log).hexdigest == command.fetch("output_sha256"), "#{id} log digest mismatch")
end


surface_tree = receipt.fetch("candidate_surface_paths").flat_map do |path|
  git("ls-tree", "-r", "--full-tree", EXPECTED_CANDIDATE, "--", path).lines.map(&:strip)
end.sort.join("\n")
assert(Digest::SHA256.hexdigest(surface_tree) == receipt.fetch("candidate_surface_tree_sha256"), "proof surface digest mismatch")

receipt.fetch("rows").each do |row|
  original = source.fetch(row.fetch("row_id"))
  assert(original.fetch("proposed_result") == "non_proving", "proving row admitted")
  assert(Digest::SHA256.hexdigest(row.fetch("criterion_text")) == row.fetch("criterion_text_digest"), "criterion digest mismatch")
  resolution = row.fetch("resolution")
  assert(resolution.fetch("type") == "governed_disposition_proposal", "row lacks governed disposition")
  assert(resolution.fetch("status") == "pending_operator_review", "row bypasses operator review")
  assert(resolution.fetch("approval_state") == "pending_operator_review", "approval state bypasses operator review")
  assert(resolution.fetch("operator_review_target") == "Merging the closing PR for issue #820 approves only this exact proposal and digest.", "operator review target drift")
  assert(resolution.fetch("operator_review_effect") == "Before merge this proposal remains pending and release-blocking.", "operator review effect drift")
  assert(resolution.fetch("behavioral_pass_claim") == false, "row claims behavioral pass")
  assert(resolution.fetch("candidate_fixture_execution") == "passed", "candidate fixture support missing")
  assert(resolution.fetch("proof_boundary").include?(original.fetch("remaining_action")), "remaining action lost")
  expected_digest = Digest::SHA256.hexdigest([row.fetch("row_id"), row.fetch("criterion_text_digest"), resolution.fetch("proposed_disposition"), resolution.fetch("removal_scope"), resolution.fetch("criterion_specific_basis"), resolution.fetch("proof_boundary"), resolution.fetch("approval_state"), resolution.fetch("operator_review_target"), resolution.fetch("operator_review_effect")].join("\0"))
  assert(resolution.fetch("proposal_digest") == expected_digest, "proposal digest mismatch")
  row.fetch("candidate_evidence").each do |evidence|
    bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{evidence.fetch('path')}")
    assert(Digest::SHA256.hexdigest(bytes) == evidence.fetch("sha256"), "candidate evidence hash mismatch")
    assert(git("rev-parse", "#{EXPECTED_CANDIDATE}:#{evidence.fetch('path')}") == evidence.fetch("git_blob"), "candidate evidence blob mismatch")
  end
end

puts "validated issue 820: 25 unique DRT rows, candidate-bound fixture proof, zero behavioral passes, 25 governed dispositions"
