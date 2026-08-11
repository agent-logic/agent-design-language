#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"
require "yaml"

module CiWorkflowPolicy
  module_function

  AUTOMATIC_EVENTS = %w[pull_request push schedule workflow_run].freeze
  OPTIONAL_JOB_IDS = %w[
    adl_demo_proof
    adl_spot_ci_and_coverage
    adl-slow-proof
    adl_coverage_runtime_hosted
    adl_coverage_workspace_fast_hosted
    adl_coverage_workspace_hosted
    adl_coverage_hosted
  ].freeze
  REQUIRED_PR_JOB_IDS = %w[adl_path_policy adl_ci adl_coverage].freeze

  def parse(source, path)
    value = YAML.safe_load(source, permitted_classes: [], permitted_symbols: [], aliases: true)
    raise "#{path}: workflow root must be a mapping" unless value.is_a?(Hash)

    value
  rescue Psych::Exception => error
    raise "#{path}: invalid YAML: #{error.message}"
  end

  def event_names(document)
    value = document["on"] || document[true]
    case value
    when String then [value]
    when Array then value.map(&:to_s)
    when Hash then value.keys.map(&:to_s)
    else []
    end
  end

  def jobs(document)
    value = document["jobs"]
    value.is_a?(Hash) ? value.transform_keys(&:to_s) : {}
  end

  def matrix_cardinality(job)
    strategy = job.is_a?(Hash) ? job["strategy"] : nil
    matrix = strategy.is_a?(Hash) ? strategy["matrix"] : nil
    return 1 unless matrix.is_a?(Hash)

    axes = matrix.reject { |key, _| %w[include exclude].include?(key.to_s) }
    product = axes.values.reduce(1) do |count, values|
      count * (values.is_a?(Array) ? values.length : 1)
    end
    include_count = matrix["include"].is_a?(Array) ? matrix["include"].length : 0
    [product, include_count].max
  end

  def validate_sources(sources)
    errors = []
    inventory = []
    ci_path = ".github/workflows/ci.yaml"

    sources.sort.each do |path, source|
      document = parse(source, path)
      events = event_names(document)
      automatic = events & AUTOMATIC_EVENTS
      if path == ci_path
        errors << "#{path}: pull_request entrypoint is missing" unless events.include?("pull_request")
        errors << "#{path}: workflow_dispatch entrypoint is missing" unless events.include?("workflow_dispatch")
        disallowed = events - %w[pull_request workflow_dispatch]
        errors << "#{path}: automatic event #{disallowed.join(',')} is not allowed" unless disallowed.empty?
      else
        errors << "#{path}: automatic event #{automatic.join(',')} is not allowed" unless automatic.empty?
        errors << "#{path}: standalone workflow must remain explicitly dispatchable" unless events.include?("workflow_dispatch")
        disallowed = events - %w[workflow_dispatch workflow_call]
        errors << "#{path}: event #{disallowed.join(',')} is not allowed" unless disallowed.empty?
      end
      inventory << { "path" => path, "events" => events, "automatic" => automatic }
    rescue RuntimeError => error
      errors << error.message
    end

    ci_source = sources[ci_path]
    unless ci_source
      errors << "#{ci_path}: workflow is missing"
      return [errors, inventory]
    end

    ci = parse(ci_source, ci_path)
    ci_jobs = jobs(ci)
    missing = REQUIRED_PR_JOB_IDS - ci_jobs.keys
    extra = ci_jobs.keys - REQUIRED_PR_JOB_IDS
    errors << "#{ci_path}: required PR jobs missing: #{missing.join(',')}" unless missing.empty?
    errors << "#{ci_path}: unrelated or optional PR jobs materialize: #{extra.join(',')}" unless extra.empty?
    present_optional = OPTIONAL_JOB_IDS & ci_jobs.keys
    errors << "#{ci_path}: optional jobs must not materialize: #{present_optional.join(',')}" unless present_optional.empty?

    concurrency = ci["concurrency"]
    group = concurrency.is_a?(Hash) ? concurrency["group"].to_s : ""
    errors << "#{ci_path}: concurrency must identify repository, workflow, source repository, and source head" unless
      %w[github.repository github.workflow pull_request.head.repo.full_name pull_request.head.ref].all? { |token| group.include?(token) }
    errors << "#{ci_path}: same-head PRs must coalesce across bases" if group.include?("pull_request.base.ref")
    errors << "#{ci_path}: superseded runs must cancel" unless concurrency.is_a?(Hash) && concurrency["cancel-in-progress"] == true

    heavy_consumers = ci_jobs.each_with_object([]) do |(job_id, job), selected|
      runner = job.is_a?(Hash) ? job["runs-on"].to_s : ""
      selected << job_id if runner.include?("required_runner")
    end
    errors << "#{ci_path}: exactly one job must consume required_runner" unless heavy_consumers == ["adl_ci"]
    direct_heavy = ci_jobs.each_with_object([]) do |(job_id, job), selected|
      runner = job.is_a?(Hash) ? job["runs-on"].to_s : ""
      selected << job_id if runner.include?("ADL_HEAVY_RUNNER")
    end
    errors << "#{ci_path}: jobs must not bypass the sole required_runner selector: #{direct_heavy.join(',')}" unless direct_heavy.empty?

    heavy_expansions = heavy_consumers.sum { |job_id| matrix_cardinality(ci_jobs.fetch(job_id)) }
    errors << "#{ci_path}: heavy-runner matrix expansion count must be exactly one, got #{heavy_expansions}" unless heavy_expansions == 1

    [errors, inventory]
  end

  def validate_root(root)
    workflow_dir = Pathname.new(root).join(".github/workflows")
    sources = Dir[workflow_dir.join("*.{yml,yaml}").to_s].sort.to_h do |path|
      relative = Pathname.new(path).relative_path_from(Pathname.new(root)).to_s
      [relative, File.read(path)]
    end
    validate_sources(sources)
  end
end

if $PROGRAM_NAME == __FILE__
  root = Pathname.new(ARGV[0] || Pathname.new(__dir__).join("../..")).cleanpath
  errors, inventory = CiWorkflowPolicy.validate_root(root)
  result = {
    "schema" => "adl.ci.workflow-policy.v2",
    "status" => errors.empty? ? "pass" : "fail",
    "automatic_pr_entrypoint" => ".github/workflows/ci.yaml",
    "required_pr_jobs" => CiWorkflowPolicy::REQUIRED_PR_JOB_IDS,
    "ordinary_pr_heavy_runner_max" => 1,
    "optional_policy" => "explicit_dispatch_only",
    "workflows" => inventory,
    "errors" => errors
  }
  puts JSON.pretty_generate(result)
  exit(errors.empty? ? 0 : 1)
end
