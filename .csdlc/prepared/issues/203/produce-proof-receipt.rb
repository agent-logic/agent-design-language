#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/203/v1/"
OUTPUT = ROOT.join(PREFIX)
PROOF = OUTPUT.join("authority-store-proof.json")
CASE_MARKER = "ADL_ISSUE_203_CASE_V1 "
SUBASSERTION_MARKER = "ADL_ISSUE_203_SUBASSERTION_V1 "
PROTECTED = %w[
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
COMMANDS = {
  "identity_authority" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- --nocapture --test-threads=1],
  "identity_clippy" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- -D warnings]
}.freeze

def fail_proof(message)
  abort("issue 203 producer: #{message}")
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
    "argv" => argv,
    "exit_code" => status.exitstatus,
    "started_at" => started,
    "finished_at" => finished,
    "stdout_path" => "#{PREFIX}#{name}.stdout.log",
    "stdout_sha256" => Digest::SHA256.hexdigest(stdout),
    "stderr_path" => "#{PREFIX}#{name}.stderr.log",
    "stderr_sha256" => Digest::SHA256.hexdigest(stderr)
  }
end

if PROOF.file?
  _out, status = Open3.capture2("ruby", ".csdlc/prepared/issues/203/validate-proof-receipt.rb", chdir: ROOT.to_s)
  fail_proof("retained immutable proof is invalid") unless status.success?
  puts "PASS: retained immutable issue #203 proof is current"
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
identity_output = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands.fetch("identity_authority").fetch("#{stream}_path"))) }.join
running = identity_output.scan(/^running (\d+) tests?$/).flatten.map(&:to_i)
summary = identity_output.scan(/^test result: ok\. (\d+) passed; (\d+) failed;/).map { |passed, failed| [passed.to_i, failed.to_i] }
fail_proof("identity test denominator mismatch") unless running == [3] && summary == [[3, 0]]
observed_cases = identity_output.lines.map do |line|
  next unless line.include?(CASE_MARKER)
  match = line.split(CASE_MARKER, 2).fetch(1).strip.match(/\Acase=([^ ]+) result=pass\z/)
  fail_proof("malformed case marker") unless match
  match[1]
end.compact
fail_proof("case denominator or substitution mismatch") unless observed_cases == EXPECTED_CASES
observed_subassertions = identity_output.lines.map do |line|
  next unless line.include?(SUBASSERTION_MARKER)
  match = line.split(SUBASSERTION_MARKER, 2).fetch(1).strip.match(/\Acase=([^ ]+) subassertion=([^ ]+) result=pass\z/)
  fail_proof("malformed subassertion marker") unless match
  [match[1], match[2]]
end.compact
expected_subassertions = EXPECTED_CASES.flat_map { |case_name| EXPECTED_SUBASSERTIONS.map { |subassertion| [case_name, subassertion] } }
fail_proof("subassertion denominator or substitution mismatch") unless observed_subassertions == expected_subassertions
tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source tree unavailable") unless status.success?
proof = {
  "schema" => "adl.issue203.authority_store_adapter_proof.v1",
  "issue" => 203,
  "source_revision" => source,
  "source_tree" => tree.strip,
  "required_main_ancestor" => origin_main,
  "protected_files" => PROTECTED.map { |path| { "path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest } },
  "commands" => commands,
  "test_summary" => {
    "identity_tests" => 3,
    "identity_passed" => 3,
    "cases" => EXPECTED_CASES.length,
    "subassertions" => expected_subassertions.length,
    "source_assertions" => 18,
    "clippy_targets" => 1
  },
  "cases" => EXPECTED_CASES.map { |name| { "case" => name, "result" => "pass", "subassertions" => EXPECTED_SUBASSERTIONS } },
  "subassertions" => expected_subassertions.map { |case_name, subassertion| { "case" => case_name, "subassertion" => subassertion, "result" => "pass" } }
}
File.binwrite(PROOF, JSON.generate(proof) + "\n")
puts "PASS: produced issue #203 exact 44-case, 132-subassertion authority-store proof at #{source}"
