#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" >/dev/null
ruby "$ROOT_DIR/adl/tools/test_validate_ci_workflow_policy.rb"

ruby -ryaml - "$ROOT_DIR/.github/workflows/ci.yaml" "$ROOT_DIR/.github/workflows/ci-out-of-band.yaml" <<'RUBY'
def load_workflow(path)
  YAML.safe_load(File.read(path), permitted_classes: [], permitted_symbols: [], aliases: true)
end

ci = load_workflow(ARGV.fetch(0))
out_of_band = load_workflow(ARGV.fetch(1))
ci_jobs = ci.fetch("jobs").keys
expected = %w[adl_path_policy adl_ci adl_coverage]
abort "ordinary PR workflow must expose only required checks" unless ci_jobs == expected

runner = ci.fetch("jobs").fetch("adl_ci").fetch("runs-on").to_s
abort "adl-ci must be the sole required-runner consumer" unless runner.include?("required_runner")
abort "adl-ci must not define a matrix" if ci.fetch("jobs").fetch("adl_ci").dig("strategy", "matrix")

manual_events = out_of_band["on"] || out_of_band[true]
manual_events = manual_events.keys.map(&:to_s) if manual_events.is_a?(Hash)
abort "out-of-band workflow must be dispatch-only" unless manual_events == ["workflow_dispatch"]

manual_jobs = out_of_band.fetch("jobs").keys
%w[adl_demo_proof adl_spot_ci_and_coverage adl-slow-proof adl_coverage_runtime_hosted adl_coverage_workspace_fast_hosted adl_coverage_workspace_hosted adl_coverage_hosted].each do |job|
  abort "out-of-band workflow lost #{job}" unless manual_jobs.include?(job)
end

puts "PASS test_ci_runtime_contracts"
RUBY
