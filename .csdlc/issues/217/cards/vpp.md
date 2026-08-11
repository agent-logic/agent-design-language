# Validation Planning Prompt

Template: 1.0.0

Issue: 217

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/217/design.md

Diagram: .csdlc/prepared/issues/217/diagram.mmd

## Selected Lanes

[
  {
    "lane": "historical-denominator-contract",
    "proof_role": "Execute exact ten-path/count/unique/SHA-256 structural validation for the historical denominator.",
    "acceptance_ids": [
      "AC-1",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
    "argv": [
      "jq",
      "-e",
      ".expected_file_count == 10 and (.files | length) == 10 and ([.files[].path] | unique | length) == 10 and ([.files[].sha256 | test(\"^[0-9a-f]{64}$\")] | all)",
      ".csdlc/prepared/issues/217/historical-c640-denominator.json"
    ],
    "parallel_group": "217-prep",
    "defer_reason": null
  },
  {
    "lane": "protected-source-denominator-contract",
    "proof_role": "Execute exact seventeen-path/count/unique validation for the independent protected-source denominator.",
    "acceptance_ids": [
      "AC-4",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
    "argv": [
      "jq",
      "-e",
      ".expected_path_count == 17 and (.paths | length) == 17 and (.paths | unique | length) == 17",
      ".csdlc/prepared/issues/217/protected-source-denominator.json"
    ],
    "parallel_group": "217-prep",
    "defer_reason": null
  },
  {
    "lane": "retention-allowlist-contract",
    "proof_role": "Execute exact proof-contract equality plus H2/H3 lifecycle, evidence, retained-surface manifest, status, and confinement validation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 300,
    "argv": [
      "jq",
      "-e",
      "--slurpfile",
      "proof",
      ".csdlc/prepared/issues/217/proof-contract-paths.json",
      ".expected_evidence_file_count == 10 and ($proof[0].expected_path_count) == 8 and ($proof[0].paths | length) == 8 and ($proof[0].paths | unique | length) == 8 and .expected_proof_path_count == 8 and .proof_paths == ($proof[0].paths) and (.proof_paths | unique | length) == 8 and .expected_lifecycle_path_count == 14 and (.lifecycle_paths | length) == 14 and (.lifecycle_paths | unique | length) == 14 and .expected_retained_surface_fixed_path_count == 9 and (.retained_surface_fixed_paths | length) == 9 and (.retained_surface_fixed_paths | unique | length) == 9 and .retained_surface_fixed_paths == ([.evidence_denominator_path] + .proof_paths) and .expected_retained_surface_entry_count == 19 and .retained_surface_dynamic_path_source.expected_count == 10 and .retained_surface_dynamic_path_source.path == .evidence_denominator_path and (.retained_surface_manifest_path | startswith(\".csdlc/evidence/217/\")) and .retained_surface_manifest_digest_binding == \"h3_review_receipt\" and .h3_allowed_addition_paths == [.review_receipt_path] and .review_receipt_anchor_policy.kind == \"unique_ancestral_introduction_blob\" and .review_receipt_anchor_policy.require_unique_first_addition_on_current_head_ancestry == true and .review_receipt_anchor_policy.require_anchor_commit_object == true and .review_receipt_anchor_policy.require_anchor_blob_object == true and .review_receipt_anchor_policy.objects_permitted_missing == [\"H2_commit\",\"H2_tree\"] and (.review_receipt_anchor_policy.fail_closed_cases | index(\"coherent_receipt_and_manifest_rewrite\")) != null and .later_head_policy.requires_h2_git_objects == false and .later_head_policy.requires_receipt_anchor_git_objects == true and .allowed_statuses == [\"A\",\"M\"] and .forbidden_statuses == [\"D\",\"R\",\"C\",\"T\",\"U\",\"X\",\"B\"] and (.evidence_denominator_path | startswith(\".csdlc/evidence/217/\")) and ([.lifecycle_paths[] | startswith(\".csdlc/issues/217/\")] | all)",
      ".csdlc/prepared/issues/217/h2-retention-allowlist.json"
    ],
    "parallel_group": "217-prep",
    "defer_reason": null
  },
  {
    "lane": "historical-c640-packet",
    "proof_role": "Create a detached c640 worktree, overlay exact historical evidence, set original GitHub environment, and run the unchanged #209 validator as provenance-only proof.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/verify-historical-c640-packet.rb",
      ".csdlc/prepared/issues/217/historical-c640-denominator.json"
    ],
    "parallel_group": "217-historical",
    "defer_reason": "Implemented only after the fresh second independent full-package review passes."
  },
  {
    "lane": "fresh-native-producer-contract",
    "proof_role": "Prove the producer consumes exact source/proof denominators and writes confined issue #217 artifacts.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/produce-native-receipt.rb",
      "--self-test"
    ],
    "parallel_group": "217-contract",
    "defer_reason": "Implemented only after the fresh second independent full-package review passes."
  },
  {
    "lane": "fresh-native-linux-macos",
    "proof_role": "At reviewed H, produce/aggregate the fresh packet and exact nineteen-entry retained-surface manifest; validate evidence and proof-contract digests before exact-allowlist H2 retention.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/validate-retained-native-proof.rb",
      ".csdlc/evidence/217/retained-proof-denominator.json"
    ],
    "parallel_group": "217-native",
    "defer_reason": "Runs on GitHub Actions at exact reviewed H; missing fresh proof blocks H2 retention and merge."
  },
  {
    "lane": "retention-chain-regressions",
    "proof_role": "Prove H/H2 and H2/H3 allowlisting, retained-surface manifest plus unique ancestral H3/integration receipt-blob anchoring, validation with H2 commit/tree objects unavailable, and fail-closed coherent receipt+manifest rewrite, missing/ambiguous anchor, protected, H-to-H2 unprotected-source, producer, validator, workflow, semantic, provenance, and path drift.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 420,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/validate-retained-native-proof.rb",
      "--self-test"
    ],
    "parallel_group": "217-contract",
    "defer_reason": "Implemented only after the fresh second independent full-package review passes."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `jq -e .expected_file_count == 10 and (.files | length) == 10 and ([.files[].path] | unique | length) == 10 and ([.files[].sha256 | test("^[0-9a-f]{64}$")] | all) .csdlc/prepared/issues/217/historical-c640-denominator.json`
- `jq -e .expected_path_count == 17 and (.paths | length) == 17 and (.paths | unique | length) == 17 .csdlc/prepared/issues/217/protected-source-denominator.json`
- `jq -e --slurpfile proof .csdlc/prepared/issues/217/proof-contract-paths.json .expected_evidence_file_count == 10 and ($proof[0].expected_path_count) == 8 and ($proof[0].paths | length) == 8 and ($proof[0].paths | unique | length) == 8 and .expected_proof_path_count == 8 and .proof_paths == ($proof[0].paths) and (.proof_paths | unique | length) == 8 and .expected_lifecycle_path_count == 14 and (.lifecycle_paths | length) == 14 and (.lifecycle_paths | unique | length) == 14 and .expected_retained_surface_fixed_path_count == 9 and (.retained_surface_fixed_paths | length) == 9 and (.retained_surface_fixed_paths | unique | length) == 9 and .retained_surface_fixed_paths == ([.evidence_denominator_path] + .proof_paths) and .expected_retained_surface_entry_count == 19 and .retained_surface_dynamic_path_source.expected_count == 10 and .retained_surface_dynamic_path_source.path == .evidence_denominator_path and (.retained_surface_manifest_path | startswith(".csdlc/evidence/217/")) and .retained_surface_manifest_digest_binding == "h3_review_receipt" and .h3_allowed_addition_paths == [.review_receipt_path] and .review_receipt_anchor_policy.kind == "unique_ancestral_introduction_blob" and .review_receipt_anchor_policy.require_unique_first_addition_on_current_head_ancestry == true and .review_receipt_anchor_policy.require_anchor_commit_object == true and .review_receipt_anchor_policy.require_anchor_blob_object == true and .review_receipt_anchor_policy.objects_permitted_missing == ["H2_commit","H2_tree"] and (.review_receipt_anchor_policy.fail_closed_cases | index("coherent_receipt_and_manifest_rewrite")) != null and .later_head_policy.requires_h2_git_objects == false and .later_head_policy.requires_receipt_anchor_git_objects == true and .allowed_statuses == ["A","M"] and .forbidden_statuses == ["D","R","C","T","U","X","B"] and (.evidence_denominator_path | startswith(".csdlc/evidence/217/")) and ([.lifecycle_paths[] | startswith(".csdlc/issues/217/")] | all) .csdlc/prepared/issues/217/h2-retention-allowlist.json`
- `ruby .csdlc/prepared/issues/217/verify-historical-c640-packet.rb .csdlc/prepared/issues/217/historical-c640-denominator.json`
- `ruby .csdlc/prepared/issues/217/produce-native-receipt.rb --self-test`
- `ruby .csdlc/prepared/issues/217/validate-retained-native-proof.rb .csdlc/evidence/217/retained-proof-denominator.json`
- `ruby .csdlc/prepared/issues/217/validate-retained-native-proof.rb --self-test`

## Failure Semantics

Fail closed on missing or changed packet bytes, digest/provenance mismatch, unconfined paths, incomplete protected inventory, source relationship ambiguity, protected-source drift, stale typed truth, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
