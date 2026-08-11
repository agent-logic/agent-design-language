#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

root = Pathname.new(__dir__).join("../..").cleanpath
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
  automatic = events & %w[pull_request push schedule]

  errors << "#{relative}: automatic event #{automatic.join(',')} is not allowed" if path != ci_path && automatic.any?
  errors << "#{relative}: scheduled CI is optional and must require explicit dispatch" if events.include?("schedule")
  errors << "#{relative}: push-triggered validation duplicates reviewed PR validation" if events.include?("push")
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

concurrency = "group: ${{ github.repository }}-${{ github.workflow }}-${{ github.event.pull_request.head.sha || github.sha }}"
errors << ".github/workflows/ci.yaml: concurrency must coalesce repository/head SHA" unless ci.include?(concurrency)
errors << ".github/workflows/ci.yaml: superseded revisions must cancel in progress" unless ci.include?("cancel-in-progress: true")

heavy_selector = "runs-on: ${{ vars.ADL_HEAVY_RUNNER || 'adl-ubuntu-24.04-16core' }}"
ci.scan(/^  ([A-Za-z0-9_-]+):\n(.*?)(?=^  [A-Za-z0-9_-]+:\n|\z)/m).each do |job, body|
  next unless body.include?(heavy_selector)

  header = body.split("runs-on:", 2).first
  errors << "ci.yaml #{job}: required heavy job must depend on adl_path_policy" unless header.include?("adl_path_policy")
  errors << "ci.yaml #{job}: required heavy job must have a job-level selector" unless header.match?(/^    if:/)
end

slow = job_block(ci, "adl-slow-proof")
unless slow&.include?("if: github.event_name == 'workflow_dispatch'")
  errors << "ci.yaml adl-slow-proof: long proof must be explicit-dispatch only"
end
if slow&.include?("slow_proof_contract_required == 'true'") || slow&.include?("github.event_name == 'schedule'")
  errors << "ci.yaml adl-slow-proof: PR or schedule may not allocate slow-proof runners"
end

coverage = job_block(ci, "adl_coverage_hosted")
unless coverage&.include?("if: always() && needs.adl_path_policy.outputs.coverage_required == 'true'")
  errors << "ci.yaml adl_coverage_hosted: heavy aggregation must skip before allocation when coverage is not required"
end

required_outputs = {
  "automatic_pr_entrypoint" => "ci",
  "optional_workflows_status" => "deferred",
  "optional_workflows_reason" => "explicit_dispatch_required",
  "soak_workflows_status" => "deferred",
  "duplicate_head_status" => "canceled",
  "duplicate_head_reason" => "head_sha_concurrency_cancel_in_progress"
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
  "required_heavy_runner" => "vars.ADL_HEAVY_RUNNER",
  "optional_policy" => "explicit_dispatch_only",
  "scheduled_heavy_validation" => "disabled",
  "workflow_count" => inventory.length,
  "workflows" => inventory,
  "errors" => errors
}
puts JSON.pretty_generate(result)
exit(errors.empty? ? 0 : 1)
