# Structured Review Prompt

Template: 1.0.0

Issue: 723

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/proof.rs
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

Revision: Some("git-blake3:952651016b159d2793081832adf162868d025adb:192bd26ba1c16b40f0eee8542b49c9fe8257b941ecd5610fbcc3c40d9e21c03e")

Reviewer: Some("codex:/root/review_sprint10_523_526_prep")

Result: pass
