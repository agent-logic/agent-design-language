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

Revision: Some("git-blake3:11dc51a88da7611d7793c483bf412b55bf6f35b1:89ed44fc648d82eaacd67f4bd143cd3d5b5e9f4b24286382f5738725b918d735")

Reviewer: Some("codex:/root/execute_aws_727_728/review_723_7854750")

Result: pass
