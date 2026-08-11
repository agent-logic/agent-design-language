#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/191/"
EXPECTED_PROTECTED = [
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
EXPECTED_ARGV = {
  "nextest" => ["cargo", "nextest", "run", "--locked", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_runtime_transport", "--no-tests=fail", "--test-threads=1"],
  "clippy" => ["cargo", "clippy", "--locked", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_runtime_transport", "--", "-D", "warnings"],
  "workspace_compile" => ["cargo", "test", "--locked", "--manifest-path", "adl-runtime/Cargo.toml", "--workspace", "--no-run"],
  "machine_cases" => ["cargo", "test", "--locked", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_runtime_transport", "--", "--nocapture", "--test-threads=1"]
}.freeze

def fail_receipt(message)
  abort "issue 191 receipt: #{message}"
end

def ordinary_file(relative, prefix = nil)
  fail_receipt("path is not normalized: #{relative}") unless relative.is_a?(String) && !Pathname.new(relative).absolute? && Pathname.new(relative).cleanpath.to_s == relative
  fail_receipt("path escapes required prefix: #{relative}") if prefix && !relative.start_with?(prefix)
  current = ROOT
  relative.split("/").each_with_index do |component, index|
    current = current.join(component)
    metadata = File.lstat(current)
    fail_receipt("symlink path component: #{relative}") if metadata.symlink?
    fail_receipt("non-directory ancestor: #{relative}") if index < relative.split("/").length - 1 && !metadata.directory?
  rescue Errno::ENOENT
    fail_receipt("missing file: #{relative}")
  end
  metadata = File.lstat(current)
  fail_receipt("not an ordinary file: #{relative}") unless metadata.file? && !metadata.symlink?
  current
end

def git_output(*arguments)
  stdout, stderr, status = Open3.capture3("git", *arguments, chdir: ROOT.to_s)
  fail_receipt("git #{arguments.join(' ')} failed: #{stderr.strip}") unless status.success?
  stdout
end

proof_relative = ARGV.fetch(0, ".csdlc/evidence/191/v5/execution-proof.json")
proof_path = ordinary_file(proof_relative, PREFIX)
proof = JSON.parse(File.binread(proof_path))
fail_receipt("schema mismatch") unless proof["schema"] == "adl.issue191.secure_raft_proof.v1"
fail_receipt("issue mismatch") unless proof["issue"] == 191
fail_receipt("source revision malformed") unless proof["source_revision"].to_s.match?(/\A[0-9a-f]{40}\z/)
fail_receipt("source tree malformed") unless proof["source_tree"].to_s.match?(/\A[0-9a-f]{40}\z/)

protected = Array(proof["protected_files"])
fail_receipt("protected denominator mismatch") unless protected.map { |entry| entry["path"] } == EXPECTED_PROTECTED
protected.each do |entry|
  path = ordinary_file(entry.fetch("path"))
  digest = entry.fetch("sha256")
  fail_receipt("protected digest malformed") unless digest.match?(/\A[0-9a-f]{64}\z/)
  fail_receipt("protected source drift: #{entry['path']}") unless Digest::SHA256.file(path).hexdigest == digest
end

source = proof.fetch("source_revision")
source_exists = system(
  "git", "cat-file", "-e", "#{source}^{commit}",
  chdir: ROOT.to_s,
  out: File::NULL,
  err: File::NULL
)
if source_exists
  source_tree = git_output("rev-parse", "#{source}^{tree}").strip
  fail_receipt("source tree mismatch") unless source_tree == proof.fetch("source_tree")
  protected.each do |entry|
    committed = git_output("show", "#{source}:#{entry.fetch('path')}")
    fail_receipt("source object digest mismatch: #{entry['path']}") unless Digest::SHA256.hexdigest(committed) == entry.fetch("sha256")
  end
end

summary = proof.fetch("test_summary")
fail_receipt("test summary mismatch") unless summary == { "selected" => 14, "passed" => 14, "skipped" => 0 }
commands = proof.fetch("commands")
fail_receipt("command denominator mismatch") unless commands.keys.sort == EXPECTED_ARGV.keys.sort
commands.each do |name, command|
  fail_receipt("wrong #{name} argv") unless command.fetch("argv") == EXPECTED_ARGV.fetch(name)
  fail_receipt("#{name} failed") unless command.fetch("exit_code") == 0
  started = Time.iso8601(command.fetch("started_at"))
  finished = Time.iso8601(command.fetch("finished_at"))
  fail_receipt("#{name} timestamps inverted") if finished < started
  %w[stdout stderr].each do |stream|
    path = ordinary_file(command.fetch("#{stream}_path"), PREFIX)
    digest = command.fetch("#{stream}_sha256")
    fail_receipt("#{name} #{stream} digest malformed") unless digest.match?(/\A[0-9a-f]{64}\z/)
    fail_receipt("#{name} #{stream} digest mismatch") unless Digest::SHA256.file(path).hexdigest == digest
  end
end

nextest = commands.fetch("nextest")
nextest_text = File.binread(ROOT.join(nextest.fetch("stdout_path"))) + File.binread(ROOT.join(nextest.fetch("stderr_path")))
fail_receipt("nextest nonzero denominator missing") unless nextest_text.match?(/14 tests run: 14 passed, 0 skipped/)
machine = commands.fetch("machine_cases")
machine_text = File.binread(ROOT.join(machine.fetch("stdout_path"))) + File.binread(ROOT.join(machine.fetch("stderr_path")))
observed = machine_text.lines.each_with_object([]) do |line, entries|
  next unless line.include?("ADL_ISSUE_191_CASE ")
  payload = line.split("ADL_ISSUE_191_CASE ", 2).fetch(1).strip
  name, result = payload.split("=", 2)
  entries << [name, result, Digest::SHA256.hexdigest(line.chomp)]
end
cases = Array(proof["cases"])
fail_receipt("case denominator mismatch") unless cases.length == EXPECTED_CASES.length && observed.length == EXPECTED_CASES.length
fail_receipt("case order/names mismatch") unless cases.map { |entry| entry["case"] }.sort == EXPECTED_CASES && observed.map { |entry| entry[0] }.sort == EXPECTED_CASES
cases.each do |entry|
  match = observed.find { |value| value[0] == entry.fetch("case") }
  fail_receipt("case result mismatch") unless entry.fetch("result") == "passed" && match[1] == "passed"
  fail_receipt("case marker digest mismatch") unless entry.fetch("observed_line_sha256") == match[2]
end

introductions = git_output("log", "--format=%H", "--diff-filter=A", "--", proof_relative).lines.map(&:strip).reject(&:empty?)
fail_receipt("execution proof must have exactly one immutable introduction") unless introductions.length == 1
introduction = introductions.fetch(0)
lineage = source_exists && system(
  "git", "merge-base", "--is-ancestor", source, introduction,
  chdir: ROOT.to_s,
  out: File::NULL,
  err: File::NULL
)
unless lineage
  introduced_paths = git_output(
    "diff-tree", "--no-commit-id", "--name-only", "-r", "#{introduction}^", introduction
  ).lines.map(&:strip).reject(&:empty?)
  fail_receipt("squash introduction does not contain the exact protected source") unless (EXPECTED_PROTECTED - introduced_paths).empty?
end
protected_drift = git_output("diff", "--name-only", "#{introduction}..HEAD", "--", *EXPECTED_PROTECTED)
fail_receipt("protected source changed after proof introduction") unless protected_drift.empty?
evidence_drift = git_output("diff", "--name-only", "#{introduction}..HEAD", "--", PREFIX)
fail_receipt("issue evidence changed after immutable introduction") unless evidence_drift.empty?
worktree_drift = git_output("status", "--porcelain=v1", "--untracked-files=all", "--", *EXPECTED_PROTECTED, PREFIX)
fail_receipt("protected source or evidence worktree is dirty") unless worktree_drift.empty?

evidence_bytes = Dir.glob(ROOT.join(PREFIX, "**", "*")).select { |path| File.file?(path) }.sum { |path| File.size(path) }
fail_receipt("evidence exceeds 16 MiB") if evidence_bytes > 16 * 1024 * 1024
forbidden = /(BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY|aws_secret_access_key|github_pat_|ghp_[A-Za-z0-9]{20,})/
Dir.glob(ROOT.join(PREFIX, "**", "*")).select { |path| File.file?(path) }.each do |path|
  fail_receipt("secret-like evidence: #{path}") if File.binread(path).match?(forbidden)
end

puts "PASS: issue 191 merge-safe proof binds production registration, compatibility compile, 14/14 tests, strict Clippy, 14 machine cases, and current protected-source digests"
