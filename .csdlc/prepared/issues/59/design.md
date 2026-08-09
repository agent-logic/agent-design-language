# Issue 59 design: route blocked-goal replacement to its owning platform

## Decision

Do not implement goal replacement in ADL. The reported transition is owned by
the Codex goal tool and its backing thread-goal service, which are supplied to
the agent by the Codex platform. This repository contains policies that call
`create_goal`, `get_goal`, and `update_goal`, plus consumers that read retained
goal telemetry, but it does not contain the implementation, persistence model,
or API dispatcher that rejects a replacement goal.

Issue 59 therefore publishes an externally routed readiness checkpoint. The
existing blocked goal remains historically truthful. The checkpoint PR contains
only the reviewed authority packet, uses the typed non-closing `part_of`
relationship to `agent-logic/agent-design-language#59`, and does not claim that
ADL implements the platform fix. No ADL code, policy exception, or fake
`complete` transition is introduced to work around the platform rule.

## Authority evidence

- The failure is returned directly by the platform-provided `create_goal`
  operation: `cannot create a new goal because this thread has an unfinished
  goal; complete the existing goal first`.
- Repository search finds goal-tool names only in agent policy, legacy workflow
  documentation, retained session-log parsers, and tests of those parsers.
- No ADL crate or C-SDLC v2 binary owns thread-goal creation, replacement,
  persistence, or the unfinished-goal admission check.
- The available platform contract permits `update_goal` only to set `complete`
  or `blocked`; it exposes no `superseded`, `abandoned`, or replacement
  transition.

The external code authority is the OpenAI Codex product's goal-tool service,
not `agent-logic/agent-design-language`. Only that authority can safely change
the admission rule while preserving the old goal's status and accounting.

## Required upstream contract

The next action is to route the following bounded product defect to the Codex
goal-tool owner:

1. Reproduce with `create -> update_goal(blocked) -> create` in one thread.
2. Preserve the first goal as terminally blocked, including its accounting.
3. Permit an explicitly operator-redirected replacement goal, or expose a typed
   `supersede`/`abandon` transition that records the replacement objective.
4. Keep replacement of active, nonterminal goals fail-closed unless an explicit
   operator-authorized transition is used.
5. Add product-owned tests for history retention, accounting retention,
   replacement admission, and rejection of accidental active-goal overwrite.

Once the platform ships that contract, ADL can verify it with a live canary and
then close issue 59 without changing repository implementation code. Until
then, the checkpoint PR must not close issue 59.

## Boundary diagram

The companion diagram shows the ownership boundary and the exact failing
transition. ADL policy is a caller and consumer; it is not the state owner.

## Invariants

- A goal already marked `blocked` is never rewritten to `complete` merely to
  unblock later work.
- Historical status, time, and token accounting remain attached to the old
  goal.
- ADL does not shadow or fork Codex thread-goal state.
- No repository policy is weakened to bypass the issue-bound-goal requirement.
- Any repository PR is an authority-routing checkpoint, never an implementation
  or completion claim, and uses a typed non-closing relationship.

## Non-goals

- Adding an ADL-local second goal database.
- Parsing or mutating Codex session files as operational state.
- Marking the prior goal complete inaccurately.
- Removing the ADL issue-bound session-goal policy.
- Implementing nested sprint-goal accounting.
- Guessing at an undocumented Codex storage or API implementation.
- Closing issue 59 before the product-owned fix and live canary exist.

## Readiness and estimate

This is a bounded authority-classification checkpoint, not a code
implementation. Publication is authorized only through the typed `part_of`
linkage supplied by issue 75; a closing keyword is forbidden.
The typed cards retain the `small` profile's reviewable upper bounds: 7,200
elapsed seconds, 40,000 total tokens, 1,200 validation seconds, and 10,000
validation tokens. The single deterministic routing lane itself is bounded to
120 seconds and 1,000 tokens. Stop if an actual repository-owned goal admission
implementation is discovered; that would require redesign and fresh review
before execution.
