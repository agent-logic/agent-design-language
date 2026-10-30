> **Superseded combined plan.** The operator-approved split is planned under #922 in [v0.93.1](../../v0.93.1/README.md) (repo split, templates and CodeFriend Beta 1 launch) and [v0.93.2](../../v0.93.2/README.md) (remaining platform work). The original combined scope below is retained for traceability; it does not require Runtime v4 before Beta 1 launch or authorize execution.

# Security WP-S2: Policy Enforcement And Authorization v0.93

## Metadata

- Feature Name: Policy enforcement and authorization
- Milestone Target: v0.93
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: runtime, policy, artifact
- Proof Modes: tests, fixtures, schema, demo, review

## Purpose

Make IAM, delegation, standing, tool authority, capability envelopes, and
citizen/action policy enforceable under least privilege. v0.93 should not only
describe authority; it should produce reviewable allow/deny evidence that fails
closed when authority is missing, stale, ambiguous, revoked, or overbroad.

## Dependencies

- WP-S1 zero-trust architecture.
- v0.90.5 ACC/UTS governed-tool authority.
- v0.92 ACIP binary schema, public schema catalog, and optional WebSocket
  carrier planning.
- v0.92 identity, capability envelopes, and continuity.
- v0.93 delegation, IAM, standing, rights, and duties models.

## Required Work Products

- Policy decision contract for subject, action, resource, context, authority
  chain, standing, capability, and disposition.
- Per-message authorization contract for ACIP messages carried over persistent
  transports such as WebSocket.
- Least-privilege fixtures for citizen, guest, operator, service, tool, and
  delegated action.
- Deny-by-default tests for missing, expired, conflicting, or overbroad
  authority.
- Reviewer report explaining decisions without exposing private state.

## Invariants

- Missing authority denies.
- Stale or revoked authority denies.
- Delegation cannot exceed the delegator's authority.
- Tool authority must bind both capability and policy, not just tool name.
- Standing restrictions must constrain otherwise valid authority.
- Persistent transport sessions do not confer blanket permission. Each
  WebSocket-carried ACIP message must be evaluated as its own policy event.

## Demo Candidate

Show a delegated tool action. One request should be accepted only when identity,
standing, delegation, capability, tool authority, and policy all align. A near
miss should be denied with a reviewable reason.

For WebSocket-carried ACIP, show an authorized session carrying one allowed
message and one denied message so reviewers can see that session establishment
does not bypass per-message policy.

## Acceptance Criteria

- Policy decisions are deterministic for identical inputs.
- Allow and deny outputs cite explicit authority evidence.
- Negative fixtures cover missing, stale, revoked, and overbroad authority.
- Transport fixtures cover malformed, replayed, out-of-order, and
  policy-invalid WebSocket-carried ACIP messages.
- The contract composes with constitutional review and audit evidence.

## Non-Goals

- No blanket administrator bypass.
- No hidden operator override treated as citizen authority.
- No production IAM product claim.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows accepted v0.92.2 closure and the opening repository split through RD-11. Existing Beta 1 behavior remains a predecessor obligation. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-enterprise-security. Candidate results: WP-S2. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **WP-S2:** Enterprise policy narrows Runtime grants for identity, standing, delegation, tools and ACIP messages.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- WP-S2 follows WP-S1, GOV-12.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- WP-S2: Stale/revoked grant; Malformed/replayed/out-of-order message.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.md defines release evidence; planning validation does not prove these behaviors.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.yaml), [decision register](../DECISIONS_v0.93.md) and [quality gate](../QUALITY_GATE_v0.93.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

WP-S2; owner repositories: agent-logic-enterprise-security. The current execution graph supersedes older sequencing or placement in retained source text.
