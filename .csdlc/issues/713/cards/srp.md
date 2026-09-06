# Structured Review Prompt

Template: 1.0.0

Issue: 713

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/conversation_history.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/conversation_history.rs
demos/html-observatory/app.js
demos/html-observatory/tests/conversation_sessions.test.mjs
demos/html-observatory/tests/security_privacy_adversarial.test.mjs
adl/tools/test_issue713_a2a_history.sh
.csdlc/evidence/713

## Prompts

- Can both halves be reconstructed from Runtime-authoritative evidence?
- Is the path identical for all agents?
- Can replay, restart, or rehydration duplicate or misattribute a turn?
- Does the API redact private provider and prompt data?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live Wuji Runtime/ACIP proof remains deferred unless explicitly operator-authorized with ADL_LIVE_WUJI_A2A_HISTORY=1.

## Review Result

Revision: Some("git-blake3:577a2f10a6c44195907b352487f47d4939a37b32:27dc98640f4514326175200321e191b5e5031bdb02f78dc071f391ee06d9fdb1")

Reviewer: Some("subagent:/root/review_713_a2a_history_r1")

Result: pass
