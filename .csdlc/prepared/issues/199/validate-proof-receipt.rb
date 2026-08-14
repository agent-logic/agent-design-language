#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/199/v19/"
PROOF_RELATIVE = "#{PREFIX}execution-proof.json"
EXPECTED_PROTECTED = %w[
  adl-runtime/src/distributed/mod.rs adl-runtime/src/distributed/authority_protocol.rs
  adl-runtime/src/distributed/membership.rs adl-runtime/src/distributed/lease.rs
  adl-runtime/src/distributed/membership_coordinator.rs adl-runtime/src/distributed/membership_coordinator/tests.rs
  adl-runtime/src/distributed/transport/governed/learner_transport.rs
  adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
  adl-runtime/src/distributed/transport/governed/polis_runtime.rs
  adl-runtime/tests/distributed_membership_transition.rs
  .csdlc/prepared/issues/199/design.md .csdlc/prepared/issues/199/diagram.mmd
  .csdlc/prepared/issues/199/produce-proof-receipt.rb .csdlc/prepared/issues/199/validate-proof-receipt.rb
].freeze
EXPECTED_CASES = %w[
  join_promote_remove_order epoch_gap_denied_without_partial_change
  exact_retry_and_conflicting_reuse snapshot_restore_and_corruption_denial
  stable_map_digest_and_collision_denial authority_membership_preserves_stable_ids
  promote_artifact_binds_distinct_maps duplicate_control_key_denied
  wrong_domain_denied_without_epoch_advance governed_rejoin_from_stale_state
  wrong_coarse_operation_kind capacity_n_plus_one_no_partial
].freeze
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
EXPECTED_COMMANDS = {
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

def fail_receipt(message)
  abort("issue 199 receipt: #{message}")
end

def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  fail_receipt("git failed: #{err.strip}") unless status.success?
  out
end

def ordinary(relative)
  fail_receipt("unsafe path #{relative}") if Pathname.new(relative).absolute? || Pathname.new(relative).cleanpath.to_s != relative
  current = ROOT
  parts = relative.split("/")
  parts.each_with_index do |part, index|
    current = current.join(part)
    metadata = File.lstat(current)
    fail_receipt("symlink path #{relative}") if metadata.symlink?
    fail_receipt("non-directory ancestor #{relative}") if index < parts.length - 1 && !metadata.directory?
  end
  fail_receipt("not ordinary file #{relative}") unless current.file? && !current.symlink?
  current
rescue Errno::ENOENT
  fail_receipt("missing file #{relative}")
end

proof = JSON.parse(File.binread(ordinary(PROOF_RELATIVE)))
fail_receipt("top-level key mismatch") unless proof.keys.sort == %w[assertions cases commands issue protected_files required_main_ancestor schema source_revision source_tree subassertions test_summary]
fail_receipt("schema/issue mismatch") unless proof["schema"] == "adl.issue199.governed_membership_transition_proof.v19" && proof["issue"] == 199
source = proof.fetch("source_revision")
source_tree = proof.fetch("source_tree")
main = proof.fetch("required_main_ancestor")
fail_receipt("revision malformed") unless [source, source_tree, main].all? { |value| value.match?(/\A[0-9a-f]{40}\z/) }
fail_receipt("proof is not bound to exact current origin/main") unless git("rev-parse", "refs/remotes/origin/main").strip == main
fail_receipt("current origin/main ancestry missing") unless system("git", "merge-base", "--is-ancestor", main, source, chdir: ROOT.to_s)
protected = proof.fetch("protected_files")
fail_receipt("protected denominator mismatch") unless protected.map { |entry| entry["path"] } == EXPECTED_PROTECTED
protected.each do |entry|
  fail_receipt("protected entry key mismatch") unless entry.keys.sort == %w[path sha256]
  fail_receipt("protected digest malformed") unless entry.fetch("sha256").match?(/\A[0-9a-f]{64}\z/)
  fail_receipt("protected digest drift #{entry['path']}") unless Digest::SHA256.file(ordinary(entry.fetch("path"))).hexdigest == entry.fetch("sha256")
end
fail_receipt("test summary mismatch") unless proof.fetch("test_summary") == { "integration_cases" => 12, "integration_passed" => 12, "internal_test_counts" => EXPECTED_TEST_COUNTS, "discriminator_subassertions" => 1, "source_assertions" => 9 }
cases = proof.fetch("cases")
fail_receipt("case denominator/order mismatch") unless cases.length == 12 && cases.map { |entry| entry["case"] } == EXPECTED_CASES && cases.map { |entry| entry["case"] }.uniq.length == 12
cases.each do |entry|
  fail_receipt("case key/result mismatch") unless entry.keys.sort == %w[case marker_sha256 result] && entry["result"] == "pass"
  fail_receipt("case marker digest mismatch") unless entry["marker_sha256"] == Digest::SHA256.hexdigest("ADL_ISSUE_199_CASE_V1 case=#{entry['case']} result=pass")
end
subassertions = proof.fetch("subassertions")
fail_receipt("discriminator mismatch") unless subassertions == [{ "name" => "wrong_artifact_discriminator", "boundary" => "sealed_publication_consumer" }]
assertions = proof.fetch("assertions")
fail_receipt("assertion denominator/order mismatch") unless assertions.map { |entry| [entry["case"], entry["assertion"]] } == EXPECTED_ASSERTIONS
assertions.each do |entry|
  fail_receipt("assertion key mismatch") unless entry.keys.sort == %w[assertion case marker_sha256]
  expected = Digest::SHA256.hexdigest("ADL_ISSUE_199_ASSERTION_V1 case=#{entry['case']} assertion=#{entry['assertion']}")
  fail_receipt("assertion digest mismatch") unless entry["marker_sha256"] == expected
end
commands = proof.fetch("commands")
fail_receipt("command denominator mismatch") unless commands.keys.sort == EXPECTED_COMMANDS.keys.sort
commands.each do |name, command|
  fail_receipt("command key mismatch #{name}") unless command.keys.sort == %w[argv exit_code finished_at started_at stderr_path stderr_sha256 stdout_path stdout_sha256]
  fail_receipt("command argv mismatch #{name}") unless command.fetch("argv") == EXPECTED_COMMANDS.fetch(name)
  fail_receipt("command failed #{name}") unless command.fetch("exit_code") == 0
  fail_receipt("command time inverted #{name}") if Time.iso8601(command.fetch("finished_at")) < Time.iso8601(command.fetch("started_at"))
  %w[stdout stderr].each do |stream|
    relative = command.fetch("#{stream}_path")
    fail_receipt("stream escapes evidence") unless relative.start_with?(PREFIX)
    fail_receipt("stream digest mismatch") unless Digest::SHA256.file(ordinary(relative)).hexdigest == command.fetch("#{stream}_sha256")
  end
end
EXPECTED_TEST_COUNTS.each do |name, expected|
  output = %w[stdout stderr].map { |stream| File.binread(ordinary(commands.fetch(name).fetch("#{stream}_path"))) }.join
  running = output.scan(/^running (\d+) tests?$/).flatten.map(&:to_i)
  summaries = output.scan(/^test result: ok\. (\d+) passed; (\d+) failed;/).map { |passed, failed| [passed.to_i, failed.to_i] }
  fail_receipt("test denominator mismatch #{name}") unless running == [expected] && summaries == [[expected, 0]]
end
integration = %w[stdout stderr].map { |stream| File.binread(ordinary(commands.fetch("integration_cases").fetch("#{stream}_path"))) }.join
observed_cases = integration.lines.map do |line|
  next unless line.include?("ADL_ISSUE_199_CASE_V1 ")
  match = line.split("ADL_ISSUE_199_CASE_V1 ", 2).fetch(1).strip.match(/\Acase=([^ ]+) result=pass detail=([^ ]+)\z/)
  fail_receipt("malformed observed case marker") unless match
  match[1]
end.compact
fail_receipt("observed case denominator/substitution mismatch") unless observed_cases.length == 12 && observed_cases.uniq.length == 12 && observed_cases.sort == EXPECTED_CASES.sort
observed_subassertions = integration.lines.map do |line|
  next unless line.include?("ADL_ISSUE_199_SUBASSERTION_V1 ")
  match = line.split("ADL_ISSUE_199_SUBASSERTION_V1 ", 2).fetch(1).strip.match(/\Aname=([^ ]+) result=pass boundary=([^ ]+)\z/)
  fail_receipt("malformed observed discriminator marker") unless match
  [match[1], match[2]]
end.compact
fail_receipt("observed discriminator substitution") unless observed_subassertions == [["wrong_artifact_discriminator", "sealed_publication_consumer"]]
all_text = commands.values.flat_map { |command| %w[stdout stderr].map { |stream| File.binread(ordinary(command.fetch("#{stream}_path"))) } }.join
observed_assertions = all_text.lines.map do |line|
  next unless line.include?("ADL_ISSUE_199_ASSERTION_V1 ")
  match = line.split("ADL_ISSUE_199_ASSERTION_V1 ", 2).fetch(1).strip.match(/\Acase=([^ ]+) assertion=([^ ]+)\z/)
  fail_receipt("malformed observed assertion marker") unless match
  [match[1], match[2]]
end.compact
fail_receipt("observed assertion denominator/substitution mismatch") unless observed_assertions.length == 9 && observed_assertions.uniq.length == 9 && observed_assertions.sort == EXPECTED_ASSERTIONS.sort
introductions = git("log", "--format=%H", "--diff-filter=A", "--", PROOF_RELATIVE).lines.map(&:strip).reject(&:empty?)
fail_receipt("proof requires immutable introduction") if introductions.empty?
introduction = introductions.first
fail_receipt("proof not introduced from absence") if system("git", "cat-file", "-e", "#{introduction}^:#{PROOF_RELATIVE}", chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
fail_receipt("source not ancestral") unless system("git", "merge-base", "--is-ancestor", source, introduction, chdir: ROOT.to_s)
fail_receipt("source tree mismatch") unless git("rev-parse", "#{source}^{tree}").strip == source_tree
protected.each do |entry|
  fail_receipt("source object mismatch #{entry['path']}") unless Digest::SHA256.hexdigest(git("show", "#{source}:#{entry.fetch('path')}")) == entry.fetch("sha256")
end
fail_receipt("protected source changed after proof") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", *EXPECTED_PROTECTED).empty?
fail_receipt("immutable proof changed after introduction") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", PREFIX).empty?
fail_receipt("worktree must be exactly clean") unless git("status", "--porcelain=v1", "--untracked-files=all").empty?
puts "PASS: issue #199 proof binds exact argv, 12 behavior-specific cases, discriminator denial, nine production assertions, protected implementation/design/proof source, immutable evidence, and exact current origin/main ancestry"
