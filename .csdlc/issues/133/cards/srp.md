# Structured Review Prompt

Template: 1.0.0

Issue: 133

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Issue #132 five authority modules, focused integration tests, redaction boundary, revision/drift behavior, bounded enumeration, and restart parity.

## Prompts

- Can any caller synthesize rows or snapshots without the owning authority?
- Does every mutation, removal, replacement, and restore update or preserve revision truth correctly?
- Are rows complete, deterministic, bounded, and explicit about unavailable state?
- Can any private key, raw probe, signature, migration payload, or recovery payload escape through the snapshot APIs?
- Do focused tests prove N/N+1 drift and restart parity across all five authorities?

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
