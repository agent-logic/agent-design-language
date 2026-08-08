#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "stringio"
require "tmpdir"
require "zlib"

SHA256 = /\A[0-9a-f]{64}\z/
ISSUE = 5820
REPAIR_ISSUE = 27
PLATFORMS = %w[linux macos windows].freeze
COMMON_ARTIFACT_ROLES = %w[
  guardian_binary
  kernel_binary
  canonical_init
  https_transcript
  wss_transcript
  lifecycle_report
  lifecycle_component_report
  runner_provenance
].freeze
LINUX_ARTIFACT_ROLES = (COMMON_ARTIFACT_ROLES + %w[
  aws_summary
  volume_deletion_receipt
]).freeze
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
POST_PROOF_EXACT_PATHS = %w[
  adl/tools/validate_v092_runtime_native_receipts.rb
  adl/tools/test_validate_v092_runtime_native_receipts.sh
].freeze
POST_PROOF_PATH_PREFIXES = [
  ".csdlc/issues/#{ISSUE}/",
  ".csdlc/prepared/issues/#{ISSUE}/",
  ".csdlc/issues/#{REPAIR_ISSUE}/",
  ".csdlc/prepared/issues/#{REPAIR_ISSUE}/"
].freeze
IGNORED_WORKTREE_STATUS_LINES = ["?? .csdlc/locks/#{REPAIR_ISSUE}.lock"].freeze

def same_denominator?(observed, expected)
  observed.sort == expected.sort
end

def unique_values?(values)
  values.uniq.length == values.length
end

def run_git!(repository, *args)
  output, status = Open3.capture2e("git", *args, chdir: repository)
  raise "git #{args.join(' ')} failed: #{output.strip}" unless status.success?
  output.strip
end

def changed_paths_between(repository, proof_revision, head_revision)
  _, ancestry_status = Open3.capture2e(
    "git", "merge-base", "--is-ancestor", proof_revision, head_revision,
    chdir: repository
  )
  raise "proof revision is not an ancestor of verifier revision" unless ancestry_status.success?

  changed, diff_status = Open3.capture2(
    "git", "diff", "--no-renames", "--name-only",
    "#{proof_revision}..#{head_revision}", "--",
    chdir: repository
  )
  raise "cannot compare proof and verifier revisions" unless diff_status.success?
  changed.lines.map(&:strip).reject(&:empty?)
end

def clean_worktree?(repository)
  status, git_status = Open3.capture2(
    "git", "status", "--porcelain", "--untracked-files=all",
    chdir: repository
  )
  return false unless git_status.success?

  status.lines.map(&:strip).reject(&:empty?).all? do |line|
    IGNORED_WORKTREE_STATUS_LINES.include?(line)
  end
end

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

def checked_artifact(artifact, label)
  if artifact["chunks"]
    chunks = Array(artifact["chunks"])
    abort "#{label} has no chunks" if chunks.empty?
    compressed = chunks.each_with_index.map do |chunk, index|
      resolved, = checked_file(
        chunk.fetch("path"),
        chunk.fetch("sha256"),
        "#{label} chunk #{index}"
      )
      File.binread(resolved)
    end.join
    archive_digest = Digest::SHA256.hexdigest(compressed)
    abort "#{label} aggregate digest mismatch" unless archive_digest == artifact.fetch("sha256")
    resolved = nil
  else
    resolved, archive_digest = checked_file(
      artifact.fetch("path"),
      artifact.fetch("sha256"),
      label
    )
    compressed = File.binread(resolved)
  end
  compression = artifact["compression"]
  return [resolved, archive_digest, archive_digest] if compression.nil?
  abort "#{label} uses unsupported compression" unless compression == "gzip"

  content_digest = Digest::SHA256.new
  Zlib::GzipReader.wrap(StringIO.new(compressed)) do |gzip|
    while (chunk = gzip.read(1024 * 1024))
      break if chunk.empty?
      content_digest.update(chunk)
    end
  end
  expected_content_digest = artifact.fetch("content_sha256")
  abort "#{label} content digest mismatch" unless content_digest.hexdigest == expected_content_digest
  [resolved, archive_digest, expected_content_digest]
end

def post_proof_change_allowed?(path)
  POST_PROOF_EXACT_PATHS.include?(path) ||
    POST_PROOF_PATH_PREFIXES.any? { |prefix| path.start_with?(prefix) }
end

if ARGV.first == "--self-test-policy"
  observed_roles = COMMON_ARTIFACT_ROLES.rotate(3)
  abort "artifact role comparison is order-sensitive" unless same_denominator?(observed_roles, COMMON_ARTIFACT_ROLES)
  abort "valid artifact roles are not unique" unless unique_values?(observed_roles)

  duplicate_roles = observed_roles.drop(1) + [observed_roles.first, observed_roles.first]
  abort "duplicate artifact roles were accepted" if unique_values?(duplicate_roles)
  abort "artifact role denominator accepted a duplicate/missing set" if same_denominator?(duplicate_roles, COMMON_ARTIFACT_ROLES)

  allowed = [
    "adl/tools/validate_v092_runtime_native_receipts.rb",
    "adl/tools/test_validate_v092_runtime_native_receipts.sh",
    ".csdlc/issues/5820/index.json",
    ".csdlc/prepared/issues/5820/record-final-review.json",
    ".csdlc/issues/27/index.json",
    ".csdlc/prepared/issues/27/design.md"
  ]
  rejected = [
    "adl-runtime/src/guardian.rs",
    "adl-runtime-kernel/src/config.rs",
    "infra/runtime-v3/runtime-init.toml",
    "adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
    "adl/tools/validate_v092_runtime_native_receipts.rb.bak",
    ".csdlc/issues/270/index.json",
    ".csdlc/evidence/5820/runtime-native-receipts.json",
    ".csdlc/evidence/27/validation.json",
    ".csdlc/evidence/19/deployment-manifest.json"
  ]
  abort "finalization allowlist rejected lifecycle evidence" unless allowed.all? { |path| post_proof_change_allowed?(path) }
  abort "finalization allowlist accepted Runtime product drift" unless rejected.none? { |path| post_proof_change_allowed?(path) }

  Dir.mktmpdir("native-receipt-policy") do |repository|
    run_git!(repository, "init", "-b", "main")
    run_git!(repository, "config", "user.email", "native-receipt-policy@example.invalid")
    run_git!(repository, "config", "user.name", "Native Receipt Policy")
    FileUtils.mkdir_p(File.join(repository, "adl-runtime/src"))
    FileUtils.mkdir_p(File.join(repository, "adl/tools"))
    File.write(File.join(repository, "adl-runtime/src/guardian.rs"), "product-v1\n")
    File.write(File.join(repository, POST_PROOF_EXACT_PATHS.first), "verifier-v1\n")
    run_git!(repository, "add", ".")
    run_git!(repository, "commit", "-m", "proof revision")
    proof = run_git!(repository, "rev-parse", "HEAD")

    File.write(File.join(repository, POST_PROOF_EXACT_PATHS.first), "verifier-v2\n")
    run_git!(repository, "add", POST_PROOF_EXACT_PATHS.first)
    run_git!(repository, "commit", "-m", "verifier repair")
    verifier = run_git!(repository, "rev-parse", "HEAD")
    verifier_paths = changed_paths_between(repository, proof, verifier)
    abort "Git verifier-only change was rejected" unless verifier_paths.all? { |path| post_proof_change_allowed?(path) }

    FileUtils.mkdir_p(File.join(repository, ".csdlc/locks"))
    File.write(File.join(repository, ".csdlc/locks/#{REPAIR_ISSUE}.lock"), "lifecycle lock\n")
    abort "exact lifecycle lock made validation worktree dirty" unless clean_worktree?(repository)

    File.write(File.join(repository, "adl-runtime/src/guardian.rs"), "product-v2\n")
    abort "dirty product worktree was accepted" if clean_worktree?(repository)
    run_git!(repository, "restore", "adl-runtime/src/guardian.rs")

    run_git!(repository, "switch", "--detach", proof)
    FileUtils.mkdir_p(File.join(repository, ".csdlc/issues/#{REPAIR_ISSUE}"))
    run_git!(repository, "mv", "adl-runtime/src/guardian.rs", ".csdlc/issues/#{REPAIR_ISSUE}/guardian.rs")
    run_git!(repository, "commit", "-m", "rename product into allowlist")
    renamed = run_git!(repository, "rev-parse", "HEAD")
    renamed_paths = changed_paths_between(repository, proof, renamed)
    abort "rename hid a Runtime product change" if renamed_paths.all? { |path| post_proof_change_allowed?(path) }

    run_git!(repository, "checkout", "--orphan", "unrelated")
    run_git!(repository, "rm", "-rf", ".")
    File.write(File.join(repository, "unrelated.txt"), "unrelated\n")
    run_git!(repository, "add", "unrelated.txt")
    run_git!(repository, "commit", "-m", "unrelated revision")
    unrelated = run_git!(repository, "rev-parse", "HEAD")
    begin
      changed_paths_between(repository, proof, unrelated)
      abort "non-ancestor verifier revision was accepted"
    rescue RuntimeError => error
      abort error.message unless error.message == "proof revision is not an ancestor of verifier revision"
    end
  end

  puts "PASS: native receipt role and finalization policies"
  exit 0
end

packet_argument = ARGV.fetch(0, ".csdlc/evidence/#{ISSUE}/runtime-native-receipts.json")
packet_path = contained_issue_file(packet_argument, "native receipt packet")
packet = JSON.parse(File.read(packet_path))
abort "wrong schema" unless packet["schema"] == "adl.runtime_guardian_native_receipts.v3"

head, status = Open3.capture2("git", "rev-parse", "HEAD")
abort "cannot resolve HEAD" unless status.success?
head = head.strip
proof_revision = packet["source_revision"].to_s
abort "invalid proof revision" unless proof_revision.match?(/\A[0-9a-f]{40}\z/)
abort "native receipt validation requires a clean worktree" unless clean_worktree?(REPOSITORY_ROOT)
unless proof_revision == head
  changed_paths = begin
    changed_paths_between(REPOSITORY_ROOT, proof_revision, head)
  rescue RuntimeError => error
    abort error.message
  end
  abort "runtime product changed after native proof" unless changed_paths.all? { |path| post_proof_change_allowed?(path) }
end

receipts = Array(packet["receipts"])
blockers = Array(packet["blockers"])
covered_platforms = receipts.map { |entry| entry["platform"] } + blockers.map { |entry| entry["platform"] }
abort "platform denominator drift" unless same_denominator?(covered_platforms, PLATFORMS)
abort "platform entries must be unique" unless unique_values?(covered_platforms)
abort "production receipts must cover macOS and Linux" unless same_denominator?(receipts.map { |entry| entry["platform"] }, %w[linux macos])
abort "only native Windows may be blocked" unless blockers.map { |entry| entry["platform"] } == ["windows"]

blockers.each do |blocker|
  platform = blocker.fetch("platform")
  abort "wrong #{platform} blocker schema" unless blocker["schema"] == "adl.runtime_v3.platform_blocker.v1"
  abort "stale #{platform} blocker" unless blocker["source_revision"] == proof_revision
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
  abort "stale #{platform} receipt" unless receipt["source_revision"] == proof_revision

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
  expected_roles = platform == "linux" ? LINUX_ARTIFACT_ROLES : COMMON_ARTIFACT_ROLES
  abort "#{platform} artifact role denominator drift" unless same_denominator?(roles, expected_roles)
  abort "#{platform} artifact roles must be unique" unless unique_values?(roles)
  artifacts_by_role = artifacts.to_h do |artifact|
    role = artifact.fetch("role")
    resolved, digest, content_digest = checked_artifact(artifact, "#{platform} #{role}")
    [role, {
      "path" => artifact["path"] || artifact.fetch("chunks").map { |chunk| chunk.fetch("path") }.join(","),
      "resolved" => resolved,
      "sha256" => digest,
      "content_sha256" => content_digest
    }]
  end

  provenance = JSON.parse(File.read(artifacts_by_role.fetch("runner_provenance").fetch("resolved")))
  abort "#{platform} runner provenance schema mismatch" unless provenance["schema"] == "adl.runtime_guardian.runner_provenance.v1"
  abort "#{platform} runner provenance revision drift" unless provenance["source_revision"] == proof_revision
  %w[provider run_id os arch].each do |field|
    abort "#{platform} runner provenance #{field} mismatch" unless provenance[field] == runner[field]
  end
  abort "#{platform} runner identity digest mismatch" unless runner["identity_sha256"] == artifacts_by_role.fetch("runner_provenance").fetch("sha256")

  lifecycle = JSON.parse(File.read(artifacts_by_role.fetch("lifecycle_report").fetch("resolved")))
  abort "#{platform} lifecycle report schema mismatch" unless lifecycle["schema"] == "adl.runtime_v3.guardian_lifecycle_proof.v1"
  abort "#{platform} lifecycle report failed" unless lifecycle["status"] == "pass" && lifecycle["acceptance_eligible"] == true
  abort "#{platform} lifecycle report revision drift" unless lifecycle["source_revision"] == proof_revision
  {
    "guardian_binary_sha256" => "guardian_binary",
    "kernel_binary_sha256" => "kernel_binary",
    "canonical_init_sha256" => "canonical_init",
    "https_transcript_sha256" => "https_transcript",
    "wss_transcript_sha256" => "wss_transcript"
  }.each do |field, role|
    abort "#{platform} lifecycle #{role} digest mismatch" unless lifecycle[field] == artifacts_by_role.fetch(role).fetch("content_sha256")
  end
  ASSERTION_ROLES.each_key do |name|
    abort "#{platform} lifecycle report did not prove #{name}" unless lifecycle.dig("assertions", name) == true
  end

  component = JSON.parse(
    File.read(artifacts_by_role.fetch("lifecycle_component_report").fetch("resolved"))
  )
  abort "#{platform} component report digest mismatch" unless lifecycle["lifecycle_report_sha256"] == artifacts_by_role.fetch("lifecycle_component_report").fetch("sha256")
  abort "#{platform} component report schema mismatch" unless component["schema"] == "adl.runtime_v3.lifecycle_soak.v1"
  abort "#{platform} component report failed" unless component["status"] == "pass"
  abort "#{platform} component suite mismatch" unless component["suite"] == "stress_100x10s"
  abort "#{platform} component report is not acceptance eligible" unless component["acceptance_eligible"] == true
  abort "#{platform} component revision drift" unless component["revision"] == proof_revision
  abort "#{platform} component platform mismatch" unless component["platform"] == platform
  abort "#{platform} component run denominator mismatch" unless component["requested_runs"] == 100 && component["completed_runs"] == 100
  abort "#{platform} component duration mismatch" unless component["duration_seconds_per_run"] == 10
  completed_cycles = component["completed_cycles"].to_i
  abort "#{platform} component cycle minimum failed" unless completed_cycles >= component["minimum_cycles_per_run"].to_i * component["completed_runs"].to_i
  abort "#{platform} continuity mismatch" unless component["continuity_generation"] == completed_cycles
  abort "#{platform} Guardian count mismatch" unless component["guardian_launch_count"] == completed_cycles && component["guardian_process_count"] == completed_cycles
  abort "#{platform} Runtime start count mismatch" unless component["runtime_start_count"] == completed_cycles + 1 && component["runtime_instance_count"] == completed_cycles + 1
  abort "#{platform} restart proof mismatch" unless component["total_restarts"] == 1 && component["restart_budget_exercised"] == true
  abort "#{platform} anti-rollback proof missing" unless component["anti_rollback_minimum_enforced"] == true
  abort "#{platform} log continuity mismatch" unless component["log_checked_cycles"] == completed_cycles
  abort "#{platform} logging proof failed" unless component["logging_complete"] == true && component["master_log_status"] == "clean"
  abort "#{platform} component kernel digest mismatch" unless component["kernel_sha256"] == artifacts_by_role.fetch("kernel_binary").fetch("content_sha256")

  if platform == "linux"
    aws = JSON.parse(File.read(artifacts_by_role.fetch("aws_summary").fetch("resolved")))
    abort "linux AWS summary schema mismatch" unless aws["schema_version"] == "adl.aws_remote_validation_run.v1"
    abort "linux AWS run failed" unless aws["status"] == "passed" && aws["issue"] == ISSUE
    abort "linux AWS runner mismatch" unless aws["run_id"] == runner["run_id"]
    abort "linux AWS revision drift" unless aws.dig("remote_summary", "resolved_commit") == proof_revision
    abort "linux AWS command failed" unless aws.dig("command", "status") == "Success" && aws.dig("command", "response_code") == 0
    abort "linux AWS run was not Spot" unless aws.dig("launch", "purchase_option") == "spot"
    abort "linux AWS instance teardown failed" unless aws.dig("cleanup", "final_instance_state") == "terminated" && aws.dig("cleanup", "termination_error").nil?
    abort "linux AWS launch surface teardown failed" unless %w[instance_profile_deleted role_deleted security_group_deleted].all? { |field| aws.dig("launch_surface_cleanup", field) == true }

    deletion = JSON.parse(
      File.read(artifacts_by_role.fetch("volume_deletion_receipt").fetch("resolved"))
    )
    abort "linux volume deletion schema mismatch" unless deletion["schema"] == "adl.aws_volume_deletion_receipt.v1"
    abort "linux volume deletion was not verified" unless deletion["deleted"] == true && deletion["observation"] == "InvalidVolume.NotFound"
    abort "linux volume region mismatch" unless deletion["region"] == aws["region"]
    abort "linux volume identity mismatch" unless deletion["volume_id_sha256"] == aws.dig("cache_volume", "volume_id", "sha256")
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
