# V3-14 Design

Issue: #177

## Objective

Implement idempotent PR publication and bounded foreground waiting over the reviewed GitHub adapter.

## Scope

Mode-bound publication intents, issue/PR/comment mutation, operation markers, exact linkage readback, `pr publish`, `pr watch`, check/review/mergeability updates, signal cancellation, and optional explicitly authorized merge policy.

## Dependencies

- V3-04: issue #165
- V3-08: issue #169
- V3-09: issue #170
- V3-12: issue #175
- V3-13: issue #176

## Architecture Decisions

- `V3-D08`

## Deliverables

- Typed mutation operations, durable intent integration, publication command with explicit `closing | part_of` linkage selection, mode-bound publication evidence and reconciliation, foreground watch with 30-minute default, 24-hour maximum, 15-second default poll interval and stderr progress, idempotency/readback fixtures, and bounded live publication canary.

## Owned Paths

- `csdlc-v3/src/adapters/github/write/**`
- `csdlc-v3/src/commands/pr/**`
- `csdlc-v3/tests/github/write/**`
- `.csdlc/issues/177/**`
- `.csdlc/prepared/issues/177/**`
- `.csdlc/prepared/issues/177/validate-outcome.rb`
- `.csdlc/evidence/177/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. No remote mutation begins before its durable intent commit.
2. Every mutation is idempotent and verified by exact remote readback.
3. `closing` requires the exact closing relation; `part_of` requires the exact non-closing relation and proves the target issue remains open after PR publication and checkpoint merge observation.
4. Same-repository shorthand normalizes to a qualified identity, while split repositories reject unqualified linkage in either mode.
5. `pr watch` is foreground, cancellable by root signals, bounded, and leaves no persistent job or unjoined task.
6. Fake-adapter tests prove that a `part_of` watch cannot report checkpoint-ready unless exact REST issue readback still observes the qualified target issue open; closed, missing, stale, or contradictory observations produce reconciliation-required.
7. Every watch sleep and network await is selected against root cancellation; cancellation drains and joins the watch scope before exit 130.
8. Default and overridden timeout/poll values remain within the V3-01 bounds and timeout exits without a persistent job or unjoined task.
9. If `now + max(poll_interval, retry_after)` exceeds the fixed deadline, watch exits immediately without sleeping past the deadline.
10. Merge occurs only when the approved explicit policy and operator authority are both present.

## PVF Lanes

- `v3-14-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/177/validate-outcome.rb`.
- `v3-14-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-14-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Intent crash matrix, duplicate-marker tests, same- and split-repository `closing | part_of` positive/negative matrices, missing/mixed/ ambiguous/wrong-target linkage negatives, evidence/reconciliation fixtures, watch cancellation and timeout tests, stale-head negatives, merge-policy tests, and bounded live canaries for both linkage modes.

## Authority Boundary

- Issue V3-14 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Finish, cleanup, detached watchers, polling daemons, implicit merge, remote rollback, or terminal issue closure reconciliation.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Mutation lacks a resumable intent, linkage mode or target is not durable, readback can conflate `part_of` with closing, watch detaches, exact readback is unavailable, merge becomes implicit, or cancellation leaves work running.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-14`
