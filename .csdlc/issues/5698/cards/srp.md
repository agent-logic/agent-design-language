# Structured Review Prompt

Template: 1.0.0

Issue: 5698

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

Runtime v3 redb durable state module, checkpoint/lifelog adapter wiring, tests, and retained proof evidence.

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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
