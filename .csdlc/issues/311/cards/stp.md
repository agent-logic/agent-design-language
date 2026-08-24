# Structured Task Prompt

Template: 1.0.0

Issue: 311

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Build and run the evidence gate only; report blockers without absorbing their fixes.

## Deliverables

- docs/milestones/v0.92/QUALITY_GATE_v0.92.md
- docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md
- docs/reviews/v0.92/quality-gate-311/feature-completion-matrix.json
- docs/reviews/v0.92/quality-gate-311/quality-gate-record.json
- docs/reviews/v0.92/quality-gate-311/blocker-report.md
- .csdlc/prepared/issues/311/validate-quality-gate.rb
- .csdlc/prepared/issues/311/test-validate-quality-gate.rb
- .csdlc/evidence/311/validation.json

## Acceptance

1. AC-1: Canonical #310 and every declared canonical or legacy predecessor are observed from the correct repository and canonical typed authority; required terminal, review, merge, ancestry, cleanup, and migration identities are exact.
2. AC-2: Every indexed v0.92 feature and supporting critical path has exactly one matrix row with stable identity, owner, implementation paths, reviewed head, PR/merge, validation, negative, integration, platform, claim-boundary, and typed-terminal evidence.
3. AC-3: Planned, open, unknown, fixture-only, receipt-only, demo-only, synthetic, substituted-provider, stale-review, non-ancestral, malformed-cache, fabricated-check, or platform-unproven rows fail closed.
4. AC-4: Documentation/planning rows are source-grounded and executable; tooling/cleanup rows prove measured value and regression safety; runtime/provider/consumer rows prove production-path behavior.
5. AC-5: Matrix, gate record, validation receipt, and findings-first blocker report are schema-valid and reproducible; downstream remains blocked unless every required row is accepted.
6. AC-6: A fresh exact-head independent review validates the denominator, validator behavior, and all dispositions with no unresolved actionable finding.

## Dependencies

- Canonical #310 terminal, reconciled, ancestral, and cleaned
- Canonical #309 terminal merge 5b3657582fea2109f000623bb121b7998185ac0a
- Canonical #308 terminal WP-20 evidence
- Legacy WP-04 #5821, WP-05 #5822, WP-06 #5823, WP-07 #5824, and WP-13A #5831 typed/terminal evidence
- Current v0.92 feature index and milestone proof documents

## Inputs

- GitHub issue #311 and archived legacy #5842 body
- docs/milestones/v0.92/features/README.md
- docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
- docs/milestones/v0.92/QUALITY_GATE_v0.92.md
- docs/milestones/v0.92/DEMO_MATRIX_v0.92.md
- docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md
- Canonical and legacy C-SDLC terminal caches plus live GitHub and Git evidence

## Non Goals

- Repairing incomplete product or documentation work
- Waiving blockers to preserve schedule
- Crediting synthetic, fixture, demo, receipt-only, or provider-substituted proof
- Executing WP-23, WP-25, release approval, or milestone ceremony
- Mutating #310, dependency worktrees, external repositories, or cloud resources
