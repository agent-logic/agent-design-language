# Design: deterministic conversation cleanup-race admission

## Boundary

Correct only the Runtime conversation session admission/deadline behavior introduced by PR #228. Keep capability authority issue #237 and PR #242 unchanged.

## Decision

The initial `accepted` result acknowledges successful session admission and must not wait for the turn's dispatch gate or canonical ingress completion. Execution retains a bounded deadline, but a newly admitted session generation receives its own execution window; cleanup or re-authentication must not cause that generation to consume a stale predecessor window.

Duplicate attachment to the same active generation remains `accepted` with `conversation_in_flight`. Exactly one terminal result is emitted for the active turn.

## Proof

Run the focused `conversation_sessions` integration target repeatedly to expose scheduling races, then run the required Runtime fast lane. Preserve cancellation, explicit timeout, ordering, capacity, token-rotation, and single-terminal-result assertions.

## Non-goals

- No capability-authority changes.
- No changes to PR #242.
- No unrelated Observatory or conversation API redesign.
- No optional CI or broad workspace runner.
