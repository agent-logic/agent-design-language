#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

DENOMINATOR = "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json"
SOURCE = "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/retained-v3.json"
PLAN = ENV.fetch("ISSUE819_PLAN", ".csdlc/prepared/issues/819/retained-v3-resolution-plan.json")
RECEIPT = ENV.fetch("ISSUE819_RECEIPT", ".csdlc/evidence/819/retained-v3/reconciliation.json")
EXPECTED_CANDIDATE = "fb6cbc7f619daa54f901fd2d12f480add682ace3"
EXPECTED_COMMANDS = {
  "csdlc-v3-all-tests" => ["cargo", "test", "--manifest-path", "csdlc-v3/Cargo.toml"],
  "csdlc-v3-clippy" => ["cargo", "clippy", "--manifest-path", "csdlc-v3/Cargo.toml", "--all-targets", "--", "-D", "warnings"],
  "v3a-current-contract" => ["ruby", ".csdlc/prepared/issues/571/validate-v3a-followup.rb"]
}.freeze

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
assert(plan.fetch("denominator") == DENOMINATOR, "plan denominator pointer drift")
assert(plan.fetch("source_mapping") == SOURCE, "plan source mapping pointer drift")
denominator_bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{DENOMINATOR}")
source_bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{SOURCE}")
assert(File.binread(DENOMINATOR) == denominator_bytes, "working denominator differs from exact candidate")
assert(File.binread(SOURCE) == source_bytes, "working source mapping differs from exact candidate")
denominator = JSON.parse(denominator_bytes).fetch("remediation_rows").select { |row| row.fetch("mapping_file") == "retained-v3.json" }
expected_ids = denominator.map { |row| row.fetch("row_id") }.sort
plan_ids = plan.fetch("rows").map { |row| row.fetch("row_id") }
receipt_ids = receipt.fetch("rows").map { |row| row.fetch("row_id") }

assert(expected_ids.length == 152, "denominator must contain exactly 152 rows")
assert(plan_ids.length == plan_ids.uniq.length && plan_ids.sort == expected_ids, "plan must consume every denominator row exactly once")
assert(receipt_ids.length == receipt_ids.uniq.length && receipt_ids.sort == expected_ids, "receipt must consume every denominator row exactly once")
assert(receipt.fetch("row_count") == 152, "receipt row count drift")
assert(receipt.fetch("unclassified") == 0, "receipt has unclassified rows")
assert(receipt.fetch("operator_approval_pending") == 101, "operator-review denominator drift")
assert(receipt.fetch("release_ready") == false, "receipt may not claim release readiness before operator approval")
assert(receipt.fetch("candidate").match?(/\A[0-9a-f]{40}\z/), "candidate is not an exact SHA")
assert(receipt.fetch("candidate") == EXPECTED_CANDIDATE, "receipt is not bound to the #520 candidate")
assert(git("cat-file", "-t", receipt.fetch("candidate")) == "commit", "candidate commit unavailable")

source_rows = JSON.parse(source_bytes).fetch("rows").to_h { |row| [row.fetch("row_id"), row] }
denominator_by_id = denominator.to_h { |row| [row.fetch("row_id"), row] }
plan.fetch("rows").each do |row|
  source = source_rows.fetch(row.fetch("row_id"))
  denom = denominator_by_id.fetch(row.fetch("row_id"))
  assert(row.fetch("criterion_id") == denom.fetch("criterion_id"), "criterion identity drift for #{row.fetch('row_id')}")
  assert(row.fetch("owner") == denom.fetch("owner"), "owner drift for #{row.fetch('row_id')}")
  assert(row.fetch("criterion_text") == source.fetch("criterion_text"), "criterion text drift for #{row.fetch('row_id')}")
  assert(row.fetch("criterion_text_digest") == denom.fetch("criterion_text_digest"), "criterion digest source drift for #{row.fetch('row_id')}")
  assert(row.fetch("source_evidence_paths") == source.fetch("evidence").map { |entry| entry.fetch("path") }.uniq.sort, "source evidence denominator drift for #{row.fetch('row_id')}")
  assert(row.fetch("source_assessment") == source.fetch("proposed_result"), "source assessment drift for #{row.fetch('row_id')}")
  assert(row.fetch("root_cause") == source.fetch("root_cause"), "root cause drift for #{row.fetch('row_id')}")
  resolution = row.fetch("resolution")
  assert(resolution.fetch("criterion_specific_basis") == source.fetch("rationale"), "criterion-specific basis drift for #{row.fetch('row_id')}")
  assert(resolution.fetch("source_proof_boundary") == source.fetch("proof_boundary"), "source proof boundary drift for #{row.fetch('row_id')}")
end

receipt_by_id = receipt.fetch("rows").to_h { |row| [row.fetch("row_id"), row] }
plan.fetch("rows").each do |planned|
  observed = receipt_by_id.fetch(planned.fetch("row_id"))
  %w[criterion_id criterion_text criterion_text_digest owner source_assessment root_cause source_evidence_paths].each do |field|
    assert(observed.fetch(field) == planned.fetch(field), "receipt #{field} drift for #{planned.fetch('row_id')}")
  end
  planned_resolution = planned.fetch("resolution")
  observed_resolution = observed.fetch("resolution").reject { |key, _| %w[status candidate].include?(key) }
  assert(observed_resolution == planned_resolution, "receipt resolution drift for #{planned.fetch('row_id')}")
end

commands = receipt.fetch("commands").to_h { |command| [command.fetch("id"), command] }
assert(commands.keys.sort == EXPECTED_COMMANDS.keys.sort, "proof command denominator drift")
EXPECTED_COMMANDS.each do |id, argv|
  command = commands.fetch(id)
  assert(command.fetch("argv") == argv, "#{id} argv drift")
  assert(command.fetch("status") == "passed" && command.fetch("exit_code") == 0, "#{id} did not pass")
  log = ".csdlc/evidence/819/retained-v3/#{id}.log"
  assert(File.file?(log), "missing #{log}")
  assert(Digest::SHA256.file(log).hexdigest == command.fetch("output_sha256"), "#{id} log digest mismatch")
end

surface_paths = receipt.fetch("candidate_surface_paths")
expected_surface_paths = %w[csdlc-v3 docs/csdlc-v3 .csdlc/evidence/505 .csdlc/issues/505 .csdlc/prepared/issues/571/validate-v3a-followup.rb]
assert(surface_paths == expected_surface_paths, "candidate proof-surface denominator drift")
surface_tree = surface_paths.flat_map do |path|
  git("ls-tree", "-r", "--full-tree", receipt.fetch("candidate"), "--", path).lines.map(&:strip)
end.sort.join("\n")
assert(Digest::SHA256.hexdigest(surface_tree) == receipt.fetch("candidate_surface_tree_sha256"), "candidate proof-surface digest drift")

test_log = File.read(".csdlc/evidence/819/retained-v3/csdlc-v3-all-tests.log")
assert(test_log.scan(/^test .* \.\.\. ok$/).length == 211, "complete C-SDLC v3 execution denominator drift")
assert(!test_log.match?(/^test result: FAILED/), "C-SDLC v3 execution contains a failed test binary")
receipt.fetch("rows").each do |row|
  resolution = row.fetch("resolution")
  assert(Digest::SHA256.hexdigest(row.fetch("criterion_text")) == row.fetch("criterion_text_digest"), "criterion digest drift for #{row.fetch('row_id')}")
  case resolution.fetch("type")
  when "candidate_bound_execution"
    assert(resolution.fetch("status") == "passed", "execution row is not passed")
    assert(row.fetch("source_assessment") == "proven", "only source-supported rows may join candidate execution")
    assert(!resolution.fetch("criterion_specific_basis").strip.empty?, "execution row lacks criterion-specific basis")
    assert(resolution.fetch("execution_join").include?("exact #520 candidate"), "execution row lacks candidate join")
  when "governed_disposition_proposal"
    assert(resolution.fetch("status") == "pending_operator_review", "disposition proposal must remain operator-review pending")
    assert(resolution.fetch("behavioral_pass_claim") == false, "disposition may not claim behavioral pass")
    assert(row.fetch("source_assessment") == "non_proving", "only non-proving rows may enter disposition review")
    assert(!resolution.fetch("criterion_specific_basis").strip.empty?, "disposition lacks criterion-specific basis")
    assert(resolution.fetch("proposed_disposition") == "remove_from_v0.92.1_retained_release_gate", "disposition action is not exact")
    expected_scope = "Remove only #{row.fetch('row_id')} (#{row.fetch('criterion_text_digest')}) from the v0.92.1 D520-RET-001 retained-v3 release gate; no product behavior is claimed implemented or removed."
    assert(resolution.fetch("removal_scope") == expected_scope, "removal scope drift for #{row.fetch('row_id')}")
    assert(resolution.fetch("replacement_text").nil?, "removal proposal may not carry implicit replacement text")
    expected_proposal_digest = Digest::SHA256.hexdigest([
      row.fetch("row_id"), row.fetch("criterion_text_digest"), resolution.fetch("proposed_disposition"),
      expected_scope, resolution.fetch("criterion_specific_basis"), resolution.fetch("source_proof_boundary")
    ].join("\0"))
    assert(resolution.fetch("proposal_digest") == expected_proposal_digest, "proposal digest drift for #{row.fetch('row_id')}")
    assert(resolution.fetch("operator_review_target").include?("exact removal proposal and digest"), "disposition lacks explicit operator-review target")
    assert(resolution.fetch("operator_review_effect").include?("before merge it remains pending and release-blocking"), "disposition lacks pre-merge authority boundary")
    assert(resolution.fetch("preexisting_cutover_context").include?("not treated as criterion-specific approval"), "disposition fabricates prior approval")
  else
    raise "unknown resolution type"
  end
  evidence = row.fetch("candidate_evidence")
  assert(!evidence.empty?, "candidate evidence is empty for #{row.fetch('row_id')}")
  assert(evidence.map { |entry| entry.fetch("path") }.sort == row.fetch("source_evidence_paths"), "candidate evidence path drift for #{row.fetch('row_id')}")
  evidence.each do |entry|
    assert(entry.fetch("status") == "present", "missing candidate evidence #{entry.fetch('path')}")
    candidate_bytes = git_bytes("show", "#{receipt.fetch('candidate')}:#{entry.fetch('path')}")
    assert(Digest::SHA256.hexdigest(candidate_bytes) == entry.fetch("sha256"), "candidate evidence digest drift")
    assert(git("rev-parse", "#{receipt.fetch('candidate')}:#{entry.fetch('path')}") == entry.fetch("git_blob"), "candidate evidence blob drift")
  end
end

execution = receipt.fetch("rows").count { |row| row.dig("resolution", "type") == "candidate_bound_execution" }
amendments = receipt.fetch("rows").count { |row| row.dig("resolution", "type") == "governed_disposition_proposal" }
assert(execution == receipt.fetch("execution_passed"), "execution count drift")
assert(amendments == receipt.fetch("governed_disposition_proposals"), "disposition count drift")
assert(execution + amendments == 152, "resolution partition drift")
assert(execution == 51 && amendments == 101, "source-assessment partition drift")
puts "PASS issue #819 retained-v3 packet: 152/152 unique, #{execution} candidate-executed, #{amendments} exact removals pending operator review, 0 unclassified, release_ready=false"
