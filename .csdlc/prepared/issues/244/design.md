# Design: deterministic conversation cleanup-race admission

## Boundary

Stabilize only the cleanup-race regression proof introduced by PR #228. Keep Runtime production semantics, capability authority issue #237, PR #242, and #112 authority work unchanged.

## Decision

The initial `accepted` result already acknowledges successful session admission before dispatch-gate or canonical ingress completion. The proof must queue re-authentication and duplicate attachment in server processing order without spending the existing turn's bounded execution window on a client-side authentication round trip.

Duplicate attachment from the new authentication generation to the same active turn remains `accepted` with `conversation_in_flight`. Exactly one terminal result is emitted for that turn. No production deadline or admission behavior changes.

## Proof

Run the focused `conversation_sessions` integration target repeatedly to expose scheduling races, then run the required Runtime fast lane. Preserve cancellation, explicit timeout, ordering, capacity, token-rotation, and single-terminal-result assertions.

## Non-goals

- No capability-authority changes.
- No changes to PR #242.
- No unrelated Observatory or conversation API redesign.
- No optional CI or broad workspace runner.
