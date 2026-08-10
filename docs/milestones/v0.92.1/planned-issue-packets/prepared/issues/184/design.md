# DRT-04 Design

Issue: #184

## Objective

Prove continuity, fencing, healing, and halt behavior across Wuji and two private AWS availability zones.

## Scope

One Wuji voter, two private AWS voters in separate AZs, authenticated private transport, independent snapshots, isolation, AWS-only quorum, asymmetric partition, healing, stale-owner fencing, and cleanup.

## Dependencies

- DRT-03: issue #183

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Single-command hybrid qualification runner using the Agent Logic business AWS profile.
- Placement, transport, snapshot, quorum, election, commit, fence, partition, heal, halt, resource, cost, and cleanup receipts.

## Owned Paths

- `adl-runtime/tests/v0921_hybrid_continuity.rs`
- `adl/tools/v0921/drt-04/**`
- `adl/tools/v0921/drt-04/validate.sh`
- `.csdlc/issues/184/**`
- `.csdlc/prepared/issues/184/**`
- `.csdlc/prepared/issues/184/validate-outcome.rb`
- `.csdlc/evidence/184/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. AWS identity resolves to the approved Agent Logic business account before provisioning.
2. AWS voters use separate AZs, private authenticated transport, distinct state and independently materialized snapshots.
3. Isolating Wuji preserves AWS-only quorum continuity while the isolated stale voter cannot mutate; loss of quorum halts mutation.
4. Healing converges term, commit index, state digest, fence, and Observatory ownership before traffic resumes; every phase cleans up.

## PVF Lanes

- `drt-04-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/184/validate-outcome.rb`.
- `drt-04-production-proof`: Execute the exact production-path qualification or deterministic conformance command for this Runtime slice. Command: `bash adl/tools/v0921/drt-04/validate.sh`.
- `drt-04-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

AWS account/placement/network readback, authenticated transport, independent snapshot provenance, quorum/partition/fence/heal/halt receipts, billing tags, and provider-verified cleanup.

## Authority Boundary

- Issue DRT-04 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Public control endpoints
- Shared state or manually copied snapshots
- Dynamic IAM profile creation
- Leaving cloud resources after failure

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- AWS identity is wrong
- A public endpoint or self-signed production certificate appears
- Snapshots share materialization history
- Resources cannot be enumerated and removed

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-04`
