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

Revision: Some("git-blake3:f711b6787f9573d631fdb0ac93a37eff1d123512:c0915a0be588d788e3c92942d07929040726be2eb8afd8fd4b095fc4dfff71ef")

Reviewer: Some("codex:/root/review_sprint10_523_526_prep")

Result: pass
