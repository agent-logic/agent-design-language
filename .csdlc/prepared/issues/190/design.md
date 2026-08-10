# INT-03 Design

Issue: #190

## Objective

Publish the accepted next-milestone handoff and deferred V3-R01 eligibility contract after release closeout.

## Scope

Terminal v0.92.1 evidence, residual risks, deferred work, rollback-window metrics, V3-R01 eligibility, next milestone inputs, owners, dependencies, and explicit non-claims.

## Dependencies

- INT-02: issue #189

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Accepted next-milestone handoff packet.
- V3-R01 eligibility record that remains blocked until rollback expiry and stability approval.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/integration/int-03/**`
- `.csdlc/issues/190/**`
- `.csdlc/prepared/issues/190/**`
- `.csdlc/evidence/190/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. The handoff cites exact terminal release evidence and every accepted residual risk.
2. Deferred work retains owners, dependencies, proof requirements, and routing without being presented as complete.
3. V3-R01 remains ineligible until rollback expiry, stability metrics, historical readability, and explicit operator approval all pass.
4. The downstream milestone can consume the packet without relying on chat or machine-local state.

## PVF Lanes

- `int-03-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/190/validate-outcome.rb`.
- `int-03-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Terminal reference, deferred denominator, owner/dependency, V3-R01 eligibility, portability, link, placeholder, and independent handoff review.

## Authority Boundary

- Issue INT-03 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Starting the downstream milestone
- Deleting v2
- Rewriting v0.92.1 history

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A residual risk or deferred item lacks an owner
- Rollback window is still active for V3-R01
- The packet depends on untracked or machine-local state

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#int-03`
