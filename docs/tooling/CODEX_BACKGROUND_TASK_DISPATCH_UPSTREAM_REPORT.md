# Codex Background Task Dispatch Upstream Report

## Expected Behavior

Project discovery and projectless task creation should each return a terminal
result within 120 seconds. A successful creation returns a task ID that can be
found as the unique inventory delta and verified through task readback.

## Reproduction

Issue 35 used one uniquely named projectless no-op canary with no repository,
issue, or worktree authority. The retained
`background-task-dispatch-reproduction.json`, `task-inventory-receipts.json`,
and `task-readback-receipt.json` contain sanitized timing and identity proof.

## Observed Result

Project discovery returned the saved ADL project in 0.134 seconds. Projectless
dispatch returned a task ID in 1.092 seconds, classified as `created` rather
than a failure `diagnostic_class`. Readback completed with the exact canary
identity and prompt digest.

## Inventory Reconciliation

The current API operation is `codex.list_threads`; it returns a bounded response
without a caller-supplied cursor. The retained receipt records the interface's
terminal null cursor and complete response. The validator derives one new task
ID, matching creation and readback. `ownership-reconciliation.json` records the
same derived delta and binds both source receipts by SHA-256.

## Ownership Route

The verified result sets `ownership_disposition` to `transferred_to_canary` and
keeps `retry_allowed` false. Had creation timed out or become indeterminate,
ownership would have remained with the caller and no retry would be allowed.

## Non-Claims

This run proves one current happy-path canary. It does not prove that historical
UI spinners represented active work, that every dispatch failure is fixed, or
that ADL owns the task service. No ADL product code, lifecycle scheduler, claim,
lease, heartbeat, or retry wrapper was added.
