# Structured Review Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/conversation_rooms.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/lib.rs
demos/html-observatory/app.js
demos/html-observatory/index.html
demos/html-observatory/styles.css
adl/tools/validate_v092_governed_room_observatory.mjs
.csdlc/issues/115
.csdlc/prepared/issues/115
.csdlc/evidence/115

## Prompts

- Verify #115 implements governed multi-agent rooms and message routing only within its owned Runtime/Observatory scope.
- Verify Runtime governed-room routes require explicit bounded recipients, reject implicit broadcast, preserve ordering/replay refusal, and reuse Layer 8 AddressRecipients authority without redefining #112 or #270.
- Verify accepted governed-room routes do not claim recipient delivery unless actual delivery evidence exists; #270 acknowledgement trust remains a dependency/non-goal.
- Verify Observatory renders participant selection, room transcript, accepted/partial/refused/unavailable/revoked states, and per-room turn sequencing without hidden browser recipient expansion.
- Verify focused Runtime, UI, smoke, formatting, clippy, and diff-hygiene proof truth matches the exact reviewed revision and no #278/#114/#116/#117/#110 work is absorbed.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
