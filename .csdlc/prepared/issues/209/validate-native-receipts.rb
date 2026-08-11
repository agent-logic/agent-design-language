#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ISSUE = 209
WORKFLOW = ".github/workflows/wp14-production-acip-repair.yml"
TESTS = %w[canonical_ingress_applies_bounded_pressure production_binary_acip_wss_produces_observed_receipt].freeze
ASSERTIONS = %w[
  acip_write_token_authenticated
  binary_protobuf_dispatch_completed
  exact_production_binary_tls_ready
  observatory_read_token_rejected
  production_wss_pressure_rolls_back_and_recovers
  replay_rejected
  signed_graceful_shutdown
  terminal_sequence_rejected_without_cross_domain_poisoning
  text_frame_rejected
  typed_ingress_error_rolls_back_sequence
].freeze
SOURCE_PATHS = %w[adl-runtime-kernel/Cargo.toml adl-runtime-kernel/src/assembly.rs adl-runtime-kernel/src/acip.rs adl-runtime-kernel/src/bin/adl-runtime-kernel.rs adl-runtime-kernel/src/config.rs adl-runtime-kernel/src/control.rs adl-runtime-kernel/src/governed_operations.rs adl-runtime-kernel/src/lib.rs adl-runtime-kernel/tests/assembly.rs adl-runtime-kernel/tests/openapi_contract.rs adl-runtime-kernel/tests/production_acip_wss.rs adl-runtime-kernel/tests/support/runtime_init.rs adl-runtime/Cargo.toml adl-runtime/src/runtime_api_auth.rs adl/tools/install_vector_component.sh docs/api/runtime-v3/v1/openapi.json docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md].freeze

def fail!(message)
  warn(message)
  exit 1
end

def canonical_json(value)
  case value
  when Hash then "{" + value.keys.sort.map { |key| "#{JSON.generate(key)}:#{canonical_json(value.fetch(key))}" }.join(",") + "}"
  when Array then "[" + value.map { |entry| canonical_json(entry) }.join(",") + "]"
  else JSON.generate(value)
  end
end

def manifest(root)
  SOURCE_PATHS.map { |relative| { "path" => relative, "sha256" => Digest::SHA256.file(root.join(relative)).hexdigest } }
end

def unsafe_log?(text, root)
  text.include?(root.to_s) || [%r{/(?:users?|home|private)/}i, %r{[a-z]:[\\/]}i, %r{\\\\[^\\/\s]+[\\/]}, %r{/volumes/(?:fastwork|home)/}i, %r{/var/folders/}i].any? { |pattern| text.match?(pattern) }
end

if ARGV == ["--self-test"]
  fail!("authority source omitted") unless SOURCE_PATHS.include?("adl-runtime/src/runtime_api_auth.rs")
  fail!("pressure configuration omitted") unless SOURCE_PATHS.include?("adl-runtime-kernel/src/config.rs")
  fail!("verified Vector installer omitted") unless SOURCE_PATHS.include?("adl/tools/install_vector_component.sh")
  fail!("workflow missing") unless File.read(WORKFLOW).include?("include-hidden-files: true")
  fail!("host path accepted") unless unsafe_log?("/Users/runner/work/repo/file", Pathname.new("/repo"))
  fail!("relative log rejected") if unsafe_log?("./adl-runtime-kernel/src/control.rs", Pathname.new("/repo"))
  puts JSON.generate(status: "passed", check: "wp14-native-validator")
  exit 0
end

fail!("expected macOS and Linux receipts") unless ARGV.length == 2
fail!("validator requires GitHub Actions") unless ENV["GITHUB_ACTIONS"] == "true"
root_text, status = Open3.capture2("git", "rev-parse", "--show-toplevel")
fail!("cannot resolve repository root") unless status.success?
root = Pathname.new(root_text.strip).realpath
head, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: root.to_s)
fail!("cannot resolve HEAD") unless status.success?
head = head.strip
workflow_ref = ENV.fetch("GITHUB_WORKFLOW_REF")
fail!("workflow mismatch") unless workflow_ref.start_with?("agent-logic/agent-design-language/#{WORKFLOW}@")
expected_manifest = manifest(root)
producer = root.join(".csdlc/prepared/issues/209/produce-native-receipt.rb")
expected_names = TESTS.map { |name| "adl-runtime-kernel::production_acip_wss$#{name}" }.sort
expected_argv = ["cargo", "nextest", "run", "--manifest-path", "adl-runtime-kernel/Cargo.toml", "--test", "production_acip_wss", "--no-tests=fail", "--status-level", "all", "--message-format", "libtest-json-plus"]

payloads = ARGV.map do |relative|
  path = root.join(relative).cleanpath
  fail!("receipt escapes issue evidence") unless path.to_s.start_with?("#{root.join('.csdlc/evidence/209/native-platform')}/")
  packet = JSON.parse(path.read)
  payload = packet.fetch("payload")
  fail!("receipt schema mismatch") unless packet["schema"] == "adl.native_ci_receipt.v1"
  fail!("payload digest mismatch") unless packet["payload_sha256"] == Digest::SHA256.hexdigest(canonical_json(payload))
  payload
end
fail!("platform denominator mismatch") unless payloads.map { |item| item["platform"] }.sort == %w[linux macos]
payloads.each do |payload|
  platform = payload.fetch("platform")
  fail!("#{platform}: stale head") unless payload["source_sha"] == head
  fail!("#{platform}: producer drift") unless payload["producer_sha256"] == Digest::SHA256.file(producer).hexdigest
  fail!("#{platform}: producer command drift") unless payload["test_argv"] == expected_argv
  fail!("#{platform}: test inventory mismatch") unless payload["passed_tests"] == expected_names && payload["tests_run"] == TESTS.length
  runner = payload.fetch("runner")
  fail!("#{platform}: provenance mismatch") unless runner["repository"] == "agent-logic/agent-design-language" && runner["workflow_ref"] == workflow_ref && runner["run_id"] == ENV.fetch("GITHUB_RUN_ID") && runner["run_attempt"] == ENV.fetch("GITHUB_RUN_ATTEMPT")
  %w[command_output semantic_output source_manifest].each do |kind|
    path = root.join(payload.fetch("#{kind}_path")).cleanpath
    fail!("#{platform}: #{kind} path escapes evidence") unless path.to_s.start_with?("#{root.join('.csdlc/evidence/209/native-platform')}/")
    fail!("#{platform}: #{kind} digest mismatch") unless payload["#{kind}_sha256"] == Digest::SHA256.file(path).hexdigest
  end
  log = root.join(payload.fetch("command_output_path")).read
  fail!("#{platform}: machine-local log") if unsafe_log?(log, root)
  parsed_manifest = JSON.parse(root.join(payload.fetch("source_manifest_path")).read)
  fail!("#{platform}: source manifest mismatch") unless parsed_manifest == expected_manifest
  semantic = JSON.parse(root.join(payload.fetch("semantic_output_path")).read)
  fail!("#{platform}: semantic schema mismatch") unless semantic["schema"] == "adl.acip_native_platform_proof.v2"
  fail!("#{platform}: semantic platform mismatch") unless semantic["platform"] == platform
  assertions = Array(semantic["assertions"])
  fail!("#{platform}: semantic assertion inventory mismatch") unless assertions.map { |entry| entry["name"] }.sort == ASSERTIONS.sort
  fail!("#{platform}: semantic assertion failure") unless assertions.all? { |entry| entry["result"] == "passed" }
  projection = semantic.reject { |key, _value| key == "platform" }
  fail!("#{platform}: semantic projection digest mismatch") unless payload["semantic_projection_sha256"] == Digest::SHA256.hexdigest(canonical_json(projection))
end
fail!("run mismatch") unless payloads.map { |item| item.dig("runner", "run_id") }.uniq.one?
fail!("semantic mismatch") unless payloads.map { |item| item["semantic_projection_sha256"] }.uniq.one?
puts JSON.generate(issue: ISSUE, status: "passed", reviewed_head: head, platforms: %w[linux macos])
