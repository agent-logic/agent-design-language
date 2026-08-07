#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
EVIDENCE = ROOT.join(".csdlc/evidence/5853")
TARGET = "github-hosted-ubuntu-16-core"
BASE_REQUIRED_FILES = %w[
  experiment-status.json
  eligibility.json
  cost-model.json
  frozen-manifest.json
  progress-snapshot.json
  analysis-integration.json
].freeze
TERMINAL_REQUIRED_FILES = %w[
  trials.jsonl
  optimization-canaries.json
  parity.json
  decision.json
  final-state.json
].freeze
REQUIRED_METRICS = %w[
  queue_seconds
  setup_seconds
  cache_seconds
  compile_link_seconds
  execution_seconds
  artifact_seconds
  total_seconds
  critical_path_seconds
  billed_rounded_minutes
  cost
].freeze

def read_json(path)
  JSON.parse(path.read)
rescue JSON::ParserError => e
  abort "invalid JSON #{path.relative_path_from(ROOT)}: #{e.message}"
end

def number!(value, label)
  number = Float(value)
  abort "#{label} must be finite and nonnegative" unless number.finite? && number >= 0
  number
rescue ArgumentError, TypeError
  abort "#{label} must be numeric"
end

def percentile(values, fraction)
  sorted = values.sort
  return sorted.first if sorted.length == 1
  rank = fraction * (sorted.length - 1)
  lower = sorted[rank.floor]
  upper = sorted[rank.ceil]
  lower + ((upper - lower) * (rank - rank.floor))
end

def variance(values)
  mean = values.sum.fdiv(values.length)
  values.sum { |value| (value - mean)**2 }.fdiv(values.length)
end

def close!(reported, computed, label)
  actual = number!(reported, label)
  tolerance = [computed.abs * 0.001, 0.001].max
  abort "#{label} inconsistent: #{actual} != #{computed}" if (actual - computed).abs > tolerance
end

BASE_REQUIRED_FILES.each do |name|
  path = EVIDENCE.join(name)
  abort "missing evidence #{path.relative_path_from(ROOT)}" unless path.file? && !path.zero?
end

status = read_json(EVIDENCE.join("experiment-status.json"))
phase = status["phase"]
abort "invalid experiment phase" unless %w[collecting complete].include?(phase)

eligibility = read_json(EVIDENCE.join("eligibility.json"))
%w[
  migration_gate ci_reliability_gate organization_plan_ready owner_budget_approved
  budget_alerts_configured spend_alerts_configured selected_repository_access
  concurrency_one rollback_verified
].each do |gate|
  abort "eligibility gate not proven: #{gate}" unless eligibility[gate] == true
end
approved_max_total_cost = number!(eligibility["approved_max_total_cost"], "approved maximum total cost")
abort "approved maximum total cost must equal the operator cap" unless approved_max_total_cost == 10.0

cost_model = read_json(EVIDENCE.join("cost-model.json"))
rate = number!(cost_model.dig("pricing", "candidate_rate_per_minute"), "candidate rate")
abort "candidate rate drifted" unless rate == 0.042
abort "hard-stop budget drifted" unless cost_model.dig("hard_stop", "budget_amount").to_f == approved_max_total_cost
abort "hard-stop enforcement disabled" unless cost_model.dig("hard_stop", "prevent_further_usage") == true
abort "paid work began before cost freeze" unless cost_model.dig("actual_paid_state_before_candidate_wave", "candidate_jobs_started") == 0

manifest = read_json(EVIDENCE.join("frozen-manifest.json"))
%w[commit_sha workflow_revision rust_toolchain lockfile_digest cache_design permissions required_checks workloads].each do |field|
  abort "frozen manifest missing #{field}" if manifest[field].nil? || manifest[field] == ""
end
abort "standard experiment dispatch remains enabled" unless manifest.dig("historical_baseline", "dispatch_allowed") == false
abort "wrong production runner" unless manifest.dig("production_target", TARGET, "workflow_label") == "adl-ubuntu-24.04-16core"
abort "runner concurrency drifted" unless manifest.dig("production_target", TARGET, "maximum_runners") == 1
dimensions = Array(manifest["optimization_dimensions"])
abort "optimization dimension inventory must contain eight unique entries" unless dimensions.length == 8 && dimensions.uniq.length == 8

progress = read_json(EVIDENCE.join("progress-snapshot.json"))
abort "progress phase drift" unless progress["phase"] == phase
abort "progress target drift" unless progress["production_runner"] == "adl-ubuntu-24.04-16core"
abort "progress permits standard-runner dispatch" unless progress["standard_experiment_dispatch_disabled"] == true
abort "progress lost concurrency-one" unless progress["runner_group_concurrency"] == 1
accrued = number!(progress["accrued_rounded_cost"], "progress accrued rounded cost")
abort "progress exceeded approved total cost" if accrued > approved_max_total_cost
Array(progress["completed_paid_runs"]).each do |run|
  abort "completed progress run used wrong runner" unless run["runner_label"] == "adl-ubuntu-24.04-16core"
  abort "completed progress run is not successful" unless run["conclusion"] == "success"
  expected = number!(run["billed_rounded_minutes"], "progress billed minutes") * rate
  close!(run["cost"], expected, "progress run cost")
end
Array(progress["active_or_queued_paid_runs"]).each do |run|
  abort "pending progress run used wrong runner" unless run["runner_label"] == "adl-ubuntu-24.04-16core"
  abort "pending progress run has invalid state" unless %w[queued in_progress].include?(run["status"])
end
analysis = read_json(EVIDENCE.join("analysis-integration.json"))
abort "analysis canary priority drift" unless Array(analysis["canary_priority"]) == Array(manifest["canary_priority"])
abort "warm p50 rollout gate drift" unless analysis.dig("rollout_gates", "warm_p50_seconds_max") == 90
abort "warm p95 rollout gate drift" unless analysis.dig("rollout_gates", "warm_p95_seconds_max") == 120

if phase == "collecting"
  abort "collecting snapshot falsely claims terminal acceptance" unless status["terminal_acceptance"] == false
  puts "WP-02B 16-core evidence snapshot valid: collecting, nonterminal, accrued $#{format('%.3f', accrued)}"
  exit 0
end

TERMINAL_REQUIRED_FILES.each do |name|
  path = EVIDENCE.join(name)
  abort "missing terminal evidence #{path.relative_path_from(ROOT)}" unless path.file? && !path.zero?
end

trials = EVIDENCE.join("trials.jsonl").each_line.filter_map do |line|
  next if line.strip.empty?
  JSON.parse(line)
rescue JSON::ParserError => e
  abort "invalid trial JSON: #{e.message}"
end
abort "no trials retained" if trials.empty?

trials.each do |trial|
  abort "non-production platform entered candidate evidence" unless trial["platform"] == TARGET
  if trial["sample_kind"] == "baseline"
    abort "baseline trial revision drift" unless trial["commit_sha"] == manifest["commit_sha"]
  else
    abort "canary trial lacks exact revision" unless trial["commit_sha"].to_s.match?(/\A[0-9a-f]{40}\z/)
  end
  abort "trial failed or was retried" unless trial["outcome"] == "passed"
  abort "trial lacks runner identity" if trial["runner_name"].to_s.empty? || trial["runner_id"].to_s.empty?
  REQUIRED_METRICS.each { |metric| number!(trial[metric], "#{trial['run_id']} #{metric}") }
  expected_cost = number!(trial["billed_rounded_minutes"], "#{trial['run_id']} billed minutes") * rate
  close!(trial["cost"], expected_cost, "#{trial['run_id']} cost")
  if trial["cache_state"] == "warm"
    abort "warm trial lacks cache-hit evidence" unless trial["cache_hit_evidence"] == true
  end
  abort "trial must classify sample kind" unless %w[baseline optimization_canary production_canary].include?(trial["sample_kind"])
end

baseline = trials.select { |trial| trial["sample_kind"] == "baseline" }
cold = baseline.select { |trial| trial["cache_state"] == "cold" }
warm = baseline.select { |trial| trial["cache_state"] == "warm" }
abort "insufficient 16-core cold samples: #{cold.length}/5" if cold.length < 5
abort "insufficient confirmed 16-core warm samples: #{warm.length}/20" if warm.length < 20
abort "baseline samples must be included in statistics" unless baseline.all? { |trial| trial["included_in_statistics"] == true }

optimization = read_json(EVIDENCE.join("optimization-canaries.json"))
rows = Array(optimization["canaries"])
abort "optimization canaries do not cover the exact frozen dimensions" unless rows.map { |row| row["dimension"] }.sort == dimensions.sort
rows.each do |row|
  abort "optimization canary missing run" if row["run_id"].to_s.empty?
  abort "optimization canary lacks exact revision" unless row["commit_sha"].to_s.match?(/\A[0-9a-f]{40}\z/)
  abort "optimization canary lacks hypothesis" if row["hypothesis"].to_s.empty?
  abort "optimization canary lacks configuration" if row["configuration"].nil?
  abort "optimization canary lacks measured result" if row["result"].nil?
  abort "invalid optimization disposition" unless %w[keep reject].include?(row["disposition"])
  retained = trials.find { |trial| trial["run_id"] == row["run_id"] && trial["sample_kind"] == "optimization_canary" }
  abort "optimization canary is absent from retained trials: #{row['dimension']}" unless retained
  abort "optimization canary revision mismatch: #{row['dimension']}" unless retained["commit_sha"] == row["commit_sha"]
end

production_canaries = trials.select { |trial| trial["sample_kind"] == "production_canary" }
abort "experiment must contain exactly one production canary" unless production_canaries.length == 1

parity = read_json(EVIDENCE.join("parity.json"))
%w[result artifact validation required_check].each do |kind|
  abort "parity failed: #{kind}" unless parity[kind] == true
end

decision = read_json(EVIDENCE.join("decision.json"))
warm_totals = warm.map { |trial| number!(trial["total_seconds"], "warm total") }
statistics = decision["warm_baseline_statistics"] || abort("decision missing warm baseline statistics")
close!(statistics["mean"], warm_totals.sum.fdiv(warm_totals.length), "warm mean")
close!(statistics["p50"], percentile(warm_totals, 0.50), "warm p50")
close!(statistics["p90"], percentile(warm_totals, 0.90), "warm p90")
close!(statistics["p95"], percentile(warm_totals, 0.95), "warm p95")
close!(statistics["variance"], variance(warm_totals), "warm variance")
abort "invalid production recommendation" unless %w[keep disable].include?(decision["recommendation"])
abort "decision lacks concrete production workflow change" if decision["production_workflow_change"].to_s.empty?
abort "decision lacks selected configuration" if decision["selected_configuration"].nil?
abort "production canary did not prove the selected configuration" unless decision["production_canary_passed"] == true
abort "production canary revision mismatch" unless production_canaries.first["commit_sha"] == decision["production_canary_commit_sha"]

total_cost = trials.sum { |trial| number!(trial["cost"], "trial cost") }
abort "experiment exceeded approved total cost" if total_cost > approved_max_total_cost

final_state = read_json(EVIDENCE.join("final-state.json"))
abort "standard experiment dispatch remains enabled" unless final_state["standard_experiment_dispatch_disabled"] == true
abort "selected production runner is not retained" unless final_state["production_runner"] == "adl-ubuntu-24.04-16core"
abort "required-check identity changed" unless final_state["required_check_identity_preserved"] == true
abort "runner-group concurrency changed" unless final_state["runner_group_concurrency"] == 1
abort "warm observation requirement not met" unless final_state["representative_warm_runs"].to_i >= 20
abort "production canary is not terminal-successful" unless final_state["production_canary_passed"] == true
close!(final_state["total_cost"], total_cost, "final total cost")
abort "final state lost approved budget ceiling" unless final_state["approved_max_total_cost"].to_f == approved_max_total_cost

puts "WP-02B 16-core optimization evidence valid: #{cold.length} cold, #{warm.length} warm, #{rows.length} optimization canaries"
