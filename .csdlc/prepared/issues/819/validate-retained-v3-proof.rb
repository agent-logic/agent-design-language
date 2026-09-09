#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

DENOMINATOR = ENV.fetch("ISSUE819_DENOMINATOR", "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json")
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

denominator = JSON.parse(File.read(DENOMINATOR)).fetch("remediation_rows").select { |row| row.fetch("mapping_file") == "retained-v3.json" }
plan = JSON.parse(File.read(PLAN))
receipt = JSON.parse(File.read(RECEIPT))
expected_ids = denominator.map { |row| row.fetch("row_id") }.sort
plan_ids = plan.fetch("rows").map { |row| row.fetch("row_id") }
receipt_ids = receipt.fetch("rows").map { |row| row.fetch("row_id") }

assert(expected_ids.length == 152, "denominator must contain exactly 152 rows")
assert(plan_ids.length == plan_ids.uniq.length && plan_ids.sort == expected_ids, "plan must consume every denominator row exactly once")
assert(receipt_ids.length == receipt_ids.uniq.length && receipt_ids.sort == expected_ids, "receipt must consume every denominator row exactly once")
assert(receipt.fetch("row_count") == 152, "receipt row count drift")
assert(receipt.fetch("unresolved") == 0, "receipt has unresolved rows")
assert(receipt.fetch("candidate").match?(/\A[0-9a-f]{40}\z/), "candidate is not an exact SHA")
assert(receipt.fetch("candidate") == EXPECTED_CANDIDATE, "receipt is not bound to the #520 candidate")
assert(git("cat-file", "-t", receipt.fetch("candidate")) == "commit", "candidate commit unavailable")

source_rows = JSON.parse(File.read(plan.fetch("source_mapping"))).fetch("rows").to_h { |row| [row.fetch("row_id"), row] }
denominator_by_id = denominator.to_h { |row| [row.fetch("row_id"), row] }
plan.fetch("rows").each do |row|
  source = source_rows.fetch(row.fetch("row_id"))
  denom = denominator_by_id.fetch(row.fetch("row_id"))
  assert(row.fetch("criterion_id") == denom.fetch("criterion_id"), "criterion identity drift for #{row.fetch('row_id')}")
  assert(row.fetch("criterion_text") == source.fetch("criterion_text"), "criterion text drift for #{row.fetch('row_id')}")
  assert(row.fetch("criterion_text_digest") == denom.fetch("criterion_text_digest"), "criterion digest source drift for #{row.fetch('row_id')}")
  assert(row.fetch("source_evidence_paths") == source.fetch("evidence").map { |entry| entry.fetch("path") }.uniq.sort, "source evidence denominator drift for #{row.fetch('row_id')}")
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
receipt.fetch("rows").each do |row|
  resolution = row.fetch("resolution")
  assert(Digest::SHA256.hexdigest(row.fetch("criterion_text")) == row.fetch("criterion_text_digest"), "criterion digest drift for #{row.fetch('row_id')}")
  case resolution.fetch("type")
  when "candidate_bound_execution"
    assert(resolution.fetch("status") == "passed", "execution row is not passed")
    test = resolution.fetch("required_test")
    assert(test_log.match?(/^test .*#{Regexp.escape(test)} \.\.\. ok$/), "test did not execute for #{row.fetch('row_id')}")
  when "governed_amendment"
    assert(resolution.fetch("status") == "accepted_by_operator_reviewed_cutover", "amendment lacks accepted cutover basis")
    assert(resolution.fetch("behavioral_pass_claim") == false, "amendment may not claim behavioral pass")
    authority = resolution.fetch("authority")
    assert(authority.fetch("operator_decision").include?("PR #591"), "amendment lacks operator decision")
    %w[current_contract contract_detail].each { |key| assert(File.file?(authority.fetch(key)), "missing amendment authority #{key}") }
  else
    raise "unknown resolution type"
  end
  row.fetch("candidate_evidence").each do |entry|
    assert(entry.fetch("status") == "present", "missing candidate evidence #{entry.fetch('path')}")
    candidate_bytes = git_bytes("show", "#{receipt.fetch('candidate')}:#{entry.fetch('path')}")
    assert(Digest::SHA256.hexdigest(candidate_bytes) == entry.fetch("sha256"), "candidate evidence digest drift")
    assert(git("rev-parse", "#{receipt.fetch('candidate')}:#{entry.fetch('path')}") == entry.fetch("git_blob"), "candidate evidence blob drift")
  end
end

execution = receipt.fetch("rows").count { |row| row.dig("resolution", "type") == "candidate_bound_execution" }
amendments = receipt.fetch("rows").count { |row| row.dig("resolution", "type") == "governed_amendment" }
assert(execution == receipt.fetch("execution_passed"), "execution count drift")
assert(amendments == receipt.fetch("governed_amendments"), "amendment count drift")
assert(execution + amendments == 152, "resolution partition drift")
puts "PASS issue #819 retained-v3: 152/152 unique, #{execution} executed, #{amendments} governed amendments, 0 unresolved"
