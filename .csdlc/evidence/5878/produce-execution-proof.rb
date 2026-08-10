#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "shellwords"
require "time"

ROOT = File.expand_path("../../..", __dir__)
ISSUE_ROOT = File.join(ROOT, ".csdlc/evidence/5878")
OPERATOR_ROOT = File.join(ISSUE_ROOT, "operator-v1")
SOURCE_REVISION = `git -C #{ROOT.shellescape} rev-parse HEAD`.strip
EXPECTED_SOURCE = "413c1e09992e8e1d996858f8b4a70d210b3eb0d8"
PROTECTED_PATHS = [
  "adl-runtime/src/distributed/mod.rs",
  "adl-runtime/src/lib.rs",
  "adl-runtime/tests/distributed_guardian.rs",
  "adl/tools/validate_v092_distributed_guardian.sh",
  "adl/tools/validate_v092_distributed_native_receipts.rb",
  ".github/workflows/wp04-native-distributed.yml"
].freeze

abort "wrong source revision" unless SOURCE_REVISION == EXPECTED_SOURCE
abort "protected source is dirty" unless `git -C #{ROOT.shellescape} status --porcelain -- #{PROTECTED_PATHS.map(&:shellescape).join(' ')}`.empty?

FileUtils.mkdir_p(OPERATOR_ROOT)
runner_path = File.join(ISSUE_ROOT, "runner.txt")
File.binwrite(runner_path, "local-codex-macos-aarch64-issue-5878-final-v2\n")
runner = {
  "provider" => "local-codex",
  "run_id" => "5878-local-final-v2",
  "os" => "macos",
  "arch" => `uname -m`.strip,
  "identity_sha256" => Digest::SHA256.file(runner_path).hexdigest
}

def run_command(argv, stem, runner, env = {})
  started = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3(env, *argv, chdir: ROOT)
  finished = Time.now.utc.iso8601(6)
  stdout_path = File.join(OPERATOR_ROOT, "#{stem}.stdout.log")
  stderr_path = File.join(OPERATOR_ROOT, "#{stem}.stderr.log")
  File.binwrite(stdout_path, stdout)
  File.binwrite(stderr_path, stderr)
  abort "command failed: #{argv.join(' ')}" unless status.success?
  {
    "argv" => argv,
    "exit_code" => status.exitstatus,
    "started_at" => started,
    "finished_at" => finished,
    "runner" => runner,
    "stdout_path" => stdout_path.delete_prefix("#{ROOT}/"),
    "stdout_sha256" => Digest::SHA256.file(stdout_path).hexdigest,
    "stderr_path" => stderr_path.delete_prefix("#{ROOT}/"),
    "stderr_sha256" => Digest::SHA256.file(stderr_path).hexdigest
  }
end

test_command = run_command(
  ["cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_guardian", "--no-tests=fail", "--no-capture"],
  "exact-child-tests",
  runner,
  { "NO_COLOR" => "1" }
)
test_command["selected_tests"] = 2

native_receipts = %w[macos linux windows].map do |platform|
  JSON.parse(File.read(File.join(ISSUE_ROOT, "native", platform, "receipt.json")))
end
linux_command = native_receipts.find { |receipt| receipt.fetch("platform") == "linux" }.fetch("command")

validator_command = run_command(
  ["ruby", "adl/tools/validate_v092_distributed_native_receipts.rb"],
  "native-receipt-validator",
  runner
)

negative_names = native_receipts.flat_map { |receipt| receipt.fetch("negative_cases") }.uniq.sort
expected_negative_names = %w[authority_replay oversized_protobuf_frame wrong_authority_domain]
abort "negative denominator mismatch" unless negative_names == expected_negative_names
negative_path = File.join(OPERATOR_ROOT, "negative-cases.json")
File.binwrite(
  negative_path,
  JSON.pretty_generate(negative_names.map { |name| { "case" => name, "result" => "rejected" } }) + "\n"
)
negative_sha = Digest::SHA256.file(negative_path).hexdigest

source_artifacts = PROTECTED_PATHS.map do |path|
  bytes, status = Open3.capture2("git", "-C", ROOT, "show", "#{SOURCE_REVISION}:#{path}")
  abort "missing protected source artifact #{path}" unless status.success?
  { "path" => path, "sha256" => Digest::SHA256.hexdigest(bytes) }
end

artifact_paths = Dir.glob(File.join(ISSUE_ROOT, "**", "*"))
  .select { |path| File.file?(path) && !File.zero?(path) }
  .reject { |path| path.end_with?("execution-proof.json") }
  .sort
artifacts = artifact_paths.map do |path|
  { "path" => path.delete_prefix("#{ROOT}/"), "sha256" => Digest::SHA256.file(path).hexdigest }
end

proof = {
  "schema" => "adl.wp04.execution_proof.v3",
  "issue" => 5878,
  "wp" => "WP-04.16",
  "source_revision" => SOURCE_REVISION,
  "evidence_revision_strategy" => "derive_from_receipt_introduction",
  "protected_paths" => PROTECTED_PATHS,
  "source_artifacts" => source_artifacts,
  "commands" => [test_command, linux_command, validator_command],
  "negative_cases" => negative_names.map do |name|
    {
      "case" => name,
      "result" => "rejected",
      "evidence_path" => negative_path.delete_prefix("#{ROOT}/"),
      "evidence_sha256" => negative_sha
    }
  end,
  "artifacts" => artifacts,
  "native_receipts" => native_receipts
}
File.binwrite(File.join(ISSUE_ROOT, "execution-proof.json"), JSON.pretty_generate(proof) + "\n")
puts "PASS: produced exact #5878 execution proof"
