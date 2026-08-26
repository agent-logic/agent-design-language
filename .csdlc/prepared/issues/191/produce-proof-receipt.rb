#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/191/"
MARKER = "ADL_ISSUE_191_CASE "
PROTECTED = [
  "adl-runtime/Cargo.toml",
  "adl-runtime/Cargo.lock",
  "adl-runtime/src/distributed/mod.rs",
  "adl-runtime/src/distributed/transport.rs",
  "adl-runtime/src/distributed/polis_runtime.rs",
  "adl-runtime/tests/distributed_runtime_transport.rs",
  "adl-runtime/tests/distributed_transport.rs",
  "adl-runtime/tests/distributed_discovery.rs",
  ".csdlc/prepared/issues/191/produce-proof-receipt.rb",
  ".csdlc/prepared/issues/191/validate-proof-receipt.rb"
].freeze
EXPECTED_CASES = %w[
  authority_cut_and_polis_quorum
  boot_generation_rollback
  canonical_snapshot_identity
  canonical_transport_frame
  certificate_overlap_boundary
  durable_vote_restart
  exact_retry_after_boot_and_cert_rotation
  journaled_initial_checkpoint
  path_and_state_bounds
  retry_cache_conflict_and_rollback
  secure_three_two_one_real_restart
  signed_mtls_polis_session
  stalled_rpc_idle_timeout
  unproved_polis_and_oversized_frame
].freeze

def fail_proof(message)
  abort "issue 191 producer: #{message}"
end

def ordinary_path(path, must_exist)
  relative = Pathname.new(path)
  fail_proof("path is not normalized: #{path}") if relative.absolute? || relative.cleanpath.to_s != path
  current = ROOT
  path.split("/").each_with_index do |component, index|
    current = current.join(component)
    begin
      metadata = File.lstat(current)
    rescue Errno::ENOENT
      fail_proof("missing path component: #{path}") if must_exist || index < path.split("/").length - 1
      return current
    end
    fail_proof("symlink path component: #{path}") if metadata.symlink?
    fail_proof("non-directory ancestor: #{path}") if index < path.split("/").length - 1 && !metadata.directory?
  end
  if must_exist
    metadata = File.lstat(current)
    fail_proof("not an ordinary file: #{path}") unless metadata.file? && !metadata.symlink?
  end
  current
end

def prepare_output(relative)
  fail_proof("output escapes issue evidence") unless relative.start_with?(PREFIX)
  path = Pathname.new(relative)
  fail_proof("output is not normalized") unless path.cleanpath.to_s == relative
  current = ROOT
  path.each_filename do |component|
    current = current.join(component)
    if current.exist?
      metadata = File.lstat(current)
      fail_proof("output contains symlink") if metadata.symlink?
      fail_proof("output ancestor is not a directory") unless metadata.directory?
    else
      Dir.mkdir(current, 0o700)
      metadata = File.lstat(current)
      fail_proof("created output is unsafe") unless metadata.directory? && !metadata.symlink?
    end
  end
  fail_proof("output canonical mismatch") unless current.realpath == current.expand_path
  current
end

def run_command(name, argv, output)
  started = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3({ "NEXTEST_TEST_THREADS" => "1" }, *argv, chdir: ROOT.to_s)
  finished = Time.now.utc.iso8601(6)
  stdout_path = output.join("#{name}.stdout.log")
  stderr_path = output.join("#{name}.stderr.log")
  File.binwrite(stdout_path, stdout)
  File.binwrite(stderr_path, stderr)
  {
    "argv" => argv,
    "exit_code" => status.exitstatus,
    "started_at" => started,
    "finished_at" => finished,
    "stdout_path" => stdout_path.relative_path_from(ROOT).to_s,
    "stdout_sha256" => Digest::SHA256.hexdigest(stdout),
    "stderr_path" => stderr_path.relative_path_from(ROOT).to_s,
    "stderr_sha256" => Digest::SHA256.hexdigest(stderr)
  }
end

mode = ARGV.fetch(0, "produce")
fail_proof("unsupported mode") unless mode == "produce"
source = ARGV.fetch(1)
output_relative = ARGV.fetch(2, ".csdlc/evidence/191/v5")
fail_proof("source revision malformed") unless source.match?(/\A[0-9a-f]{40}\z/)
head, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail_proof("source must be exact current HEAD") unless status.success? && head.strip == source
status_text, status = Open3.capture2(
  "git", "status", "--porcelain=v1", "--untracked-files=all", chdir: ROOT.to_s
)
fail_proof("source worktree must be clean") unless status.success? && status_text.empty?
PROTECTED.each { |path| ordinary_path(path, true) }
source_tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source commit tree unavailable") unless status.success? && source_tree.strip.match?(/\A[0-9a-f]{40}\z/)
PROTECTED.each do |path|
  committed, committed_status = Open3.capture2("git", "show", "#{source}:#{path}", chdir: ROOT.to_s)
  fail_proof("protected path absent from source commit: #{path}") unless committed_status.success?
  fail_proof("dirty protected path: #{path}") unless Digest::SHA256.hexdigest(committed) == Digest::SHA256.file(ROOT.join(path)).hexdigest
end
output = prepare_output(output_relative)
fail_proof("output directory must be empty") unless Dir.children(output).empty?

nextest = run_command("nextest", [
  "cargo", "nextest", "run", "--locked", "--manifest-path", "adl-runtime/Cargo.toml",
  "--test", "distributed_runtime_transport", "--no-tests=fail", "--test-threads=1"
], output)
fail_proof("focused nextest failed") unless nextest["exit_code"] == 0
nextest_text = File.binread(ROOT.join(nextest["stdout_path"])) + File.binread(ROOT.join(nextest["stderr_path"]))
fail_proof("focused denominator mismatch") unless nextest_text.match?(/14 tests run: 14 passed, 0 skipped/)

clippy = run_command("clippy", [
  "cargo", "clippy", "--locked", "--manifest-path", "adl-runtime/Cargo.toml",
  "--test", "distributed_runtime_transport", "--", "-D", "warnings"
], output)
fail_proof("strict Clippy failed") unless clippy["exit_code"] == 0

workspace_compile = run_command("workspace-compile", [
  "cargo", "test", "--locked", "--manifest-path", "adl-runtime/Cargo.toml",
  "--workspace", "--no-run"
], output)
fail_proof("full workspace compatibility compile failed") unless workspace_compile["exit_code"] == 0

machine = run_command("machine-cases", [
  "cargo", "test", "--locked", "--manifest-path", "adl-runtime/Cargo.toml",
  "--test", "distributed_runtime_transport", "--", "--nocapture", "--test-threads=1"
], output)
fail_proof("machine case run failed") unless machine["exit_code"] == 0
machine_text = File.binread(ROOT.join(machine["stdout_path"])) + File.binread(ROOT.join(machine["stderr_path"]))
observed = machine_text.lines.each_with_object([]) do |line, entries|
  next unless line.include?(MARKER)
  payload = line.split(MARKER, 2).fetch(1).strip
  name, result = payload.split("=", 2)
  entries << [name, result, Digest::SHA256.hexdigest(line.chomp)]
end
fail_proof("machine case denominator mismatch") unless observed.length == EXPECTED_CASES.length
fail_proof("machine case names/results mismatch") unless observed.map { |entry| entry[0] }.sort == EXPECTED_CASES && observed.all? { |entry| entry[1] == "passed" }

proof = {
  "schema" => "adl.issue191.secure_raft_proof.v1",
  "issue" => 191,
  "source_revision" => source,
  "source_tree" => source_tree.strip,
  "protected_files" => PROTECTED.map { |path| { "path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest } },
  "commands" => { "nextest" => nextest, "clippy" => clippy, "workspace_compile" => workspace_compile, "machine_cases" => machine },
  "test_summary" => { "selected" => 14, "passed" => 14, "skipped" => 0 },
  "cases" => observed.map { |name, result, digest| { "case" => name, "result" => result, "observed_line_sha256" => digest } }
}
proof_path = output.join("execution-proof.json")
File.binwrite(proof_path, JSON.generate(proof) + "\n")
puts "PASS: issue 191 proof produced at #{proof_path.relative_path_from(ROOT)}"
