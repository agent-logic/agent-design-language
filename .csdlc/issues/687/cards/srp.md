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

- GitHub CI remains the publication-time integration readback and has not run before PR publication.
- No live Runtime restart, AWS action, paid cloud action, provider credential use, or live provider call was performed by this issue.

## Review Result

Revision: Some("git-blake3:1e8853949079564e3e21ccf9010492bf05a8c960:ba4afc8508bdc4f7e5e7f043b39d1d80ddb5255a80ffa6cd49ad3b1c185fd1c7")

Reviewer: Some("subagent:/root/review_687_prepr_r1")

Result: pass
