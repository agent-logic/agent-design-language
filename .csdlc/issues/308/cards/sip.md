# Structured Intent Prompt

Template: 1.0.0

Issue: 308

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Reconcile and validate v0.92 demo, AEE, activation, feature-coverage, and artifact-index truth at one exact revision.

## Required Outcome

A reconciled demo matrix, exact-revision AEE artifact index, proof-coverage truth, corrected ownership boundaries, and a fail-closed validator with focused tests.

## Scope

- docs/milestones/v0.92/DEMO_MATRIX_v0.92.md
- docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
- docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md
- docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md
- adl/tools/validate_v092_demo_proof_coverage.py
- adl/tools/test_v092_demo_proof_coverage.sh
- .csdlc/evidence/308

## Authority

- #256, #340, #341, and legacy #5839 are read-only entry-gate authorities and must be terminal, reconciled, and ancestral before execution
- Child-produced proof artifacts and sibling issue outputs remain read-only inputs
- WP-20 may reconcile proof truth but cannot implement features, create synthetic evidence, or absorb WP-21/WP-21A work

## Assumptions

- none

## Operator Constraints

- Use the typed C-SDLC v2 lifecycle and a dedicated issue-bound worktree for execution
- Reverify the predecessor entry gate immediately before binding
- Fail closed on missing, contradictory, non-ancestral, or non-exact-revision evidence
