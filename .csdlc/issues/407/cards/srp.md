# Structured Review Prompt

Template: 1.0.0

Issue: 407

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/407
.csdlc/prepared/issues/407
.csdlc/evidence/407

## Prompts

- Verify the new operation is SIP Goal specific and recovery-provenance gated.
- Verify tests prove both allowed recovered repair and rejected unrecovered mutation.
- Verify no publication/review guard is weakened.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Validation is focused rather than full-suite; no uncovered issue-specific correctness risk was identified.

## Review Result

Revision: Some("git-blake3:1cc0afa63b1906b18064778d0f367bf93e5b20b6:167be6441a652fdf1fdfdc491c860ecbb46ce103f4cd74c680c57bdc25085e51")

Reviewer: Some("fresh-session:5842e447-3520-4686-ab09-08b49d028904")

Result: pass
