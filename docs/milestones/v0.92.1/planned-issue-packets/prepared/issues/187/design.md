# DRT-07 Design

Issue: #187

## Objective

Complete bounded local and hybrid soak, resource, deterministic replay, cleanup, and final qualification synthesis.

## Scope

Two-hour local soak, four-hour hybrid soak, workload and fault schedule, CPU/memory/disk/network/cost bounds, exact commands and terms, committed indexes, source/model digests, independent replay, cleanup after success and every failure, and residual-risk synthesis.

## Dependencies

- DRT-05: issue #185
- DRT-06: issue #186

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Producer-derived soak and resource receipt bundles for both live windows.
- Independent replay and cleanup verification plus final qualification report with explicit non-claims and residual risks.

## Owned Paths

- `adl/tools/v0921/drt-07/**`
- `adl/tools/v0921/drt-07/validate.sh`
- `docs/milestones/v0.92.1/evidence/runtime-qualification/**`
- `.csdlc/issues/187/**`
- `.csdlc/prepared/issues/187/**`
- `.csdlc/prepared/issues/187/validate-outcome.rb`
- `.csdlc/evidence/187/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Both soak durations complete under declared workload, fault, resource, and error thresholds.
2. Receipts bind exact commands, terms, committed indexes, envelopes, source revisions, model digests, clocks, and cleanup outcomes.
3. Independent replay reproduces the declared deterministic outcomes without live-provider dependence.
4. Provider and process readback proves cleanup after normal completion and every injected or unexpected failure phase.

## PVF Lanes

- `drt-07-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/187/validate-outcome.rb`.
- `drt-07-production-proof`: Execute the exact production-path qualification or deterministic conformance command for this Runtime slice. Command: `bash adl/tools/v0921/drt-07/validate.sh`.
- `drt-07-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Duration and workload denominator, resource/cost thresholds, failure accounting, exact-input digest, independent replay, cleanup readback, report/non-claim, and independent review.

## Authority Boundary

- Issue DRT-07 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Extending soak duration after seeing results
- Replacing failed proof with screenshots
- Claiming release approval

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A soak restarts without retaining the failed attempt
- Resource or error thresholds are exceeded
- Replay diverges
- Any cloud or local process survives cleanup

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-07`
