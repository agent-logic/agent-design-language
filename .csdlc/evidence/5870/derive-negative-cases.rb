#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ISSUE = 5870
MARKER = "ADL_ISSUE_5870_NEGATIVE_CASE_V1 "
EXPECTED = {
  "fence_without_old_holder_activation_proof" => "fenced",
  "revoke_without_old_holder_activation_proof" => "fenced",
  "fence_same_epoch" => "denied",
  "fence_epoch_gap" => "denied",
  "fence_uncommitted_next_epoch" => "denied",
  "stale_authority_membership" => "denied",
  "no_current_authority_membership" => "denied",
  "unauthorized_operation" => "denied",
  "fenced_mutation" => "denied",
  "replay_receipt_mismatch" => "denied",
  "atomic_receipt_failure" => "fail_closed",
  "restart_floor_retained" => "fenced",
  "rollback_below_floor" => "denied",
  "unsafe_state_path" => "denied",
  "symlink_state_path" => "denied",
  "state_capacity" => "denied"
}.freeze
EXACT = ["ruby", ".csdlc/evidence/5870/run-exact-child-tests.rb", "cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_fencing", "--no-tests=fail"].freeze
CLIPPY = ["cargo", "clippy", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_fencing", "--", "-D", "warnings"].freeze
NEGATIVE = ["cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_fencing", "--", "--nocapture"].freeze
ROOT = Pathname.new(__dir__).join("../../..").cleanpath.expand_path
EVIDENCE = ROOT.join(".csdlc/evidence/5870")

def fail!(message)
  abort(message)
end

def sha(path)
  Digest::SHA256.file(path).hexdigest
end

def relative(path)
  Pathname.new(path).expand_path.relative_path_from(ROOT).to_s
end

def normalized(text)
  lines = text.lines.map(&:rstrip)
  lines.pop while lines.last == ""
  lines.empty? ? "" : lines.join("\n") + "\n"
end

def run(command)
  started = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3({"CARGO_TERM_COLOR" => "never"}, *command, chdir: ROOT.to_s)
  [stdout, stderr, status, started, Time.now.utc.iso8601(6)]
end

if ARGV.first == "verify"
  machine = JSON.parse(File.read(ARGV.fetch(1)))
  fail!("source mismatch") unless machine["source_revision"] == ARGV.fetch(2)
  fail!("producer mismatch") unless machine["producer_sha256"] == sha(__FILE__)
  puts "PASS: machine evidence verified"
  exit
end

source = ARGV.fetch(0)
output = Pathname.new(ARGV.fetch(1)).expand_path
fail!("source revision malformed") unless source.match?(/\A[0-9a-f]{40}\z/)
fail!("producer is not at source revision") unless `git -C #{ROOT} rev-parse HEAD`.strip == source
fail!("output escapes issue evidence") unless output.to_s.start_with?(EVIDENCE.to_s + "/")
fail!("output already exists") if output.exist?
FileUtils.mkdir_p(output)

nextest_out, nextest_err, nextest_status, nextest_start, nextest_finish = run(EXACT)
File.binwrite(output.join("exact-child-tests.stdout.log"), normalized(nextest_out))
File.binwrite(output.join("exact-child-tests.stderr.log"), normalized(nextest_err))
File.binwrite(output.join("exact-child-tests.log"), normalized(nextest_out + nextest_err))
fail!("exact nextest failed") unless nextest_status.success?
summary = (nextest_out + nextest_err).match(/Summary .*?(\d+) tests run: (\d+) passed, 0 skipped/)
fail!("exact nextest denominator mismatch") unless summary && summary[1].to_i.positive? && summary[1] == summary[2]

clippy_out, clippy_err, clippy_status, clippy_start, clippy_finish = run(CLIPPY)
File.binwrite(output.join("strict-focused-clippy.stdout.log"), normalized(clippy_out))
File.binwrite(output.join("strict-focused-clippy.stderr.log"), normalized(clippy_err))
File.binwrite(output.join("strict-focused-clippy.log"), normalized(clippy_out + clippy_err))
fail!("strict focused Clippy failed") unless clippy_status.success?

negative_out, negative_err, negative_status, negative_start, negative_finish = run(NEGATIVE)
File.binwrite(output.join("negative-cases.stdout.log"), normalized(negative_out))
File.binwrite(output.join("negative-cases.stderr.log"), normalized(negative_err))
fail!("negative producer command failed") unless negative_status.success?
observed = negative_out.lines.each_with_object([]) do |line, entries|
  next unless line.start_with?(MARKER)
  payload = JSON.parse(line.delete_prefix(MARKER))
  entries << {"case" => payload.fetch("case"), "result" => payload.fetch("result"), "observed_line_sha256" => Digest::SHA256.hexdigest(line.chomp)}
end
fail!("negative denominator mismatch") unless observed.length == EXPECTED.length && observed.map { |entry| entry["case"] }.uniq.length == EXPECTED.length
cases = EXPECTED.map do |name, result|
  entry = observed.find { |candidate| candidate["case"] == name }
  fail!("negative result mismatch: #{name}") unless entry && entry["result"] == result
  entry
end

machine_path = output.join("negative-cases.json")
machine = {
  "schema" => "adl.wp04.negative_cases.machine.v1", "issue" => ISSUE,
  "source_revision" => source, "producer_path" => relative(__FILE__), "producer_sha256" => sha(__FILE__),
  "command" => {"argv" => NEGATIVE, "exit_code" => negative_status.exitstatus, "started_at" => negative_start, "finished_at" => negative_finish},
  "stdout_path" => relative(output.join("negative-cases.stdout.log")), "stdout_sha256" => sha(output.join("negative-cases.stdout.log")),
  "stderr_path" => relative(output.join("negative-cases.stderr.log")), "stderr_sha256" => sha(output.join("negative-cases.stderr.log")), "cases" => cases
}
File.write(machine_path, JSON.pretty_generate(machine) + "\n")

manifest_path = output.join("validation-manifest.json")
manifest = {"schema" => "adl.wp04.issue_validation_manifest.v1", "issue" => ISSUE, "source_revision" => source, "commands" => [
  {"argv" => EXACT, "exit_code" => nextest_status.exitstatus, "selected_tests" => summary[1].to_i, "started_at" => nextest_start, "finished_at" => nextest_finish, "combined_log_path" => relative(output.join("exact-child-tests.log")), "combined_log_sha256" => sha(output.join("exact-child-tests.log"))},
  {"argv" => CLIPPY, "exit_code" => clippy_status.exitstatus, "started_at" => clippy_start, "finished_at" => clippy_finish, "combined_log_path" => relative(output.join("strict-focused-clippy.log")), "combined_log_sha256" => sha(output.join("strict-focused-clippy.log"))}
]}
File.write(manifest_path, JSON.pretty_generate(manifest) + "\n")

runner_path = EVIDENCE.join("runner.txt")
runner_sha = sha(runner_path)
runner = {"provider" => "local-codex", "run_id" => "5870-local-operator-v1", "os" => "macos", "arch" => "aarch64", "identity_sha256" => runner_sha}
artifacts = [Pathname.new(__FILE__), EVIDENCE.join("run-exact-child-tests.rb"), runner_path,
  output.join("exact-child-tests.log"), output.join("exact-child-tests.stdout.log"), output.join("exact-child-tests.stderr.log"),
  output.join("strict-focused-clippy.log"), output.join("strict-focused-clippy.stdout.log"), output.join("strict-focused-clippy.stderr.log"),
  machine_path, output.join("negative-cases.stdout.log"), output.join("negative-cases.stderr.log"), manifest_path]
proof = {
  "schema" => "adl.wp04.execution_proof.v3", "issue" => ISSUE, "wp" => "WP-04.08", "source_revision" => source,
  "evidence_revision_strategy" => "derive_from_receipt_introduction",
  "protected_paths" => ["adl-runtime/src/distributed/fencing.rs", "adl-runtime/tests/distributed_fencing.rs"],
  "source_artifacts" => ["adl-runtime/src/distributed/fencing.rs", "adl-runtime/tests/distributed_fencing.rs"].map { |path| {"path" => path, "sha256" => Digest::SHA256.hexdigest(`git -C #{ROOT} show #{source}:#{path}`)} },
  "commands" => [{"argv" => EXACT, "exit_code" => 0, "selected_tests" => summary[1].to_i, "started_at" => nextest_start, "finished_at" => nextest_finish, "runner" => runner,
    "stdout_path" => relative(output.join("exact-child-tests.stdout.log")), "stdout_sha256" => sha(output.join("exact-child-tests.stdout.log")),
    "stderr_path" => relative(output.join("exact-child-tests.stderr.log")), "stderr_sha256" => sha(output.join("exact-child-tests.stderr.log"))},
    {"argv" => CLIPPY, "exit_code" => 0, "started_at" => clippy_start, "finished_at" => clippy_finish, "runner" => runner,
    "stdout_path" => relative(output.join("strict-focused-clippy.stdout.log")), "stdout_sha256" => sha(output.join("strict-focused-clippy.stdout.log")),
    "stderr_path" => relative(output.join("strict-focused-clippy.stderr.log")), "stderr_sha256" => sha(output.join("strict-focused-clippy.stderr.log"))}],
  "negative_cases" => EXPECTED.map { |name, result| {"case" => name, "result" => result, "evidence_path" => relative(machine_path), "evidence_sha256" => sha(machine_path)} },
  "artifacts" => artifacts.map { |path| {"path" => relative(path), "sha256" => sha(path)} }, "native_receipts" => []
}
File.write(output.parent.join("execution-proof.json"), JSON.pretty_generate(proof) + "\n")
puts JSON.generate({"schema" => "adl.wp04.negative_cases.producer_result.v1", "issue" => ISSUE, "source_revision" => source, "selected_tests" => summary[1].to_i, "selected_cases" => cases.length})
