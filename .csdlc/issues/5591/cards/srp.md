# Structured Review Prompt

Template: 1.0.0

Issue: 5591

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/live_continuity.rs
adl-runtime-kernel/src/telemetry.rs
adl-runtime-kernel/src/weather.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/guardian_soak.rs
infra/horust/adl-runtime-kernel.toml
.csdlc/issues/5591/cards/sor.values.json
.csdlc/issues/5591/cards/vpp.values.json

## Prompts

- Does every acceptance criterion require guardian-launched live production behavior rather than fixture or library evidence?
- Are checkpoint/replay/resume determinism, authenticity, duplicate prevention, and corrupt-state negatives fully specified?
- Does pressure handling prove admission quiescence, durable serialization, terminal Observatory output, bounded shutdown, and correct restart?
- Are local/remote access and Observatory configuration-driven, TLS-authenticated, redacted, and free of hard-coded addresses?
- Are Runtime v2, AWS, cutover, deletion, provider deployment, and later-parity ownership boundaries explicit?
- Do plan steps and validation lanes cover AC-1 through AC-8 bidirectionally with no deferred acceptance?
- Is #5336 integration the sole current stop while the future claim-scope proposal remains non-authoritative?

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
