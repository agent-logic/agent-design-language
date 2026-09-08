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

Revision: Some("git-blake3:e76700b4a44575410ea4ccab5a1830e85ea89d29:66e46f77e59c9f9691eb5087f8b566448b5f0ae6e68d92a63af7a51cd3d5f653")

Reviewer: Some("codex:/root/review_sprint10_523_526_prep")

Result: pass
