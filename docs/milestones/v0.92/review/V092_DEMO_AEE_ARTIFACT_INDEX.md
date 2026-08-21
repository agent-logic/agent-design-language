# v0.92 Demo / AEE Artifact Index

## Metadata

- Milestone: `v0.92`
- WP owner: `WP-20`
- Current issue: `agent-logic/agent-design-language#308`
- Legacy predecessor: `danielbaustin/agent-design-language#5840`
- Purpose: bind demo, AEE, and feature-coverage rows to exact owner, status,
  command, positive proof, negative proof, and review posture.

## Status Contract

Rows marked `accepted` are the only rows that may support downstream quality or
release gates. Rows marked `blocked_with_evidence`, `deferred_non_claim`, or
`planned` are explicit non-claims.

## Artifact Index

| Row | Owner | Surface | Status | Exact revision | Positive artifact | Negative artifact | Review state | Command | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| AEE-001 | WP-01, WP-01B | Canonical milestone and version truth | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-002 | WP-02 | Agent Logic repository copies | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-003 | WP-02A | Reliable CI and coverage | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-004 | WP-02B | Evidence-based build acceleration | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-005 | WP-03 | Resilient local Runtime | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-006 | WP-04 | Distributed Guardian/polis | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-007 | WP-05, WP-06, WP-07 | Faster C-SDLC and remote validation | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-008 | WP-08, WP-09, WP-10 | Birthday and identity | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-009 | WP-11, WP-12 | Memory and capability | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-010 | WP-13, WP-13A | Cognitive profile and adaptation queue | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-011 | WP-14 | ACIP/A2A transport | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-012 | WP-15, WP-16 | Witness, receipt, and review packet | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-013 | WP-17 | Cross-polis continuity | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-014 | WP-18 | Demonstrable birthday | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-015 | WP-18A | Observatory and Unity consumers | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Separate Observatory component/session; not accepted release evidence here. |
| AEE-016 | WP-18B | Provider-neutral multi-agent execution | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-017 | WP-19 | v0.93 governance handoff | blocked_with_evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-evidence | pending-owner-review | pending-owner-command | Scheduling only; not accepted release evidence. |
| AEE-018 | WP-20 | Demo matrix and proof coverage | blocked_with_evidence | pending-typed-review-publication | pending-typed-review-publication | pending-typed-review-publication | pending-typed-review-publication | python3 adl/tools/validate_v092_demo_proof_coverage.py --root . | WP-20 reconciliation is complete only after exact-head typed review and publication; not accepted release evidence in the pre-publication artifact index. |
| AEE-019 | WP-21, WP-21A | Reduction and refactoring | planned | pending-#309 | pending-#309 | pending-#309 | pending-#309-review | pending-#309-command | Blocked until #308 is terminal; not accepted release evidence. |
| AEE-020 | WP-22 through WP-30 | Quality, release, and publication | planned | pending-release-tail | pending-release-tail | pending-release-tail | pending-release-tail-review | pending-release-tail-command | Not accepted release evidence. |

## Validator Expectations

The validator must fail closed when:

- an accepted row lacks exact revision, positive artifact, negative artifact,
  review state, or command
- an accepted row uses mutable revision prose, pre-review state, or docs/scripts
  as proof in place of retained evidence artifacts
- a matrix or feature-coverage row points at a missing artifact-index row
- a row is planned or blocked in one surface but accepted in another
- a row relies on synthetic proof, provider substitution, or unsupported
  platform claims
- duplicate artifact-index row identifiers are present
- feature owner or status diverges from the artifact index
