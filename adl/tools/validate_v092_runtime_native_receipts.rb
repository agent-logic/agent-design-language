#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

SHA256 = /\A[0-9a-f]{64}\z/
ISSUE = 5820
PLATFORMS = %w[linux macos windows].freeze
ARTIFACT_ROLES = %w[
  guardian_binary
  kernel_binary
  canonical_init
  https_transcript
  wss_transcript
  lifecycle_report
  runner_provenance
].freeze
ASSERTION_ROLES = {
  "guardian_launched" => "lifecycle_report",
  "kernel_ready" => "lifecycle_report",
  "authenticated_https" => "https_transcript",
  "authenticated_wss" => "wss_transcript",
  "child_killed" => "lifecycle_report",
  "bounded_restart" => "lifecycle_report",
  "state_preserved" => "lifecycle_report",
  "clean_shutdown" => "lifecycle_report",
  "clean_logs" => "lifecycle_report"
}.freeze

def repository_root
  root, status = Open3.capture2("git", "rev-parse", "--show-toplevel")
  abort "cannot resolve repository root" unless status.success?
  root = root.strip
  abort "repository root must not be a symlink" if File.lstat(root).symlink?
  File.realpath(root)
end

REPOSITORY_ROOT = repository_root
ISSUE_ROOT = File.join(REPOSITORY_ROOT, ".csdlc", "evidence", ISSUE.to_s)

def contained_issue_file(path, label, allow_empty: false)
  pathname = Pathname.new(path.to_s)
  abort "#{label} path must be repository-relative" if pathname.absolute?
  abort "#{label} path contains traversal" if pathname.each_filename.any? { |part| part == ".." }
  expanded = File.expand_path(pathname.to_s, REPOSITORY_ROOT)
  prefix = ISSUE_ROOT + File::SEPARATOR
  abort "#{label} path must be issue-local" unless expanded.start_with?(prefix)

  current = REPOSITORY_ROOT
  Pathname.new(expanded.delete_prefix(REPOSITORY_ROOT + File::SEPARATOR)).each_filename do |part|
    current = File.join(current, part)
    abort "#{label} path traverses a symlink" if File.symlink?(current)
  end
  abort "missing #{label}: #{path}" unless File.file?(expanded)
  resolved = File.realpath(expanded)
  abort "#{label} resolves outside issue evidence" unless resolved.start_with?(prefix)
  abort "empty #{label}: #{path}" if !allow_empty && File.zero?(resolved)
  resolved
end

def checked_file(path, digest, label, allow_empty: false)
  resolved = contained_issue_file(path, label, allow_empty: allow_empty)
  abort "invalid #{label} digest" unless digest.to_s.match?(SHA256)
  actual = Digest::SHA256.file(resolved).hexdigest
  abort "#{label} digest mismatch" unless actual == digest
  [resolved, actual]
end

packet_argument = ARGV.fetch(0, ".csdlc/evidence/#{ISSUE}/runtime-native-receipts.json")
packet_path = contained_issue_file(packet_argument, "native receipt packet")
packet = JSON.parse(File.read(packet_path))
abort "wrong schema" unless packet["schema"] == "adl.runtime_guardian_native_receipts.v3"

head, status = Open3.capture2("git", "rev-parse", "HEAD")
abort "cannot resolve HEAD" unless status.success?
head = head.strip
abort "stale packet" unless packet["source_revision"] == head

receipts = Array(packet["receipts"])
blockers = Array(packet["blockers"])
covered_platforms = receipts.map { |entry| entry["platform"] } + blockers.map { |entry| entry["platform"] }
abort "platform denominator drift" unless covered_platforms.sort == PLATFORMS
abort "platform entries must be unique" unless covered_platforms.uniq.length == covered_platforms.length
abort "production receipts must cover macOS and Linux" unless receipts.map { |entry| entry["platform"] }.sort == %w[linux macos]
abort "only native Windows may be blocked" unless blockers.map { |entry| entry["platform"] } == ["windows"]

blockers.each do |blocker|
  platform = blocker.fetch("platform")
  abort "wrong #{platform} blocker schema" unless blocker["schema"] == "adl.runtime_v3.platform_blocker.v1"
  abort "stale #{platform} blocker" unless blocker["source_revision"] == head
  abort "#{platform} blocker did not fail closed" unless blocker["status"] == "blocked"
  abort "missing #{platform} blocker reason" if blocker["reason"].to_s.empty?
  abort "missing #{platform} blocker authority" if blocker["unavailable_authority"].to_s.empty?
  checked_file(
    blocker.fetch("evidence_path"),
    blocker.fetch("evidence_sha256"),
    "#{platform} blocker evidence"
  )
end

receipts.each do |receipt|
  platform = receipt.fetch("platform")
  abort "stale #{platform} receipt" unless receipt["source_revision"] == head

  runner = receipt.fetch("runner")
  %w[provider run_id os arch].each do |field|
    abort "missing #{platform} runner #{field}" if runner[field].to_s.empty?
  end
  abort "runner OS does not match #{platform}" unless runner["os"] == platform
  abort "invalid #{platform} runner identity" unless runner["identity_sha256"].to_s.match?(SHA256)

  command = receipt.fetch("command")
  expected_argv = [
    "bash",
    "adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
    "--suite",
    "stress_100x10s"
  ]
  abort "wrong #{platform} producer command" unless Array(command["argv"]) == expected_argv
  abort "#{platform} producer failed" unless command["exit_code"] == 0
  checked_file(command["stdout_path"], command["stdout_sha256"], "#{platform} stdout")
  checked_file(command["stderr_path"], command["stderr_sha256"], "#{platform} stderr", allow_empty: true)

  artifacts = Array(receipt["artifacts"])
  roles = artifacts.map { |artifact| artifact["role"] }
  abort "#{platform} artifact role denominator drift" unless roles.sort == ARTIFACT_ROLES
  abort "#{platform} artifact roles must be unique" unless roles.uniq.length == roles.length
  artifacts_by_role = artifacts.to_h do |artifact|
    role = artifact.fetch("role")
    resolved, digest = checked_file(artifact.fetch("path"), artifact.fetch("sha256"), "#{platform} #{role}")
    [role, {"path" => artifact.fetch("path"), "resolved" => resolved, "sha256" => digest}]
  end

  provenance = JSON.parse(File.read(artifacts_by_role.fetch("runner_provenance").fetch("resolved")))
  abort "#{platform} runner provenance schema mismatch" unless provenance["schema"] == "adl.runtime_guardian.runner_provenance.v1"
  abort "#{platform} runner provenance revision drift" unless provenance["source_revision"] == head
  %w[provider run_id os arch].each do |field|
    abort "#{platform} runner provenance #{field} mismatch" unless provenance[field] == runner[field]
  end
  abort "#{platform} runner identity digest mismatch" unless runner["identity_sha256"] == artifacts_by_role.fetch("runner_provenance").fetch("sha256")

  lifecycle = JSON.parse(File.read(artifacts_by_role.fetch("lifecycle_report").fetch("resolved")))
  abort "#{platform} lifecycle report schema mismatch" unless lifecycle["schema"] == "adl.runtime_v3.guardian_lifecycle_proof.v1"
  abort "#{platform} lifecycle report failed" unless lifecycle["status"] == "pass" && lifecycle["acceptance_eligible"] == true
  abort "#{platform} lifecycle report revision drift" unless lifecycle["source_revision"] == head
  {
    "guardian_binary_sha256" => "guardian_binary",
    "kernel_binary_sha256" => "kernel_binary",
    "canonical_init_sha256" => "canonical_init",
    "https_transcript_sha256" => "https_transcript",
    "wss_transcript_sha256" => "wss_transcript"
  }.each do |field, role|
    abort "#{platform} lifecycle #{role} digest mismatch" unless lifecycle[field] == artifacts_by_role.fetch(role).fetch("sha256")
  end
  ASSERTION_ROLES.each_key do |name|
    abort "#{platform} lifecycle report did not prove #{name}" unless lifecycle.dig("assertions", name) == true
  end

  canonical_init = File.read(artifacts_by_role.fetch("canonical_init").fetch("resolved"))
  abort "#{platform} canonical init schema missing" unless canonical_init.match?(/^schema = "adl\.runtime_v3\.init\.v1"$/)
  https_text = File.read(artifacts_by_role.fetch("https_transcript").fetch("resolved"))
  abort "#{platform} HTTPS transcript leaked authorization" if https_text.match?(/authorization:\s*bearer/i)
  https = JSON.parse(https_text)
  abort "#{platform} HTTPS transcript schema mismatch" unless https["schema"] == "adl.runtime_v3.guardian_https_transcript.v1"
  abort "#{platform} HTTPS transcript did not prove authentication" unless https.dig("request", "authentication") == "bearer_redacted" && https.dig("response", "status") == 200

  wss_text = File.read(artifacts_by_role.fetch("wss_transcript").fetch("resolved"))
  abort "#{platform} WSS transcript leaked authorization" if wss_text.match?(/authorization:\s*bearer/i)
  wss = JSON.parse(wss_text)
  abort "#{platform} WSS transcript schema mismatch" unless wss["schema"] == "adl.runtime_v3.guardian_wss_transcript.v1"
  abort "#{platform} WSS transcript did not prove authenticated upgrade" unless wss.dig("request", "authentication") == "bearer_redacted" && wss.dig("upgrade", "status") == 101
  abort "#{platform} WSS transcript did not prove bounded completion" unless wss.dig("bounded_request", "bytes").to_i.between?(1, 65_536) && wss.dig("response", "status") == "completed"

  assertions = Array(receipt["assertions"])
  abort "#{platform} assertion denominator drift" unless assertions.map { |entry| entry["name"] }.sort == ASSERTION_ROLES.keys.sort
  abort "#{platform} assertion names must be unique" unless assertions.map { |entry| entry["name"] }.uniq.length == assertions.length
  assertions.each do |assertion|
    name = assertion.fetch("name")
    expected_role = ASSERTION_ROLES.fetch(name)
    abort "#{platform} did not prove #{name}" unless assertion["result"] == "passed"
    abort "#{platform} #{name} evidence role mismatch" unless assertion["evidence_role"] == expected_role
    artifact = artifacts_by_role.fetch(expected_role)
    abort "#{platform} #{name} evidence path mismatch" unless assertion["evidence_path"] == artifact["path"]
    abort "#{platform} #{name} evidence digest mismatch" unless assertion["evidence_sha256"] == artifact["sha256"]
  end
end

abort "native runner runs are not distinct" unless receipts.map { |receipt| receipt.dig("runner", "run_id") }.uniq.length == receipts.length
abort "native runner identities are not distinct" unless receipts.map { |receipt| receipt.dig("runner", "identity_sha256") }.uniq.length == receipts.length
puts "PASS: exact-head named production Guardian artifacts on macOS and Linux; native Windows remains explicitly blocked"
