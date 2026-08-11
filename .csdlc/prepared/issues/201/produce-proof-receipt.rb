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
OUTPUT = ROOT.join(PREFIX, "v7")
MARKER = "ADL_ISSUE_201_CASE_V2 "
PROTECTED = [
  "adl-runtime/Cargo.toml", "adl-runtime/Cargo.lock",
  "adl-runtime/src/distributed/mod.rs",
  "adl-runtime/src/distributed/authority_protocol.rs",
  "adl-runtime/src/distributed/identity.rs",
  "adl-runtime/src/distributed/polis_runtime.rs",
  "adl-runtime/src/distributed/transport.rs",
  "adl-runtime/src/distributed/authority_protocol_contract_tests.rs",
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
  snapshot_valid_multi_prepared_finalized_restart snapshot_current_polis_mismatch
  snapshot_current_epoch_mismatch snapshot_current_membership_mismatch snapshot_current_boot_mismatch
  snapshot_prepared_polis_mismatch snapshot_prepared_epoch_mismatch snapshot_prepared_membership_mismatch
  snapshot_prepared_boot_mismatch snapshot_later_prepared_custody_mismatch
  snapshot_legacy_owner_injection snapshot_legacy_shepherd_injection
  snapshot_legacy_observatory_injection snapshot_legacy_fence_injection snapshot_legacy_demotion_injection
  snapshot_finalized_missing_proposal snapshot_finalized_missing_endorsements
  snapshot_finalized_wrong_operation snapshot_finalized_insufficient_quorum
  snapshot_finalized_duplicate_quorum snapshot_finalized_bad_signature
  snapshot_finalized_stale_certificate snapshot_finalized_wrong_boot snapshot_finalized_invalid_time
  snapshot_finalized_wrong_prepare_index snapshot_finalized_wrong_finalize_index
  snapshot_custody_omitted snapshot_custody_reencoded snapshot_custody_injected
  snapshot_custody_substituted snapshot_custody_byte_digest_mismatch
  snapshot_evidence_omitted snapshot_evidence_reencoded snapshot_evidence_injected
  snapshot_evidence_substituted snapshot_evidence_byte_digest_mismatch
  validator_available_divergent_rejected validator_available_ancestral_passed
  validator_unavailable_protected_fallback_passed
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
  "snapshot_valid_multi_prepared_finalized_restart" => "passed",
  "validator_available_ancestral_passed" => "passed",
  "validator_unavailable_protected_fallback_passed" => "passed",
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
  stdout = stdout.rstrip + (stdout.empty? ? "" : "\n")
  stderr = stderr.rstrip + (stderr.empty? ? "" : "\n")
  stdout_path = OUTPUT.join("#{name}.stdout.log")
  stderr_path = OUTPUT.join("#{name}.stderr.log")
  File.binwrite(stdout_path, stdout)
  File.binwrite(stderr_path, stderr)
  {
    "argv" => argv, "exit_code" => status.exitstatus, "started_at" => started,
    "finished_at" => finished,
    "stream_normalization" => "trailing_blank_lines_removed",
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
commands["nextest"] = run_command("nextest", ["cargo", "nextest", "run", "--locked", "--manifest-path", "adl-runtime/Cargo.toml", "--lib", "-E", "test(/^distributed::authority_protocol::contract_tests::/) or test(/^distributed::polis_runtime::authority_consensus_tests::snapshot_/)", "--no-tests=fail"])
fail_proof("focused nextest failed") unless commands["nextest"]["exit_code"] == 0
nextest_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["nextest"]["#{stream}_path"])) }.join
fail_proof("nextest denominator mismatch") unless nextest_text.match?(/83 tests run: 83 passed, \d+ skipped/)

commands["full_runtime"] = run_command("full-runtime", %w[cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --lib --no-tests=fail])
fail_proof("full runtime lane failed") unless commands["full_runtime"]["exit_code"] == 0
full_runtime_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["full_runtime"]["#{stream}_path"])) }.join
fail_proof("full runtime denominator mismatch") unless full_runtime_text.match?(/230 tests run: 230 passed, 0 skipped/)

commands["clippy"] = run_command("clippy", %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings])
fail_proof("strict Clippy failed") unless commands["clippy"]["exit_code"] == 0
commands["machine_cases"] = run_command("machine-cases", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::authority_protocol::contract_tests:: -- --nocapture --test-threads=1])
fail_proof("machine cases failed") unless commands["machine_cases"]["exit_code"] == 0
commands["snapshot_cases"] = run_command("snapshot-cases", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::polis_runtime::authority_consensus_tests::snapshot_ -- --nocapture --test-threads=1])
fail_proof("snapshot cases failed") unless commands["snapshot_cases"]["exit_code"] == 0
machine_text = [commands["machine_cases"], commands["snapshot_cases"]].flat_map { |command| %w[stdout stderr].map { |stream| File.binread(ROOT.join(command["#{stream}_path"])) } }.join
observed = machine_text.lines.each_with_object([]) do |line, rows|
  next unless line.include?(MARKER)
  name, result = line.split(MARKER, 2).fetch(1).strip.split(" ", 2)
  rows << [name, result, Digest::SHA256.hexdigest("#{MARKER}#{name} #{result}")]
end
commands["validator_modes"] = run_command("validator-modes", ["ruby", ".csdlc/prepared/issues/201/validate-proof-receipt.rb", "--self-test"])
fail_proof("validator mode self-test failed") unless commands["validator_modes"]["exit_code"] == 0
validator_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["validator_modes"]["#{stream}_path"])) }.join
validator_text.lines.each do |line|
  next unless line.include?(MARKER)
  name, result = line.split(MARKER, 2).fetch(1).strip.split(" ", 2)
  observed << [name, result, Digest::SHA256.hexdigest("#{MARKER}#{name} #{result}")]
end
fail_proof("case denominator mismatch") unless observed.length == 86 && observed.map(&:first).sort == EXPECTED_CASES.sort
observed_by_name = observed.to_h { |name, result, digest| [name, [result, digest]] }
EXPECTED_CASES.each { |name| fail_proof("wrong result for #{name}") unless observed_by_name.fetch(name).first == EXPECTED_RESULTS.fetch(name, "rejected") }
commands["openraft"] = run_command("openraft", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::polis_runtime::authority_consensus_tests::real_three_voter_authority_prepare_finalize_uses_applied_log_ids -- --exact --nocapture])
fail_proof("real three-voter OpenRaft proof failed") unless commands["openraft"]["exit_code"] == 0
openraft_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["openraft"]["#{stream}_path"])) }.join
fail_proof("real three-voter OpenRaft denominator mismatch") unless openraft_text.match?(/1 passed; 0 failed/) && openraft_text.include?("real_three_voter_authority_prepare_finalize_uses_applied_log_ids")
tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source tree unavailable") unless status.success?
proof = {
  "schema" => "adl.issue201.committed_authority_proof.v2", "issue" => 201,
  "source_revision" => source, "source_tree" => tree.strip,
  "protected_files" => PROTECTED.map { |path| { "path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest } },
  "commands" => commands,
  "test_summary" => { "selected" => 86, "passed" => 86, "skipped" => 0 },
  "runtime_summary" => { "selected" => 230, "passed" => 230, "skipped" => 0 },
  "result_summary" => { "passed" => 11, "reconciled" => 6, "rejected" => 69 },
  "cases" => EXPECTED_CASES.map { |name| result, digest = observed_by_name.fetch(name); { "case" => name, "result" => result, "observed_line_sha256" => digest } }
}
File.binwrite(OUTPUT.join("execution-proof.json"), JSON.generate(proof) + "\n")
puts "PASS: produced exact issue #201 86-case proof plus full runtime 230/230 at source #{source}"
