#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/202/v9/"
OUTPUT = ROOT.join(PREFIX)
PROOF = OUTPUT.join("execution-proof.json")
MARKER = "ADL_ISSUE_202_CASE_V1 "
ASSERTION_MARKER = "ADL_ISSUE_202_ASSERTION_V1 "
MAIN_ANCESTOR = "1567469e395f9a6ea6c2e736366a8008f5ee1e06"
PROTECTED = %w[
  adl-runtime/src/distributed/mod.rs
  adl-runtime/src/distributed/authority_protocol.rs
  adl-runtime/src/distributed/transport.rs
  adl-runtime/src/distributed/transport/core.rs
  adl-runtime/src/distributed/transport/root.rs
  adl-runtime/src/distributed/transport/governed/learner_transport.rs
  adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
  adl-runtime/src/distributed/transport/governed/polis_runtime.rs
  adl-runtime/tests/distributed_authorized_learner_transport.rs
  adl-runtime/tests/distributed_transport.rs
  adl-runtime/tests/distributed_discovery.rs
  adl-runtime/tests/distributed_runtime_transport.rs
  adl/tools/check_coverage_impact.sh
  adl/tools/test_check_coverage_impact.sh
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
  stale_live_learner_boot_handshake_denied exclusion_exact_publisher_and_target_required
  exclusion_waits_for_inflight_dispatch_fence
  production_factory_boot_custody_current_then_stale_denied
  transport_instance_and_peer_pin_are_durable_and_unique
  fresh_connection_requires_durable_peer_instance_pin
].freeze
EXPECTED_ASSERTIONS = [
  %w[real_four_node_learner_replication raft_add_learner_replicated],
  %w[real_four_node_learner_replication voter_quorum_unchanged],
  %w[real_four_node_learner_replication quinn_append_snapshot_only],
  %w[real_four_node_learner_replication expiry_writer_waits_through_real_raft_effect_and_response],
  %w[exact_retry_session exclusion_exact_retry_cached],
  %w[exact_retry_session removal_deadline_and_target_membership_bound_cache_first],
  %w[exact_retry_session admission_exact_retry_cached],
  %w[excluded_node_recovery_learner production_factory_enforces_recovery_identity_index_and_membership],
  %w[certificate_overlap_authorized successor_private_before_flip],
  %w[certificate_overlap_authorized successor_atomic_flip],
  %w[certificate_overlap_authorized retained_old_clones_atomically_revoked],
  %w[exclusion_ordinary_session_denied published_exclusion_denies_retained_identity],
  %w[exclusion_ordinary_session_denied production_endorsement_uses_durable_exclusion],
  %w[exclusion_ordinary_session_denied retained_excluded_session_zero_bytes_all_public_dispatch],
  %w[exclusion_ordinary_session_denied actual_request_stream_fenced_against_exclusion_race],
  %w[exclusion_exact_publisher_and_target_required wrong_publisher_node_denied],
  %w[exclusion_exact_publisher_and_target_required wrong_target_certificate_and_boot_denied],
  %w[exclusion_waits_for_inflight_dispatch_fence exclusive_exclusion_waits_for_shared_dispatch],
  %w[stale_live_learner_boot_handshake_denied live_boot_generation_must_match_signed_admission_binding],
  %w[production_factory_boot_custody_current_then_stale_denied factory_attestation_rechecks_generation_under_signing_guard],
  %w[transport_instance_and_peer_pin_are_durable_and_unique restart_preserves_instance_and_exact_peer_pin],
  %w[transport_instance_and_peer_pin_are_durable_and_unique alternate_root_and_identity_alias_are_denied],
  %w[fresh_connection_requires_durable_peer_instance_pin fresh_connection_accepts_restarted_peer_with_persisted_instance],
  %w[fresh_connection_requires_durable_peer_instance_pin alternate_factory_denied_before_session_or_post_denial_stream],
  %w[crash_before_exclusion_checkpoint failed_admission_and_exclusion_cas_recover_old_view],
  %w[crash_after_exclusion_checkpoint committed_admission_and_exclusion_survive_restart],
  %w[wrong_boot_generation live_stale_voter_boot_rejected],
  %w[wrong_polis cross_polis_published_result_denied],
  %w[wrong_polis cross_polis_live_install_denied],
  %w[wrong_address live_authorized_address_rejected],
  %w[wrong_address live_wrong_direction_rejected]
].freeze

def fail_proof(message)
  abort("issue 202 producer: #{message}")
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

source, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail_proof("cannot resolve source") unless status.success? && source.strip.match?(/\A[0-9a-f]{40}\z/)
source = source.strip
origin_main, origin_status = Open3.capture2("git", "rev-parse", "refs/remotes/origin/main", chdir: ROOT.to_s)
fail_proof("current origin/main unavailable") unless origin_status.success? && origin_main.strip == MAIN_ANCESTOR
fail_proof("exact current origin/main ancestry absent") unless system("git", "merge-base", "--is-ancestor", MAIN_ANCESTOR, source, chdir: ROOT.to_s)
dirty, status = Open3.capture2("git", "status", "--porcelain=v1", "--untracked-files=all", chdir: ROOT.to_s)
dirty = dirty.lines.reject do |line|
  relative = line[3..]
  relative&.start_with?(PREFIX)
end
fail_proof("source worktree must be clean") unless status.success? && dirty.empty?
PROTECTED.each do |relative|
  path = ROOT.join(relative)
  fail_proof("missing protected path #{relative}") unless path.file? && !path.symlink?
  committed, committed_status = Open3.capture2("git", "show", "#{source}:#{relative}", chdir: ROOT.to_s)
  fail_proof("protected path dirty #{relative}") unless committed_status.success? && Digest::SHA256.hexdigest(committed) == Digest::SHA256.file(path).hexdigest
end
%w[adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs adl-runtime/tests/distributed_authorized_learner_transport.rs].each do |relative|
  fail_proof("machine-local temp root") if File.binread(ROOT.join(relative)).include?("/private/tmp")
end
runtime_source = File.binread(ROOT.join("adl-runtime/src/distributed/transport/governed/polis_runtime.rs"))
authority_source = File.binread(ROOT.join("adl-runtime/src/distributed/authority_protocol.rs"))
transport_source = File.binread(ROOT.join("adl-runtime/src/distributed/transport/core.rs"))
transport_root = File.binread(ROOT.join("adl-runtime/src/distributed/transport/root.rs"))
learner_source = File.binread(ROOT.join("adl-runtime/src/distributed/transport/governed/learner_transport.rs"))
private_tests = File.binread(ROOT.join("adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs"))
fail_proof("production allow-all authority bypass remains") if [runtime_source, authority_source, transport_source].any? { |source_text| source_text.include?("AllowAll") }
fail_proof("governed private module root missing") unless transport_root.include?("mod governed") && transport_root.include?("pub use governed::{learner_transport, polis_runtime}")
fail_proof("factory sole mutation owner missing") unless runtime_source.include?("transport_owner: Arc<tokio::sync::Mutex<TransportAuthorityOwner>>")
fail_proof("transport owner is crate-visible") if transport_source.match?(/pub(?:\(crate\))?\s+struct TransportAuthorityOwner/)
fail_proof("ordinary session exclusion policy remains public") if transport_source.include?("pub trait OrdinarySessionExclusion")
fail_proof("authority eligibility exclusion policy remains public") if authority_source.include?("pub trait AuthorityEligibilityExclusion")
fail_proof("published admission does not bind publisher identity to live cut") unless learner_source.include?("published_identity_matches_cut") && runtime_source.include?("published_result_matches_trusted_cut")
fail_proof("raw authenticated primitives are not private") unless transport_source.include?("async fn send_inner") && transport_source.include?("async fn receive_inner")
fail_proof("opaque learner pending response lease missing") unless transport_source.include?("struct LearnerPendingResponse") && learner_source.include?("PendingLearnerRpcResponse")
fail_proof("ordinary pending response lease missing") unless transport_source.include?("pub struct PendingPolisResponse") && transport_source.include?("_guard: tokio::sync::OwnedRwLockReadGuard<()>")
fail_proof("durable transport instance and peer pins missing") unless learner_source.include?("struct TransportInstanceState") && learner_source.include?("peer_instances: BTreeMap<String, [u8; 32]>") && transport_source.include?("transport_peer_identity_key")
fail_proof("boot signing is not guarded by current durable generation") unless learner_source.include?("with_current(|| signer.sign(payload))") && runtime_source.include?("pub(crate) fn with_current<T>")
fail_proof("transition serialization missing") unless runtime_source.include?("authority_transition: Arc<tokio::sync::Mutex<()>>") && runtime_source.include?("pub async fn replace_authority_cut")
fail_proof("exact removal target route-cut validation missing") unless transport_source.include?("exact_removal_target_matches") && runtime_source.include?("trusted_node_identities")
fail_proof("behavioral request/exclusion race proof missing") unless private_tests.include?("actual_request_stream_fenced_against_exclusion_race")
fail_proof("real learner effect/expiry race proof missing") unless private_tests.include?("expiry_writer_waits_through_real_raft_effect_and_response")
fail_proof("learner-owned production factory missing") unless runtime_source.include?("pub struct SecureLearnerNetworkFactory") && runtime_source.include?("LearnerBootAttestationCustody::establish")
fail_proof("production recovery enforcement missing") unless private_tests.include?("production_factory_enforces_recovery_identity_index_and_membership") && learner_source.include?("learner_route_allowed")
fail_proof("removal deadline or target-membership binding missing") unless private_tests.include?("removal_deadline_and_target_membership_bound_cache_first") && learner_source.include?("expected_target_membership_sha256") && learner_source.include?("now_unix_seconds >= verified.payload.deadline_unix_seconds")
fail_proof("durable peer restart and mismatch proof missing") unless private_tests.include?("fresh_connection_requires_durable_peer_instance_pin") && private_tests.include?("transport_instance_and_peer_pin_are_durable_and_unique")
%w[install_learner_route request_bytes serve_authorized_learner_connection add_learner].each do |behavior|
  fail_proof("real fourth-Raft behavior missing #{behavior}") unless private_tests.include?(behavior)
end
FileUtils.mkdir_p(OUTPUT, mode: 0o700)

commands = {}
commands["private_cases"] = run_command("private-cases", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::learner_transport::tests -- --nocapture --test-threads=1])
commands["public_cases"] = run_command("public-cases", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- --test-threads=1])
commands["transport_compile"] = run_command("transport-compile", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_transport --no-run])
commands["discovery_compile"] = run_command("discovery-compile", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_discovery --no-run])
commands["runtime_compile"] = run_command("runtime-compile", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport --no-run])
commands["runtime_route_rotation"] = run_command("runtime-route-rotation", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport route_replacement_retries_exact_sequence_after_peer_restart_and_certificate_rotation -- --exact --nocapture])
commands["coverage_contract"] = run_command("coverage-contract", %w[bash adl/tools/test_check_coverage_impact.sh])
commands["clippy_lib"] = run_command("clippy-lib", %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings])
commands["clippy_public"] = run_command("clippy-public", %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- -D warnings])
fail_proof("command failed") unless commands.values.all? { |command| command["exit_code"] == 0 }
private_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["private_cases"]["#{stream}_path"])) }.join
public_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["public_cases"]["#{stream}_path"])) }.join
fail_proof("private runner test count mismatch") unless private_text.include?("test result: ok. 42 passed; 0 failed")
fail_proof("public test count mismatch") unless public_text.include?("test result: ok. 13 passed; 0 failed")
observed = private_text.lines.map do |line|
  next unless line.include?(MARKER)
  name, result = line.split(MARKER, 2).fetch(1).strip.split("=", 2)
  [name, result]
end.compact
fail_proof("runner marker denominator mismatch") unless observed.length == 42 && observed.map(&:first).sort == EXPECTED_CASES.sort && observed.all? { |_, result| result == "passed" }
assertions = private_text.lines.map do |line|
  next unless line.include?(ASSERTION_MARKER)
  line.split(ASSERTION_MARKER, 2).fetch(1).strip.split(" ", 2)
end.compact
fail_proof("subassertion mismatch") unless assertions.sort == EXPECTED_ASSERTIONS.sort
tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source tree unavailable") unless status.success?
proof = {
  "schema" => "adl.issue202.authorized_learner_transport_proof.v9", "issue" => 202,
  "source_revision" => source, "source_tree" => tree.strip, "required_main_ancestor" => MAIN_ANCESTOR,
  "protected_files" => PROTECTED.map { |path| { "path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest } },
  "commands" => commands, "test_summary" => { "semantic_cases" => 36, "private_runner_selected" => 42, "private_runner_passed" => 42, "public_selected" => 13, "public_passed" => 13, "named_subassertions" => 31 },
  "cases" => EXPECTED_CASES.map { |name| { "case" => name, "result" => "passed", "marker_sha256" => Digest::SHA256.hexdigest("#{MARKER}#{name}=passed") } },
  "subassertions" => EXPECTED_ASSERTIONS.map { |case_name, name| { "case" => case_name, "assertion" => name, "marker_sha256" => Digest::SHA256.hexdigest("#{ASSERTION_MARKER}#{case_name} #{name}") } }
}
File.binwrite(PROOF, JSON.generate(proof) + "\n")
puts "PASS: produced issue #202 exact 36 semantic / 42 runner + 13 public / 31 assertion v9 proof with coverage-routing and route-rotation regressions at #{source}"
