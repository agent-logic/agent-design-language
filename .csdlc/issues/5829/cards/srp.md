# Structured Review Prompt

Template: 1.0.0

Issue: 5829

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5829
.csdlc/prepared/issues/5829/design.md
.csdlc/prepared/issues/5829/produce-native-receipt.rb
.csdlc/prepared/issues/5829/validate-native-receipts.rb
.csdlc/evidence/5829
.github/workflows/wp12-native-capability-envelope.yml
adl-runtime-kernel/src/capability_envelope.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/capability_envelope.rs
adl-runtime-kernel/tests/fixtures/capability_envelope
docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md

## Prompts

- Can any envelope grant authority, prove invocation, or imply unlimited capacity from missing fields?
- Are provider/model/tool/skill identifiers, grants, denials, limits, provenance, and unsupported claims canonical and complete?
- Do stale evidence, escalation, secret-like content, private paths, and host paths fail closed without leaking values?
- Are #5825, #5826, #4761, and every WP-12 acceptance claim current at exact HEAD?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Exact-head native macOS and Linux capability-envelope proof remains mandatory after publication before merge.

## Review Result

Revision: Some("git-blake3:a3d8022b9830279b5b6c32dfcecde9f183632f6a:eaafc0e803e7fee02356aed1b57a3a643d5a4e2c9de4bcdca2e7ed902ff51ed1")

Reviewer: Some("/root/sprint4_5857/review_5829_exact_head")

Result: pass
