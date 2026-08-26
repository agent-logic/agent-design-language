#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/199/v19/"
OUTPUT = ROOT.join(PREFIX)
PROOF = OUTPUT.join("execution-proof.json")
MARKER = "ADL_ISSUE_199_CASE_V1 "
SUBASSERTION_MARKER = "ADL_ISSUE_199_SUBASSERTION_V1 "
ASSERTION_MARKER = "ADL_ISSUE_199_ASSERTION_V1 "
PROTECTED = %w[
  adl-runtime/src/distributed/mod.rs
  adl-runtime/src/distributed/authority_protocol.rs
  adl-runtime/src/distributed/membership.rs
  adl-runtime/src/distributed/lease.rs
  adl-runtime/src/distributed/membership_coordinator.rs
  adl-runtime/src/distributed/membership_coordinator/tests.rs
  adl-runtime/src/distributed/transport/governed/learner_transport.rs
  adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
  adl-runtime/src/distributed/transport/governed/polis_runtime.rs
  adl-runtime/tests/distributed_membership_transition.rs
  .csdlc/prepared/issues/199/design.md
  .csdlc/prepared/issues/199/diagram.mmd
  .csdlc/prepared/issues/199/produce-proof-receipt.rb
  .csdlc/prepared/issues/199/validate-proof-receipt.rb
].freeze
EXPECTED_CASES = %w[
  join_promote_remove_order epoch_gap_denied_without_partial_change
  exact_retry_and_conflicting_reuse snapshot_restore_and_corruption_denial
  stable_map_digest_and_collision_denial authority_membership_preserves_stable_ids
  promote_artifact_binds_distinct_maps duplicate_control_key_denied
  wrong_domain_denied_without_epoch_advance governed_rejoin_from_stale_state
  wrong_coarse_operation_kind capacity_n_plus_one_no_partial
].freeze
EXPECTED_SUBASSERTIONS = ["wrong_artifact_discriminator"].freeze
EXPECTED_ASSERTIONS = [
  %w[add_learner_joint_final_publish same_batch_joint_and_uniform_history_survives_restart],
  %w[add_learner_joint_final_publish factory_admission_receipt_exact_current_and_mismatch_denied],
  %w[remove_voter_pending_exclusion factory_exclusion_receipt_exact_current_and_mismatch_denied],
  %w[crash_every_phase durable_saga_restart_exact_retry_no_duplicate_publication],
  %w[conflicting_retry conflicting_operation_and_receipt_denied_before_effect],
  %w[old_cut_mismatch authorized_stable_maps_and_target_membership_bound_before_raft_effect],
  %w[leader_change_resume membership_history_entries_newer_than_authority_log_index_required],
  %w[remove_rejoin_real_nodes exclusion_retain_false_separate_enrollment_promotion_catchup_and_parity_publication],
  %w[crash_phase_matrix enrollment_removal_promotion_boundaries_retry_without_duplicate_visibility]
].freeze
COMMANDS = {
  "integration_cases" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_membership_transition -- --nocapture --test-threads=1],
  "coordinator_lib" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::membership_coordinator::tests -- --nocapture --test-threads=1],
  "admission_receipt" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::learner_transport::tests::real_four_node_learner_replication -- --exact --nocapture --test-threads=1],
  "exclusion_receipt" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::learner_transport::tests::excluded_node_recovery_learner -- --exact --nocapture --test-threads=1],
  "membership_history_restart" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::polis_runtime::authority_consensus_tests::membership_history_retains_joint_and_uniform_entries_from_one_apply_batch -- --exact --nocapture --test-threads=1],
  "clippy_lib" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings],
  "clippy_integration" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_membership_transition -- -D warnings]
}.freeze
EXPECTED_TEST_COUNTS = {
  "integration_cases" => 12,
  "coordinator_lib" => 7,
  "admission_receipt" => 1,
  "exclusion_receipt" => 1,
  "membership_history_restart" => 1
}.freeze

def fail_proof(message)
  abort("issue 199 producer: #{message}")
end

def run_command(name, argv)
  started = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3({ "NEXTEST_TEST_THREADS" => "1" }, *argv, chdir: ROOT.to_s)
  finished = Time.now.utc.iso8601(6)
  stdout = stdout.rstrip + (stdout.empty? ? "" : "\n")
  stderr = stderr.rstrip + (stderr.empty? ? "" : "\n")
  File.binwrite(OUTPUT.join("#{name}.stdout.log"), stdout)
  File.binwrite(OUTPUT.join("#{name}.stderr.log"), stderr)
  {
    "argv" => argv, "exit_code" => status.exitstatus, "started_at" => started, "finished_at" => finished,
    "stdout_path" => "#{PREFIX}#{name}.stdout.log", "stdout_sha256" => Digest::SHA256.hexdigest(stdout),
    "stderr_path" => "#{PREFIX}#{name}.stderr.log", "stderr_sha256" => Digest::SHA256.hexdigest(stderr)
  }
end

if PROOF.file?
  _out, status = Open3.capture2("ruby", ".csdlc/prepared/issues/199/validate-proof-receipt.rb", chdir: ROOT.to_s)
  fail_proof("retained immutable proof is invalid") unless status.success?
  puts "PASS: retained immutable issue #199 proof is current"
  exit 0
end

source, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail_proof("cannot resolve source") unless status.success? && source.strip.match?(/\A[0-9a-f]{40}\z/)
source = source.strip
origin_main, status = Open3.capture2("git", "rev-parse", "refs/remotes/origin/main", chdir: ROOT.to_s)
fail_proof("cannot resolve current origin/main") unless status.success? && origin_main.strip.match?(/\A[0-9a-f]{40}\z/)
origin_main = origin_main.strip
fail_proof("current origin/main is not ancestral to source") unless system("git", "merge-base", "--is-ancestor", origin_main, source, chdir: ROOT.to_s)
dirty, status = Open3.capture2("git", "status", "--porcelain=v1", "--untracked-files=all", chdir: ROOT.to_s)
fail_proof("source worktree must be exactly clean") unless status.success? && dirty.empty?
PROTECTED.each do |relative|
  path = ROOT.join(relative)
  fail_proof("missing or unsafe protected path #{relative}") unless path.file? && !path.symlink?
  committed, committed_status = Open3.capture2("git", "show", "#{source}:#{relative}", chdir: ROOT.to_s)
  fail_proof("protected path absent or dirty #{relative}") unless committed_status.success? && Digest::SHA256.hexdigest(committed) == Digest::SHA256.file(path).hexdigest
end
FileUtils.mkdir_p(OUTPUT, mode: 0o700)

commands = COMMANDS.to_h { |name, argv| [name, run_command(name.tr("_", "-"), argv)] }
fail_proof("command failed") unless commands.values.all? { |command| command["exit_code"] == 0 }
observed_test_counts = EXPECTED_TEST_COUNTS.to_h do |name, expected|
  output = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands.fetch(name).fetch("#{stream}_path"))) }.join
  running = output.scan(/^running (\d+) tests?$/).flatten.map(&:to_i)
  summaries = output.scan(/^test result: ok\. (\d+) passed; (\d+) failed;/).map { |passed, failed| [passed.to_i, failed.to_i] }
  fail_proof("test denominator mismatch #{name}") unless running == [expected] && summaries == [[expected, 0]]
  [name, expected]
end
integration = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands.fetch("integration_cases").fetch("#{stream}_path"))) }.join
observed_cases = integration.lines.map do |line|
  next unless line.include?(MARKER)
  match = line.split(MARKER, 2).fetch(1).strip.match(/\Acase=([^ ]+) result=pass detail=([^ ]+)\z/)
  fail_proof("malformed case marker") unless match
  [match[1], match[2]]
end.compact
fail_proof("case denominator or substitution mismatch") unless observed_cases.length == 12 && observed_cases.map(&:first).uniq.length == 12 && observed_cases.map(&:first).sort == EXPECTED_CASES.sort
subassertions = integration.lines.map do |line|
  next unless line.include?(SUBASSERTION_MARKER)
  match = line.split(SUBASSERTION_MARKER, 2).fetch(1).strip.match(/\Aname=([^ ]+) result=pass boundary=([^ ]+)\z/)
  fail_proof("malformed discriminator marker") unless match
  [match[1], match[2]]
end.compact
fail_proof("discriminator subassertion mismatch") unless subassertions.length == 1 && subassertions.map(&:first) == EXPECTED_SUBASSERTIONS
all_text = commands.values.flat_map { |command| %w[stdout stderr].map { |stream| File.binread(ROOT.join(command.fetch("#{stream}_path"))) } }.join
assertions = all_text.lines.map do |line|
  next unless line.include?(ASSERTION_MARKER)
  match = line.split(ASSERTION_MARKER, 2).fetch(1).strip.match(/\Acase=([^ ]+) assertion=([^ ]+)\z/)
  fail_proof("malformed assertion marker") unless match
  [match[1], match[2]]
end.compact
fail_proof("assertion denominator or substitution mismatch") unless assertions.length == 9 && assertions.uniq.length == 9 && assertions.sort == EXPECTED_ASSERTIONS.sort
tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source tree unavailable") unless status.success?
proof = {
  "schema" => "adl.issue199.governed_membership_transition_proof.v19", "issue" => 199,
  "source_revision" => source, "source_tree" => tree.strip, "required_main_ancestor" => origin_main,
  "protected_files" => PROTECTED.map { |path| { "path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest } },
  "commands" => commands,
  "test_summary" => { "integration_cases" => 12, "integration_passed" => 12, "internal_test_counts" => observed_test_counts, "discriminator_subassertions" => 1, "source_assertions" => 9 },
  "cases" => EXPECTED_CASES.map { |name| { "case" => name, "result" => "pass", "marker_sha256" => Digest::SHA256.hexdigest("#{MARKER}case=#{name} result=pass") } },
  "subassertions" => subassertions.map { |name, boundary| { "name" => name, "boundary" => boundary } },
  "assertions" => EXPECTED_ASSERTIONS.map { |case_name, assertion| { "case" => case_name, "assertion" => assertion, "marker_sha256" => Digest::SHA256.hexdigest("#{ASSERTION_MARKER}case=#{case_name} assertion=#{assertion}") } }
}
File.binwrite(PROOF, JSON.generate(proof) + "\n")
puts "PASS: produced issue #199 exact 12-case, discriminator, nine-assertion, seven-command proof at #{source}"
