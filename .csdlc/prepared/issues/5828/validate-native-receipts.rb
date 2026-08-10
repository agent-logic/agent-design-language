#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ISSUE_CONFIG = {
  5825 => ["birthday", "docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md"],
  5826 => ["birthday_identity", "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"],
  5827 => ["birthday_continuity", "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"],
  5828 => ["memory_palace", "docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md"],
  5829 => ["capability_envelope", "docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md"],
  5830 => ["cognitive_profile", "docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md"],
  5831 => ["adaptive_learning", "docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md"],
  5833 => ["birth_witness", "docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md"]
}.freeze

def fail!(message)
  warn(message)
  exit 1
end

def canonical_json(value)
  case value
  when Hash
    "{" + value.keys.sort.map { |key| "#{JSON.generate(key)}:#{canonical_json(value.fetch(key))}" }.join(",") + "}"
  when Array
    "[" + value.map { |entry| canonical_json(entry) }.join(",") + "]"
  else
    JSON.generate(value)
  end
end

def repo_file(root, value, label, required_prefix: nil)
  path = Pathname.new(value.to_s)
  fail!("#{label} must be repository-relative") if value.to_s.empty? || path.absolute? || path.each_filename.include?("..")
  absolute = root.join(path).cleanpath
  fail!("#{label} escapes repository root") unless absolute.to_s.start_with?("#{root}/")
  fail!("#{label} escapes required evidence directory") if required_prefix && !absolute.to_s.start_with?("#{root.join(required_prefix).cleanpath}/")
  fail!("#{label} does not exist: #{value}") unless absolute.file?
  absolute
end

def source_paths(test_target, feature_path)
  [
    "adl-runtime-kernel/Cargo.toml",
    "adl-runtime-kernel/src/lib.rs",
    "adl-runtime-kernel/src/#{test_target}.rs",
    "adl-runtime-kernel/tests/#{test_target}.rs",
    "adl-runtime-kernel/tests/fixtures/#{test_target}",
    feature_path
  ]
end

def source_manifest(root, paths)
  paths.flat_map do |relative|
    absolute = root.join(relative)
    files = absolute.directory? ? Dir.glob(absolute.join("**", "*").to_s).select { |path| File.file?(path) }.sort : [absolute.to_s]
    fail!("source contract path is absent: #{relative}") if files.empty? || files.any? { |path| !File.file?(path) }
    files.map do |file|
      rel = Pathname.new(file).relative_path_from(root).to_s
      { "path" => rel, "sha256" => Digest::SHA256.file(file).hexdigest }
    end
  end.sort_by { |row| row.fetch("path") }
end

issue = File.basename(File.dirname(__FILE__)).to_i
test_target, feature_path = ISSUE_CONFIG.fetch(issue) { fail!("unsupported issue-local validator path") }
fail!("expected exactly two receipt paths") unless ARGV.length == 2
fail!("native receipts must be validated by GitHub Actions") unless ENV["GITHUB_ACTIONS"] == "true"

current_workflow_ref = ENV.fetch("GITHUB_WORKFLOW_REF")
current_run_id = ENV.fetch("GITHUB_RUN_ID")
current_run_attempt = ENV.fetch("GITHUB_RUN_ATTEMPT")
expected_workflow_prefix = "agent-logic/agent-design-language/.github/workflows/wp11-native-memory-palace.yml@"
fail!("validator workflow identity mismatch") unless current_workflow_ref.start_with?(expected_workflow_prefix)

root_text, root_status = Open3.capture2("git", "rev-parse", "--show-toplevel")
fail!("cannot resolve repository root") unless root_status.success?
root = Pathname.new(root_text.strip).realpath
head_text, head_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: root.to_s)
fail!("cannot resolve exact HEAD") unless head_status.success?
head = head_text.strip

producer_path = ".csdlc/prepared/issues/#{issue}/produce-native-receipt.rb"
producer_digest = Digest::SHA256.file(root.join(producer_path)).hexdigest
expected_test_argv = [
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime-kernel/Cargo.toml",
  "--test", test_target, "--no-tests=fail", "--status-level", "all",
  "--message-format", "libtest-json-plus"
]
expected_manifest = source_manifest(root, source_paths(test_target, feature_path))
evidence_prefix = ".csdlc/evidence/#{issue}/native-platform"
required_hex = /\A[0-9a-f]{64}\z/
required_tests = %w[
  bounded_selection_records_overflow_without_loading_it
  canonical_topology_is_deterministic_and_native_semantic_output_is_contained
  citation_path_hash_and_presence_fail_closed
  duplicate_records_and_citations_are_rejected
  identity_continuity_and_trace_substitution_fail_closed
  identifiers_secrets_and_normalized_room_collisions_fail_closed
  normalized_obsmem_adapter_binds_trace_events_and_citation
  packet_validation_rejects_self_rehashed_privacy_and_topology_forgery
  private_raw_and_unredacted_memory_never_enters_working_set
  reference_identifier_privacy_and_canonical_packet_bounds_fail_closed
  secret_like_payloads_and_unknown_json_fields_fail_closed
  stale_future_and_invalid_temporal_anchors_fail_closed
]

receipts = ARGV.map do |receipt_relative|
  receipt_file = repo_file(root, receipt_relative, "receipt", required_prefix: evidence_prefix)
  receipt = JSON.parse(receipt_file.read)
  fail!("receipt schema mismatch") unless receipt["schema"] == "adl.native_ci_receipt.v1"
  payload = receipt["payload"]
  fail!("receipt payload must be an object") unless payload.is_a?(Hash)
  fail!("receipt payload digest mismatch") unless receipt["payload_sha256"] == Digest::SHA256.hexdigest(canonical_json(payload))
  payload
rescue JSON::ParserError => error
  fail!("invalid receipt JSON: #{error.message}")
end

fail!("native receipts must cover exactly linux and macos") unless receipts.map { |receipt| receipt["platform"] }.sort == %w[linux macos]
receipts.each do |receipt|
  platform = receipt.fetch("platform")
  fail!("#{platform}: source_sha must equal exact candidate HEAD") unless receipt["source_sha"] == head
  fail!("#{platform}: producer path mismatch") unless receipt["producer_path"] == producer_path
  fail!("#{platform}: producer digest mismatch") unless receipt["producer_sha256"] == producer_digest
  expected_producer_argv = [
    "ruby", producer_path, "--platform", platform, "--receipt",
    ".csdlc/evidence/#{issue}/native-platform/#{platform}.json", "--semantic-output",
    ".csdlc/evidence/#{issue}/native-platform/#{platform}-semantic.json"
  ]
  fail!("#{platform}: producer argv mismatch") unless receipt["producer_argv"] == expected_producer_argv
  fail!("#{platform}: test argv mismatch") unless receipt["test_argv"] == expected_test_argv
  expected_semantic_path = ".csdlc/evidence/#{issue}/native-platform/#{platform}-semantic.json"
  fail!("#{platform}: semantic-output environment mismatch") unless receipt["test_environment"] == {
    "ADL_NATIVE_SEMANTIC_OUTPUT" => expected_semantic_path,
    "NEXTEST_EXPERIMENTAL_LIBTEST_JSON" => "1"
  }
  fail!("#{platform}: status must be passed") unless receipt["status"] == "passed"

  runner = receipt["runner"]
  fail!("#{platform}: runner must be an object") unless runner.is_a?(Hash)
  fail!("#{platform}: runner provider mismatch") unless runner["provider"] == "github_actions"
  %w[repository workflow_ref run_id run_attempt job os architecture].each do |field|
    fail!("#{platform}: runner #{field} is required") unless runner[field].is_a?(String) && !runner[field].strip.empty?
  end
  fail!("#{platform}: repository mismatch") unless runner["repository"] == "agent-logic/agent-design-language"
  fail!("#{platform}: workflow identity mismatch") unless runner["workflow_ref"] == current_workflow_ref
  fail!("#{platform}: workflow run mismatch") unless runner["run_id"] == current_run_id
  fail!("#{platform}: workflow attempt mismatch") unless runner["run_attempt"] == current_run_attempt
  fail!("#{platform}: producer job mismatch") unless runner["job"] == "produce-native-receipt"
  fail!("#{platform}: native OS mismatch") unless runner["os"] == (platform == "macos" ? "Darwin" : "Linux")

  command_output = repo_file(root, receipt["command_output_path"], "#{platform} command output", required_prefix: evidence_prefix)
  fail!("#{platform}: command output digest mismatch") unless required_hex.match?(receipt["command_output_sha256"].to_s) && receipt["command_output_sha256"] == Digest::SHA256.file(command_output).hexdigest
  suites = []
  passed_tests = []
  command_output.each_line do |line|
    parsed = JSON.parse(line)
    suites << parsed if parsed["type"] == "suite" && parsed["event"] == "ok"
    passed_tests << parsed["name"] if parsed["type"] == "test" && parsed["event"] == "ok"
  rescue JSON::ParserError
    next
  end
  suite = suites.last
  fail!("#{platform}: command output lacks a passing structured suite summary") unless suite && suite["passed"].to_i.positive? && suite["failed"].to_i.zero?
  fail!("#{platform}: tests_run disagrees with command output") unless receipt["tests_run"] == suite["passed"].to_i
  observed_tests = receipt["passed_tests"]
  fail!("#{platform}: passed test inventory disagrees with output") unless observed_tests == passed_tests.sort
  prefix = "adl-runtime-kernel::memory_palace "
  expected_tests = required_tests.map { |name| "#{prefix}#{name}" }.sort
  fail!("#{platform}: exact Memory Palace test inventory mismatch") unless observed_tests == expected_tests
  fail!("#{platform}: exact test count mismatch") unless receipt["tests_run"] == required_tests.length

  semantic_output = repo_file(root, receipt["semantic_output_path"], "#{platform} semantic output", required_prefix: evidence_prefix)
  fail!("#{platform}: semantic path mismatch") unless receipt["semantic_output_path"] == expected_semantic_path
  fail!("#{platform}: semantic output digest mismatch") unless required_hex.match?(receipt["semantic_output_sha256"].to_s) && receipt["semantic_output_sha256"] == Digest::SHA256.file(semantic_output).hexdigest

  manifest_file = repo_file(root, receipt["source_manifest_path"], "#{platform} source manifest", required_prefix: evidence_prefix)
  fail!("#{platform}: source manifest digest mismatch") unless required_hex.match?(receipt["source_manifest_sha256"].to_s) && receipt["source_manifest_sha256"] == Digest::SHA256.file(manifest_file).hexdigest
  parsed_manifest = JSON.parse(manifest_file.read)
  fail!("#{platform}: source manifest does not match candidate HEAD files") unless parsed_manifest == expected_manifest
end

fail!("native receipts must come from one workflow run") unless receipts.map { |receipt| receipt.dig("runner", "run_id") }.uniq.one?
fail!("native receipts must come from one workflow attempt") unless receipts.map { |receipt| receipt.dig("runner", "run_attempt") }.uniq.one?
fail!("native semantic outputs differ") unless receipts.map { |receipt| receipt["semantic_output_sha256"] }.uniq.one?
puts JSON.generate(issue: issue, status: "passed", reviewed_head: head, platforms: %w[linux macos], semantic_output_sha256: receipts.first["semantic_output_sha256"])
