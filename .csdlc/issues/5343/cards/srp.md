# Structured Review Prompt

Template: 1.0.0

Issue: 5343

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/5343
.csdlc/prepared/issues/5343
docs/milestones/v0.91.8/evidence/wp12/cutover-5343/report.json
docs/milestones/v0.91.8/evidence/wp12/cutover-handoff-5344.v1.json
adl-v2/tools/install-adl-v2.sh
adl-v2/crates/adl-cli/src/main.rs

## Prompts

- Can any missing, stale, malformed, contradictory, non-ancestral, or metadata-only #5344/#5345 fact bypass the execution gate?
- Can any argument, environment value, path, symlink, stale writer, lock race, interruption, or malformed receipt bypass exact installation verification or alter prior selector bytes outside the #5345 transaction?
- Are fresh-install selection, explicit v1 override, rollback-window checkpoints, exact restoration, and every failure class deterministic and fail-closed?
- Does #5343 own only cutover evidence and avoid selector implementation, Runtime v2 edits, legacy deletion, AWS, credentials, hidden network, and production overclaim?
- Are COTS, protected paths, LoC/test/module/time budgets, PVF classification, no-deferral, CI, exact review, and post-merge proof complete and executable?

## Findings

[
  {
    "id": "P1-malformed-selector-transaction-not-proven",
    "severity": "p1",
    "summary": "The malformed-selector case inspected an incomplete separate root but did not exercise a selector transaction against malformed active state.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "P1-host-specific-default-target-path",
    "severity": "p1",
    "summary": "The cutover proof defaulted Cargo output to a machine-specific /Volumes/FastWork path.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The reviewer did not rerun the mutating transaction-fault-matrix during read-only review.

## Review Result

Revision: Some("git-blake3:1b88b75a5d9e43a49be3ca9221396372d1677d7c:947ae36b5d44f930264b733b47d8601e747397cc01a5f16bbbee3b232e27f685")

Reviewer: Some("subagent:gpt-5.5:Anscombe")

Result: changes_required
