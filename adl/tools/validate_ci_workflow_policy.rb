#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

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
ci_events = top_level_events(ci)
errors << ".github/workflows/ci.yaml: pull_request entrypoint is missing" unless ci_events.include?("pull_request")
errors << ".github/workflows/ci.yaml: explicit full validation is missing" unless ci_events.include?("workflow_dispatch")

concurrency = "group: ${{ github.repository }}:${{ github.workflow }}:${{ github.event.pull_request.base.ref || github.ref_name }}:${{ github.event.pull_request.head.repo.id || github.repository_id }}:${{ github.event.pull_request.head.ref || github.ref }}"
errors << ".github/workflows/ci.yaml: concurrency key must unambiguously identify workflow, target, source repository, and source branch" unless ci.include?(concurrency)
errors << ".github/workflows/ci.yaml: superseded revisions must cancel in progress" unless ci.include?("cancel-in-progress: true")

standard_runner = "runs-on: ubuntu-latest"
light_jobs = %w[adl_path_policy adl_tooling_contracts adl_coverage_hosted adl-ci adl-coverage]
ci.scan(/^  ([A-Za-z0-9_-]+):\n(.*?)(?=^  [A-Za-z0-9_-]+:\n|\z)/m).each do |job, body|
  runner_line = body.lines.find { |line| line.match?(/^    runs-on:/) }
  next unless runner_line
  if light_jobs.include?(job)
    errors << "ci.yaml #{job}: light classifier/aggregator must use ubuntu-latest" unless runner_line.strip == "runs-on: ubuntu-latest"
    next
  end
  errors << "ci.yaml #{job}: selected required job must use the standard GitHub-hosted runner" unless body.include?(standard_runner)

  header = body.split("runs-on:", 2).first
  errors << "ci.yaml #{job}: required standard-runner job must depend on adl_path_policy" unless header.include?("adl_path_policy")
  errors << "ci.yaml #{job}: required standard-runner job must have a job-level selector" unless header.match?(/^    if:/)
end

slow = job_block(ci, "adl-slow-proof")
unless slow&.include?("if: github.event_name == 'workflow_dispatch'")
  errors << "ci.yaml adl-slow-proof: long proof must be explicit-dispatch only"
end
if slow&.include?("slow_proof_contract_required == 'true'") || slow&.include?("github.event_name == 'schedule'")
  errors << "ci.yaml adl-slow-proof: PR or schedule may not allocate slow-proof runners"
end

coverage = job_block(ci, "adl_coverage_hosted")
unless coverage&.include?("if: always() && needs.adl_path_policy.outputs.coverage_required == 'true' && needs.adl_path_policy.outputs.heavy_ci_backend == 'hosted'")
  errors << "ci.yaml adl_coverage_hosted: hosted aggregation must skip before allocation when coverage is not required"
end
if coverage&.include?("run_authoritative_coverage_lane.sh")
  errors << "ci.yaml adl_coverage_hosted: hosted aggregation must not re-run Rust coverage"
end
unless ci.include?('if [ "$EVENT_NAME" = pull_request ]; then') && ci.include?("backend=hosted")
  errors << "ci.yaml: pull requests must force the hosted backend before any runner allocation"
end

required_outputs = {
  "automatic_pr_entrypoint" => "ci",
  "optional_workflows_status" => "deferred",
  "optional_workflows_reason" => "explicit_dispatch_required",
  "soak_workflows_status" => "deferred",
  "duplicate_head_status" => "canceled",
  "duplicate_head_reason" => "source_branch_concurrency_cancel_in_progress"
}
policy = root.join("adl/tools/ci_path_policy.sh").read
required_outputs.each do |key, value|
  errors << "ci_path_policy.sh: missing #{key}=#{value}" unless policy.include?("#{key}=\"#{value}\"")
  errors << "ci_path_policy.sh: #{key} is not emitted" unless policy.include?("emit \"#{key}\"")
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
