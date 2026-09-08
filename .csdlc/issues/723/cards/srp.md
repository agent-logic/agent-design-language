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

Revision: Some("git-blake3:2cefb49c939ecaa3a8192a2ffeb0f685f011451b:47eb4c3b98ef72a7ce7d23c939e79542b2013b11a42962c6eafa8607f8830bdf")

Reviewer: Some("codex:/root/review_sprint10_523_526_prep")

Result: pass
