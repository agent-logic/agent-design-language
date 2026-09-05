# Structured Review Prompt

Template: 1.0.0

Issue: 687

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

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
.csdlc/issues/687

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

- GitHub CI must rerun after republishing the repaired PR head.
- No live Runtime restart, AWS action, paid cloud action, provider credential use, or live provider call was performed by this issue.

## Review Result

Revision: Some("git-blake3:998fb11928ff783f4c4dc9a8354f6ee680c30044:c74401111c0fd12515ca8e0103043d0d42c5cdca4b1649b60daa0fc869e432ee")

Reviewer: Some("subagent:/root/review_687_prepr_r1")

Result: pass
