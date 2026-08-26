# Structured Task Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair the #115 preparation packet on a clean current-main root, prove canonical #111/#112/#113/#270 terminal-cache dependencies, obtain fresh readiness/design review, and stop before bind, implementation, publication, merge, closeout, or parent mutation.

## Deliverables

- adl-runtime-kernel/src/conversation_rooms.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/lib.rs
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- demos/html-observatory/styles.css
- .csdlc/prepared/issues/115/validate_governed_room_implementation.py
- .csdlc/issues/115

## Acceptance

1. AC-1: Runtime governed-room turns require explicit bounded recipients and deny implicit broadcast or browser-expanded participant sets.
2. AC-2: Runtime room routing rejects duplicate, unknown, ineligible, unavailable, revoked, cross-Polis, duplicate-turn, and reordered-turn requests without consuming invalid turns incorrectly.
3. AC-3: Accepted room turns reuse Layer 8 AddressRecipients authority scope without redefining #112 authority or #270 acknowledgement trust.
4. AC-4: Runtime distinguishes accepted room turns from recipient-delivered turns and does not fabricate delivered state without delivery evidence.
5. AC-5: Observatory renders explicit participants, room transcript, composer, per-room turn sequencing, and accepted/partial/refused/unavailable/revoked delivery states.
6. AC-6: #115 excludes #278 durability, #114 parent coordination, #116/#117 qualification work, and #110 parent mutation.
7. AC-7: Focused Runtime, Observatory, smoke, formatting, strict clippy, diff hygiene, and exact fresh review pass before publication.

## Dependencies

- #111 terminal and ancestral to execution base
- #112 terminal and ancestral to execution base
- #113 terminal and ancestral to execution base
- #270 terminal and ancestral to execution base

## Inputs

- .git/csdlc-v2/requests/issue115-typed-read-canonical-recovery-20260813T1705Z.result.json
- .csdlc/prepared/issues/110/graph.json
- .git/csdlc-v2/derived-terminal/111.json
- .git/csdlc-v2/derived-terminal/112.json
- .git/csdlc-v2/derived-terminal/113.json
- .git/csdlc-v2/derived-terminal/270.json

## Non Goals

- Unbounded broadcast
- Implicit recipient selection by browser
- Cross-Polis federation policy
- Redefining #112 authority or #270 acknowledgement trust
- Branch/worktree bind, implementation, publication, merge, or closeout
- Mutating #110 parent staging or #114/#276/#277/#278 lifecycle state
