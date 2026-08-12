#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/203/v1/"
PROOF_RELATIVE = "#{PREFIX}authority-store-proof.json"
EXPECTED_PROTECTED = %w[
  adl-runtime/src/distributed/authority_store_adapters.rs
  adl-runtime/src/distributed/certificates.rs
  adl-runtime/src/distributed/lease.rs
  adl-runtime/src/distributed/fencing.rs
  adl-runtime/src/distributed/transport/core.rs
  adl-runtime/src/distributed/transport/governed/polis_runtime.rs
  adl-runtime/src/distributed/capability_advertisement.rs
  adl-runtime/src/distributed/placement.rs
  adl-runtime/src/distributed/projection.rs
  adl-runtime/src/distributed/resource_weather.rs
  adl-runtime/src/distributed/snapshot_catalog.rs
  adl-runtime/src/distributed/migration.rs
  adl-runtime/src/distributed/recovery.rs
  adl-runtime/tests/distributed_identity_lease_authority.rs
  adl-runtime/tests/distributed_runtime_transport.rs
  .csdlc/prepared/issues/203/design.md
  .csdlc/prepared/issues/203/produce-proof-receipt.rb
  .csdlc/prepared/issues/203/validate-proof-receipt.rb
].freeze
EXPECTED_CASES = %w[
  certificate_enroll
  certificate_rotate_overlap
  certificate_successor_post_overlap
  certificate_revoke
  certificate_compromise_identity_fence
  lease_grant
  lease_renewal
  lease_revoke
  fence_commit
  activate_after_safety
  owner_commit
  exact_retry_published
  restart_reanchor_safe
  barrier_pending_blocks_all_reads
  unsigned_certificate_rejected
  wrong_issuer_rejected
  wrong_certificate_purpose_rejected
  wrong_certificate_domain_rejected
  stale_certificate_generation_rejected
  token_artifact_digest_mismatch
  reconstructed_endorsements_rejected
  wrong_authority_membership_rejected
  stale_lease_index_rejected
  stale_lease_epoch_rejected
  wrong_activation_possession_rejected
  activate_before_safety_rejected
  floor_precedes_ledger_revocation
  local_clock_unsafe_no_effect
  local_clock_rollback_no_effect
  crash_after_certificate_effect
  crash_after_fence_floor
  crash_after_ledger_effect
  crash_after_local_anchor
  crash_after_result
  crash_before_checkpoint
  crash_after_checkpoint
  stale_read_permit_rejected
  stale_mutation_permit_rejected
  read_to_mutation_escalation_rejected
  wrong_lineage_permit_rejected
  coherent_rollback_rejected
  corrupt_noncanonical_oversized_rejected
  state_or_lock_symlink_rejected
  capacity_n_plus_one_no_partial
].freeze
EXPECTED_SUBASSERTIONS = %w[
  expected_outcome
  canonical_store_state
  publication_barrier_state
].freeze
EXPECTED_COMMANDS = {
  "identity_authority" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- --nocapture --test-threads=1],
  "identity_clippy" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- -D warnings]
}.freeze
EXPECTED_SUMMARY = {
  "identity_tests" => 3,
  "identity_passed" => 3,
  "cases" => 44,
  "subassertions" => 132,
  "source_assertions" => 18,
  "clippy_targets" => 1
}.freeze

def fail_receipt(message)
  abort("issue 203 receipt: #{message}")
end

def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  fail_receipt("git failed: #{err.strip}") unless status.success?
  out
end

def ordinary(relative)
  path = Pathname.new(relative)
  fail_receipt("unsafe path #{relative}") if path.absolute? || path.cleanpath.to_s != relative
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
fail_receipt("top-level key mismatch") unless proof.keys.sort == %w[cases commands issue protected_files required_main_ancestor schema source_revision source_tree subassertions test_summary]
fail_receipt("schema/issue mismatch") unless proof.fetch("schema") == "adl.issue203.authority_store_adapter_proof.v1" && proof.fetch("issue") == 203
source = proof.fetch("source_revision")
source_tree = proof.fetch("source_tree")
main = proof.fetch("required_main_ancestor")
fail_receipt("revision malformed") unless [source, source_tree, main].all? { |value| value.match?(/\A[0-9a-f]{40}\z/) }
fail_receipt("proof is not bound to exact current origin/main") unless git("rev-parse", "refs/remotes/origin/main").strip == main
fail_receipt("current origin/main ancestry missing") unless system("git", "merge-base", "--is-ancestor", main, source, chdir: ROOT.to_s)
protected = proof.fetch("protected_files")
fail_receipt("protected denominator mismatch") unless protected.map { |entry| entry.fetch("path") } == EXPECTED_PROTECTED
protected.each do |entry|
  fail_receipt("protected entry key mismatch") unless entry.keys.sort == %w[path sha256]
  fail_receipt("protected digest malformed") unless entry.fetch("sha256").match?(/\A[0-9a-f]{64}\z/)
  fail_receipt("protected digest drift #{entry.fetch('path')}") unless Digest::SHA256.file(ordinary(entry.fetch("path"))).hexdigest == entry.fetch("sha256")
end
fail_receipt("test summary mismatch") unless proof.fetch("test_summary") == EXPECTED_SUMMARY
cases = proof.fetch("cases")
fail_receipt("case denominator/order mismatch") unless cases.length == 44 && cases.map { |entry| entry.fetch("case") } == EXPECTED_CASES && cases.map { |entry| entry.fetch("case") }.uniq.length == 44
cases.each do |entry|
  fail_receipt("case key/result mismatch") unless entry.keys.sort == %w[case result subassertions] && entry.fetch("result") == "pass" && entry.fetch("subassertions") == EXPECTED_SUBASSERTIONS
end
expected_subassertions = EXPECTED_CASES.flat_map { |case_name| EXPECTED_SUBASSERTIONS.map { |subassertion| { "case" => case_name, "subassertion" => subassertion, "result" => "pass" } } }
fail_receipt("subassertion denominator/order mismatch") unless proof.fetch("subassertions") == expected_subassertions
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
identity_output = %w[stdout stderr].map { |stream| File.binread(ordinary(commands.fetch("identity_authority").fetch("#{stream}_path"))) }.join
running = identity_output.scan(/^running (\d+) tests?$/).flatten.map(&:to_i)
summary = identity_output.scan(/^test result: ok\. (\d+) passed; (\d+) failed;/).map { |passed, failed| [passed.to_i, failed.to_i] }
fail_receipt("identity test denominator mismatch") unless running == [3] && summary == [[3, 0]]
observed_cases = identity_output.lines.map do |line|
  next unless line.include?("ADL_ISSUE_203_CASE_V1 ")
  match = line.split("ADL_ISSUE_203_CASE_V1 ", 2).fetch(1).strip.match(/\Acase=([^ ]+) result=pass\z/)
  fail_receipt("malformed observed case marker") unless match
  match[1]
end.compact
fail_receipt("observed case denominator/substitution mismatch") unless observed_cases == EXPECTED_CASES
observed_subassertions = identity_output.lines.map do |line|
  next unless line.include?("ADL_ISSUE_203_SUBASSERTION_V1 ")
  match = line.split("ADL_ISSUE_203_SUBASSERTION_V1 ", 2).fetch(1).strip.match(/\Acase=([^ ]+) subassertion=([^ ]+) result=pass\z/)
  fail_receipt("malformed observed subassertion marker") unless match
  [match[1], match[2]]
end.compact
fail_receipt("observed subassertion denominator/substitution mismatch") unless observed_subassertions == EXPECTED_CASES.flat_map { |case_name| EXPECTED_SUBASSERTIONS.map { |subassertion| [case_name, subassertion] } }
introductions = git("log", "--format=%H", "--diff-filter=A", "--", PROOF_RELATIVE).lines.map(&:strip).reject(&:empty?)
fail_receipt("proof requires immutable introduction") if introductions.empty?
introduction = introductions.first
fail_receipt("proof not introduced from absence") if system("git", "cat-file", "-e", "#{introduction}^:#{PROOF_RELATIVE}", chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
fail_receipt("source not ancestral") unless system("git", "merge-base", "--is-ancestor", source, introduction, chdir: ROOT.to_s)
fail_receipt("source tree mismatch") unless git("rev-parse", "#{source}^{tree}").strip == source_tree
protected.each do |entry|
  fail_receipt("source object mismatch #{entry.fetch('path')}") unless Digest::SHA256.hexdigest(git("show", "#{source}:#{entry.fetch('path')}")) == entry.fetch("sha256")
end
fail_receipt("protected source changed after proof") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", *EXPECTED_PROTECTED).empty?
fail_receipt("immutable proof changed after introduction") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", PREFIX).empty?
fail_receipt("worktree must be exactly clean") unless git("status", "--porcelain=v1", "--untracked-files=all").empty?
puts "PASS: issue #203 proof binds exact 44 cases, 132 subassertions, focused identity authority test/clippy argv, protected source, immutable evidence, and exact current origin/main ancestry"
