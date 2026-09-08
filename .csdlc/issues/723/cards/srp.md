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

Revision: Some("git-blake3:e3fa669f16e45bae8e5beeff53fc16ef26935057:9b4bd6c90af640e25fc31023f31d57a5cd92edee48888b3412b030a2c7d153b4")

Reviewer: Some("codex:/root/review_sprint10_523_526_prep")

Result: pass
