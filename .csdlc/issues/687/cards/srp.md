# Structured Review Prompt

Template: 1.0.0

Issue: 687

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/config_reload.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/src/resident_shepherd.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/shepherd.rs
adl-runtime-kernel/tests/support/runtime_init.rs
adl-runtime/tests/support/tls.rs
.csdlc/evidence/687
.csdlc/prepared/issues/687

## Prompts

- Are all five readiness states mutually exclusive and semantically precise?
- Can any unsupported, unavailable, loading, or failed adapter project ready or communication-eligible?
- Do resident Shepherd and dynamic-agent paths use the same mapping?
- Does production assembly still reject missing/placeholder bindings?
- Are provider/model identifiers and errors redacted appropriately?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GitHub CI remains the publication-time integration readback and has not run before PR publication.

## Review Result

Revision: Some("git-blake3:f55daced1854473802de136f342addbb5556bacf:92cd659828bd3124bb70d7609e0a2dc36c3218ee03889c6eb2fa7cf5a2ad6dc2")

Reviewer: Some("subagent:/root/review_687_prepr_r1")

Result: pass
