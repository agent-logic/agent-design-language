# Security WP-S3: Secrets, Keys, And Cryptographic Trust v0.93

## Metadata

- Feature Name: Secrets, keys, and cryptographic trust
- Milestone Target: v0.93
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: runtime, artifact, policy
- Proof Modes: fixtures, tests, schema, demo, review

## Purpose

Represent signing, encryption, key custody, key rotation, revocation, sealed
state access, and internal ACIP encryption as explicit lifecycle contracts
rather than hidden environment folklore.

## Dependencies

- WP-S1 zero-trust architecture.
- WP-S2 policy enforcement and authorization.
- v0.90.3 signed envelopes, local sealing, private state, and witnesses.
- v0.91 secure Agent Comms and ACIP planning.
- v0.92 identity and capability envelopes.
- v0.92 ACIP binary schema, public schema catalog, and optional WebSocket
  carrier planning.

## Required Work Products

- Key/secrets lifecycle contract covering creation, custody, scope, use,
  rotation, revocation, expiration, and destruction.
- Signing and encryption acceptance rules tied to identity, standing, policy,
  and lifecycle state.
- Cryptographic acceptance rules for WebSocket-carried ACIP messages, including
  message signing, sequence/replay handling, and encrypted payload boundaries.
- Fixtures for accepted current keys and denied stale, revoked, malformed, or
  wrong-scope keys.
- Internal ACIP encryption and message-acceptance proof surface.

## Invariants

- Revoked keys cannot authorize new actions.
- Rotated keys change what signatures, messages, and sealed-state access are
  accepted.
- Secrets are never emitted into review packets or public projections.
- Encryption does not replace authorization.
- Signature validity does not imply policy permission.
- WebSocket session encryption does not replace ACIP message signing,
  authorization, replay protection, or trace evidence.

## Demo Candidate

Show an internal ACIP message or sealed-state access accepted before key
rotation and denied after revocation, with audit evidence linking the lifecycle
change to the disposition.

For WebSocket-carried ACIP, include a signed/encrypted message accepted under a
current key and denied after replay, revocation, or wrong-scope key use.

## Acceptance Criteria

- Key lifecycle events are deterministic artifacts.
- Rotation and revocation have explicit negative cases.
- Review output cites key identity and lifecycle state without exposing secret
  material.
- WebSocket-carried ACIP proofs show message-level crypto decisions rather than
  relying only on transport-level security.
- The feature composes with audit, incident, and zero-trust evidence.

## Non-Goals

- No production KMS integration claim.
- No external cross-polis TLS/federation claim.
- No secret material in docs, fixtures, logs, or reviewer packets.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows accepted v0.92.2 closure and the opening repository split through RD-11. Existing Beta 1 behavior remains a predecessor obligation. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-enterprise-security. Candidate results: WP-S3. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **WP-S3:** Current scoped keys work and rotated/revoked keys fail in signed/encrypted message and checkpoint consumers.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- WP-S3 follows WP-S2.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- WP-S3: Wrong-scope key; Secret in output; Revocation ignored.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.md defines release evidence; planning validation does not prove these behaviors.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.yaml), [decision register](../DECISIONS_v0.93.md) and [quality gate](../QUALITY_GATE_v0.93.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

WP-S3; owner repositories: agent-logic-enterprise-security. The current execution graph supersedes older sequencing or placement in retained source text.
