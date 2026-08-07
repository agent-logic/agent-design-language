# frozen_string_literal: true

module Wp02bEvidenceContract
  PRODUCTION_CANARY_SCHEMA = "adl.wp02b.production_canary.v2"
  REQUIRED_COVERAGE_PRODUCERS = [
    "adl-coverage-runtime-hosted",
    "adl-coverage-workspace-hosted (1/2)",
    "adl-coverage-workspace-hosted (2/2)"
  ].freeze
  REQUIRED_COVERAGE_AGGREGATORS = %w[adl-coverage-hosted adl-coverage].freeze

  module_function

  def successful_paid_run?(outcome)
    outcome["workflow_conclusion"] == "success" &&
      outcome["job_conclusion"] == "success" &&
      outcome["workload_step_conclusion"] == "success" &&
      outcome["artifact_step_conclusion"] == "success"
  end

  def production_canary_error(canary, expected_head:, runner_label:)
    return "production canary schema drifted" unless canary["schema"] == PRODUCTION_CANARY_SCHEMA
    return "production canary head is not the reviewed Git head" unless canary["head_sha"] == expected_head
    return "production canary workflow did not complete successfully" unless canary["workflow_conclusion"] == "success"
    return "production canary job did not complete successfully" unless canary["job_name"] == "adl-rust-tests" && canary["job_conclusion"] == "success"
    return "production canary runner label drifted" unless canary.fetch("labels", []).include?(runner_label)
    return "production canary runner name drifted" unless canary.fetch("runner_name", "").start_with?(runner_label)

    direct = canary.fetch("proof_paths", {}).fetch("direct_test", {})
    direct_passed = direct["test_step_conclusion"] == "success" &&
      direct["doc_test_step_conclusion"] == "success"

    coverage = canary.fetch("proof_paths", {}).fetch("coverage_replacement", {})
    producers = coverage.fetch("producer_jobs", [])
    aggregators = coverage.fetch("aggregator_jobs", [])
    coverage_passed = coverage["authoritative"] == true &&
      producers.map { |job| job["name"] }.sort == REQUIRED_COVERAGE_PRODUCERS.sort &&
      aggregators.map { |job| job["name"] }.sort == REQUIRED_COVERAGE_AGGREGATORS.sort &&
      producers.all? { |job| job["conclusion"] == "success" } &&
      aggregators.all? { |job| job["conclusion"] == "success" }

    return "production canary terminal acceptance is false" unless canary["terminal_acceptance"] == true
    return nil if direct_passed || coverage_passed

    "neither direct test/doc-test steps nor the authoritative coverage replacement completed successfully"
  end
end
