# Structured Review Prompt

Template: 1.0.0

Issue: 724

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/main.rs
csdlc-v3/src/commands/remote
csdlc-v3/src/commands/proof.rs
csdlc-v3/tests/operational_cli_commands.rs
csdlc-v3/tests/proof_parity_install_commands.rs
docs/csdlc-v3/CONTRACT.md
.csdlc/prepared/issues/724/validate-simple-issue-create.sh
.csdlc/evidence/724

## Prompts

- Does the convenience form use the exact shared dispatch rather than a weaker parallel path?
- Can any invalid or ambiguous input reach remote mutation?
- Are marker, assigned-number readback, receipt, and reconciliation proved?
- Does inactive v3 authority fail before side effects?
- Are docs clear without overstating cutover status?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The exact-head review was read-only; no live GitHub issue creation was performed.

## Review Result

Revision: Some("git-blake3:3945a31a74b2202325b4752a8c0d5ee64b41092b:3cbedaf78cf28ad9f6003f57c2fd34e861842abdd56db3231608071e70065bb2")

Reviewer: Some("codex:/root/execute_aws_727_728")

Result: pass
