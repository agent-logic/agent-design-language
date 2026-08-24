# v0.92 Quality Gate

This file defines the evidence required for v0.92 release credit. It is a gate
plan, not evidence that a gate has passed.

WP-22 execution is retained under `docs/reviews/v0.92/quality-gate-311/`.
Structural validation of that packet is distinct from its release result: a
schema-valid findings packet may truthfully report `blocked`, and only an exact
zero-blocker packet may report `passed` or unlock downstream review.

| Gate | Owning work | Required evidence |
| --- | --- | --- |
| Milestone truth and issue graph | WP-01, WP-01B | Live issue-number map, dependency validation, six valid typed cards per issue, current canonical docs and version declarations |
| Repository copies | WP-02 | Reviewed copy-only plan, five verified destinations with exact visibility and Git/LFS parity, Actions-disabled-before-push receipts, truthful destination configuration dispositions, source-immutability proof, negative `asksifu` control, and Horust exclusion |
| CI and coverage | WP-02A | Deterministic lane selection, separated fast/slow work, nonduplicated coverage, platform parity, exact-head green checks |
| Build acceleration | WP-02B | Same-SHA cold/warm corpus, cache and queue accounting, proof parity, cost thresholds, canary, and retained fallback or cleanup |
| Runtime resilience | WP-03, WP-04 | Guardian-owned launch, recovery and relocation proof, clean logs, distributed security review, cross-platform validation |
| Workflow efficiency | WP-05 through WP-07 | Measured cycle-time improvement, portable validation, prompt-card contract parity, regression proof |
| Birthday contract | WP-08 through WP-17 | Identity, continuity, memory, capability, profile, protocol, witness, receipt, review packet, and cross-polis semantics with negative cases |
| Integrated demonstrations | WP-18, WP-18A, WP-18B | Real first-birthday proof, working Observatory/Unity consumers, provider-neutral multi-agent evidence |
| Governance handoff | WP-19 | Evidence map for v0.93 without claiming v0.93 governance is implemented |
| Cleanup and maintainability | WP-20, WP-21, WP-21A | Proven deletion eligibility, behavior-preserving reduction, focused Rust refactoring, no parity regression |
| Review and release | WP-22 through WP-30 | Quality review, release evidence, ten articles, ten podcast packages, claim-bounded publication, external review, remediation, ceremony, handoff |

## Global Rules

- Every issue must complete its declared outcome at the exact reviewed
  revision. Scaffolding, placeholders, and partial work are not completion
  unless the issue explicitly defines that bounded slice as its full outcome.
- Planning text, fixtures, receipts, and simulated success do not replace real
  behavior where a work package requires execution.
- Runtime, protocol, provider, consumer, migration, and integration work must
  prove real positive and negative production-path behavior. Demo mode,
  synthetic success, and substituted providers receive no release credit.
- Documentation and planning work must be source-grounded and decision-ready,
  with owners, boundaries, dependencies, acceptance criteria, and executable
  next steps. Restating intent is not a useful deliverable.
- Tooling and cleanup work must demonstrate measured operator or
  maintainability value and focused regression safety.
- Focused validation is preferred, but every claimed platform or integration
  must have evidence at the reviewed revision.
- Product changes require exact-head review and green required checks.
- Release notes and public materials may describe only landed, reviewed work.
- Legal personhood, production citizenship, consciousness, and completed v0.93
  constitutional governance remain non-claims.
- Any issue that fails these rules blocks WP-22 and cannot enter internal
  review as completed work.
- A merged dependency whose typed closeout or cleanup is still asynchronous is
  recorded as incomplete evidence inside the matrix; it does not prevent WP-22
  from running, and it receives no release credit until the canonical evidence
  arrives.

# Production birthday composition gate

The first production birthday gate is satisfied only by the #451 exact-head
kernel and resident-path tests plus the retained nine-feature wiring audit and
redaction/schema validator. A library-only, fixture-only, metadata-only,
documentation-only, or unreachable feature disposition blocks the gate. Paid
AWS qualification is not part of this local composition gate.
