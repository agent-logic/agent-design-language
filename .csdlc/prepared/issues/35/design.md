# Issue 35 Design: Bounded Codex Background Dispatch

## Objective

Turn Codex background task dispatch into an observable ownership transition for
one disposable no-op canary. A dispatch is successful only when it returns a
task ID within the declared timeout and readback proves that the task carries
the expected canary identity while owning no repository, issue, or worktree.
Every other outcome is a typed failure or an indeterminate result that must be
reconciled before retry.

## Ownership Boundary

Task creation and project discovery are Codex application services, not ADL
runtime or C-SDLC lifecycle code. This issue first proves which external
operation fails. Repository changes are limited to portable evidence and
operator guidance unless the reproduction identifies a concrete ADL-owned
adapter defect. No wrapper, claim, lease, heartbeat, scheduler, or retry state
machine will be added to compensate for an unproven upstream defect.

## Canary Contract

The operator creates a unique canary identifier and a no-op prompt that forbids
repository mutation, issue ownership, worktree ownership, and child dispatch.
Before dispatch, the lane records the complete live task-ID inventory and the
prompt digest. The lane then performs exactly one project-discovery request and
one projectless task-creation request, each with a 120-second timeout. The
projectless request carries only the canary identifier and no project binding.

Terminal result classes are:

- `created`: a task ID was returned and verified by readback;
- `typed_failure`: the service returned a bounded diagnostic;
- `timeout`: no terminal response arrived before the deadline;
- `indeterminate`: transport or client state prevents proving creation or
  failure.

Only `created` transfers ownership. All other classes retain ownership in the
calling task.

## Reconciliation

After every attempt, read the complete live task-ID inventory again and retain
the sanitized paginated API response, observation timestamp, request cursor,
response cursor, and terminal nil cursor proving pagination completion. Derive
the set difference from those retained snapshots. A returned task ID must be
the unique new task and readback must match the canary identifier and prompt
digest while reporting no repository, issue, or worktree binding. A failed,
timed-out, or indeterminate attempt is safe only when the derived set difference
is empty. Only an explicit typed failure with a complete empty inventory delta
may permit retry; timeout and indeterminate results always prohibit retry
because a late task may still appear. Any unexpected or unverified new task
fails closed: do not retry, do
not transfer ownership, and do not start implementation elsewhere.

## Evidence

`.csdlc/evidence/35/background-task-dispatch-reproduction.json` records the
request mode, timeout, elapsed time, result class, diagnostic class, returned
task ID when present, and portable target identity.

`.csdlc/evidence/35/ownership-reconciliation.json` records pre/post task-ID
inventories per attempt, the derived set difference, task readback fields,
ownership disposition, and whether retry is permitted. The validator derives
the new-task set itself rather than trusting a hidden-task assertion. Neither
artifact may contain credentials, personal data, host-specific absolute paths,
or raw task content unrelated to the canary.

`.csdlc/evidence/35/task-inventory-receipts.json` retains the sanitized pre- and
post-dispatch task-list response pages, source operation, timestamps, cursor
chain, and terminal pagination state. The ownership record must identify this
artifact by SHA-256; validation derives its inventories only from this receipt.

`.csdlc/evidence/35/task-readback-receipt.json` retains the sanitized
`codex.read_thread` response for a created canary, or a typed no-task record
when dispatch returned no task ID. The ownership record binds this receipt by
SHA-256, and validation derives canary identity and absent repository ownership
from the receipt rather than from the ownership summary.

## Operator Guidance

`docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH.md` will define the success rule,
timeout classes, reconciliation requirement, retry prohibition, and escalation
path. `docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH_UPSTREAM_REPORT.md` will
retain the sanitized reproduction, expected and observed behavior, result
classes, inventory delta, environment metadata, and owner routing. If an
ADL-owned defect is proven instead, any code change requires explicit scope
reconciliation before work continues.

## Validation And Review

Focused validation parses all evidence files, checks required schema and
terminal fields, independently derives inventory deltas, validates canary
readback, rejects machine-local paths, and confirms the ownership disposition
is consistent with the task result. A retained exact-revision review receipt
is the canonical issue-35 review assignment and result: the same independently
assigned subagent, the same typed revision, completed review, and no findings.
Independent review checks timeout semantics, hidden or duplicate ownership
risk, retry safety, evidence redaction, and component ownership.

## Stop Conditions

Stop without retry when no disposable canary exists, live inventory cannot be
read, a dispatch remains indeterminate, duplicate ownership cannot be excluded,
or the proposed repair belongs to an unproven component.
