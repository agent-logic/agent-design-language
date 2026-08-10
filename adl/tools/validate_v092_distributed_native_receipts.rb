#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "time"

ISSUE = 5878
PLATFORMS = %w[linux macos windows].freeze
SHA256 = /\A[0-9a-f]{64}\z/
MAX_FILE_BYTES = 4 * 1024 * 1024
EXPECTED_ROLES = %w[integration_stdout integration_stderr runner_provenance].freeze
EXPECTED_ARGV = ["bash", "adl/tools/validate_v092_distributed_guardian.sh"].freeze

def abort_with(message)
  warn(message)
  exit(1)
end

def repository_root
  root, status = Open3.capture2("git", "rev-parse", "--show-toplevel")
  abort_with("cannot resolve repository root") unless status.success?
  File.realpath(root.strip)
end

ROOT = repository_root
EVIDENCE = File.join(ROOT, ".csdlc", "evidence", ISSUE.to_s)

def checked_path(relative, label, allow_empty: false)
  path = Pathname.new(relative.to_s)
  abort_with("#{label} path must be repository-relative") if path.absolute?
  abort_with("#{label} path contains traversal") if path.each_filename.any? { |part| part == ".." }
  expanded = File.expand_path(path.to_s, ROOT)
  prefix = EVIDENCE + File::SEPARATOR
  abort_with("#{label} escapes issue evidence") unless expanded.start_with?(prefix)
  current = ROOT
  Pathname.new(expanded.delete_prefix(ROOT + File::SEPARATOR)).each_filename do |part|
    current = File.join(current, part)
    abort_with("#{label} traverses a symlink") if File.symlink?(current)
  end
  abort_with("missing #{label}") unless File.file?(expanded)
  size = File.size(expanded)
  abort_with("empty #{label}") if !allow_empty && size.zero?
  abort_with("#{label} exceeds hard bound") if size > MAX_FILE_BYTES
  expanded
end

def checked_digest(relative, expected, label, allow_empty: false)
  abort_with("invalid #{label} digest") unless expected.to_s.match?(SHA256)
  path = checked_path(relative, label, allow_empty: allow_empty)
  abort_with("#{label} digest mismatch") unless Digest::SHA256.file(path).hexdigest == expected
  path
end

def canonical_runner_digest(runner)
  value = runner.reject { |key, _| key == "identity_sha256" }
  Digest::SHA256.hexdigest(JSON.generate(value.sort.to_h))
end

def validate_timestamp(value, label)
  Time.iso8601(value.to_s)
rescue ArgumentError
  abort_with("invalid #{label} timestamp")
end

def expected_host_fragment(platform)
  { "macos" => "apple-darwin", "linux" => "unknown-linux", "windows" => "pc-windows" }.fetch(platform)
end

def validate_receipt(path, platform, revision)
  receipt = JSON.parse(File.read(checked_path(path, "#{platform} receipt")))
  abort_with("wrong #{platform} receipt schema") unless receipt["schema"] == "adl.distributed_guardian.native_receipt.v1"
  abort_with("wrong #{platform} issue") unless receipt["issue"] == ISSUE
  abort_with("#{platform} receipt label mismatch") unless receipt["platform"] == platform
  abort_with("stale #{platform} receipt") unless receipt["source_revision"] == revision

  command = receipt.fetch("command")
  abort_with("wrong #{platform} command") unless command["argv"] == EXPECTED_ARGV
  abort_with("#{platform} command failed") unless command["exit_code"] == 0
  abort_with("#{platform} selected zero tests") unless command["selected_tests"].to_i.positive?
  started = validate_timestamp(command["started_at"], "#{platform} start")
  finished = validate_timestamp(command["finished_at"], "#{platform} finish")
  abort_with("#{platform} command interval is inverted") if finished < started

  stdout = checked_digest(command["stdout_path"], command["stdout_sha256"], "#{platform} stdout")
  checked_digest(command["stderr_path"], command["stderr_sha256"], "#{platform} stderr", allow_empty: true)
  output = File.read(stdout)
  abort_with("#{platform} output lacks test completion") unless output.match?(/\btests? run\b/)
  cases = output.scan(/ADL_ISSUE_5878_NEGATIVE_CASE_V1\s+([a-z0-9_]+)/).flatten.uniq.sort
  abort_with("#{platform} negative cases are not producer-derived") unless cases == Array(receipt["negative_cases"]).sort && !cases.empty?

  runner = command.fetch("runner")
  %w[provider run_id os arch commit provenance_path provenance_sha256 identity_sha256].each do |field|
    abort_with("missing #{platform} runner #{field}") if runner[field].to_s.empty?
  end
  abort_with("#{platform} runner OS mismatch") unless runner["os"] == platform
  abort_with("#{platform} runner revision mismatch") unless runner["commit"] == revision
  abort_with("#{platform} runner identity mismatch") unless runner["identity_sha256"] == canonical_runner_digest(runner)
  if runner["provider"] == "github_actions"
    %w[repository workflow_ref run_attempt].each do |field|
      abort_with("missing hosted #{platform} runner #{field}") if runner[field].to_s.empty?
    end
  elsif runner["provider"] != "local_native"
    abort_with("unsupported #{platform} runner provider")
  end

  provenance_path = checked_digest(
    runner["provenance_path"], runner["provenance_sha256"], "#{platform} runner provenance"
  )
  provenance = JSON.parse(File.read(provenance_path))
  abort_with("wrong #{platform} provenance schema") unless provenance["schema"] == "adl.distributed_guardian.runner_provenance.v1"
  %w[provider run_id os arch].each do |field|
    abort_with("#{platform} provenance #{field} mismatch") unless provenance[field] == runner[field]
  end
  abort_with("#{platform} provenance revision mismatch") unless provenance["source_revision"] == revision
  abort_with("#{platform} receipt is not native") unless provenance["rustc_host"].to_s.include?(expected_host_fragment(platform))

  artifacts = Array(receipt["artifacts"])
  roles = artifacts.map { |artifact| artifact["role"] }
  abort_with("#{platform} artifact denominator mismatch") unless roles.sort == EXPECTED_ROLES.sort && roles.uniq.length == roles.length
  artifacts.each do |artifact|
    checked_digest(
      artifact.fetch("path"), artifact.fetch("sha256"),
      "#{platform} #{artifact.fetch('role')}", allow_empty: artifact["role"] == "integration_stderr"
    )
  end
  true
end

if ARGV == ["--self-test"]
  abort_with("platform denominator invariant failed") unless PLATFORMS.sort == %w[linux macos windows]
  abort_with("artifact denominator contains duplicates") unless EXPECTED_ROLES.uniq.length == EXPECTED_ROLES.length
  abort_with("unsafe path policy") if Pathname.new("../escape").each_filename.none? { |part| part == ".." }
  puts "PASS: distributed native receipt policy self-test"
  exit(0)
end

revision, status = Open3.capture2("git", "rev-parse", "HEAD")
abort_with("cannot resolve source revision") unless status.success?
revision = revision.strip
paths = PLATFORMS.map { |platform| ".csdlc/evidence/#{ISSUE}/native/#{platform}/receipt.json" }
paths.each_with_index { |path, index| validate_receipt(path, PLATFORMS.fetch(index), revision) }

receipts = paths.map { |path| JSON.parse(File.read(File.join(ROOT, path))) }
run_ids = receipts.map { |receipt| receipt.dig("command", "runner", "run_id") }
identities = receipts.map { |receipt| receipt.dig("command", "runner", "identity_sha256") }
abort_with("native run identifiers are not distinct") unless run_ids.uniq.length == PLATFORMS.length
abort_with("native runner identities are not distinct") unless identities.uniq.length == PLATFORMS.length
puts "PASS: macOS, Linux, and Windows distributed Guardian native receipts"
