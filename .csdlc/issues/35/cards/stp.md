# Structured Task Prompt

Template: 1.0.0

Issue: 35

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Diagnose and prove bounded Codex dispatch behavior; do not modify unrelated ADL product code or claim an upstream application fix without executable evidence.

## Deliverables

- .csdlc/evidence/35/background-task-dispatch-reproduction.json
- .csdlc/evidence/35/ownership-reconciliation.json
- .csdlc/evidence/35/task-inventory-receipts.json
- .csdlc/evidence/35/task-readback-receipt.json
- docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH.md
- docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH_UPSTREAM_REPORT.md

## Acceptance

1. Project discovery terminates within 120 seconds and records terminal_result project_found with a nonempty returned_project_id or exactly one of typed_failure, timeout, or indeterminate with a nonempty diagnostic_class
2. One projectless no-op canary dispatch terminates within 120 seconds and records terminal_result created with a nonempty returned_task_id or exactly one of typed_failure, timeout, or indeterminate with a nonempty diagnostic_class
3. A created dispatch is the unique delta derived from complete pre/post codex.list_threads receipts with the interface's terminal cursor state, and its digest-bound codex.read_thread receipt proves the expected canary_id and prompt_digest while repository, issue, and worktree bindings are absent before ownership_disposition may equal transferred_to_canary
4. For a typed_failure, timeout, or indeterminate dispatch the validator derives an empty delta from complete inventory receipts with terminal cursor state and a typed no-task readback receipt; every incomplete inventory, unexpected, duplicate, or unverified new task fails closed and retains ownership in the caller
5. retry_allowed is true only for an explicit typed_failure with complete inventory receipts and an empty derived delta; created, timeout, indeterminate, incomplete, unexpected, duplicate, or unverified outcomes prohibit retry
6. Retained reproduction, ownership, inventory, and readback JSON record exact schemas, a unique canary_id, prompt_digest, two 120-second attempts, elapsed_seconds not exceeding 120, mode-specific terminal fields, sanitized source receipts with timestamps and digest binding, derived task delta, ownership_disposition, and retry_allowed without secrets, personal data, raw prompt content, or machine-local absolute paths
7. The canonical issue-35 review assignment and completed review name the same independently assigned subagent and typed revision, cover every declared affected area, and contain zero unresolved findings

## Dependencies

- Codex desktop task APIs must be available for the bounded canary
- A non-production canary target must be selected before dispatch

## Inputs

- AGENTS.md
- docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md
- docs/tooling/C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md

## Non Goals

- Changing C-SDLC issue creation, binding, or claim-free lifecycle authority
- Building a replacement task scheduler inside ADL
- Retry loops that can create duplicate background tasks
- Modifying issue #17 or its worktree
- Claiming that a client UI spinner proves a running task
