# v0.92.2 existing issue reconciliation

Status: issue #523 planning correction; no new issues created.

The authenticated complete issue inventory identified the following existing work. A missing GitHub milestone does not mean backlog: these issues target v0.92.2 while that milestone is not yet open. The operator explicitly corrected #718's stale backlog wording and requested urgent delivery. That current direction controls this plan.

| Existing issue | Planned ID | Scheduling | Required result |
|---|---|---|---|
| [#718](https://github.com/agent-logic/agent-design-language/issues/718) | RT-A2A | Urgent; own typed readiness now, no CodeFriend or WP-01 dependency | Canonical-name addressing, identity continuity and two ordinary agents exchanging model-generated replies |
| [#717](https://github.com/agent-logic/agent-design-language/issues/717) | RT-ORIENT | Existing authority; verify completed #708/#709 baseline | Versioned first-turn orientation, canonical capability inventory and no authority grants |
| [#720](https://github.com/agent-logic/agent-design-language/issues/720) | OBS-LIVE | Existing authority; independent of product build and #512 publication | Live-only Observatory without obsolete retained polling; preserve historical evidence |

#717 and #718 overlap welcome-package and Runtime control paths. Give #718 priority and partition or sequence overlapping edits; merge dependencies are added only when a concrete shared change requires them. The owner of #718 must prove the full governed message/reply path, not merely a name lookup test. Live provider use remains bounded by that issue's execution authorization.

Reuse all three issue numbers exactly once. Do not create replacement issues at WP-01. Completed #620 is the first-pass document refresh; completed #439 is predecessor planning input. Both remain historical, not new execution rows. Other backlog is excluded from the admitted denominator and receives no new issue or milestone assignment here.

The denominator is 41 rows: the original 30 planning rows, three reused existing issues and eight SIM sprint rows. Resolve WP-01's conductor separately; it is not its own creation target. Then reconcile 40 remaining rows: 37 prospective creations (requiring separate authorization) and three existing issues.

Snapshot: [existing-issues.json](evidence/issue-523/existing-issues.json). Captured issue text is historical evidence, not authority to override the operator correction. State/milestone metadata is a point-in-time read and must be refreshed before execution or remote reconciliation. #523 does not implement or close these issues.

The eight SIM rows are launched through their own sprint authority before or alongside Runtime, independently of WP-01. #718 remains urgent and does not wait for SIM completion.
