#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/202/v1/"
OUTPUT = ROOT.join(PREFIX)
PROOF = OUTPUT.join("execution-proof.json")
MARKER = "ADL_ISSUE_202_CASE_V1 "
ASSERTION_MARKER = "ADL_ISSUE_202_ASSERTION_V1 "
MAIN_ANCESTOR = "507d9a1e3a74c2c9c6cce14259b095139aa3bdfa"
PROTECTED = %w[
  adl-runtime/src/distributed/mod.rs
  adl-runtime/src/distributed/authority_protocol.rs
  adl-runtime/src/distributed/learner_transport.rs
  adl-runtime/src/distributed/learner_transport/tests.rs
  adl-runtime/src/distributed/polis_runtime.rs
  adl-runtime/src/distributed/transport.rs
  adl-runtime/tests/distributed_authorized_learner_transport.rs
  .csdlc/prepared/issues/202/produce-proof-receipt.rb
  .csdlc/prepared/issues/202/validate-proof-receipt.rb
].freeze
EXPECTED_CASES = %w[
  real_four_node_learner_replication current_voter_cut_unchanged excluded_node_recovery_learner
  learner_promotion_route_handoff exact_retry_session reconnect_boot_rotation
  certificate_overlap_authorized missing_201_token public_caller_denied wrong_operation_kind
  wrong_domain wrong_polis wrong_learner wrong_guardian wrong_certificate_generation
  expired_certificate revoked_certificate wrong_boot_generation wrong_address learner_vote_rpc_denied
  learner_endorsement_denied learner_finalize_denied learner_mutation_denied learner_renewal_denied
  learner_shepherd_denied learner_observatory_denied exclusion_ordinary_session_denied
  exclusion_wrong_recovery_token stale_admission replay_conflict oversized_frame truncated_frame
  capacity_n_plus_one_no_partial crash_before_exclusion_checkpoint crash_after_exclusion_checkpoint
  state_or_lock_symlink_rejected
].freeze
EXPECTED_ASSERTIONS = [
  %w[real_four_node_learner_replication raft_add_learner_replicated],
  %w[real_four_node_learner_replication voter_quorum_unchanged],
  %w[real_four_node_learner_replication quinn_append_snapshot_only],
  %w[exact_retry_session exclusion_exact_retry_cached],
  %w[exact_retry_session admission_exact_retry_cached],
  %w[certificate_overlap_authorized successor_private_before_flip],
  %w[certificate_overlap_authorized successor_restart_atomic_flip],
  %w[exclusion_ordinary_session_denied published_exclusion_denies_retained_identity],
  %w[crash_before_exclusion_checkpoint failed_cas_recovers_old_view],
  %w[crash_after_exclusion_checkpoint committed_view_survives_restart]
].freeze

def fail_proof(message) = abort("issue 202 producer: #{message}")

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

source, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail_proof("cannot resolve source") unless status.success? && source.strip.match?(/\A[0-9a-f]{40}\z/)
source = source.strip
fail_proof("required merged #200 ancestry absent") unless system("git", "merge-base", "--is-ancestor", MAIN_ANCESTOR, source, chdir: ROOT.to_s)
dirty, status = Open3.capture2("git", "status", "--porcelain=v1", "--untracked-files=all", chdir: ROOT.to_s)
dirty = dirty.lines.reject { |line| line[3..]&.start_with?(PREFIX) }
fail_proof("source worktree must be clean") unless status.success? && dirty.empty?
PROTECTED.each do |relative|
  path = ROOT.join(relative)
  fail_proof("missing protected path #{relative}") unless path.file? && !path.symlink?
  committed, committed_status = Open3.capture2("git", "show", "#{source}:#{relative}", chdir: ROOT.to_s)
  fail_proof("protected path dirty #{relative}") unless committed_status.success? && Digest::SHA256.hexdigest(committed) == Digest::SHA256.file(path).hexdigest
end
%w[adl-runtime/src/distributed/learner_transport/tests.rs adl-runtime/tests/distributed_authorized_learner_transport.rs].each do |relative|
  fail_proof("machine-local temp root") if File.binread(ROOT.join(relative)).include?("/private/tmp")
end
FileUtils.mkdir_p(OUTPUT, mode: 0o700)

commands = {}
commands["private_cases"] = run_command("private-cases", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib learner_transport::tests -- --nocapture --test-threads=1])
commands["public_cases"] = run_command("public-cases", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- --test-threads=1])
commands["clippy"] = run_command("clippy", %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- -D warnings])
fail_proof("command failed") unless commands.values.all? { |command| command["exit_code"] == 0 }
private_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["private_cases"]["#{stream}_path"])) }.join
public_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["public_cases"]["#{stream}_path"])) }.join
fail_proof("private test count mismatch") unless private_text.include?("test result: ok. 36 passed; 0 failed")
fail_proof("public test count mismatch") unless public_text.include?("test result: ok. 13 passed; 0 failed")
observed = private_text.lines.filter_map do |line|
  next unless line.include?(MARKER)
  name, result = line.split(MARKER, 2).fetch(1).strip.split("=", 2)
  [name, result]
end
fail_proof("case denominator mismatch") unless observed.length == 36 && observed.map(&:first).sort == EXPECTED_CASES.sort && observed.all? { |_, result| result == "passed" }
assertions = private_text.lines.filter_map do |line|
  next unless line.include?(ASSERTION_MARKER)
  line.split(ASSERTION_MARKER, 2).fetch(1).strip.split(" ", 2)
end
fail_proof("subassertion mismatch") unless assertions.sort == EXPECTED_ASSERTIONS.sort
tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source tree unavailable") unless status.success?
proof = {
  "schema" => "adl.issue202.authorized_learner_transport_proof.v1", "issue" => 202,
  "source_revision" => source, "source_tree" => tree.strip, "required_main_ancestor" => MAIN_ANCESTOR,
  "protected_files" => PROTECTED.map { |path| { "path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest } },
  "commands" => commands, "test_summary" => { "private_selected" => 36, "private_passed" => 36, "public_selected" => 13, "public_passed" => 13 },
  "cases" => EXPECTED_CASES.map { |name| { "case" => name, "result" => "passed", "marker_sha256" => Digest::SHA256.hexdigest("#{MARKER}#{name}=passed") } },
  "subassertions" => EXPECTED_ASSERTIONS.map { |case_name, name| { "case" => case_name, "assertion" => name, "marker_sha256" => Digest::SHA256.hexdigest("#{ASSERTION_MARKER}#{case_name} #{name}") } }
}
File.binwrite(PROOF, JSON.generate(proof) + "\n")
puts "PASS: produced issue #202 exact 36+13 proof at #{source}"
