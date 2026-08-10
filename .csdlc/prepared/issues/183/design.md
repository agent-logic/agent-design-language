# DRT-03 Design

Issue: #183

## Objective

Run the production Wuji three-voter multi-agent qualification against the exact terminal #142 revision.

## Scope

Three independent Wuji voters, three governed agents, one non-voting Shepherd, one leased Observatory, election, mutation, 3-to-2-to-1 behavior, old-lease expiry, snapshot restore, restart, and per-phase cleanup.

## Dependencies

- DRT-01: issue #181
- DRT-02: issue #182
- RUNTIME-142: issue #142 terminal exact proof

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Exact-revision production launch and scenario runner.
- Node, agent, authority, quorum, commit, lease, snapshot, Observatory, resource, replay, and cleanup receipts for every phase.

## Owned Paths

- `adl-runtime/tests/v0921_wuji_three_voter.rs`
- `adl/tools/v0921/drt-03/**`
- `.csdlc/issues/183/**`
- `.csdlc/prepared/issues/183/**`
- `.csdlc/evidence/183/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. The exact #142 merge SHA is ancestral to the tested revision and its retained Guardian/API/WSS/WP-04.16 proof passes.
2. Three independently started voters commit governed work; two voters preserve quorum; one voter cannot mutate.
3. The old Observatory lease expires before successor binding and stale-owner writes are denied.
4. Snapshot restore, voter restart, agent continuity, replay, and cleanup pass without shared state roots or direct executor bypass.

## PVF Lanes

- `drt-03-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/183/validate-outcome.rb`.
- `drt-03-production-proof`: Execute the exact production-path qualification or deterministic conformance command for this Runtime slice. Command: `bash adl/tools/v0921/drt-03/validate.sh`.
- `drt-03-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Exact ancestry, production-path attestation, 3-to-2-to-1 state and term receipts, stale fence negatives, restore/restart parity, replay, and provider/process cleanup readback.

## Authority Boundary

- Issue DRT-03 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Testing an open or merely green #142 PR
- Using in-process service objects as voters
- Leaving a failed phase running

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- #142 is not terminal with passing retained proof
- Any voter shares identity or state
- One-voter mutation succeeds
- Cleanup cannot be verified

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-03`
