#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "rbconfig"
require "time"
require "tmpdir"

SCHEMA = "adl.wp04.negative_cases.machine.v1"
ISSUE = 121
MARKER = "ADL_ISSUE_121_NEGATIVE_CASE_V1 "
EXPECTED = {
  "fence_without_holder_key" => "fenced",
  "revoke_without_holder_key" => "fenced",
  "fence_same_epoch" => "denied",
  "fence_epoch_gap" => "denied",
  "fence_stale_epoch" => "denied",
  "fenced_mutation" => "denied",
  "restore_current_index" => "recovered",
  "recovery_floor_retained" => "fenced",
  "premature_activation" => "denied",
  "holder_operation_possession" => "denied",
  "atomic_fence_failure" => "fail_closed"
}.freeze
COMMAND = [
  "cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml", "--test",
  "distributed_lease", "quorum_fence_and_restart_safety_machine_evidence", "--",
  "--exact", "--nocapture"
].freeze
EXACT_NEXTEST = [
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test",
  "distributed_lease", "--no-tests=fail"
].freeze
STRICT_CLIPPY = [
  "cargo", "clippy", "--manifest-path", "adl-runtime/Cargo.toml", "--test",
  "distributed_lease", "--", "-D", "warnings"
].freeze
REPO_ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
ISSUE_EVIDENCE_PREFIX = ".csdlc/evidence/#{ISSUE}/"

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

def checked_issue_path(relative, label, leaf: :file)
  cleaned = Pathname.new(relative.to_s).cleanpath.to_s
  abort_with("#{label} path is not normalized") unless cleaned == relative.to_s
  abort_with("#{label} path escapes issue evidence") unless cleaned.start_with?(ISSUE_EVIDENCE_PREFIX)

  current = REPO_ROOT
  missing = false
  cleaned.split("/").each_with_index do |component, index|
    current = current.join(component)
    next if missing

    begin
      metadata = File.lstat(current)
    rescue Errno::ENOENT
      missing = true
      next
    end
    abort_with("#{label} path contains a symlink component") if metadata.symlink?
    abort_with("#{label} path contains a non-directory ancestor") if index < cleaned.split("/").length - 1 && !metadata.directory?
  end

  case leaf
  when :file
    abort_with("missing #{label}: #{relative}") if missing
    metadata = File.lstat(current)
    abort_with("#{label} must be an ordinary file") unless metadata.file? && !metadata.symlink?
  when :output_directory
    unless missing
      metadata = File.lstat(current)
      abort_with("#{label} must be a directory") unless metadata.directory? && !metadata.symlink?
    end
  else
    abort_with("unsupported path check")
  end
  current
end

def ordinary_file!(relative, label)
  checked_issue_path(relative, label, leaf: :file)
end

def digest(path)
  Digest::SHA256.file(path).hexdigest
end

def observed_cases(stdout)
  lines = stdout.lines.each_with_object([]) do |line, entries|
    next unless line.start_with?(MARKER)

    payload = JSON.parse(line.delete_suffix("\n").delete_prefix(MARKER))
    name = payload.fetch("case")
    result = payload.fetch("result")
    entries << { "case" => name, "result" => result, "observed_line_sha256" => Digest::SHA256.hexdigest(line.chomp) }
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
  producer = REPO_ROOT.join(".csdlc/evidence/121/review-v2/derive-negative-cases.rb")
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
when "test-path-safety"
  head, head_error, head_status = Open3.capture3("git", "rev-parse", "HEAD", chdir: REPO_ROOT.to_s)
  abort_with("cannot resolve source revision: #{head_error.strip}") unless head_status.success?
  issue_evidence_root = REPO_ROOT.join(".csdlc/evidence/#{ISSUE}")
  Dir.mktmpdir("path-safety-", issue_evidence_root.to_s) do |inside|
    Dir.mktmpdir("adl-121-outside-") do |outside|
      link = Pathname.new(inside).join("linked-outside")
      File.symlink(outside, link)
      outside_file = Pathname.new(outside).join("negative-cases.json")
      File.write(outside_file, "{}\n")

      checks = [
        ["produce", head.strip, link.join("new-proof").to_s],
        ["verify", link.join("negative-cases.json").to_s, head.strip]
      ]
      checks.each do |arguments|
        _stdout, stderr, status = Open3.capture3(RbConfig.ruby, __FILE__, *arguments, chdir: REPO_ROOT.to_s)
        abort_with("symlink-component regression unexpectedly passed") if status.success?
        abort_with("symlink-component regression returned the wrong diagnostic") unless stderr.include?("path contains a symlink component")
      end
    end
  end
  puts("PASS: producer and verifier reject intermediate symlink components")
when "produce"
  source = ARGV.shift.to_s
  output_dir = Pathname.new(ARGV.shift.to_s).expand_path
  abort_with("source revision must be exact") unless source.match?(/\A[0-9a-f]{40,64}\z/)
  head, head_error, head_status = Open3.capture3("git", "rev-parse", "HEAD", chdir: REPO_ROOT.to_s)
  abort_with("cannot resolve source revision: #{head_error.strip}") unless head_status.success?
  abort_with("producer must run at exact source revision") unless head.strip == source
  relative_dir = relative_repo_path(output_dir)
  checked_issue_path(relative_dir, "output directory", leaf: :output_directory)
  abort_with("output directory must be absent or empty") if output_dir.exist? && !output_dir.children.empty?
  FileUtils.mkdir_p(output_dir)
  checked_issue_path(relative_dir, "output directory", leaf: :output_directory)

  nextest_started_at = Time.now.utc.iso8601(6)
  nextest_stdout, nextest_stderr, nextest_status = Open3.capture3(
    { "CARGO_TERM_COLOR" => "never" }, *EXACT_NEXTEST, chdir: REPO_ROOT.to_s
  )
  nextest_finished_at = Time.now.utc.iso8601(6)
  nextest_log = output_dir.join("exact-child-tests.log")
  nextest_stdout_path = output_dir.join("exact-child-tests.stdout.log")
  nextest_stderr_path = output_dir.join("exact-child-tests.stderr.log")
  File.binwrite(nextest_log, nextest_stdout + nextest_stderr)
  File.binwrite(nextest_stdout_path, nextest_stdout)
  File.binwrite(nextest_stderr_path, nextest_stderr)
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
  stdout = stdout.sub(/\n+\z/, "\n")
  stderr = stderr.sub(/\n+\z/, "\n")
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
    "producer_path" => ".csdlc/evidence/121/review-v2/derive-negative-cases.rb",
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
when "test-contract"
  evidence_path = Pathname.new(ARGV.shift.to_s).expand_path
  source = ARGV.shift.to_s
  verify!(evidence_path, source)
  issue_evidence_root = REPO_ROOT.join(".csdlc/evidence/#{ISSUE}")
  Dir.mktmpdir("contract-tamper-", issue_evidence_root.to_s) do |temporary|
    temporary = Pathname.new(temporary)
    original_machine = JSON.parse(File.read(evidence_path))
    original_manifest = JSON.parse(File.read(evidence_path.dirname.join("validation-manifest.json")))

    checks = {
      "machine argv" => lambda do |machine, _manifest|
        machine.fetch("command")["argv"] = ["true"]
      end,
      "machine exit" => lambda do |machine, _manifest|
        machine.fetch("command")["exit_code"] = 1
      end,
      "nextest argv" => lambda do |_machine, manifest|
        manifest.fetch("commands").fetch(0)["argv"] = ["true"]
      end,
      "Clippy exit" => lambda do |_machine, manifest|
        manifest.fetch("commands").fetch(1)["exit_code"] = 1
      end,
      "validation digest" => lambda do |_machine, manifest|
        manifest.fetch("commands").fetch(0)["combined_log_sha256"] = "0" * 64
      end
    }
    checks.each do |label, mutation|
      machine = Marshal.load(Marshal.dump(original_machine))
      manifest = Marshal.load(Marshal.dump(original_manifest))
      mutation.call(machine, manifest)
      candidate = temporary.join("negative-cases.json")
      File.write(candidate, JSON.pretty_generate(machine) + "\n")
      File.write(temporary.join("validation-manifest.json"), JSON.pretty_generate(manifest) + "\n")
      _stdout, _stderr, status = Open3.capture3(
        RbConfig.ruby, __FILE__, "verify", candidate.to_s, source, chdir: REPO_ROOT.to_s
      )
      abort_with("#{label} tamper regression unexpectedly passed") if status.success?
    end
  end
  puts("PASS: machine command, status, manifest, and digest tampering fail closed")
else
  abort_with("usage: derive-negative-cases.rb produce SOURCE OUTPUT_DIR | verify EVIDENCE SOURCE | test-path-safety | test-contract EVIDENCE SOURCE")
end
