# Architecture Review Lane

- Reviewer: `codex:/root`
- Target: `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Packet: `docs/reviews/v0.92/internal-review-5846`
- Finding count: 1

## Findings

### ARCH-001 — P1 — Review control-plane truth has two incompatible gate states

Trigger: a review or release owner follows the WP-23 handoff and milestone
entrypoints at the frozen target. The handoff and review packet identify the
WP-22 result as 33/33 blocked, while `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`
records WP-22A/#467 as the superseding authority with 30 accepted rows, three
explicitly downstream-scoped rows, and zero blockers.

Affected boundary: milestone review orchestration and release-credit authority.
The repository exposes both states as current canonical guidance, so downstream
WP-25/WP-26 actors can freeze different candidate truth from the same revision.

Evidence:

- `docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md:10,101-104`
- `docs/reviews/v0.92/docs-release-truth-312/review-packet.md:11-14`
- `docs/milestones/v0.92/QUALITY_GATE_v0.92.md:50-53`

Impact: internal and external review ordering, blocker counts, and release
claims are nondeterministic. Route to WP-27 documentation/release-truth
remediation; do not grant release authority until one canonical supersession
state is propagated across all entrypoints.

## Architecture Map And Scope

Reviewed the workspace/crate boundary represented by `adl/src/lib.rs`, the
Runtime boundary in `adl-runtime/src/lib.rs`, milestone architecture and quality
gate documents, and the common packet inventory. The `adl-runtime` crate keeps
its stated Runtime-owned contract boundary and does not import the compiler or
C-SDLC control plane in the sampled surface. The broad `adl` facade remains an
explicit integration crate rather than evidence of a new dependency-direction
defect by itself.

## Method And Limitations

Read-only static inspection of the frozen target and packet. No architecture
graph generator or full compile was run. Generated/build output and private
operator-only material were excluded. The lane does not independently prove
every module dependency direction.

## Follow-up Candidates

- Fitness check: require every canonical review/release entrypoint to name the
  same superseding quality-gate issue, result digest, and blocker count.
- ADR candidate: none; this is authority-projection drift, not a new design
  decision.
