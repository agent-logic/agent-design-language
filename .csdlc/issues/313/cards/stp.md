# Structured Task Prompt

Template: 1.0.0

Issue: 313

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare the complete internal-review contract now without starting the issue; execute later only after WP-23 and WP-24 are terminal, reconciled, ancestral, and clean, with WP-24A truthfully deferred to v0.92.1.

## Deliverables

- Internal review report and stable findings register
- Exact-SHA review packet and machine-readable digest manifest
- Nine specialist lane results with reviewer identity and completion truth
- Findings-first synthesis preserving duplicates and disagreements
- Validation, redaction, and independent meta-review records

## Acceptance

1. WP-23 #312 and WP-24 #10 are terminal, reconciled, ancestral, and cleaned before review execution; WP-24A is recorded as deferred to v0.92.1 and non-blocking
2. The packet pins agent-logic/agent-design-language and one clean exact target SHA
3. Included, excluded, unknown, local-only, generated, vendored, private, and redacted surfaces are explicitly inventoried
4. All nine independent specialist lanes record reviewer identity, scope, method, evidence, limitations, and findings or a defensible zero-finding result
5. Every finding has stable ID, severity, exact evidence, invariant or failure mode, reproduction or proof gap, owner route, duplicate links, disagreement state, and open disposition
6. Synthesis preserves source provenance, duplicates, disagreements, and residual risks without remediation
7. Packet identity, manifest digests, evidence links, lane completion, finding schema, redaction, and private-path hygiene validate fail-closed
8. An independent meta-review reports no actionable review-quality gap
9. The result makes no remediation, external-approval, release-readiness, publication, or deployment claim

## Dependencies

- Canonical WP-23 issue #312 terminal, reconciled, ancestral, and clean
- Canonical WP-24 issue #10 terminal, reconciled, ancestral, and clean
- Operator disposition deferring WP-24A to v0.92.1, reconciled into #313 and its cards before issue start
- Sprint 6 umbrella #307 graph reconciled with the canonical dependency identities

## Inputs

- docs/milestones/v0.92/
- docs/reviews/v0.91.8/internal-review-5356/
- docs/reviews/v0.91.8/internal-review-5791/
- docs/milestones/v0.91.2/review/internal_review_full/
- docs/tooling/OPUS_REVIEW_RUNBOOK.md
- .csdlc/issues/
- .csdlc/evidence/

## Non Goals

- Fixing or suppressing findings
- Dispatching external review
- Approving release, deployment, publication, or submission
- Creating one issue per finding without a separate routing decision
- Crediting predecessor artifacts as acceptance without exact-revision proof
