---
issue_card_schema: adl.issue.v1
wp: "981"
slug: "repository-scoped-issue-creation-receipt"
title: "[v0.92.2][C-SDLC v3][defect] Do not misclassify repository-scoped issue creation receipts"
labels:
  - "track:roadmap"
issue_number: 981
generated_at: "2026-09-15"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "csdlc_defect_repair"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/981"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "#981 repairs the native blocker discovered while rewriting and preparing #970."
pr_start:
  enabled: true
  slug: "repository-scoped-issue-creation-receipt"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-15

# Structured Task Prompt

## Summary

Repair the semantic residue classifier so fully authenticated repository-scoped issue creation does not block the assigned issue while all identity mismatches still require recovery.

## Goal

Restore native prepare and edit for newly created issues without weakening semantic recovery guards.

## Required Outcome

Classify the assigned-number mutation receipt through its canonical issue-zero intent and exempt it only when the filename, operation, intent, receipt, repository, assigned issue, head, adapter, and authenticated linkage agree.

## Deliverables

Narrow canonical repository-scoped receipt classifier, positive regression, six negative identity-link regressions, consistent safe Cargo test-filter admission from prepare through proof, and truthful lifecycle evidence.

## Acceptance Criteria

Positive mutation-receipt preparation succeeds; altered intent operation digest, marker, request, receipt operation digest, or receipt intent digest returns RecoveryRequired; existing semantic residue tests and the full component suite pass.

## Repo Inputs

Issue #981, retained native issue-create transaction for #970, canonical native remote intent/receipt validators, and transaction fixtures.

## Dependencies

No product dependency. The defect blocks ordinary native preparation of #970 and therefore bootstraps through the isolated #981 candidate only.

## Target Files / Surfaces

csdlc-v3/src/storage/semantic.rs; csdlc-v3/src/commands/remote/mod.rs; csdlc-v3/src/commands/proof/intent.rs; csdlc-v3/tests/transactions.rs.

## Validation Plan

Focused repository-scoped receipt tests; semantic_gate_a; full csdlc-v3 test suite; fmt; strict clippy; diff check; independent exact-head review.

## Demo Expectations

Real native preparation of #981 and deterministic transaction regressions; no external service demo.

## Non-goals

No automatic legacy migration, no changes to GitHub mutation effects, no provider implementation, and no unrelated cleanup.

## Issue-Graph Notes

After #981 integrates, reconcile the authorized #970 body edit and update its title through native v3.

## Notes

A permissive cross-link would hide damaged lifecycle evidence; every candidate exemption must use canonical native validation.

## Tooling Notes

Use the current native v3 man-page workflow and the isolated candidate only until this repair is integrated.
