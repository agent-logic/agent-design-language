# Structured Review Prompt

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/resident_shepherd.rs
adl-runtime-kernel/src/shepherd.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/shepherd.rs
adl-runtime-kernel/tests/support/runtime_init.rs
infra/runtime-v3/runtime-init.toml
.csdlc/prepared/issues/640
.csdlc/evidence/640

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

- none

## Review Result

Revision: Some("git-blake3:a1803219ce60e73ed025ad6edded2ffbda1d79c9:cf07e99f6309744022245b3028eb6d426bdde06d40efb09470e8ebfd437b513f")

Reviewer: Some("fresh-session:3f9822f6-9e17-4774-bd67-b348594312a5")

Result: pass
