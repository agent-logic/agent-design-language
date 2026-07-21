# Structured Review Prompt

Template: 1.0.0

Issue: 5591

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/control.rs

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

- The existing SIGTERM process test remains ignored because it binds the fixed control test port; signal, pressure, and signed terminal triggers share the reviewed serialization helper, and the focused pressure and signed process proofs pass.

## Review Result

Revision: Some("git-blake3:6f19349e6d6227c362f5d73dce2c977aab41c1db:816baf9d3b09ceb37995f05a501ddbebb0ed379c433fd00b4f8751a9775005ce")

Reviewer: Some("subagent:019f8277-0050-7db0-a96b-05593df9c703")

Result: pass
