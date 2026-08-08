# Codex Background Task Dispatch

## Success Rule

A dispatch succeeds only when `codex.create_thread` returns a nonempty task ID
within the 120-second bound, the task is the unique new identity in the retained
pre/post inventory, and `codex.read_thread` confirms the expected no-op canary.
Projectless canaries must have no repository, issue, or worktree binding.

## Reconciliation

Capture the task inventory immediately before and after dispatch. Use the
current `codex.list_threads` interface and retain its cursor state exactly as
returned. Derive the set difference from those receipts, then read the returned
task ID. Never infer ownership from a spinner, title, or an unverified list row.

## Retry Rule

Do not retry a successful, timed-out, indeterminate, incomplete, duplicate, or
unverified dispatch. Retry is permitted only after an explicit typed failure
and a complete inventory proves that no new task was created. A returned task
ID always prohibits retry.

## Escalation

Retain sanitized timing, inventory, and readback receipts when dispatch does
not terminate cleanly. Keep ownership with the caller and report the failure to
the Codex application owner. Do not add ADL claims, leases, wrappers, polling
state machines, or scheduler code to compensate for an application defect.

The issue-35 canary completed normally: project discovery took 0.134 seconds,
projectless creation took 1.092 seconds, and readback proved the expected no-op
completion. This establishes the current happy path; it does not prove every
historical hang is repaired.
