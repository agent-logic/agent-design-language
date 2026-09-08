# Structured Review Prompt

Template: 1.0.0

Issue: 724

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/main.rs
csdlc-v3/src/commands/remote
csdlc-v3/tests/operational_cli_commands.rs
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

- The exact-head review was read-only and relied on the retained focused validation; it did not perform a live GitHub mutation.

## Review Result

Revision: Some("git-blake3:3f6f4307ac2fab48b42742540636e7c364b6d166:86172148c6de86b4863031ed728e7ae0bc6a3a1cd52ca321d39445c093ce7d2e")

Reviewer: Some("codex:/root/remediate_v3_725")

Result: pass
