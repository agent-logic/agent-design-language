# ADR 0071: Provider-Neutral Multi-Agent Proof Boundary

## Status

Status: **Deferred**

## Context

The milestone plans a provider-neutral multi-agent birthday scenario, but the
WP-18B execution and comparison proof has not landed.

## Decision

Defer the decision until multiple configured providers run the same governed
scenario with comparable retained semantics, explicit provider attribution,
negative cases, and exact-revision review.

## Consequences

Protocol neutrality and provider adapters cannot be mistaken for demonstrated
provider-neutral behavior.

## Alternatives Considered

One provider, mocked responses, or model-name substitution was rejected as
provider-neutral proof.

## Source Evidence

- `docs/milestones/v0.92/features/PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md`

## Validation Evidence

- `adl-runtime-kernel/tests/production_acip_wss.rs`

## Supersession Relationships

May refine ADR 0004, ADR 0041, and ADR 0065 after proof lands.

## Non-Claims

No multi-provider live run, provider equivalence, model suitability, or
provider-neutral birthday proof is claimed.

## Approval Boundary

WP-18B landed executable proof and human review are required before Proposed.
