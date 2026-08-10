#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "optparse"
require "pathname"
require "rbconfig"

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

def repo_path(root, relative, label)
  path = Pathname.new(relative.to_s)
  fail!("#{label} must be repository-relative") if relative.to_s.empty? || path.absolute? || path.each_filename.include?("..")
  absolute = root.join(path).cleanpath
  fail!("#{label} escapes repository root") unless absolute.to_s.start_with?("#{root}/")
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
  rows = paths.flat_map do |relative|
    absolute = root.join(relative)
    files = absolute.directory? ? Dir.glob(absolute.join("**", "*").to_s).select { |path| File.file?(path) }.sort : [absolute.to_s]
    fail!("source contract path is absent: #{relative}") if files.empty? || files.any? { |path| !File.file?(path) }
    files.map do |file|
      rel = Pathname.new(file).relative_path_from(root).to_s
      { "path" => rel, "sha256" => Digest::SHA256.file(file).hexdigest }
    end
  end
  rows.sort_by { |row| row.fetch("path") }
end

def normalize_command_output(output, root)
  prefixes = [root.to_s, ENV["GITHUB_WORKSPACE"]].compact.reject(&:empty?).flat_map do |prefix|
    [prefix, prefix.tr("/", "\\")]
  end.uniq.sort_by { |prefix| -prefix.length }
  prefixes.reduce(output.dup) do |normalized, prefix|
    standalone = Regexp.new("#{Regexp.escape(prefix)}(?=$|[\\s\"'])")
    normalized.gsub("#{prefix}/", "./").gsub("#{prefix}\\", "./").gsub(standalone, ".")
  end
end

options = {}
OptionParser.new do |parser|
  parser.on("--platform PLATFORM") { |value| options[:platform] = value }
  parser.on("--receipt PATH") { |value| options[:receipt] = value }
  parser.on("--semantic-output PATH") { |value| options[:semantic_output] = value }
  parser.on("--self-test") { options[:self_test] = true }
end.parse!
fail!("unexpected positional arguments") unless ARGV.empty?
if options[:self_test]
  synthetic_root = Pathname.new("/Users/runner/work/agent-design-language/agent-design-language")
  test_name = "adl-runtime-kernel::adl_runtime_kernel$birth_witness::authority_tests::synthetic_case"
  synthetic = JSON.generate("type" => "test", "event" => "ok", "name" => test_name,
                            "path" => synthetic_root.join("adl-runtime-kernel/src/lib.rs").to_s) + "\n"
  normalized = normalize_command_output(synthetic, synthetic_root)
  fail!("self-test retained checkout prefix") if normalized.include?(synthetic_root.to_s)
  parsed = JSON.parse(normalized)
  fail!("self-test altered structured event inventory") unless parsed["name"] == test_name
  fail!("self-test did not use repository-relative marker") unless parsed["path"] == "./adl-runtime-kernel/src/lib.rs"
  expected = source_paths("birth_witness", "docs/birth-witness.md")
  fail!("self-test omitted birth-witness source") unless expected.include?("adl-runtime-kernel/src/birth_witness.rs")
  puts JSON.generate(status: "passed", check: "native-log-normalization")
  exit 0
end
fail!("platform must be macos or linux") unless %w[macos linux].include?(options[:platform])
fail!("native receipts must be produced by GitHub Actions") unless ENV["GITHUB_ACTIONS"] == "true"

root_text, root_status = Open3.capture2("git", "rev-parse", "--show-toplevel")
fail!("cannot resolve repository root") unless root_status.success?
root = Pathname.new(root_text.strip).realpath
head_text, head_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: root.to_s)
fail!("cannot resolve exact HEAD") unless head_status.success?
head = head_text.strip

issue = File.basename(File.dirname(__FILE__)).to_i
test_target, feature_path = ISSUE_CONFIG.fetch(issue) { fail!("unsupported issue-local producer path") }
expected_os = options[:platform] == "macos" ? "Darwin" : "Linux"
host_os, host_status = Open3.capture2("uname", "-s")
fail!("producer platform does not match native runner") unless host_status.success? && host_os.strip == expected_os

receipt_path = repo_path(root, options[:receipt], "receipt")
semantic_path = repo_path(root, options[:semantic_output], "semantic output")
evidence_root = root.join(".csdlc/evidence/#{issue}/native-platform").cleanpath
[receipt_path, semantic_path].each do |path|
  fail!("evidence output must remain below #{evidence_root.relative_path_from(root)}") unless path.to_s.start_with?("#{evidence_root}/")
end
FileUtils.mkdir_p(receipt_path.dirname)
FileUtils.mkdir_p(semantic_path.dirname)
command_output_path = receipt_path.dirname.join("#{options[:platform]}-nextest.log")
manifest_path = receipt_path.dirname.join("#{options[:platform]}-source-manifest.json")

test_argv = [
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime-kernel/Cargo.toml",
  "--lib", "-E", "test(/^birth_witness::authority_tests::/)",
  "--no-tests=fail", "--status-level", "all",
  "--message-format", "libtest-json-plus"
]
stdout, stderr, status = Open3.capture3(
  {
    "ADL_NATIVE_SEMANTIC_OUTPUT" => semantic_path.relative_path_from(root).to_s,
    "NEXTEST_EXPERIMENTAL_LIBTEST_JSON" => "1"
  },
  *test_argv,
  chdir: root.to_s
)
command_output = normalize_command_output(stdout + stderr, root)
command_output_path.write(command_output)
fail!("native nextest command failed") unless status.success?
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
fail!("native nextest output lacks a passing structured suite summary") unless suite && suite["passed"].to_i.positive? && suite["failed"].to_i.zero?
fail!("test did not produce the declared semantic output") unless semantic_path.file? && semantic_path.size.positive?

manifest = source_manifest(root, source_paths(test_target, feature_path))
manifest_path.write(JSON.pretty_generate(manifest) + "\n")
producer_rel = Pathname.new(__FILE__).realpath.relative_path_from(root).to_s
payload = {
  "issue" => issue,
  "platform" => options[:platform],
  "source_sha" => head,
  "producer_path" => producer_rel,
  "producer_sha256" => Digest::SHA256.file(root.join(producer_rel)).hexdigest,
  "producer_argv" => ["ruby", producer_rel, "--platform", options[:platform], "--receipt", options[:receipt], "--semantic-output", options[:semantic_output]],
  "test_argv" => test_argv,
  "test_environment" => {
    "ADL_NATIVE_SEMANTIC_OUTPUT" => semantic_path.relative_path_from(root).to_s,
    "NEXTEST_EXPERIMENTAL_LIBTEST_JSON" => "1"
  },
  "tests_run" => suite["passed"].to_i,
  "passed_tests" => passed_tests.sort,
  "command_output_path" => command_output_path.relative_path_from(root).to_s,
  "command_output_sha256" => Digest::SHA256.file(command_output_path).hexdigest,
  "semantic_output_path" => semantic_path.relative_path_from(root).to_s,
  "semantic_output_sha256" => Digest::SHA256.file(semantic_path).hexdigest,
  "source_manifest_path" => manifest_path.relative_path_from(root).to_s,
  "source_manifest_sha256" => Digest::SHA256.file(manifest_path).hexdigest,
  "runner" => {
    "provider" => "github_actions",
    "repository" => ENV.fetch("GITHUB_REPOSITORY"),
    "workflow_ref" => ENV.fetch("GITHUB_WORKFLOW_REF"),
    "run_id" => ENV.fetch("GITHUB_RUN_ID"),
    "run_attempt" => ENV.fetch("GITHUB_RUN_ATTEMPT"),
    "job" => ENV.fetch("GITHUB_JOB"),
    "os" => host_os.strip,
    "architecture" => RbConfig::CONFIG.fetch("host_cpu")
  },
  "status" => "passed"
}
receipt = {
  "schema" => "adl.native_ci_receipt.v1",
  "payload" => payload,
  "payload_sha256" => Digest::SHA256.hexdigest(canonical_json(payload))
}
receipt_path.write(JSON.pretty_generate(receipt) + "\n")
puts JSON.generate(issue: issue, platform: options[:platform], source_sha: head, receipt: options[:receipt])
