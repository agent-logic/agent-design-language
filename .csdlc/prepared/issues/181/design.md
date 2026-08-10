# DRT-01 Design

Issue: #181

## Objective

Freeze an exact distributed qualification contract before provisioning nodes or injecting faults.

## Scope

Topology, identities, ports, state roots, credentials, transport, AWS/Wuji placement, scenarios, timing, resource budgets, receipt schema, cleanup, and claim boundaries.

## Dependencies

- No child dependency; setup issue #146 and umbrella readiness only

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Versioned topology and scenario manifest for both live windows.
- Producer-derived receipt, resource, timing, cleanup, and claim schema with negative-case denominator.

## Owned Paths

- `docs/milestones/v0.92.1/runtime-qualification/**`
- `adl/tools/v0921/drt-01/**`
- `.csdlc/issues/181/**`
- `.csdlc/prepared/issues/181/**`
- `.csdlc/evidence/181/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. The contract names exactly three voters, three governed agents, one non-voting Shepherd, and one quorum-leased Observatory.
2. Every node has distinct identity, credential, port, state root, storage, and failure-domain placement.
3. Each scenario has setup, action, expected commit/election/fence behavior, timeout, receipt fields, cleanup, and fail-closed outcome.
4. The contract distinguishes production proof from harness orchestration and forbids in-process substitutes or hard-coded success counts.

## PVF Lanes

- `drt-01-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/181/validate-outcome.rb`.
- `drt-01-production-proof`: Execute the exact production-path qualification or deterministic conformance command for this Runtime slice. Command: `bash adl/tools/v0921/drt-01/validate.sh`.
- `drt-01-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Schema, denominator, uniqueness, dependency, timing, resource, cleanup, receipt-field, and forbidden-substitute validation.

## Authority Boundary

- Issue DRT-01 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Running live nodes
- Changing Runtime behavior
- Treating a topology diagram as proof

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A production path lacks a named proof owner
- A scenario has no bounded timeout or cleanup
- Topology can collapse to one process or shared state

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-01`
