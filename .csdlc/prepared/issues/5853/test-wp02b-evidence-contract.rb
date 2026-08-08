#!/usr/bin/env ruby
# frozen_string_literal: true

require "minitest/autorun"
require_relative "wp02b_evidence_contract"

class Wp02bEvidenceContractTest < Minitest::Test
  HEAD = "a" * 40
  RUNNER = "adl-ubuntu-24.04-16core"

  def base_canary
    {
      "schema" => Wp02bEvidenceContract::PRODUCTION_CANARY_SCHEMA,
      "head_sha" => HEAD,
      "workflow_conclusion" => "success",
      "job_name" => "adl-rust-tests",
      "job_conclusion" => "success",
      "labels" => [RUNNER],
      "runner_name" => "#{RUNNER}-1000000001",
      "terminal_acceptance" => true,
      "proof_paths" => {
        "direct_test" => {
          "test_step_conclusion" => "success",
          "doc_test_step_conclusion" => "success"
        },
        "coverage_replacement" => {
          "authoritative" => true,
          "producer_jobs" => [],
          "aggregator_jobs" => []
        }
      }
    }
  end

  def test_accepts_successful_direct_test_and_doc_test_steps
    assert_nil Wp02bEvidenceContract.production_canary_error(
      base_canary, expected_head: HEAD, runner_label: RUNNER
    )
  end

  def test_rejects_coverage_as_a_substitute_for_direct_steps
    canary = base_canary
    canary["proof_paths"]["direct_test"] = {
      "test_step_conclusion" => "skipped",
      "doc_test_step_conclusion" => "skipped"
    }
    canary["proof_paths"]["coverage_replacement"]["producer_jobs"] = [
      { "name" => "adl-coverage-runtime-hosted", "conclusion" => "success" },
      { "name" => "adl-coverage-workspace-hosted (1/2)", "conclusion" => "success" },
      { "name" => "adl-coverage-workspace-hosted (2/2)", "conclusion" => "success" }
    ]
    canary["proof_paths"]["coverage_replacement"]["aggregator_jobs"] = [
      { "name" => "adl-coverage-hosted", "conclusion" => "success" },
      { "name" => "adl-coverage", "conclusion" => "success" }
    ]

    assert_match(/direct test and doc-test steps did not complete successfully/,
                 Wp02bEvidenceContract.production_canary_error(
                   canary, expected_head: HEAD, runner_label: RUNNER
                 ))
  end

  def test_rejects_skipped_direct_steps_and_failed_coverage
    canary = base_canary
    canary["proof_paths"]["direct_test"] = {
      "test_step_conclusion" => "skipped",
      "doc_test_step_conclusion" => "skipped"
    }
    canary["proof_paths"]["coverage_replacement"]["producer_jobs"] = [
      { "name" => "adl-coverage-runtime-hosted", "conclusion" => "success" },
      { "name" => "adl-coverage-workspace-hosted (1/2)", "conclusion" => "success" },
      { "name" => "adl-coverage-workspace-hosted (2/2)", "conclusion" => "cancelled" }
    ]
    canary["proof_paths"]["coverage_replacement"]["aggregator_jobs"] = [
      { "name" => "adl-coverage-hosted", "conclusion" => "failure" },
      { "name" => "adl-coverage", "conclusion" => "failure" }
    ]

    assert_match(/direct test and doc-test steps did not complete successfully/,
                 Wp02bEvidenceContract.production_canary_error(
                   canary, expected_head: HEAD, runner_label: RUNNER
                 ))
  end

  def test_rejects_cancelled_workflow_even_when_job_shell_is_green
    canary = base_canary
    canary["workflow_conclusion"] = "cancelled"

    assert_match(/workflow did not complete successfully/,
                 Wp02bEvidenceContract.production_canary_error(
                   canary, expected_head: HEAD, runner_label: RUNNER
                 ))
  end

  def test_rejects_stale_head
    assert_match(/head is not the reviewed Git head/,
                 Wp02bEvidenceContract.production_canary_error(
                   base_canary, expected_head: "b" * 40, runner_label: RUNNER
                 ))
  end

  def test_rejects_false_terminal_acceptance
    canary = base_canary
    canary["terminal_acceptance"] = false

    assert_match(/terminal acceptance is false/,
                 Wp02bEvidenceContract.production_canary_error(
                   canary, expected_head: HEAD, runner_label: RUNNER
                 ))
  end

  def test_paid_run_success_depends_on_retained_outcomes
    passed = {
      "workflow_conclusion" => "success",
      "job_conclusion" => "success",
      "workload_step_conclusion" => "success",
      "artifact_step_conclusion" => "success"
    }
    failed = passed.merge("workload_step_conclusion" => "failure")

    assert Wp02bEvidenceContract.successful_paid_run?(passed)
    refute Wp02bEvidenceContract.successful_paid_run?(failed)
  end
end
