#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ISSUE = 5909
MARKER = "ADL_NEGATIVE_CASE_V1 "
EXPECTED = {
  "future_applied_index" => "rejected",
  "stale_applied_index" => "rejected",
  "mutation_replay" => "rejected",
  "activate_without_prior" => "denied",
  "lease_grant_existing" => "denied",
  "quorum_loss" => "denied",
  "activation_possession" => "denied",
  "lineage_capacity" => "denied",
  "serialized_snapshot_capacity" => "denied"
}.freeze
EXACT_NEXTEST = [
  "ruby", ".csdlc/evidence/5909/run-exact-child-tests.rb",
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml",
  "--test", "distributed_lease", "--no-tests=fail"
].freeze
STRICT_CLIPPY = [
  "cargo", "clippy", "--manifest-path", "adl-runtime/Cargo.toml", "--test",
  "distributed_lease", "--", "-D", "warnings"
].freeze
NEGATIVE_COMMAND = [
  "cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml", "--test",
  "distributed_lease", "machine_derived_negative_case_evidence", "--", "--exact", "--nocapture"
].freeze
REPO_ROOT = Pathname.new(__dir__).join("../../..").cleanpath.expand_path
EVIDENCE_ROOT = REPO_ROOT.join(".csdlc/evidence/#{ISSUE}")

def abort_with(message)
  warn(message)
  exit(1)
end

def digest(path)
  Digest::SHA256.file(path).hexdigest
end

def relative(path)
  value = Pathname.new(path).expand_path.relative_path_from(REPO_ROOT).to_s
  abort_with("evidence path escapes repository") if value == ".." || value.start_with?("../")
  value
rescue ArgumentError
  abort_with("evidence path escapes repository")
end

def run(command)
  started_at = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3({ "CARGO_TERM_COLOR" => "never" }, *command, chdir: REPO_ROOT.to_s)
  finished_at = Time.now.utc.iso8601(6)
  [stdout, stderr, status, started_at, finished_at]
end

def observed_cases(stdout)
  entries = stdout.lines.each_with_object([]) do |line, observed|
    next unless line.start_with?(MARKER)

    payload = JSON.parse(line.delete_prefix(MARKER))
    observed << {
      "case" => payload.fetch("case"),
      "result" => payload.fetch("result"),
      "observed_line_sha256" => Digest::SHA256.hexdigest(line.chomp)
    }
  rescue JSON::ParserError, KeyError
    abort_with("malformed machine negative-case record")
  end
  names = entries.map { |entry| entry.fetch("case") }
  abort_with("negative-case denominator or names mismatch") unless names.length == EXPECTED.length && names.uniq.length == names.length && names.sort == EXPECTED.keys.sort
  entries.each do |entry|
    abort_with("negative-case result mismatch: #{entry.fetch('case')}") unless EXPECTED.fetch(entry.fetch("case")) == entry.fetch("result")
  end
  EXPECTED.keys.map { |name| entries.find { |entry| entry.fetch("case") == name } }
end

source = ARGV.fetch(0, "")
output = Pathname.new(ARGV.fetch(1, "")).expand_path
abort_with("source revision must be exact") unless source.match?(/\A[0-9a-f]{40}\z/)
head, head_error, head_status = Open3.capture3("git", "rev-parse", "HEAD", chdir: REPO_ROOT.to_s)
abort_with("cannot resolve HEAD: #{head_error.strip}") unless head_status.success?
abort_with("producer must run at exact source revision") unless head.strip == source
abort_with("output must remain below issue evidence") unless output.to_s.start_with?(EVIDENCE_ROOT.to_s + "/")
abort_with("output directory must be absent") if output.exist?
FileUtils.mkdir_p(output)

nextest_stdout, nextest_stderr, nextest_status, nextest_started, nextest_finished = run(EXACT_NEXTEST)
nextest_log = output.join("exact-child-tests.log")
nextest_stdout_path = output.join("exact-child-tests.stdout.log")
nextest_stderr_path = output.join("exact-child-tests.stderr.log")
File.binwrite(nextest_log, nextest_stdout + nextest_stderr)
File.binwrite(nextest_stdout_path, nextest_stdout)
File.binwrite(nextest_stderr_path, nextest_stderr)
abort_with("exact nextest failed") unless nextest_status.success?
summary = (nextest_stdout + nextest_stderr).match(/Summary .*?(\d+) tests run: (\d+) passed, 0 skipped/)
abort_with("exact nextest denominator mismatch") unless summary && summary[1].to_i.positive? && summary[1] == summary[2]

clippy_stdout, clippy_stderr, clippy_status, clippy_started, clippy_finished = run(STRICT_CLIPPY)
clippy_log = output.join("strict-focused-clippy.log")
File.binwrite(clippy_log, clippy_stdout + clippy_stderr)
abort_with("strict focused Clippy failed") unless clippy_status.success?

negative_stdout, negative_stderr, negative_status, negative_started, negative_finished = run(NEGATIVE_COMMAND)
stdout_path = output.join("negative-cases.stdout.log")
stderr_path = output.join("negative-cases.stderr.log")
File.binwrite(stdout_path, negative_stdout)
File.binwrite(stderr_path, negative_stderr)
abort_with("machine negative-case test failed") unless negative_status.success?
cases = observed_cases(negative_stdout)

negative_evidence = {
  "schema" => "adl.wp04.negative_cases.machine.v1",
  "issue" => ISSUE,
  "source_revision" => source,
  "producer_path" => relative(__FILE__),
  "producer_sha256" => digest(__FILE__),
  "command" => {
    "argv" => NEGATIVE_COMMAND,
    "exit_code" => negative_status.exitstatus,
    "started_at" => negative_started,
    "finished_at" => negative_finished
  },
  "stdout_path" => relative(stdout_path),
  "stdout_sha256" => digest(stdout_path),
  "stderr_path" => relative(stderr_path),
  "stderr_sha256" => digest(stderr_path),
  "cases" => cases
}
File.write(output.join("negative-cases.json"), JSON.pretty_generate(negative_evidence) + "\n")

manifest = {
  "schema" => "adl.wp04.issue_validation_manifest.v1",
  "issue" => ISSUE,
  "source_revision" => source,
  "commands" => [
    {
      "argv" => EXACT_NEXTEST,
      "exit_code" => nextest_status.exitstatus,
      "selected_tests" => summary[1].to_i,
      "started_at" => nextest_started,
      "finished_at" => nextest_finished,
      "combined_log_path" => relative(nextest_log),
      "combined_log_sha256" => digest(nextest_log)
    },
    {
      "argv" => STRICT_CLIPPY,
      "exit_code" => clippy_status.exitstatus,
      "started_at" => clippy_started,
      "finished_at" => clippy_finished,
      "combined_log_path" => relative(clippy_log),
      "combined_log_sha256" => digest(clippy_log)
    }
  ]
}
File.write(output.join("validation-manifest.json"), JSON.pretty_generate(manifest) + "\n")
puts JSON.generate({
  "schema" => "adl.wp04.negative_cases.producer_result.v1",
  "issue" => ISSUE,
  "source_revision" => source,
  "selected_tests" => summary[1].to_i,
  "selected_cases" => cases.length,
  "negative_cases" => relative(output.join("negative-cases.json")),
  "validation_manifest" => relative(output.join("validation-manifest.json"))
})
