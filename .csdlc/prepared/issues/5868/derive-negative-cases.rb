#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

SCHEMA = "adl.wp04.negative_cases.machine.v1"
ISSUE = 5868
MARKER = "ADL_NEGATIVE_CASE_V1 "
EXPECTED = {
  "forged_signature" => "rejected",
  "historical_identity_generation" => "rejected",
  "current_generation_replay" => "rejected",
  "wrong_trust_domain" => "rejected",
  "stale_probe" => "rejected",
  "node_capacity" => "denied",
  "authority_escalation" => "denied"
}.freeze
COMMAND = [
  "cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml", "--test",
  "distributed_failure_detection", "machine_derived_negative_case_evidence", "--",
  "--exact", "--nocapture"
].freeze
EXACT_NEXTEST = [
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test",
  "distributed_failure_detection", "--no-tests=fail"
].freeze
STRICT_CLIPPY = [
  "cargo", "clippy", "--manifest-path", "adl-runtime/Cargo.toml", "--test",
  "distributed_failure_detection", "--", "-D", "warnings"
].freeze
REPO_ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path

def abort_with(message)
  warn(message)
  exit(1)
end

def relative_repo_path(path)
  expanded = Pathname.new(path).expand_path
  relative = expanded.relative_path_from(REPO_ROOT).to_s
  abort_with("evidence path escapes repository") if relative == ".." || relative.start_with?("../")
  relative
rescue ArgumentError
  abort_with("evidence path escapes repository")
end

def ordinary_file!(relative, label)
  abort_with("#{label} path escapes issue evidence") unless relative.start_with?(".csdlc/evidence/#{ISSUE}/")
  path = REPO_ROOT.join(relative)
  metadata = File.lstat(path)
  abort_with("#{label} must be an ordinary file") unless metadata.file? && !metadata.symlink?
  path
rescue Errno::ENOENT
  abort_with("missing #{label}: #{relative}")
end

def digest(path)
  Digest::SHA256.file(path).hexdigest
end

def observed_cases(stdout)
  lines = stdout.lines.filter_map do |line|
    next unless line.start_with?(MARKER)

    payload = JSON.parse(line.delete_suffix("\n").delete_prefix(MARKER))
    name = payload.fetch("case")
    result = payload.fetch("result")
    { "case" => name, "result" => result, "observed_line_sha256" => Digest::SHA256.hexdigest(line.chomp) }
  rescue JSON::ParserError, KeyError
    abort_with("malformed machine negative-case record")
  end
  names = lines.map { |entry| entry.fetch("case") }
  abort_with("machine negative-case records are missing, duplicated, or ambiguous") unless names.length == EXPECTED.length && names.uniq.length == names.length && names.sort == EXPECTED.keys.sort
  lines.each do |entry|
    abort_with("unexpected result for #{entry.fetch('case')}") unless EXPECTED.fetch(entry.fetch("case")) == entry.fetch("result")
  end
  EXPECTED.keys.map { |name| lines.find { |entry| entry.fetch("case") == name } }
end

def verify!(evidence_path, expected_source)
  relative_evidence = relative_repo_path(evidence_path)
  evidence_file = ordinary_file!(relative_evidence, "negative-case evidence")
  evidence = JSON.parse(File.read(evidence_file))
  abort_with("wrong machine negative-case schema") unless evidence["schema"] == SCHEMA
  abort_with("wrong machine negative-case issue") unless evidence["issue"] == ISSUE
  abort_with("machine negative-case source mismatch") unless evidence["source_revision"] == expected_source
  abort_with("machine negative-case command mismatch") unless evidence.dig("command", "argv") == COMMAND
  abort_with("machine negative-case command did not pass") unless evidence.dig("command", "exit_code") == 0
  producer = REPO_ROOT.join(".csdlc/prepared/issues/5868/derive-negative-cases.rb")
  abort_with("machine negative-case producer digest mismatch") unless evidence["producer_sha256"] == digest(producer)

  stdout_path = ordinary_file!(evidence.fetch("stdout_path"), "negative-case stdout")
  stderr_path = ordinary_file!(evidence.fetch("stderr_path"), "negative-case stderr")
  abort_with("negative-case stdout digest mismatch") unless evidence["stdout_sha256"] == digest(stdout_path)
  abort_with("negative-case stderr digest mismatch") unless evidence["stderr_sha256"] == digest(stderr_path)
  derived = observed_cases(File.binread(stdout_path))
  abort_with("retained negative cases do not match executed output") unless evidence["cases"] == derived

  manifest_relative = Pathname.new(relative_evidence).dirname.join("validation-manifest.json").to_s
  manifest_file = ordinary_file!(manifest_relative, "validation manifest")
  manifest = JSON.parse(File.read(manifest_file))
  abort_with("wrong validation manifest schema") unless manifest["schema"] == "adl.wp04.issue_validation_manifest.v1"
  abort_with("validation manifest source mismatch") unless manifest["source_revision"] == expected_source
  commands = Array(manifest["commands"])
  abort_with("validation manifest command denominator mismatch") unless commands.length == 2
  nextest = commands.fetch(0)
  clippy = commands.fetch(1)
  abort_with("validation manifest nextest mismatch") unless nextest["argv"] == EXACT_NEXTEST && nextest["exit_code"] == 0 && nextest["selected_tests"].to_i.positive?
  abort_with("validation manifest Clippy mismatch") unless clippy["argv"] == STRICT_CLIPPY && clippy["exit_code"] == 0
  [nextest, clippy].each do |command|
    log = ordinary_file!(command.fetch("combined_log_path"), "validation log")
    abort_with("validation log digest mismatch") unless command["combined_log_sha256"] == digest(log)
  end
  nextest_log = File.binread(REPO_ROOT.join(nextest.fetch("combined_log_path")))
  summary = nextest_log.match(/Summary .*?(\d+) tests run: (\d+) passed, 0 skipped/)
  abort_with("retained nextest summary mismatch") unless summary && summary[1] == summary[2] && summary[1].to_i == nextest["selected_tests"]
  true
rescue JSON::ParserError, KeyError => error
  abort_with("invalid machine negative-case evidence: #{error.message}")
end

mode = ARGV.shift
case mode
when "produce"
  source = ARGV.shift.to_s
  output_dir = Pathname.new(ARGV.shift.to_s).expand_path
  abort_with("source revision must be exact") unless source.match?(/\A[0-9a-f]{40,64}\z/)
  head, head_error, head_status = Open3.capture3("git", "rev-parse", "HEAD", chdir: REPO_ROOT.to_s)
  abort_with("cannot resolve source revision: #{head_error.strip}") unless head_status.success?
  abort_with("producer must run at exact source revision") unless head.strip == source
  relative_dir = relative_repo_path(output_dir)
  abort_with("output directory escapes issue evidence") unless relative_dir.start_with?(".csdlc/evidence/#{ISSUE}/")
  abort_with("output directory must be absent or empty") if output_dir.exist? && !output_dir.children.empty?
  FileUtils.mkdir_p(output_dir)

  nextest_started_at = Time.now.utc.iso8601(6)
  nextest_stdout, nextest_stderr, nextest_status = Open3.capture3(
    { "CARGO_TERM_COLOR" => "never" }, *EXACT_NEXTEST, chdir: REPO_ROOT.to_s
  )
  nextest_finished_at = Time.now.utc.iso8601(6)
  nextest_log = output_dir.join("exact-child-tests.log")
  File.binwrite(nextest_log, nextest_stdout + nextest_stderr)
  abort_with("exact nextest command failed with exit #{nextest_status.exitstatus}") unless nextest_status.success?
  summary = (nextest_stdout + nextest_stderr).match(/Summary .*?(\d+) tests run: (\d+) passed, 0 skipped/)
  abort_with("exact nextest output has no nonzero all-pass summary") unless summary && summary[1].to_i.positive? && summary[1] == summary[2]

  clippy_started_at = Time.now.utc.iso8601(6)
  clippy_stdout, clippy_stderr, clippy_status = Open3.capture3(
    { "CARGO_TERM_COLOR" => "never" }, *STRICT_CLIPPY, chdir: REPO_ROOT.to_s
  )
  clippy_finished_at = Time.now.utc.iso8601(6)
  clippy_log = output_dir.join("strict-focused-clippy.log")
  File.binwrite(clippy_log, clippy_stdout + clippy_stderr)
  abort_with("strict focused Clippy failed with exit #{clippy_status.exitstatus}") unless clippy_status.success?

  started_at = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3({ "CARGO_TERM_COLOR" => "never" }, *COMMAND, chdir: REPO_ROOT.to_s)
  finished_at = Time.now.utc.iso8601(6)
  stdout_path = output_dir.join("negative-cases.stdout.log")
  stderr_path = output_dir.join("negative-cases.stderr.log")
  File.binwrite(stdout_path, stdout)
  File.binwrite(stderr_path, stderr)
  abort_with("machine negative-case command failed with exit #{status.exitstatus}") unless status.success?
  cases = observed_cases(stdout)

  evidence = {
    "schema" => SCHEMA,
    "issue" => ISSUE,
    "source_revision" => source,
    "producer_path" => ".csdlc/prepared/issues/5868/derive-negative-cases.rb",
    "producer_sha256" => digest(__FILE__),
    "command" => {
      "argv" => COMMAND,
      "exit_code" => status.exitstatus,
      "started_at" => started_at,
      "finished_at" => finished_at
    },
    "stdout_path" => relative_repo_path(stdout_path),
    "stdout_sha256" => digest(stdout_path),
    "stderr_path" => relative_repo_path(stderr_path),
    "stderr_sha256" => digest(stderr_path),
    "cases" => cases
  }
  evidence_path = output_dir.join("negative-cases.json")
  File.write(evidence_path, JSON.pretty_generate(evidence) + "\n")
  validation_manifest = {
    "schema" => "adl.wp04.issue_validation_manifest.v1",
    "issue" => ISSUE,
    "source_revision" => source,
    "commands" => [
      {
        "argv" => EXACT_NEXTEST,
        "exit_code" => nextest_status.exitstatus,
        "selected_tests" => summary[1].to_i,
        "started_at" => nextest_started_at,
        "finished_at" => nextest_finished_at,
        "combined_log_path" => relative_repo_path(nextest_log),
        "combined_log_sha256" => digest(nextest_log)
      },
      {
        "argv" => STRICT_CLIPPY,
        "exit_code" => clippy_status.exitstatus,
        "started_at" => clippy_started_at,
        "finished_at" => clippy_finished_at,
        "combined_log_path" => relative_repo_path(clippy_log),
        "combined_log_sha256" => digest(clippy_log)
      }
    ]
  }
  manifest_path = output_dir.join("validation-manifest.json")
  File.write(manifest_path, JSON.pretty_generate(validation_manifest) + "\n")
  verify!(evidence_path, source)
  puts JSON.generate({ "schema" => "adl.wp04.negative_cases.producer_result.v1", "issue" => ISSUE, "source_revision" => source, "selected_tests" => summary[1].to_i, "selected_cases" => cases.length, "evidence_path" => relative_repo_path(evidence_path), "validation_manifest_path" => relative_repo_path(manifest_path) })
when "verify"
  evidence_path = ARGV.shift.to_s
  source = ARGV.shift.to_s
  verify!(evidence_path, source)
  puts "PASS: machine-derived negative cases bind #{source} to #{relative_repo_path(evidence_path)}"
else
  abort_with("usage: derive-negative-cases.rb produce SOURCE OUTPUT_DIR | verify EVIDENCE SOURCE")
end
