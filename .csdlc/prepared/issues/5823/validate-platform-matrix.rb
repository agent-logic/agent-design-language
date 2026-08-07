#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"

HEX40 = /\A[0-9a-f]{40}\z/
HEX64 = /\A[0-9a-f]{64}\z/

def read_json(path)
  raise("missing receipt #{path}") unless path.file? && !path.zero?

  JSON.parse(path.read)
rescue JSON::ParserError => error
  raise("malformed JSON #{path}: #{error.message}")
end

def validate_result(root, platform, row)
  %w[revision command_profile_digest result_digest receipt outcome].each do |field|
    raise("#{platform} missing #{field}") if row[field].to_s.empty?
  end
  raise("#{platform} invalid revision") unless HEX40.match?(row["revision"])
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
  "native Linux + macOS, Windows #{matrix.dig('windows', 'qualification')}"
end

if ARGV == ["--self-test"]
  raise "hex validator accepted invalid revision" if HEX40.match?("g" * 40)
  raise "digest validator accepted invalid digest" if HEX64.match?("0" * 63)
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
