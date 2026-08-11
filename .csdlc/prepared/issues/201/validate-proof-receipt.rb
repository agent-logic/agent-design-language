#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/201/"
PROOF_PREFIX = "#{PREFIX}v7/"
PROOF_RELATIVE = "#{PROOF_PREFIX}execution-proof.json"
EXPECTED_PROTECTED = [
  "adl-runtime/Cargo.toml", "adl-runtime/Cargo.lock",
  "adl-runtime/src/distributed/mod.rs", "adl-runtime/src/distributed/authority_protocol.rs",
  "adl-runtime/src/distributed/identity.rs", "adl-runtime/src/distributed/polis_runtime.rs",
  "adl-runtime/src/distributed/transport.rs", "adl-runtime/src/distributed/authority_protocol_contract_tests.rs",
  "adl-runtime/tests/distributed_authority_protocol.rs",
  ".csdlc/prepared/issues/201/produce-proof-receipt.rb",
  ".csdlc/prepared/issues/201/validate-proof-receipt.rb"
].freeze
MARKER = "ADL_ISSUE_201_CASE_V2 "
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

def fail_receipt(message)
  abort("issue 201 receipt: #{message}")
end
def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  fail_receipt("git #{args.join(' ')} failed: #{err.strip}") unless status.success?
  out
end
def ordinary(relative)
  fail_receipt("unsafe path: #{relative}") if Pathname.new(relative).absolute? || Pathname.new(relative).cleanpath.to_s != relative
  current = ROOT
  relative.split("/").each_with_index do |part, index|
    current = current.join(part)
    metadata = File.lstat(current)
    fail_receipt("symlink path: #{relative}") if metadata.symlink?
    fail_receipt("non-directory ancestor: #{relative}") if index < relative.split("/").length - 1 && !metadata.directory?
  end
  fail_receipt("not ordinary file: #{relative}") unless current.file? && !current.symlink?
  current
rescue Errno::ENOENT
  fail_receipt("missing file: #{relative}")
end

def canonical_marker_line(name, result)
  "#{MARKER}#{name} #{result}"
end

def source_validation_mode(source_available, source_is_ancestor)
  return :ancestry if source_available && source_is_ancestor
  raise "available source is not ancestral" if source_available
  :protected_tree
end

def case_contract_error(cases, observed)
  return "case denominator/order mismatch" unless cases.map { |entry| entry["case"] } == EXPECTED_CASES
  return "marker denominator mismatch" unless observed.length == EXPECTED_CASES.length
  return "duplicate marker" unless observed.map(&:first).uniq.length == EXPECTED_CASES.length

  observed_by_name = observed.to_h { |name, result, digest| [name, [result, digest]] }
  EXPECTED_CASES.each_with_index do |name, index|
    result = EXPECTED_RESULTS.fetch(name, "rejected")
    digest = Digest::SHA256.hexdigest(canonical_marker_line(name, result))
    entry = cases.fetch(index)
    return "canonical case substitution: #{name}" unless entry == {
      "case" => name,
      "result" => result,
      "observed_line_sha256" => digest
    }
    return "observed case substitution: #{name}" unless observed_by_name[name] == [result, digest]
  end
  nil
end

canonical_cases = EXPECTED_CASES.map do |name|
  result = EXPECTED_RESULTS.fetch(name, "rejected")
  {
    "case" => name,
    "result" => result,
    "observed_line_sha256" => Digest::SHA256.hexdigest(canonical_marker_line(name, result))
  }
end
canonical_observed = canonical_cases.map do |entry|
  [entry.fetch("case"), entry.fetch("result"), entry.fetch("observed_line_sha256")]
end
fail_receipt("canonical case self-check failed") if case_contract_error(canonical_cases, canonical_observed)
substituted = Marshal.load(Marshal.dump(canonical_cases))
substituted[0]["result"] = "rejected"
fail_receipt("case-substitution regression failed") unless case_contract_error(substituted, canonical_observed)
reordered = canonical_cases.rotate(1)
fail_receipt("case-reorder regression failed") unless case_contract_error(reordered, canonical_observed)

if ARGV == ["--self-test"]
  begin
    source_validation_mode(true, false)
    fail_receipt("available-divergent regression failed")
  rescue RuntimeError => error
    fail_receipt("available-divergent wrong rejection") unless error.message == "available source is not ancestral"
  end
  fail_receipt("available-ancestral regression failed") unless source_validation_mode(true, true) == :ancestry
  fail_receipt("unavailable-fallback regression failed") unless source_validation_mode(false, false) == :protected_tree
  puts "#{MARKER}validator_available_divergent_rejected rejected"
  puts "#{MARKER}validator_available_ancestral_passed passed"
  puts "#{MARKER}validator_unavailable_protected_fallback_passed passed"
  exit 0
end

proof = JSON.parse(File.binread(ordinary(PROOF_RELATIVE)))
fail_receipt("schema/issue mismatch") unless proof["schema"] == "adl.issue201.committed_authority_proof.v2" && proof["issue"] == 201
source = proof.fetch("source_revision")
fail_receipt("source malformed") unless source.match?(/\A[0-9a-f]{40}\z/)
source_tree = proof.fetch("source_tree")
fail_receipt("source tree malformed") unless source_tree.match?(/\A[0-9a-f]{40}\z/)
protected = proof.fetch("protected_files")
fail_receipt("protected denominator mismatch") unless protected.map { |entry| entry["path"] } == EXPECTED_PROTECTED
protected.each do |entry|
  path = ordinary(entry.fetch("path"))
  fail_receipt("protected digest drift: #{entry['path']}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
fail_receipt("test summary mismatch") unless proof["test_summary"] == { "selected" => 86, "passed" => 86, "skipped" => 0 }
fail_receipt("runtime summary mismatch") unless proof["runtime_summary"] == { "selected" => 230, "passed" => 230, "skipped" => 0 }
fail_receipt("result summary mismatch") unless proof["result_summary"] == { "passed" => 11, "reconciled" => 6, "rejected" => 69 }
cases = proof.fetch("cases")
commands = proof.fetch("commands")
fail_receipt("command denominator mismatch") unless commands.keys.sort == %w[clippy full_runtime machine_cases nextest openraft snapshot_cases validator_modes]
commands.each do |name, command|
  fail_receipt("#{name} failed") unless command.fetch("exit_code") == 0
  fail_receipt("#{name} stream normalization mismatch") unless command.fetch("stream_normalization") == "trailing_blank_lines_removed"
  fail_receipt("#{name} time inverted") if Time.iso8601(command.fetch("finished_at")) < Time.iso8601(command.fetch("started_at"))
  %w[stdout stderr].each do |stream|
    relative = command.fetch("#{stream}_path")
    fail_receipt("stream escapes evidence") unless relative.start_with?(PREFIX)
    fail_receipt("#{name} #{stream} digest mismatch") unless Digest::SHA256.file(ordinary(relative)).hexdigest == command.fetch("#{stream}_sha256")
  end
end
full_runtime_text = %w[stdout stderr].map do |stream|
  File.binread(ROOT.join(commands.fetch("full_runtime").fetch("#{stream}_path")))
end.join
fail_receipt("full runtime log denominator mismatch") unless full_runtime_text.match?(/230 tests run: 230 passed, 0 skipped/)
machine = commands.fetch("machine_cases")
snapshot_cases = commands.fetch("snapshot_cases")
validator_modes = commands.fetch("validator_modes")
text = [machine, snapshot_cases, validator_modes].flat_map { |command| %w[stdout stderr].map { |stream| File.binread(ROOT.join(command.fetch("#{stream}_path"))) } }.join
observed = text.lines.each_with_object([]) do |line, rows|
  next unless line.include?(MARKER)
  name, result = line.split(MARKER, 2).fetch(1).strip.split(" ", 2)
  rows << [name, result, Digest::SHA256.hexdigest(canonical_marker_line(name, result))]
end
case_error = case_contract_error(cases, observed)
fail_receipt(case_error) if case_error
introductions = git("log", "--format=%H", "--diff-filter=A", "--", PROOF_RELATIVE).lines.map(&:strip).reject(&:empty?)
fail_receipt("proof requires an immutable introduction") if introductions.empty?
# Historical superseded packets may exist before an explicit deletion. The
# newest addition is the sole live generation and must introduce the path from
# absence; immutability is enforced from that commit to HEAD below.
introduction = introductions.fetch(0)
parent_has_proof = system("git", "cat-file", "-e", "#{introduction}^:#{PROOF_RELATIVE}", chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
fail_receipt("live proof was not introduced from absence") if parent_has_proof
source_available = system("git", "cat-file", "-e", "#{source}^{commit}", chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
source_is_ancestor = source_available && system("git", "merge-base", "--is-ancestor", source, introduction, chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
begin
  mode = source_validation_mode(source_available, source_is_ancestor)
rescue RuntimeError => error
  fail_receipt(error.message)
end
if mode == :ancestry
  fail_receipt("source tree mismatch") unless git("rev-parse", "#{source}^{tree}").strip == source_tree
  protected.each do |entry|
    committed = git("show", "#{source}:#{entry.fetch('path')}")
    fail_receipt("source-object mismatch: #{entry['path']}") unless Digest::SHA256.hexdigest(committed) == entry.fetch("sha256")
  end
else
  # A depth-limited or squash-like consumer may legitimately lack the feature
  # source object. In that case, bind the receipt to the immutable introduction's
  # exact protected blobs instead of asking Git for an unavailable ancestor.
  protected.each do |entry|
    introduced = git("show", "#{introduction}:#{entry.fetch('path')}")
    fail_receipt("protected-tree mismatch: #{entry['path']}") unless Digest::SHA256.hexdigest(introduced) == entry.fetch("sha256")
  end
end
fail_receipt("protected source changed after proof") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", *EXPECTED_PROTECTED).empty?
fail_receipt("immutable proof changed after introduction") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", PROOF_PREFIX).empty?
fail_receipt("protected/proof worktree dirty") unless git("status", "--porcelain=v1", "--untracked-files=all", "--", *EXPECTED_PROTECTED, PROOF_PREFIX).empty?
puts "PASS: issue #201 merge-safe proof binds exact source, strict Clippy, ordered 86/86 cases, and full runtime 230/230"
