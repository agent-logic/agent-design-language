# Structured Review Prompt

Template: 1.0.0

Issue: 35

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/prepared/issues/35/validate-dispatch-evidence.rb
.csdlc/evidence/35/background-task-dispatch-reproduction.json
.csdlc/evidence/35/ownership-reconciliation.json
.csdlc/evidence/35/task-inventory-receipts.json
.csdlc/evidence/35/task-readback-receipt.json
docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH.md
docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH_UPSTREAM_REPORT.md

## Prompts

- Does the evidence distinguish timeout, typed failure, and verified successful ownership transfer?
- Can any dispatch attempt leave a hidden task or duplicate issue/worktree owner?
- Is the proposed repair assigned to the component proven to own the failure?
- Are retry guidance and retained records portable, bounded, and secret-free?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The evidence proves one bounded happy-path canary and does not prove historical UI hang behavior is globally fixed.

## Review Result

Revision: Some("git-blake3:dde5fe72f6c81482bf63a3abac3465a276129601:fb36f50b2a487cc152c2e24fc296145fcedbbd7f502c03f37e9b009f2a1dcf98")

Reviewer: Some("subagent:019fe208-e624-7c71-bc34-a0f4409f9079")

Result: pass
