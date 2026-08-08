# frozen_string_literal: true

module Wp02bEvidenceContract
  PRODUCTION_CANARY_SCHEMA = "adl.wp02b.production_canary.v2"
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

    return "production canary terminal acceptance is false" unless canary["terminal_acceptance"] == true
    return nil if direct_passed

    "direct test and doc-test steps did not complete successfully"
  end
end
