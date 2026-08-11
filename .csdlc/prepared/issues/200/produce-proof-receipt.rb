#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/200/v2/"
OUTPUT = ROOT.join(PREFIX)
PROOF = OUTPUT.join("execution-proof.json")
MARKER = "ADL_ISSUE_200_CASE_V1 "
PROTECTED = [
  "adl-runtime/src/distributed/mod.rs",
  "adl-runtime/src/distributed/authority_protocol.rs",
  "adl-runtime/src/distributed/authority_reconciliation.rs",
  "adl-runtime/src/distributed/authority_reconciliation/tests.rs",
  "adl-runtime/src/distributed/polis_runtime.rs",
  "adl-runtime/tests/distributed_authority_reconciliation.rs",
  ".csdlc/prepared/issues/200/produce-proof-receipt.rb",
  ".csdlc/prepared/issues/200/validate-proof-receipt.rb"
].freeze
EXPECTED_CASES = %w[
  happy_single_step happy_multi_step exact_retry_cached_result pending_blocks_read
  pending_blocks_mutation published_permit_current missing_201_token
  public_token_forgery_denied legacy_command_denied wrong_domain wrong_polis wrong_node
  wrong_guardian wrong_boot wrong_protocol_instance wrong_membership wrong_operation_kind
  wrong_adapter_version wrong_time_digest conflicting_retry reordered_step duplicate_step
  missing_step forged_step_receipt crash_after_journal crash_each_step crash_after_result
  crash_before_checkpoint crash_after_checkpoint coherent_rollback capacity_n_plus_one_no_partial
  state_or_lock_symlink_rejected corrupt_journal_rejected noncanonical_state_rejected
  opened_handle_growth_rejected checkpoint_object_collision
].freeze
PASSED = %w[happy_single_step happy_multi_step exact_retry_cached_result published_permit_current].freeze
RECONCILED = %w[crash_after_journal crash_each_step crash_after_result crash_before_checkpoint crash_after_checkpoint].freeze

def fail_proof(message)
  abort("issue 200 producer: #{message}")
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
    "argv" => argv, "exit_code" => status.exitstatus,
    "started_at" => started, "finished_at" => finished,
    "stdout_path" => stdout_path.relative_path_from(ROOT).to_s,
    "stdout_sha256" => Digest::SHA256.hexdigest(stdout),
    "stderr_path" => stderr_path.relative_path_from(ROOT).to_s,
    "stderr_sha256" => Digest::SHA256.hexdigest(stderr)
  }
end

if PROOF.file?
  _out, status = Open3.capture2("ruby", ".csdlc/prepared/issues/200/validate-proof-receipt.rb", chdir: ROOT.to_s)
  fail_proof("retained immutable proof is invalid") unless status.success?
  puts "PASS: retained immutable issue #200 proof is current"
  exit 0
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
commands["nextest"] = run_command("nextest", %w[cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --lib --test distributed_authority_reconciliation --no-tests=fail -E test(/authority_reconciliation/)])
fail_proof("focused nextest failed") unless commands["nextest"]["exit_code"] == 0
nextest_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["nextest"]["#{stream}_path"])) }.join
fail_proof("nextest denominator mismatch") unless nextest_text.match?(/36 tests run: 36 passed, \d+ skipped/)

commands["clippy"] = run_command("clippy", %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib --test distributed_authority_reconciliation -- -D warnings])
fail_proof("strict Clippy failed") unless commands["clippy"]["exit_code"] == 0

commands["machine_cases"] = run_command("machine-cases", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib --test distributed_authority_reconciliation authority_reconciliation -- --nocapture --test-threads=1])
fail_proof("machine cases failed") unless commands["machine_cases"]["exit_code"] == 0
machine_text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands["machine_cases"]["#{stream}_path"])) }.join
observed = machine_text.lines.map do |line|
  next unless line.include?(MARKER)
  name, result = line.split(MARKER, 2).fetch(1).strip.split(" ", 2)
  [name, result, Digest::SHA256.hexdigest("#{MARKER}#{name} #{result}")]
end.compact
fail_proof("case denominator mismatch") unless observed.length == 36 && observed.map(&:first).sort == EXPECTED_CASES.sort
observed_by_name = observed.to_h { |name, result, digest| [name, [result, digest]] }
EXPECTED_CASES.each do |name|
  expected = PASSED.include?(name) ? "passed" : (RECONCILED.include?(name) ? "reconciled" : "rejected")
  fail_proof("wrong result for #{name}") unless observed_by_name.fetch(name).first == expected
end

tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source tree unavailable") unless status.success?
proof = {
  "schema" => "adl.issue200.authority_reconciliation_proof.v1",
  "issue" => 200,
  "source_revision" => source,
  "source_tree" => tree.strip,
  "protected_files" => PROTECTED.map { |path| { "path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest } },
  "commands" => commands,
  "test_summary" => { "selected" => 36, "passed" => 36, "skipped" => 0 },
  "result_summary" => { "passed" => 4, "reconciled" => 5, "rejected" => 27 },
  "cases" => EXPECTED_CASES.map do |name|
    result, digest = observed_by_name.fetch(name)
    { "case" => name, "result" => result, "observed_line_sha256" => digest }
  end
}
File.binwrite(PROOF, JSON.generate(proof) + "\n")
puts "PASS: produced exact issue #200 36-case proof at source #{source}"
