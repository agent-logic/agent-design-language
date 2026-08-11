#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/201/"
OUTPUT = ROOT.join(PREFIX, "v2")
MARKER = "ADL_ISSUE_201_CASE_V1 "
PROTECTED = [
  "adl-runtime/Cargo.toml", "adl-runtime/Cargo.lock",
  "adl-runtime/src/distributed/mod.rs",
  "adl-runtime/src/distributed/authority_protocol.rs",
  "adl-runtime/src/distributed/polis_runtime.rs",
  "adl-runtime/tests/distributed_authority_protocol.rs",
  ".csdlc/prepared/issues/201/produce-proof-receipt.rb",
  ".csdlc/prepared/issues/201/validate-proof-receipt.rb"
].freeze
EXPECTED_CASES = %w[
  current_three_voter_finalize exact_retry_returns_cached_result
  signer_rotation_current_generation joint_majority_each_config finalize_at_deadline
  three_node_checkpoint_restart_reconcile missing_quorum duplicate_signer wrong_voter
  signer_unavailable expired_signer_cert stale_membership config_digest_mismatch
  joint_old_only joint_new_only joint_union_majority_only joint_duplicate_guardian_reuse
  declared_finalize_time_after_deadline finalize_before_prepare_time
  replay_with_regressed_finalize_time local_clock_skew_apply_parity checkpoint_object_collision
  node_a_local_before_cas node_a_cas_before_final_marker node_b_local_before_cas
  node_b_cas_before_final_marker node_c_local_before_cas node_c_cas_before_final_marker
  checkpoint_result_retry_digest_mismatch coherent_rollback_rejected corrupt_journal_rejected
  corrupt_retry_cache_rejected capacity_n_plus_one_no_partial state_symlink_rejected
  lock_symlink_rejected legacy_fence_voter_rejected legacy_activate_owner_rejected
  legacy_activate_shepherd_rejected legacy_acquire_observatory_rejected
  legacy_demote_voter_rejected exact_store_artifact_bytes_retained
  artifact_bytes_digest_substitution_rejected sealed_continuity_transfer_projection
  continuity_projection_consumer_confusion_rejected continuity_projection_wrong_lineage_rejected
  continuity_projection_wrong_source_checkpoint_handle_rejected
  continuity_projection_wrong_bundle_handle_rejected
].freeze
EXPECTED_RESULTS = {
  "current_three_voter_finalize" => "passed",
  "exact_retry_returns_cached_result" => "passed",
  "joint_majority_each_config" => "passed",
  "finalize_at_deadline" => "passed",
  "three_node_checkpoint_restart_reconcile" => "passed",
  "local_clock_skew_apply_parity" => "passed",
  "exact_store_artifact_bytes_retained" => "passed",
  "sealed_continuity_transfer_projection" => "passed",
  "node_a_local_before_cas" => "reconciled",
  "node_a_cas_before_final_marker" => "reconciled",
  "node_b_local_before_cas" => "reconciled",
  "node_b_cas_before_final_marker" => "reconciled",
  "node_c_local_before_cas" => "reconciled",
  "node_c_cas_before_final_marker" => "reconciled"
}.freeze

def fail_proof(message)
  abort("issue 201 producer: #{message}")
end

def run_command(name, argv)
  started = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3({ "NEXTEST_TEST_THREADS" => "1" }, *argv, chdir: ROOT.to_s)
  finished = Time.now.utc.iso8601(6)
  stdout_path = OUTPUT.join("#{name}.stdout.log")
  stderr_path = OUTPUT.join("#{name}.stderr.log")
  File.binwrite(stdout_path, stdout)
  File.binwrite(stderr_path, stderr)
  {
    "argv" => argv, "exit_code" => status.exitstatus, "started_at" => started,
    "finished_at" => finished,
    "stdout_path" => stdout_path.relative_path_from(ROOT).to_s,
    "stdout_sha256" => Digest::SHA256.hexdigest(stdout),
    "stderr_path" => stderr_path.relative_path_from(ROOT).to_s,
    "stderr_sha256" => Digest::SHA256.hexdigest(stderr)
  }
end

source, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail_proof("cannot resolve source") unless status.success? && source.strip.match?(/\A[0-9a-f]{40}\z/)
source = source.strip
dirty, status = Open3.capture2("git", "status", "--porcelain=v1", "--untracked-files=all", chdir: ROOT.to_s)
dirty_lines = dirty.lines.reject { |line| line[3..]&.start_with?(PREFIX) }
fail_proof("source worktree must be clean") unless status.success? && dirty_lines.empty?
PROTECTED.each do |relative|
  path = ROOT.join(relative)
  fail_proof("missing or unsafe protected path: #{relative}") unless path.file? && !path.symlink?
  committed, committed_status = Open3.capture2("git", "show", "#{source}:#{relative}", chdir: ROOT.to_s)
  fail_proof("protected path absent at source: #{relative}") unless committed_status.success?
  fail_proof("protected path dirty: #{relative}") unless Digest::SHA256.hexdigest(committed) == Digest::SHA256.file(path).hexdigest
end
FileUtils.mkdir_p(OUTPUT, mode: 0o700)

commands = {}
commands["nextest"] = run_command("nextest", %w[cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_protocol --no-tests=fail])
fail_proof("focused nextest failed") unless commands["nextest"]["exit_code"] == 0
nextest_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["nextest"]["#{stream}_path"])) }.join
fail_proof("nextest denominator mismatch") unless nextest_text.match?(/47 tests run: 47 passed, 0 skipped/)

commands["clippy"] = run_command("clippy", %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_protocol -- -D warnings])
fail_proof("strict Clippy failed") unless commands["clippy"]["exit_code"] == 0
commands["machine_cases"] = run_command("machine-cases", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_protocol -- --nocapture --test-threads=1])
fail_proof("machine cases failed") unless commands["machine_cases"]["exit_code"] == 0
machine_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["machine_cases"]["#{stream}_path"])) }.join
observed = machine_text.lines.each_with_object([]) do |line, rows|
  next unless line.include?(MARKER)
  name, result = line.split(MARKER, 2).fetch(1).strip.split(" ", 2)
  rows << [name, result, Digest::SHA256.hexdigest(line.chomp)]
end
fail_proof("case denominator mismatch") unless observed.length == 47 && observed.map(&:first).sort == EXPECTED_CASES.sort
observed_by_name = observed.to_h { |name, result, digest| [name, [result, digest]] }
EXPECTED_CASES.each { |name| fail_proof("wrong result for #{name}") unless observed_by_name.fetch(name).first == EXPECTED_RESULTS.fetch(name, "rejected") }
tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source tree unavailable") unless status.success?
proof = {
  "schema" => "adl.issue201.committed_authority_proof.v1", "issue" => 201,
  "source_revision" => source, "source_tree" => tree.strip,
  "protected_files" => PROTECTED.map { |path| { "path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest } },
  "commands" => commands, "test_summary" => { "selected" => 47, "passed" => 47, "skipped" => 0 },
  "cases" => EXPECTED_CASES.map { |name| result, digest = observed_by_name.fetch(name); { "case" => name, "result" => result, "observed_line_sha256" => digest } }
}
File.binwrite(OUTPUT.join("execution-proof.json"), JSON.generate(proof) + "\n")
puts "PASS: produced exact issue #201 47-case proof at source #{source}"
