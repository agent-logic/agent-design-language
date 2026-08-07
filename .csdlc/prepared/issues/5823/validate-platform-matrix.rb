#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

HEX40 = /\A[0-9a-f]{40}\z/
HEX64 = /\A[0-9a-f]{64}\z/
POST_VALIDATION_TRUTH_PATHS = [
  ".csdlc/evidence/5823/",
  ".csdlc/issues/5823/",
  ".csdlc/prepared/issues/5823/publish-final.json",
  ".csdlc/publication/5823.intent.json",
].freeze

def post_validation_truth_path?(path)
  POST_VALIDATION_TRUTH_PATHS.any? do |allowed|
    allowed.end_with?("/") ? path.start_with?(allowed) : path == allowed
  end
end

def read_json(path)
  raise("missing receipt #{path}") unless path.file? && !path.zero?

  JSON.parse(path.read)
rescue JSON::ParserError => error
  raise("malformed JSON #{path}: #{error.message}")
end

def repo_relative_path(root, value)
  path = Pathname.new(value.to_s)
  raise("absolute or escaping evidence path #{value}") if path.absolute? || path.each_filename.include?("..")

  root.join(path).cleanpath
end

def verify_git_revision(root, revision, label)
  raise("#{label} invalid Git revision") unless HEX40.match?(revision.to_s)
  _out, _err, exists = Open3.capture3("git", "-C", root.to_s, "cat-file", "-e", "#{revision}^{commit}")
  raise("#{label} revision does not exist in Git") unless exists.success?
  _out, _err, ancestor = Open3.capture3(
    "git", "-C", root.to_s, "merge-base", "--is-ancestor", revision, "HEAD"
  )
  raise("#{label} revision is not an ancestor of HEAD") unless ancestor.success?
end

def verify_validation_revision(root, revision)
  verify_git_revision(root, revision, "deterministic validation")
  changed, error, status = Open3.capture3(
    "git", "-C", root.to_s, "diff", "--name-only", "#{revision}..HEAD"
  )
  raise("failed to inspect deterministic validation drift: #{error}") unless status.success?

  unexpected = changed.lines.map(&:strip).reject(&:empty?).reject { |path| post_validation_truth_path?(path) }
  unless unexpected.empty?
    raise("deterministic validation revision is stale for: #{unexpected.join(', ')}")
  end
  dirty_paths = []
  [
    ["diff", "--no-renames", "--name-only"],
    ["diff", "--cached", "--no-renames", "--name-only"],
    ["ls-files", "--others", "--exclude-standard"],
  ].each do |arguments|
    output, error, status = Open3.capture3("git", "-C", root.to_s, *arguments)
    raise("failed to inspect deterministic validation worktree: #{error}") unless status.success?
    dirty_paths.concat(output.lines.map(&:strip).reject(&:empty?))
  end
  dirty_paths = dirty_paths.uniq.reject { |path| post_validation_truth_path?(path) }
  unless dirty_paths.empty?
    raise("deterministic validation worktree is dirty for: #{dirty_paths.join(', ')}")
  end
end

def machine_local_absolute_path?(text)
  text.match?(%r{/(?:Users|Volumes|private|home|var/folders)/})
end

def verify_receipt(root, value, digest, label)
  raise("#{label} missing receipt digest") unless HEX64.match?(digest.to_s)
  path = repo_relative_path(root, value)
  raise("#{label} missing receipt #{value}") unless path.file?
  raise("#{label} receipt SHA-256 mismatch") unless Digest::SHA256.file(path).hexdigest == digest

  text = path.read
  if machine_local_absolute_path?(text)
    raise("#{label} retained a machine-local absolute path")
  end
  path
end

def validate_deterministic_manifest(root)
  manifest_path = root.join(".csdlc/evidence/5823/deterministic-validation-summary.json")
  manifest = read_json(manifest_path)
  raise("deterministic manifest schema mismatch") unless manifest["schema"] == "adl.wp06.deterministic_validation_summary.v1"
  raise("deterministic manifest storage policy mismatch") unless manifest["storage_policy"] == "external_volume_only"
  verify_git_revision(root, manifest["subject_revision"], "deterministic manifest subject")
  verify_validation_revision(root, manifest["validation_revision"])
  lanes = manifest.fetch("lanes")
  raise("deterministic manifest has duplicate lane names") unless lanes.map { |lane| lane["name"] }.uniq.length == lanes.length
  expected_lanes = %w[
    portable-contract
    aws-portable-adapter
    nessus-shell-adapter
    aws-shell-adapter
    platform-validator-negative-self-test
    macos-native-local
    windows-deterministic-fixture
    linux-native-live
  ]
  raise("deterministic manifest lane set mismatch") unless lanes.map { |lane| lane["name"] }.sort == expected_lanes.sort
  lanes.each do |lane|
    name = lane.fetch("name")
    raise("#{name} outcome is not proving") unless %w[passed passed_non_native].include?(lane["outcome"])
    receipt_path = verify_receipt(root, lane.fetch("receipt"), lane["receipt_sha256"], name)
    if lane.key?("tests_passed")
      observed = receipt_path.read.scan(/running (\d+) tests/).flatten.map(&:to_i).last
      raise("#{name} test count mismatch") unless observed == lane["tests_passed"]
    end
    lane.each do |key, value|
      next unless key.end_with?("_receipt")

      verify_receipt(root, value, lane["#{key}_sha256"], "#{name} #{key}")
    end
  end
  linux = lanes.find { |lane| lane["name"] == "linux-native-live" }
  request = read_json(root.join(".csdlc/evidence/5823/linux-native-request.json"))
  provider = read_json(root.join(".csdlc/evidence/5823/linux-native-remote-summary.json"))
  raise("Linux manifest is not marked as live paid proof") unless manifest["live_paid_provider_work_run"] == true
  raise("Linux manifest revision mismatch") unless manifest["subject_revision"] == request["revision"]
  unless linux["request_timeout_seconds"] == request.dig("resource_budget", "timeout_seconds") &&
         linux["request_timeout_seconds"] == provider.dig("request_limits", "timeout_seconds")
    raise("Linux manifest timeout mismatch")
  end
  unless (linux["request_max_cost_usd"].to_f * 1_000_000).round == request.dig("resource_budget", "estimated_max_cost_microusd") &&
         linux["request_max_cost_usd"].to_f == provider.dig("request_limits", "estimated_max_cost_usd").to_f
    raise("Linux manifest cost ceiling mismatch")
  end
  unless linux["estimated_compute_cost_usd"].to_f == provider.dig("cost", "estimated_compute_cost_usd").to_f
    raise("Linux manifest observed cost mismatch")
  end
  unless linux["cleanup_status"] == provider.dig("cleanup", "explicit_post_run_status")
    raise("Linux manifest cleanup mismatch")
  end
  manifest
end

def validate_result(root, platform, row)
  %w[revision command_profile_digest result_digest receipt outcome].each do |field|
    raise("#{platform} missing #{field}") if row[field].to_s.empty?
  end
  raise("#{platform} invalid revision") unless HEX40.match?(row["revision"])
  verify_git_revision(root, row["revision"], "#{platform} proof")
  raise("#{platform} invalid profile digest") unless HEX64.match?(row["command_profile_digest"])
  raise("#{platform} invalid result digest") unless HEX64.match?(row["result_digest"])
  raise("#{platform} failed") unless row["outcome"] == "passed"

  result_path = root.join(row["receipt"])
  request_path = Pathname.new(result_path.to_s.sub(/-result\.json\z/, "-request.json"))
  request = read_json(request_path)
  result = read_json(result_path)
  actual_digest = Digest::SHA256.file(result_path).hexdigest
  raise("#{platform} result SHA-256 mismatch") unless actual_digest == row["result_digest"]
  raise("#{platform} request/result id mismatch") unless request["request_id"] == result["request_id"]
  raise("#{platform} revision mismatch") unless [request["revision"], result["revision"]].uniq == [row["revision"]]
  unless [request["command_profile_digest"], result["command_profile_digest"]].uniq == [row["command_profile_digest"]]
    raise("#{platform} command profile digest mismatch")
  end
  %w[resource_budget artifact_policy cancellation_file].each do |field|
    raise("#{platform} did not preserve #{field}") unless request[field] == result[field]
  end
  raise("#{platform} adapter mismatch") unless request["adapter"] == result["adapter"]
  raise("#{platform} platform mismatch") unless result.dig("platform", "os") == request["requested_platform"]
  raise("#{platform} qualification mismatch") unless result.dig("platform", "qualification") == row["qualification"]
  raise("#{platform} native marker mismatch") unless result.dig("platform", "native") == row["native"]
  raise("#{platform} redaction failed") unless result["redaction_passed"] == true
  raise("#{platform} cleanup incomplete") unless result.dig("cleanup", "complete") == true
  raise("#{platform} fallback policy mismatch") unless result.dig("fallback", "policy") == request["fallback"]
  raise("#{platform} fallback result overclaims execution") if result.dig("fallback", "ran") && result.dig("fallback", "local_profile_digest").to_s.empty?

  artifacts = result.fetch("artifact_digests")
  raise("#{platform} missing required artifacts") if request.dig("artifact_policy", "required") && artifacts.empty?
  total_bytes = 0
  artifacts.each do |artifact|
    artifact_path = root.join(artifact.fetch("path"))
    raise("#{platform} missing artifact #{artifact_path}") unless artifact_path.file?
    raise("#{platform} artifact SHA-256 mismatch") unless Digest::SHA256.file(artifact_path).hexdigest == artifact["sha256"]
    raise("#{platform} artifact byte count mismatch") unless artifact_path.size == artifact["bytes"]
    total_bytes += artifact_path.size
  end
  raise("#{platform} artifact budget exceeded") if total_bytes > request.dig("artifact_policy", "max_total_bytes")
end

def validate_linux_evidence(root)
  request = read_json(root.join(".csdlc/evidence/5823/linux-native-request.json"))
  receipt = read_json(root.join(".csdlc/evidence/5823/linux-native-remote-summary.json"))
  first = read_json(root.join(".csdlc/evidence/5823/linux-native-first-attempt.json"))
  authorization = read_json(root.join(".csdlc/evidence/5823/operator-authorization.json"))

  ceiling = request.dig("resource_budget", "estimated_max_cost_microusd").to_i
  raise("Linux cost ceiling must be nonzero") unless ceiling.positive?
  raise("Linux proof revision mismatch") unless receipt["source_commit"] == request["revision"] && receipt.dig("run", "resolved_commit") == request["revision"]
  raise("Linux proof exceeded cost ceiling") unless (receipt.dig("cost", "estimated_compute_cost_usd").to_f * 1_000_000).ceil <= ceiling
  raise("Linux proof timeout mismatch") unless receipt.dig("request_limits", "timeout_seconds") == request.dig("resource_budget", "timeout_seconds")
  raise("Linux proof cost limit mismatch") unless (receipt.dig("request_limits", "estimated_max_cost_usd").to_f * 1_000_000).round == ceiling
  cleanup = receipt.fetch("cleanup")
  raise("Linux cleanup incomplete") unless %w[instance_terminated instance_profile_deleted role_deleted security_group_deleted].all? { |key| cleanup[key] == true }
  raise("Linux cleanup status is not clean") unless cleanup["explicit_post_run_status"] == "clean"
  redaction = receipt.fetch("redaction")
  raise("Linux receipt retained sensitive output") unless redaction.values.all? { |value| value == false }
  raise("first attempt was not classified as failed") unless first["status"] == "failed_before_validation"
  raise("first attempt lacks corrective disposition") unless first["disposition"].to_s.include?("corrected")
  raise("first attempt cleanup incomplete") unless first.fetch("resources").values_at("instance_terminated", "instance_profile_deleted", "role_deleted", "security_group_deleted").all?(true)
  raise("operator authorization issue mismatch") unless authorization["issue"] == 5823
  raise("operator authorization profile mismatch") unless authorization["authorized_profile"] == receipt["profile"]
  raise("operator authorization timeout mismatch") unless authorization["timeout_seconds"] == request.dig("resource_budget", "timeout_seconds")
  raise("operator authorization cost mismatch") unless (authorization["maximum_cost_usd"].to_f * 1_000_000).round == ceiling
  raise("operator authorization did not require cleanup") unless authorization["immediate_resource_cleanup_required"] == true
  raise("operator authorization retained credentials") unless authorization["credentials_retained"] == false
end

def validate_matrix(root, matrix)
  manifest = validate_deterministic_manifest(root)
  %w[linux macos windows].each do |platform|
    row = matrix[platform] || raise("missing #{platform}")
    if %w[linux macos].include?(platform)
      raise("#{platform} must be native live proof") unless row["qualification"] == "live" && row["native"] == true
      raise("#{platform} missing runner") if row["runner"].to_s.empty?
    else
      raise("Windows qualification invalid") unless %w[live fixture].include?(row["qualification"])
      expected_native = row["qualification"] == "live"
      raise("Windows native marker contradicts qualification") unless row["native"] == expected_native
    end
    validate_result(root, platform, row)
  end
  validate_linux_evidence(root)
  matrix_revisions = matrix.values.map { |row| row["revision"] }.uniq
  raise("manifest subject does not match matrix proof revision") unless matrix_revisions == [manifest["subject_revision"]]
  "native Linux + macOS, Windows #{matrix.dig('windows', 'qualification')}"
end

if ARGV == ["--self-test"]
  root = Pathname.new(__dir__).join("../../../..").cleanpath
  raise "hex validator accepted invalid revision" if HEX40.match?("g" * 40)
  raise "digest validator accepted invalid digest" if HEX64.match?("0" * 63)
  raise "issue lifecycle path must be allowed after validation" unless post_validation_truth_path?(".csdlc/issues/5823/index.json")
  raise "publication request path must be allowed after validation" unless post_validation_truth_path?(".csdlc/prepared/issues/5823/publish-final.json")
  raise "publication intent path must be allowed after validation" unless post_validation_truth_path?(".csdlc/publication/5823.intent.json")
  raise "unrelated documentation path was accepted after validation" if post_validation_truth_path?("docs/unrelated.md")
  raise "unrelated lifecycle path was accepted after validation" if post_validation_truth_path?(".csdlc/issues/9999/index.json")
  raise "receipt scanner missed /home path" unless machine_local_absolute_path?("/home/runner/proof.json")
  raise "receipt scanner missed /var/folders path" unless machine_local_absolute_path?("/var/folders/example/proof.json")
  begin
    repo_relative_path(Pathname.new("."), "/Volumes/FastWork/proof.json")
    raise "path validator accepted absolute machine path"
  rescue RuntimeError => error
    raise unless error.message.include?("absolute or escaping")
  end
  begin
    verify_git_revision(root, "0" * 40, "negative fixture")
    raise "Git validator accepted nonexistent revision"
  rescue RuntimeError => error
    raise unless error.message.include?("does not exist")
  end
  begin
    verify_receipt(
      root,
      ".csdlc/evidence/5823/platform-matrix.json",
      "0" * 64,
      "negative fixture"
    )
    raise "receipt validator accepted a stale digest"
  rescue RuntimeError => error
    raise unless error.message.include?("SHA-256 mismatch")
  end
  puts "WP-06 platform validator self-test passed"
  exit 0
end

root = Pathname.new(__dir__).join("../../../..").cleanpath
matrix_path = root.join(".csdlc/evidence/5823/platform-matrix.json")

begin
  summary = validate_matrix(root, read_json(matrix_path))
  puts "WP-06 platform matrix valid: #{summary}"
rescue RuntimeError, KeyError => error
  abort error.message
end
