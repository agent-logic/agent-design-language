# Structured Review Prompt

Template: 1.0.0

Issue: 723

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/lib.rs
csdlc-v3/src/commands/proof.rs
csdlc-v3/tests/foundation.rs
csdlc-v3/tests/proof_parity_install_commands.rs
csdlc-v3/tests/real_issue_canary.rs

## Prompts

- Verify stderr fallback accepts only one typed document and preserves nonzero exit.
- Verify install authorization binds exact head and digests.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:c1abb1e6baeb2cf0ea24e9af678ef4360ab230b1:da5988e7c365cb34906d8d32496f97a14621259962a7f81d65cbfc639be022d9")

Reviewer: Some("codex:/root/review_sprint10_523_526_prep")

Result: pass
