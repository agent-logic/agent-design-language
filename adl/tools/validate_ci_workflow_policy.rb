#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "yaml"

root = Pathname.new(ARGV[0] || Pathname.new(__dir__).join("../..")).cleanpath
workflow_dir = root.join(".github/workflows")
workflow_paths = Dir[workflow_dir.join("*.{yml,yaml}").to_s].sort.map { |path| Pathname.new(path) }
ci_path = workflow_dir.join("ci.yaml")
errors = []
inventory = []

def top_level_events(text)
  lines = text.lines
  start = lines.index { |line| line == "on:\n" }
  return [] unless start

  events = []
  lines[(start + 1)..].each do |line|
    next if line.strip.empty?
    break unless line.start_with?("  ")

    match = line.match(/^  ([a-zA-Z0-9_-]+):/)
    events << match[1] if match
  end
  events
end

def job_block(text, job)
  match = text.match(/^  #{Regexp.escape(job)}:\n(?<body>.*?)(?=^  [A-Za-z0-9_-]+:\n|\z)/m)
  match && match[0]
end

PINNED_RUST_TOOLCHAIN = "1.92.0"
toolchain_path = root.join("rust-toolchain.toml")
if toolchain_path.file?
  toolchain = toolchain_path.read
  unless toolchain.match?(/^channel\s*=\s*"#{Regexp.escape(PINNED_RUST_TOOLCHAIN)}"\s*$/)
    errors << "rust-toolchain.toml: channel must pin Rust #{PINNED_RUST_TOOLCHAIN}"
  end
else
  errors << "rust-toolchain.toml: missing required Rust toolchain declaration"
end

workflow_paths.each do |path|
  text = path.read
  events = top_level_events(text)
  relative = path.relative_path_from(root).to_s
  allowed_events = path == ci_path ? %w[pull_request workflow_dispatch] : %w[workflow_dispatch workflow_call]
  disallowed_events = events - allowed_events
  errors << "#{relative}: event #{disallowed_events.join(',')} is not allowed" unless disallowed_events.empty?
  if path != ci_path && !events.include?("workflow_dispatch")
    errors << "#{relative}: standalone workflow must remain explicitly dispatchable"
  end

  inventory << {
    "path" => relative,
    "events" => events,
    "automatic_pr" => events.include?("pull_request"),
    "dispatch_policy" => path == ci_path ? "automatic_classifier" : "explicit_only"
  }
end

ci = ci_path.read
# #916: a 35-minute wall deadline cancelled a successful 1,499-test run during
# profile processing. Keep full producer execution without a shorter job/step cap.
begin
  parsed_ci = YAML.safe_load(ci, permitted_classes: [], permitted_symbols: [], aliases: true)
  workspace_coverage = parsed_ci.fetch("jobs").fetch("public_adl_coverage")
  if workspace_coverage.key?("timeout-minutes") || workspace_coverage.fetch("steps").any? { |step| step.key?("timeout-minutes") }
    errors << "ci.yaml public_adl_coverage: custom job/step deadline can cancel successful coverage production"
  end
rescue Psych::Exception, KeyError, NoMethodError => error
  errors << "ci.yaml: workspace coverage deadline policy cannot inspect workflow (#{error.class})"
end
ci_events = top_level_events(ci)
errors << ".github/workflows/ci.yaml: pull_request entrypoint is missing" unless ci_events.include?("pull_request")
errors << ".github/workflows/ci.yaml: explicit full validation is missing" unless ci_events.include?("workflow_dispatch")

ci.scan(/^\s*-\s+name:\s*(?<name>.*?)\s*\n(?<body>.*?)(?=^\s*-\s+name:|\n  [A-Za-z0-9_-]+:\n|\z)/m).each do |name, body|
  next unless body.include?("uses: dtolnay/rust-toolchain@")

  unless body.match?(/^\s+toolchain:\s*#{Regexp.escape(PINNED_RUST_TOOLCHAIN)}\s*$/)
    errors << ".github/workflows/ci.yaml #{name}: Rust toolchain must pin #{PINNED_RUST_TOOLCHAIN}"
  end
  if body.match?(/^\s+toolchain:\s*(stable|nightly|beta)\s*$/)
    errors << ".github/workflows/ci.yaml #{name}: floating Rust channel is not allowed"
  end
end

concurrency = "group: ${{ github.repository }}:${{ github.workflow }}:${{ github.event.pull_request.base.ref || github.ref_name }}:${{ github.event.pull_request.head.repo.id || github.repository_id }}:${{ github.event.pull_request.head.ref || github.ref }}"
errors << ".github/workflows/ci.yaml: concurrency key must unambiguously identify workflow, target, source repository, and source branch" unless ci.include?(concurrency)
errors << ".github/workflows/ci.yaml: superseded revisions must cancel in progress" unless ci.include?("cancel-in-progress: true")

standard_runner = "runs-on: ubuntu-latest"
public_adl_runner = "runs-on: macos-latest"
light_jobs = %w[adl_path_policy adl_tooling_contracts adl_coverage_hosted adl-ci adl-coverage]
ci.scan(/^  ([A-Za-z0-9_-]+):\n(.*?)(?=^  [A-Za-z0-9_-]+:\n|\z)/m).each do |job, body|
  runner_line = body.lines.find { |line| line.match?(/^    runs-on:/) }
  next unless runner_line
  if light_jobs.include?(job)
    errors << "ci.yaml #{job}: light classifier/aggregator must use ubuntu-latest" unless runner_line.strip == "runs-on: ubuntu-latest"
    next
  end
  # The public distribution proof enforces network/home/source denial through
  # macOS sandbox-exec. It has no truthful Ubuntu fallback, so this one named
  # job must remain on GitHub-hosted macOS while every other selected job stays
  # on the standard Ubuntu runner.
  required_runner = job == "public_adl_validation" ? public_adl_runner : standard_runner
  errors << "ci.yaml #{job}: selected required job must use #{required_runner}" unless runner_line.strip == required_runner

  header = body.split("runs-on:", 2).first
  errors << "ci.yaml #{job}: required standard-runner job must depend on adl_path_policy" unless header.include?("adl_path_policy")
  errors << "ci.yaml #{job}: required standard-runner job must have a job-level selector" unless header.match?(/^    if:/)
end

required_outputs = {
  "automatic_pr_entrypoint" => "ci",
  "optional_workflows_status" => "deferred",
  "optional_workflows_reason" => "explicit_dispatch_required",
  "soak_workflows_status" => "deferred",
  "duplicate_head_status" => "canceled",
  "duplicate_head_reason" => "source_branch_concurrency_cancel_in_progress"
}
policy_stdout, policy_stderr, policy_status = Open3.capture3("python3", root.join("tools/public_adl/ci_policy.py").to_s, "--event-name", "workflow_dispatch")
errors << "public CI selector did not execute successfully" unless policy_status.success?
actual_outputs = policy_stdout.lines.map { |line| line.strip.split("=", 2) }.to_h
required_outputs.each do |key, value|
  errors << "public CI selector: missing #{key}=#{value}" unless actual_outputs[key] == value
end

result = {
  "schema" => "adl.ci.workflow-policy.v1",
  "status" => errors.empty? ? "pass" : "fail",
  "automatic_pr_entrypoint" => ".github/workflows/ci.yaml",
  "required_runner" => "ubuntu-latest",
  "optional_policy" => "explicit_dispatch_only",
  "scheduled_heavy_validation" => "disabled",
  "workflow_count" => inventory.length,
  "workflows" => inventory,
  "errors" => errors
}
puts JSON.pretty_generate(result)
exit(errors.empty? ? 0 : 1)
