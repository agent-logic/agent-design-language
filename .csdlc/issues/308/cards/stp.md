# Structured Task Prompt

Template: 1.0.0

Issue: 308

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Reconcile WP-20 demo and AEE proof surfaces and validate their exact-revision consistency without modifying child-owned evidence or declaring milestone readiness.

## Deliverables

- docs/milestones/v0.92/DEMO_MATRIX_v0.92.md
- docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
- docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md
- docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md
- adl/tools/validate_v092_demo_proof_coverage.py
- adl/tools/test_v092_demo_proof_coverage.sh
- .csdlc/evidence/308

## Acceptance

1. Matrix, coverage, activation-ledger, and artifact-index rows agree on owner, command, status, and exact revision
2. Every accepted demo or AEE claim has positive evidence, required negative evidence, platform and credential posture, review state, and non-claims
3. WP-20 proof ownership is corrected without absorbing WP-21 or WP-21A reduction and refactoring work
4. The validator rejects missing paths, duplicate ownership, planned-as-passed status, synthetic proof, unsupported platform claims, and revision drift
5. Exact-head review has no unresolved actionable finding

## Dependencies

- #256 WP-18 terminal, reconciled, and ancestral
- #340 WP-18A terminal, reconciled, and ancestral
- #341 WP-18B terminal, reconciled, and ancestral
- Legacy #5839 WP-19 validated terminal authority ancestral to the selected base

## Inputs

- docs/milestones/v0.92/WBS_v0.92.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md
- .csdlc/prepared/issues/308/design.md
- .csdlc/prepared/issues/308/diagram.mmd

## Non Goals

- Feature implementation or child ownership changes
- Repository-wide reduction or Rust refactoring
- Synthetic proof generation
- Milestone quality, publication, release, or ceremony approval
