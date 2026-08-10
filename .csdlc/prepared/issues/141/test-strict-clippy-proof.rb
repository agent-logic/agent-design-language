# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "tmpdir"

require_relative "../5862/proof-receipt-contract"

ARGV_CLIPPY = [
  "cargo", "clippy", "--manifest-path", "adl-runtime/Cargo.toml",
  "--test", "distributed_lease", "--", "-D", "warnings"
].freeze

def expect_rejection
  rejected = false
  begin
    yield
  rescue SystemExit
    rejected = true
  end
  abort "expected validation rejection" unless rejected
end

Dir.mktmpdir("adl-141-clippy-", "/Volumes/FastWork") do |root|
  Dir.chdir(root) do
    evidence = ".csdlc/evidence/141"
    FileUtils.mkdir_p(evidence)
    log = "#{evidence}/clippy.log"
    runner_identity = "#{evidence}/runner.txt"
    File.write(log, "strict clippy passed\n")
    File.write(runner_identity, "rustc fixture identity\n")
    command = {
      "argv" => ARGV_CLIPPY,
      "exit_code" => 0,
      "started_at" => "2026-08-10T17:00:00Z",
      "finished_at" => "2026-08-10T17:00:01Z",
      "runner" => {
        "provider" => "fixture",
        "run_id" => "141-fixture",
        "os" => "macos",
        "arch" => "aarch64",
        "identity_sha256" => Digest::SHA256.file(runner_identity).hexdigest
      },
      "runner_identity_path" => runner_identity,
      "combined_log_path" => log,
      "combined_log_sha256" => Digest::SHA256.file(log).hexdigest
    }
    manifest = {
      "schema" => "adl.wp04.issue_validation_manifest.v1",
      "issue" => 141,
      "source_revision" => "a" * 40,
      "commands" => [command]
    }
    path = "#{evidence}/manifest.json"
    File.write(path, JSON.pretty_generate(manifest))

    Wp04ProofReceiptContract.validate_validation_manifest(
      path: path,
      issue: 141,
      source_revision: "a" * 40,
      required_commands: [ARGV_CLIPPY]
    )

    cases = {
      "missing command" => manifest.merge("commands" => []),
      "failed command" => manifest.merge("commands" => [command.merge("exit_code" => 1)]),
      "reversed timestamps" => manifest.merge("commands" => [command.merge(
        "started_at" => "2026-08-10T17:00:02Z",
        "finished_at" => "2026-08-10T17:00:01Z"
      )]),
      "malformed timestamp" => manifest.merge("commands" => [command.merge("started_at" => "bad")]),
      "wrong log digest" => manifest.merge("commands" => [command.merge("combined_log_sha256" => "0" * 64)]),
      "escaped log path" => manifest.merge("commands" => [command.merge("combined_log_path" => "../clippy.log")]),
      "missing runner" => manifest.merge("commands" => [command.reject { |key, _| key == "runner" }]),
      "invalid runner identity" => manifest.merge("commands" => [command.merge(
        "runner" => command.fetch("runner").merge("identity_sha256" => "0" * 64)
      )])
    }
    cases.each_value do |candidate|
      File.write(path, JSON.pretty_generate(candidate))
      expect_rejection do
        Wp04ProofReceiptContract.validate_validation_manifest(
          path: path,
          issue: 141,
          source_revision: "a" * 40,
          required_commands: [ARGV_CLIPPY]
        )
      end
    end

    File.write(log, "")
    empty_log_manifest = manifest.merge("commands" => [command.merge(
      "combined_log_sha256" => Digest::SHA256.file(log).hexdigest
    )])
    File.write(path, JSON.pretty_generate(empty_log_manifest))
    expect_rejection do
      Wp04ProofReceiptContract.validate_validation_manifest(
        path: path,
        issue: 141,
        source_revision: "a" * 40,
        required_commands: [ARGV_CLIPPY]
      )
    end
    File.write(log, "strict clippy passed\n")

    File.write(path, JSON.pretty_generate(manifest))
    expect_rejection do
      Wp04ProofReceiptContract.validate_validation_manifest(
        path: path,
        issue: 141,
        source_revision: "b" * 40,
        required_commands: [ARGV_CLIPPY]
      )
    end
  end
end

puts "PASS: strict Clippy proof requires one exact successful structured command"
