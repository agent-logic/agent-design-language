# Structured Review Prompt

Template: 1.0.0

Issue: 5698

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/Cargo.lock
adl-runtime-kernel/Cargo.toml
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/durable_state.rs
adl-runtime-kernel/src/governed_operations.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/durable_state.rs
adl-runtime-kernel/tests/governed_operations.rs
.csdlc/evidence/5698

## Prompts

- Verify Runtime v3 checkpoint and lifelog production adapters use redb as the single durable state authority.
- Verify restart restores exact committed bytes and rejects identity, schema, generation, and hash mismatches.
- Verify writer locking, corruption, and interrupted-write behavior fail closed without fallback JSON/JSONL storage.
- Verify scope coordination with #5344 and no tracked main or /private/tmp edits.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The initial Gemini exact-head review found non-atomic redb sequence allocation; current exact head 8dd143ab4aad09529315ae7b400b2e793fef2ec5 fixes it and Gemini remediation verification returned PASS / no findings.

## Review Result

Revision: Some("git-blake3:8dd143ab4aad09529315ae7b400b2e793fef2ec5:e5d0df256c00e828a3086d2e3982436536632d9e512110cb5e890e2de6268d26")

Reviewer: Some("provider:gemini-3.1-pro-preview")

Result: pass
