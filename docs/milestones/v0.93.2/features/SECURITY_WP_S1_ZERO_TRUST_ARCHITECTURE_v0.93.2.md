# Security WP-S1: Zero-Trust Architecture v0.93.2

## Metadata

- Feature Name: Zero-trust architecture
- Milestone Target: v0.93.2
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: architecture, policy, artifact
- Proof Modes: schema, fixtures, tests, demo, review

## Purpose

Define the ADL polis trust model so no citizen, guest, operator, service, tool,
message, projection, or data boundary receives implicit trust. Every meaningful
boundary crossing should be authenticated, authorized, state-aware, traceable,
and deny-by-default.

## Dependencies

- v0.90.3 citizen state, access control, projection, sanctuary, and quarantine.
- v0.90.5 governed tools, Universal Tool Schema, and ADL Capability Contract.
- v0.91 secure Agent Comms and ACIP boundary planning.
- v0.92 identity, continuity, names, and capability envelopes.
- v0.92 ACIP binary schema, public schema catalog, and optional WebSocket
  carrier planning.

## Required Work Products

- Trust-boundary contract for citizen, guest, operator, service, tool, polis,
  communication, projection, and data boundaries.
- Transport-boundary contract for HTTP, local, mock, and WebSocket-carried ACIP
  messages, including connection/session authority and schema authority.
- Actor and zone model with explicit authentication and authority requirements.
- Default-deny fixture set for unauthorized, stale, ambiguous, or
  overprivileged boundary crossings.
- Reviewer-facing trust-boundary report.

## Invariants

- No boundary crossing succeeds by default.
- Internal polis traffic is not automatically trusted merely because it is
  internal.
- Human/operator action is not citizen action unless mediated through identity,
  Freedom Gate, signed trace, temporal anchoring, and policy.
- Communication never grants private-state or private-ToM inspection rights.
- A live WebSocket connection is not authority. Every ACIP message still needs
  schema, identity, sequence, policy, and trace acceptance.

## Demo Candidate

Show a citizen, service, tool, or operator request crossing a protected
boundary. The accepted case should cite identity, standing, capability, and
policy authority. The denied case should fail closed with a redacted reason.

For WebSocket-carried ACIP, include a near-miss case where the connection is
open but the message is denied because schema authority, identity, sequence, or
policy evidence is missing or invalid.

## Acceptance Criteria

- The trust-boundary contract names every protected actor and data boundary.
- Unauthorized and ambiguous boundary crossings have negative fixtures.
- WebSocket-carried ACIP messages have explicit accept/deny fixtures that do not
  rely on connection state as implicit trust.
- The review packet explains what was denied without leaking protected data.
- Later WP cards can implement the contract without inventing new trust zones.

## Non-Goals

- No production certification claim.
- No production external federation or cross-polis networking claim.
- No replacement of v0.90.3 private-state access-control rules.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows the accepted v0.93.1 handoff and repository lockset. Preserve launched Beta 1 behavior through compatible upgrade and rollback; Runtime v4 is not a retroactive launch prerequisite. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-enterprise-security. Candidate results: WP-S1. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **WP-S1:** A private policy provider drives Runtime allow/deny decisions over all declared actor/message/data boundaries.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- WP-S1 follows GOV-11, v0.93.1/RD-09.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- WP-S1: Implicit internal trust; Connection treated as permission.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.2.md defines release evidence; planning validation does not prove these behaviors.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93.2 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml), [decision register](../DECISIONS_v0.93.2.md) and [quality gate](../QUALITY_GATE_v0.93.2.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

WP-S1; owner repositories: agent-logic-enterprise-security. The current execution graph supersedes older sequencing or placement in retained source text.

## Approved split routing

This planned feature is owned by v0.93.2 under #922. Consume accepted v0.93.1/TAIL-10 and the pinned split lockset; do not repeat repository extraction. Numeric implementation issues and named owners remain pending. No execution, live-provider or release approval is inferred.
