#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

DENOMINATOR = "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json"
SOURCE = "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/retained-v3.json"
OUTPUT = ".csdlc/prepared/issues/819/retained-v3-resolution-plan.json"

AMENDMENTS = {
  "output-filter-removal" => %w[
    V3-A:retained-161-ac-6 V3-A:retained-161-ac-15
    V3-B:retained-164-ac-5 V3-B:retained-164-ac-6
  ],
  "bounded-filesystem-state" => %w[
    V3-A:retained-161-ac-11 V3-A:retained-161-ac-12
    V3-A:retained-161-ac-13 V3-A:retained-161-ac-14
  ],
  "accepted-operational-construction" => %w[
    V3-A:retained-162-ac-6 V3-A:retained-162-ac-7
    V3-B:retained-165-ac-2
  ],
  "synchronous-per-invocation-services" => %w[
    V3-B:retained-165-ac-1 V3-B:retained-165-ac-3
    V3-B:retained-165-ac-4 V3-B:retained-165-ac-5
    V3-B:retained-165-ac-6 V3-B:retained-165-ac-7
    V3-B:retained-165-ac-8 V3-B:retained-165-ac-9
    V3-B:retained-165-ac-10 V3-E:retained-174-ac-1
    V3-E:retained-174-ac-2 V3-E:retained-174-ac-3
    V3-E:retained-177-ac-5 V3-E:retained-177-ac-6
    V3-E:retained-177-ac-7 V3-E:retained-177-ac-8
    V3-E:retained-177-ac-9
  ],
  "projection-state-shape" => %w[
    V3-B:retained-167-ac-1 V3-B:retained-167-ac-5
    V3-C:retained-169-ac-1 V3-C:retained-169-ac-3
  ],
  "supported-platform-scope" => %w[
    V3-A:retained-163-ac-1 V3-A:retained-163-ac-2
    V3-C:retained-169-ac-8 V3-C:retained-169-ac-9
  ],
  "dependency-policy-separation" => %w[V3-B:retained-164-ac-8],
  "process-output-shape" => %w[V3-E:retained-174-ac-5]
}.freeze

TEST_BY_CAUSE = {
  "architecture" => "tests::architecture_boundary",
  "architecture-proof" => "tests::predecessor_coverage",
  "canary" => "foundation_and_local_commands_accept_real_issue_596_with_native_v3_authority",
  "cancellation" => "adapter_outcomes_preserve_status_output_timeout_cancel_and_redaction",
  "capability-matrix" => "local_routes_reject_unsupported_transitions_from_observed_phase",
  "card-phase" => "card_roundtrip_uses_active_registry_denominator",
  "cards" => "card_roundtrip_uses_active_registry_denominator",
  "checkpoint-state" => "part_of_checkpoint_completes_through_end_to_end_delivery_without_cleanup",
  "child-credential-environment-isolation" => "scoped_child_process_receives_only_selected_credential_and_minimal_environment",
  "cleanup" => "post_cutover_cleanup_removes_exact_clean_registered_terminal_worktree",
  "cleanup-relative-input-rejection" => "cleanup_denies_existing_relative_candidate_before_canonicalization",
  "command-surface" => "help_exposes_one_binary_command_surface",
  "contract-delta" => "current_surfaces_agree_with_authenticated_post_cutover_authority",
  "coverage" => "tests::predecessor_coverage",
  "cutover" => "approved_cutover_atomically_installs_selector_and_rollback_receipt",
  "fake-adapter" => "adapter_outcomes_preserve_status_output_timeout_cancel_and_redaction",
  "github-contract" => "authenticated_github_observation_builds_readback_receipts_without_trusting_caller_json",
  "identity" => "same_principal_cannot_self_authorize_publication",
  "import" => "read_only_v2_import_rejects_unsupported_record_fields_and_card_identity_drift",
  "import-unsupported-field-denominator" => "read_only_v2_import_rejects_unsupported_record_fields_and_card_identity_drift",
  "install-proof" => "stable_binary_authenticates_invoking_worktree_before_every_operational_write",
  "issue-readback" => "authenticated_github_observation_builds_readback_receipts_without_trusting_caller_json",
  "linkage" => "publication_modes_require_exact_relation",
  "local-idempotency" => "issue_route_requires_expected_digest_before_overwriting_existing_v3_state",
  "merge-authority" => "executable_cutover_requires_authenticated_github_authority_before_mutation",
  "migration-fence" => "legacy_primary_state_is_preserved_without_any_route_writes",
  "output-contract" => "current_surfaces_agree_with_authenticated_post_cutover_authority",
  "output-stream" => "adapter_outcomes_preserve_status_output_timeout_cancel_and_redaction",
  "parity" => "legacy_proof_requests_cannot_retain_primary_checkout_mutation_authority",
  "parser-adapter-isolation-execution" => "single_binary_foundation_command_is_read_only_and_explicit",
  "process" => "adapter_invocations_are_argv_based_and_shell_strings_are_rejected",
  "proof-integrity" => "transaction_store_rejects_tampered_record_digest_on_ingress",
  "recovery-journey" => "review_recovery_requires_structured_stale_truth_provenance",
  "redaction" => "real_process_adapter_injects_child_credentials_and_redacts_process_output",
  "remote-intent" => "retained_intent_requires_explicit_authenticated_absence_recovery_before_retry",
  "repository-context" => "repository_context_rejects_symlink_escape_for_issue_records",
  "retirement" => "authority_fitness_rejects_obsolete_policy_help_and_automatic_v2_fallback",
  "review" => "review_binds_exact_scope_revision_reviewer_and_findings",
  "state-authority" => "projection_replay_is_deterministic",
  "terminal" => "observed_finish_derives_terminal_closeout_from_github_pr_and_issue_readbacks",
  "topology" => "transition_branch_observation_alone_does_not_authorize_bind",
  "topology-handoff" => "bound_checkout_owns_local_cards_without_primary_checkout_writes",
  "transaction" => "transaction_state_commit_is_atomic_and_projection_failure_requires_repair",
  "transaction-proof" => "recovery_classifies_interrupted_writes_without_losing_provenance",
  "transition" => "transition_matrix_explicitly_classifies_every_state_command_pair",
  "transition-proof" => "transition_matrix_explicitly_classifies_every_state_command_pair",
  "validation-planner" => "operational_schedule_preserves_the_six_dimension_readiness_denominator"
}.freeze

amendment_by_row = {}
AMENDMENTS.each { |kind, ids| ids.each { |id| raise "duplicate amendment #{id}" if amendment_by_row[id]; amendment_by_row[id] = kind } }
denominator = JSON.parse(File.read(DENOMINATOR)).fetch("remediation_rows").select { |row| row.fetch("mapping_file") == "retained-v3.json" }
source_rows = JSON.parse(File.read(SOURCE)).fetch("rows").to_h { |row| [row.fetch("row_id"), row] }

rows = denominator.map do |denom|
  source = source_rows.fetch(denom.fetch("row_id"))
  kind = amendment_by_row[source.fetch("row_id")]
  resolution = if kind
    {
      "type" => "governed_amendment",
      "amendment" => kind,
      "behavioral_pass_claim" => false,
      "authority" => {
        "operator_decision" => "PR #591 merged the V3-F/#505 operational architecture",
        "current_contract" => "docs/csdlc-v3/CURRENT_AUTHORITY.md",
        "contract_detail" => "docs/csdlc-v3/CONTRACT.md",
        "exact_mapping_review" => "required at the final #819 exact head"
      }
    }
  else
    test = TEST_BY_CAUSE[source.fetch("root_cause")]
    raise "no proving test for #{source.fetch('row_id')} cause=#{source.fetch('root_cause')}" unless test
    {
      "type" => "candidate_bound_execution",
      "behavioral_pass_claim" => true,
      "command_id" => "csdlc-v3-all-tests",
      "required_test" => test
    }
  end
  {
    "row_id" => source.fetch("row_id"),
    "criterion_id" => denom.fetch("criterion_id"),
    "criterion_text" => source.fetch("criterion_text"),
    "criterion_text_digest" => denom.fetch("criterion_text_digest"),
    "owner" => denom.fetch("owner"),
    "source_assessment" => source.fetch("proposed_result"),
    "root_cause" => source.fetch("root_cause"),
    "source_evidence_paths" => source.fetch("evidence").map { |entry| entry.fetch("path") }.uniq.sort,
    "resolution" => resolution
  }
end

unknown_amendments = amendment_by_row.keys - rows.map { |row| row.fetch("row_id") }
raise "unknown amendments: #{unknown_amendments.join(', ')}" unless unknown_amendments.empty?
raise "expected 152 rows, got #{rows.length}" unless rows.length == 152

document = {
  "schema" => "adl.v0921.issue819.retained_v3_resolution_plan.v1",
  "issue" => 819,
  "parent_issue" => 522,
  "finding" => "D520-RET-001",
  "denominator" => DENOMINATOR,
  "source_mapping" => SOURCE,
  "row_count" => rows.length,
  "execution_count" => rows.count { |row| row.dig("resolution", "type") == "candidate_bound_execution" },
  "amendment_count" => rows.count { |row| row.dig("resolution", "type") == "governed_amendment" },
  "rows_digest" => Digest::SHA256.hexdigest(rows.map { |row| row.fetch("row_id") }.sort.join("\0")),
  "rows" => rows
}
File.write(OUTPUT, JSON.pretty_generate(document) + "\n")
puts "wrote #{OUTPUT}: #{document['execution_count']} execution, #{document['amendment_count']} governed amendment"
