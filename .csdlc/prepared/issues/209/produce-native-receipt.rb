#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "optparse"
require "pathname"
require "rbconfig"

ISSUE = 209
WORKFLOW = ".github/workflows/wp14-production-acip-repair.yml"
TESTS = %w[
  canonical_ingress_applies_bounded_pressure
  production_binary_acip_wss_produces_observed_receipt
].freeze
SOURCE_PATHS = %w[
  adl-runtime-kernel/Cargo.toml
  adl-runtime-kernel/src/assembly.rs
  adl-runtime-kernel/src/acip.rs
  adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
  adl-runtime-kernel/src/config.rs
  adl-runtime-kernel/src/control.rs
  adl-runtime-kernel/src/governed_operations.rs
  adl-runtime-kernel/src/lib.rs
  adl-runtime-kernel/tests/assembly.rs
  adl-runtime-kernel/tests/openapi_contract.rs
  adl-runtime-kernel/tests/production_acip_wss.rs
  adl-runtime-kernel/tests/support/runtime_init.rs
  adl-runtime/Cargo.toml
  adl-runtime/src/runtime_api_auth.rs
  adl/tools/install_vector_component.sh
  docs/api/runtime-v3/v1/openapi.json
  docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md
].freeze

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

def normalize(text, root)
  [root.to_s, ENV["GITHUB_WORKSPACE"]].compact.reject(&:empty?).flat_map { |prefix| [prefix, prefix.tr("/", "\\")] }
    .uniq.sort_by { |prefix| -prefix.length }.reduce(text.dup) do |value, prefix|
      value.gsub("#{prefix}/", "./").gsub("#{prefix}\\", "./").gsub(Regexp.new("#{Regexp.escape(prefix)}(?=$|[\\s\"'])"), ".")
    end
end

def manifest(root)
  SOURCE_PATHS.map do |relative|
    path = root.join(relative)
    fail!("source path missing: #{relative}") unless path.file?
    { "path" => relative, "sha256" => Digest::SHA256.file(path).hexdigest }
  end
end

def output_environment_path(path)
  path.expand_path.to_s
end

options = {}
OptionParser.new do |parser|
  parser.on("--platform PLATFORM") { |value| options[:platform] = value }
  parser.on("--receipt PATH") { |value| options[:receipt] = value }
  parser.on("--semantic-output PATH") { |value| options[:semantic] = value }
  parser.on("--self-test") { options[:self_test] = true }
end.parse!
fail!("unexpected positional arguments") unless ARGV.empty?

if options[:self_test]
  root = Pathname.new("/Users/runner/work/repo/repo")
  sample = JSON.generate("type" => "test", "event" => "ok", "name" => TESTS.first, "path" => root.join(SOURCE_PATHS.first).to_s)
  normalized = normalize(sample, root)
  fail!("normalizer retained host root") if normalized.include?(root.to_s)
  fail!("normalizer damaged test name") unless JSON.parse(normalized).fetch("name") == TESTS.first
  fail!("authority source omitted") unless SOURCE_PATHS.include?("adl-runtime/src/runtime_api_auth.rs")
  fail!("pressure configuration omitted") unless SOURCE_PATHS.include?("adl-runtime-kernel/src/config.rs")
  fail!("verified Vector installer omitted") unless SOURCE_PATHS.include?("adl/tools/install_vector_component.sh")
  semantic = root.join(".csdlc/evidence/209/native-platform/linux-semantic.json")
  crate_working_directory = root.join("adl-runtime-kernel")
  exported = output_environment_path(semantic)
  exported_path = Pathname.new(exported)
  resolved = exported_path.absolute? ? exported_path : crate_working_directory.join(exported).cleanpath
  fail!("semantic output is not absolute") unless exported_path.absolute?
  fail!("semantic output changes under package working directory") unless resolved == semantic
  puts JSON.generate(status: "passed", check: "wp14-native-producer")
  exit 0
end

platform = options[:platform]
fail!("platform must be linux or macos") unless %w[linux macos].include?(platform)
fail!("producer requires GitHub Actions") unless ENV["GITHUB_ACTIONS"] == "true"
root_text, status = Open3.capture2("git", "rev-parse", "--show-toplevel")
fail!("cannot resolve repository root") unless status.success?
root = Pathname.new(root_text.strip).realpath
head, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: root.to_s)
fail!("cannot resolve HEAD") unless status.success?
head = head.strip
expected_os = platform == "macos" ? "Darwin" : "Linux"
os, status = Open3.capture2("uname", "-s")
fail!("runner OS mismatch") unless status.success? && os.strip == expected_os

prefix = ".csdlc/evidence/#{ISSUE}/native-platform"
receipt = root.join(options.fetch(:receipt)).cleanpath
semantic = root.join(options.fetch(:semantic)).cleanpath
[receipt, semantic].each { |path| fail!("output escapes issue evidence") unless path.to_s.start_with?("#{root.join(prefix)}/") }
FileUtils.mkdir_p(receipt.dirname)
log = receipt.dirname.join("#{platform}-nextest.log")
source_manifest = receipt.dirname.join("#{platform}-source-manifest.json")
argv = ["cargo", "nextest", "run", "--manifest-path", "adl-runtime-kernel/Cargo.toml", "--test", "production_acip_wss", "--no-tests=fail", "--status-level", "all", "--message-format", "libtest-json-plus"]
stdout, stderr, status = Open3.capture3(
  { "ADL_ACIP_PLATFORM" => platform, "ADL_ACIP_PROOF_OUTPUT" => output_environment_path(semantic), "NEXTEST_EXPERIMENTAL_LIBTEST_JSON" => "1" },
  *argv, chdir: root.to_s
)
log.write(normalize(stdout + stderr, root))
fail!("native nextest failed") unless status.success?
passed = []
suites = []
log.each_line do |line|
  parsed = JSON.parse(line)
  passed << parsed["name"] if parsed["type"] == "test" && parsed["event"] == "ok"
  suites << parsed if parsed["type"] == "suite" && parsed["event"] == "ok"
rescue JSON::ParserError
  next
end
fail!("missing passing suite") unless suites.last && suites.last["failed"].to_i.zero?
fail!("semantic proof missing") unless semantic.file? && semantic.size.positive?
semantic_document = JSON.parse(semantic.read)
fail!("semantic platform mismatch") unless semantic_document["platform"] == platform
semantic_projection = semantic_document.reject { |key, _value| key == "platform" }
source_manifest.write(JSON.pretty_generate(manifest(root)) + "\n")
producer_path = Pathname.new(__FILE__).realpath.relative_path_from(root).to_s
payload = {
  "issue" => ISSUE, "platform" => platform, "source_sha" => head,
  "producer_path" => producer_path, "producer_sha256" => Digest::SHA256.file(root.join(producer_path)).hexdigest,
  "test_argv" => argv, "tests_run" => suites.last["passed"].to_i, "passed_tests" => passed.sort,
  "command_output_path" => log.relative_path_from(root).to_s, "command_output_sha256" => Digest::SHA256.file(log).hexdigest,
  "semantic_output_path" => semantic.relative_path_from(root).to_s, "semantic_output_sha256" => Digest::SHA256.file(semantic).hexdigest,
  "semantic_projection_sha256" => Digest::SHA256.hexdigest(canonical_json(semantic_projection)),
  "source_manifest_path" => source_manifest.relative_path_from(root).to_s, "source_manifest_sha256" => Digest::SHA256.file(source_manifest).hexdigest,
  "runner" => { "provider" => "github_actions", "repository" => ENV.fetch("GITHUB_REPOSITORY"), "workflow_ref" => ENV.fetch("GITHUB_WORKFLOW_REF"), "run_id" => ENV.fetch("GITHUB_RUN_ID"), "run_attempt" => ENV.fetch("GITHUB_RUN_ATTEMPT"), "job" => ENV.fetch("GITHUB_JOB"), "os" => os.strip, "architecture" => RbConfig::CONFIG.fetch("host_cpu") },
  "status" => "passed"
}
packet = { "schema" => "adl.native_ci_receipt.v1", "payload" => payload, "payload_sha256" => Digest::SHA256.hexdigest(canonical_json(payload)) }
receipt.write(JSON.pretty_generate(packet) + "\n")
puts JSON.generate(issue: ISSUE, platform: platform, source_sha: head)
