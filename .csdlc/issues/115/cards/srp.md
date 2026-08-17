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
adl/tools/test_html_observatory.sh
.csdlc/prepared/issues/115/validate_governed_room_implementation.py
.csdlc/issues/115

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

Revision: Some("git-blake3:28da28014cf0db439100e08f4f8436e46ed61844:babb103592fd2dfef7aaf86fa2382ddcb6e5aaa8c6e43688f95822a60478c00e")

Reviewer: Some("fresh-session:c98af1d7-23f3-4826-a820-7d48a41dfb02")

Result: pass
