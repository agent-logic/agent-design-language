# Structured Task Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Correct only the cleanup-race admission/deadline behavior introduced by PR #228.

## Deliverables

- deterministic cleanup-race regression sequencing
- repeated focused cleanup-race proof
- required Runtime lane evidence

## Acceptance

1. AC-1: Initial accepted acknowledgement remains before dispatch-gate or ingress completion.
2. AC-2: Re-authentication and duplicate attachment are queued in server processing order without consuming the active turn's existing deadline on a client round trip.
3. AC-3: Cleanup-race proof deterministically observes accepted, in-flight attachment, and exactly one delivered terminal result.
4. AC-4: Cancellation, timeout, ordering, capacity, and token-rotation semantics remain green and production behavior is unchanged.
5. AC-5: Required Runtime fast lane passes at the reviewed exact revision.

## Dependencies

- Merged PR #228
- Blocks PR #242 and corrective issue #237

## Inputs

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/conversation_sessions.rs
- .github/workflows/ci.yml

## Non Goals

- Changing #237 capability authority
- Editing PR #242
- Redesigning unrelated Observatory APIs
- Running optional CI or broad workspace validation
