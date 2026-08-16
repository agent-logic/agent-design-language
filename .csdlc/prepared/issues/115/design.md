# #115 design packet

## Decision

Prepare #115 as the governed multi-agent rooms and message-routing implementation child. This packet is design-only and unbound; it records future implementation boundaries while preserving the exact dependency gate, including #270.

## Owned outcome

When implementation becomes allowed, #115 owns:

- explicit room, participant, mention, routing, and delivery contracts;
- Runtime membership and policy enforcement for bounded multi-agent rooms;
- Observatory room list, participant list, transcript, composer, and delivery states;
- ordering, fan-out, partial-failure, replay, and adversarial proof for rooms.

## Boundary

#115 consumes, but does not redefine:

- #111 canonical conversation session semantics;
- #112 Layer 8 authority and audit;
- #113 complete live roster;
- #270 trusted recipient-acknowledgement Runtime API protocol.

## Dependency gate

Bind/implementation is blocked until #111, #112, #113, and #270 are terminal and ancestral. If #112 or #270 changes its public authority/acknowledgement contract, #115 must be re-read and re-reviewed before binding.

## Planned implementation shape

1. Define explicit room and participant contracts with no hidden recipient expansion.
2. Route addressed messages only to stable, visible participant sets.
3. Enforce membership and policy at Runtime boundaries, consuming #112/#270 authority outcomes.
4. Attribute every response to a stable agent identity and triggering turn.
5. Represent joins, leaves, refusals, unavailable, timeout, partial delivery, duplicate, and reordered event states in Observatory without inventing authority.

## Validation plan

- Focused room membership and explicit-recipient tests.
- Focused fan-out, partial delivery, refusal, timeout, duplicate, and reorder tests.
- Focused cross-Polis-denial tests.
- Focused Observatory state tests after Runtime contracts exist.
- Strict Clippy for touched Runtime/Observatory surfaces when implementation is authorized.

## Current non-claims

- No branch/worktree is bound.
- No product code is changed.
- No executable behavior is proven yet.
