# Structured Review Prompt

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/640
.csdlc/evidence/640
adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/resident_shepherd.rs
adl-runtime-kernel/src/shepherd.rs
adl-runtime-kernel/tests
adl-runtime/src/guardian.rs
adl-runtime/src/bin/adl-runtime-guardian.rs
infra/runtime-v3/runtime-init.toml

## Prompts

- Does configuration remain provider-neutral and keep credentials outside tracked and API surfaces?
- Does every Shepherd execute through the existing governed operation boundary?
- Can any preload or inference timeout terminate or globally block the Runtime or unrelated agents?
- Do /v1/ready, blocking_reasons, roster/detail, and Observatory agree for model_loading, ready, and degraded?
- Is recovery lifetime-long with bounded probes and intervals and idempotent per configured canonical identity?
- Does the branch start from merged #617 rather than carrying an accidental PR stack?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The full crate had one concurrency-only Guardian-soak timeout among 648 tests; the affected four process tests passed 4/4 when rerun serially.

## Review Result

Revision: Some("git-blake3:cfa88fea982503ae3b4daceccd741b01b38d2b51:e9f934ceead9dadf3dfaba45362ea0c5b21d30c81928105af920a12dfcabe943")

Reviewer: Some("fresh-session:25cad475-d5d1-4eac-a0d6-41d924dea0f7")

Result: pass
