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

Revision: Some("git-blake3:afd8c1b616c485099be1a2035c7ef7cb46ef726e:233e1acae9167b95ff3da54c8c3f8fd01e4afabbee6354793b927259bf182a7e")

Reviewer: Some("codex:/root/review_sprint10_523_526_prep")

Result: pass
