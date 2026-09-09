# v0.92.2 existing issue reconciliation

Status: issue #523 planning correction; no new issues created.

The authenticated complete issue inventory originally admitted three existing issues. On 2026-09-09 the operator promoted #717 and #718 into the active v0.92.1 bugfix band. They are predecessor work consumed by v0.92.2, not part of this milestone's execution denominator.

| Existing issue | Planned ID | Scheduling | Required result |
|---|---|---|---|
| [#720](https://github.com/agent-logic/agent-design-language/issues/720) | OBS-LIVE | Existing authority; independent of product build and #512 publication | Live-only Observatory without obsolete retained polling; preserve historical evidence |

#717 and #718 still overlap welcome-package and Runtime control paths, but that collision is resolved in v0.92.1. v0.92.2 consumes their merged outcomes and must not recreate either issue.

Reuse #720 exactly once. Do not create replacement issues for #717 or #718 at WP-01. Completed #620 is the first-pass document refresh; completed #439 is predecessor planning input. Both remain historical, not new execution rows. Other backlog is excluded from the admitted denominator and receives no new issue or milestone assignment here.

The denominator is 39 rows: the original 30 planning rows, one reused existing issue and eight SIM sprint rows. Resolve WP-01's conductor separately; it is not its own creation target. Then reconcile 38 remaining rows: 37 prospective creations (requiring separate authorization) and one existing issue.

Snapshot: [existing-issues.json](evidence/issue-523/existing-issues.json). Captured issue text is historical evidence, not authority to override the operator correction. State/milestone metadata is a point-in-time read and must be refreshed before execution or remote reconciliation. #523 does not implement or close these issues.

The eight SIM rows are launched through their own sprint authority independently of WP-01. #717 and #718 are completed or closed out in v0.92.1 and do not wait for SIM completion.
