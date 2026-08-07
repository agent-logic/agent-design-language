#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
EVIDENCE = ROOT.join(".csdlc/evidence/5853")
RUN_IDS = %w[
  31143391281 31143497492 31143499581 31143501603
  31143972613 31143974425 31143976182
].freeze
SEED_RUN_ID = "31143176102"
ACCOUNTED_RUN_IDS = [SEED_RUN_ID, *RUN_IDS].freeze
BASELINE_SHA = "085736a546e62a51d242626fd42f4bda07ff12ea"
CANARY_SHA = "c68b5dac56a351c88a465e8510a1dbc4fcdf8e88"
RUNNER_LABEL = "adl-ubuntu-24.04-16core"

def read_json(path)
  JSON.parse(path.read)
rescue JSON::ParserError => e
  abort "invalid JSON #{path.relative_path_from(ROOT)}: #{e.message}"
end

def percentile(values, fraction)
  sorted = values.sort
  rank = fraction * (sorted.length - 1)
  lower = sorted[rank.floor]
  upper = sorted[rank.ceil]
  lower + ((upper - lower) * (rank - rank.floor))
end

def close!(actual, expected, label)
  abort "#{label} drifted: #{actual} != #{expected}" if (Float(actual) - expected).abs > 0.001
end

%w[
  eligibility.json cost-model.json frozen-manifest.json experiment-status.json
  decision.json final-state.json run-accounting.json production-canary.json
].each do |name|
  path = EVIDENCE.join(name)
  abort "missing evidence #{path.relative_path_from(ROOT)}" unless path.file? && !path.zero?
end

eligibility = read_json(EVIDENCE.join("eligibility.json"))
%w[
  migration_gate ci_reliability_gate organization_plan_ready owner_budget_approved
  budget_alerts_configured spend_alerts_configured selected_repository_access
  concurrency_one rollback_verified
].each { |gate| abort "eligibility gate not proven: #{gate}" unless eligibility[gate] == true }
abort "cost ceiling drifted" unless eligibility["approved_max_total_cost"] == 10

status = read_json(EVIDENCE.join("experiment-status.json"))
abort "measurement status drifted" unless status["phase"] == "measurement_complete" && status["terminal_acceptance"] == false

manifest = read_json(EVIDENCE.join("frozen-manifest.json"))
abort "wrong selected runner" unless manifest.dig("production_target", "github-hosted-ubuntu-16-core", "workflow_label") == RUNNER_LABEL
abort "runner concurrency drifted" unless manifest.dig("production_target", "github-hosted-ubuntu-16-core", "maximum_runners") == 1
abort "standard comparison was re-enabled" unless manifest.dig("historical_baseline", "dispatch_allowed") == false
abort "trial denominator drifted" unless manifest["trial_counts"] == {
  "cold_baseline" => 1, "warm_baseline" => 3, "cache_seed_total" => 1, "test_only_canary" => 3
}
thresholds = manifest.fetch("adoption_thresholds")
close!(thresholds.fetch("minimum_workload_reduction_fraction"), 0.35, "minimum reduction threshold")
close!(thresholds.fetch("minimum_reliability"), 1.0, "minimum reliability threshold")
close!(thresholds.fetch("maximum_test_only_p95_seconds"), 120.0, "maximum test-only p95")
close!(thresholds.fetch("maximum_cost_usd"), 10.0, "maximum cost threshold")

runs = RUN_IDS.map do |id|
  root = EVIDENCE.join("run-#{id}")
  runner_paths = Pathname.glob(root.join("*/runner.json").to_s)
  benchmark_paths = Pathname.glob(root.join("*/benchmark.json").to_s)
  abort "run #{id} must retain exactly one runner and benchmark receipt" unless runner_paths.length == 1 && benchmark_paths.length == 1
  [id, read_json(runner_paths.first), read_json(benchmark_paths.first)]
end

runs.each do |id, runner, benchmark|
  abort "run id mismatch" unless runner["workflow_run_id"] == id
  abort "wrong runner platform" unless runner["platform"] == "candidate" && runner["cpu_count"] == 16
  abort "wrong runner OS" unless runner["runner_os"] == "Linux" && runner["image_os"] == "ubuntu24"
  abort "toolchain drifted" unless runner["rustc_version"].start_with?("rustc 1.92.0") && runner["cargo_version"].start_with?("cargo 1.92.0")
  abort "benchmark failed" unless benchmark["status"] == "passed"
  abort "benchmark platform drifted" unless benchmark["platform"] == "github-hosted-candidate"
  abort "invalid elapsed time" unless benchmark["total_elapsed_seconds"].is_a?(Numeric) && benchmark["total_elapsed_seconds"].positive?
end

baseline = runs.select { |_, runner, _| runner["commit_sha"] == BASELINE_SHA }
canaries = runs.select { |_, runner, _| runner["commit_sha"] == CANARY_SHA }
abort "baseline denominator drifted" unless baseline.length == 4
abort "canary denominator drifted" unless canaries.length == 3

cold = baseline.select { |_, runner, _| runner["cache_state"] == "cold" && runner["cache_hit"] == false }
warm = baseline.select { |_, runner, _| runner["cache_state"] == "warm" && runner["cache_hit"] == true }
test_only = canaries.select do |_, runner, benchmark|
  runner["cache_state"] == "warm" && runner["cache_hit"] == true &&
    runner["sample_role"] == "canary" && runner["variant"] == "test_only" &&
    benchmark["build_command"] == "skipped:test_only"
end
abort "cold denominator drifted" unless cold.length == 1
abort "warm denominator drifted" unless warm.length == 3
abort "test-only denominator drifted" unless test_only.length == 3

decision = read_json(EVIDENCE.join("decision.json"))
warm_seconds = warm.map { |_, _, benchmark| benchmark["total_elapsed_seconds"] }
test_seconds = test_only.map { |_, _, benchmark| benchmark["total_elapsed_seconds"] }
warm_p50 = percentile(warm_seconds, 0.50)
warm_p95 = percentile(warm_seconds, 0.95)
test_p50 = percentile(test_seconds, 0.50)
test_p95 = percentile(test_seconds, 0.95)
close!(decision.dig("evidence", "warm_baseline_p50_seconds"), warm_p50, "warm p50")
close!(decision.dig("evidence", "warm_baseline_p95_seconds"), warm_p95, "warm p95")
close!(decision.dig("evidence", "test_only_p50_seconds"), test_p50, "test-only p50")
close!(decision.dig("evidence", "test_only_p95_seconds"), test_p95, "test-only p95")
close!(decision.dig("evidence", "median_workload_reduction_fraction"), (warm_p50 - test_p50) / warm_p50, "median reduction")
successful_paid_runs = ACCOUNTED_RUN_IDS.length
reliability = successful_paid_runs.fdiv(ACCOUNTED_RUN_IDS.length)
abort "successful paid-run denominator drifted" unless decision.dig("evidence", "successful_paid_runs") == successful_paid_runs
close!(decision.dig("evidence", "candidate_reliability"), reliability, "candidate reliability")
abort "adoption threshold not met" unless test_p95 <= thresholds["maximum_test_only_p95_seconds"] &&
  (warm_p50 - test_p50) / warm_p50 >= thresholds["minimum_workload_reduction_fraction"] &&
  reliability >= thresholds["minimum_reliability"]
abort "wrong production decision" unless decision["decision"] == "adopt" && decision["selected_route"] == "adl-rust-tests"

cost = read_json(EVIDENCE.join("cost-model.json"))
accounting = read_json(EVIDENCE.join("run-accounting.json"))
rate = accounting.fetch("rate_per_rounded_job_minute_usd")
close!(rate, 0.042, "candidate rate")
accounted = accounting.fetch("runs")
abort "run-accounting denominator drifted" unless accounted.map { |run| run["workflow_run_id"] } == ACCOUNTED_RUN_IDS
total_minutes = 0
total_cost = 0.0
accounted.each do |run|
  abort "negative queue or wall time" unless run["queue_seconds"].is_a?(Integer) && run["queue_seconds"] >= 0 &&
    run["job_wall_seconds"].is_a?(Integer) && run["job_wall_seconds"].positive?
  billed = (run["job_wall_seconds"] / 60.0).ceil
  run_cost = billed * rate
  abort "billed-minute drift for #{run['workflow_run_id']}" unless run["billed_rounded_minutes"] == billed
  close!(run["cost_usd"], run_cost, "run cost #{run['workflow_run_id']}")
  total_minutes += billed
  total_cost += run_cost
  close!(run["cumulative_cost_usd"], total_cost, "cumulative cost #{run['workflow_run_id']}")
  close!(run["remaining_budget_usd"], 10.0 - total_cost, "remaining budget #{run['workflow_run_id']}")
end
close!(decision.dig("evidence", "total_billed_minutes"), total_minutes, "decision billed minutes")
close!(decision.dig("evidence", "total_cost_usd"), total_cost, "decision cost")
abort "cost-model billed minutes drifted" unless cost.dig("actual_paid_state", "candidate_rounded_minutes") == total_minutes
close!(cost.dig("actual_paid_state", "candidate_cost"), total_cost, "candidate cost")
abort "candidate cost exceeded budget" unless total_cost < thresholds["maximum_cost_usd"]

canary = read_json(EVIDENCE.join("production-canary.json"))
abort "production routing canary failed" unless canary["proof_role"] == "routing_canary" &&
  canary["job_name"] == "adl-rust-tests" && canary["conclusion"] == "success"
abort "production canary runner drifted" unless canary.fetch("labels").include?(RUNNER_LABEL) &&
  canary["runner_name"].start_with?(RUNNER_LABEL)

workflow = ROOT.join(".github/workflows/ci.yaml").read
rust_job = workflow[/^  adl_rust_tests:\n.*?(?=^  [a-zA-Z0-9_-]+:\n|\z)/m] || abort("missing adl_rust_tests job")
abort "Rust test lane is not on the selected runner" unless rust_job.include?("runs-on: #{RUNNER_LABEL}")
abort "experiment harness was not removed" if workflow.include?("build_acceleration_experiment:")

final_state = read_json(EVIDENCE.join("final-state.json"))
abort "final-state runner drifted" unless final_state["production_runner"] == RUNNER_LABEL
abort "proof semantics changed" unless final_state["required_check_identity_preserved"] && final_state["validation_breadth_preserved"]
abort "security boundary drifted" unless final_state["selected_repository_access"] && !final_state["untrusted_fork_privilege"]
abort "terminal canary gate missing" unless final_state.dig("production_canary", "terminal_gate")&.include?("final reviewed PR head")

puts "WP-02B valid: 1 cold, 3 warm, 3 test-only canaries; p50 #{warm_p50}s -> #{test_p50}s; #{total_minutes} billed minutes; $#{format('%.3f', total_cost)}"
