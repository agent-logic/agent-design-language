# Structured Review Prompt

Template: 1.0.0

Issue: 5335

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.91.8
docs/milestones/v0.91.7/README.md
docs/milestones/v0.92/README.md

## Prompts

- Does the architecture actually enable at least 80 percent deletion rather than moving legacy complexity?
- Are Runtime v3, C-SDLC v2, ADL core, adapters, demos, and proof tooling assigned to non-overlapping owners?
- Does the issue graph sequence characterization, construction, parity, cutover, and deletion safely?
- Does any planning language overclaim proof, completion, or v0.92 readiness?

## Findings

[
  {
    "id": "F-5335-01",
    "severity": "p1",
    "summary": "Seeded and moved issue bodies referenced sunset v1 lifecycle routing.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:7c3e1e0e86a4ca982231ce91c39073530c5408e6:a2cb6e69050799dbf591108612b6e2233eab2fded3106d8f99ab41e8bfd2fcdf",
    "route": null
  },
  {
    "id": "F-5335-02",
    "severity": "p2",
    "summary": "Moved #5107 retained stale Runtime v2 and pre-deployment acceptance truth.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:7c3e1e0e86a4ca982231ce91c39073530c5408e6:a2cb6e69050799dbf591108612b6e2233eab2fded3106d8f99ab41e8bfd2fcdf",
    "route": null
  },
  {
    "id": "F-5335-03",
    "severity": "p2",
    "summary": "Canonical FEATURE_DOCS_v0.91.8.md index was missing.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:7c3e1e0e86a4ca982231ce91c39073530c5408e6:a2cb6e69050799dbf591108612b6e2233eab2fded3106d8f99ab41e8bfd2fcdf",
    "route": null
  },
  {
    "id": "F-5335-04",
    "severity": "p2",
    "summary": "Quality gate assigned the default selector switch to WP-13 instead of WP-12.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:7c3e1e0e86a4ca982231ce91c39073530c5408e6:a2cb6e69050799dbf591108612b6e2233eab2fded3106d8f99ab41e8bfd2fcdf",
    "route": null
  },
  {
    "id": "F-5335-05",
    "severity": "p2",
    "summary": "Bound-phase SPP cannot be truthfully replanned after operator scope expansion.",
    "actionable": true,
    "in_scope": true,
    "disposition": "accepted_risk",
    "fix_revision": null,
    "route": "https://github.com/danielbaustin/agent-design-language/issues/5364"
  },
  {
    "id": "F-5335-06",
    "severity": "p1",
    "summary": "WP-12 parent #5344 was accidentally emptied during bulk issue-body reconciliation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:7c3e1e0e86a4ca982231ce91c39073530c5408e6:a2cb6e69050799dbf591108612b6e2233eab2fded3106d8f99ab41e8bfd2fcdf",
    "route": null
  },
  {
    "id": "F-5335-07",
    "severity": "p1",
    "summary": "Generated VPP references deleted v1 planning validation wrapper.",
    "actionable": true,
    "in_scope": true,
    "disposition": "accepted_risk",
    "fix_revision": null,
    "route": "https://github.com/danielbaustin/agent-design-language/issues/5365"
  },
  {
    "id": "F-5335-08",
    "severity": "p2",
    "summary": "Active helper-skill guidance conflicts with final typed-v2 authority.",
    "actionable": false,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "https://github.com/danielbaustin/agent-design-language/issues/5366"
  },
  {
    "id": "F-5335-09",
    "severity": "p2",
    "summary": "Staleness checker cannot represent planned milestone posture without premature root activation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "accepted_risk",
    "fix_revision": null,
    "route": "https://github.com/danielbaustin/agent-design-language/issues/5367"
  },
  {
    "id": "F-5335-10",
    "severity": "p1",
    "summary": "Typed review can accept a dirty substantive tree that typed publication cannot publish after commit, with no reviewed-phase reassignment path.",
    "actionable": false,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "https://github.com/danielbaustin/agent-design-language/issues/5368"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The bound issue retains pending SPP step statuses because typed mutation is prohibited after bind; #5364 owns an audited replan path.
- The generated VPP retains a deleted wrapper command; equivalent focused validation passed and #5365 owns generator repair.
- Root README and CHANGELOG activation checks remain intentionally deferred until the milestone is activated; #5367 owns planned-mode validation.
- Installed helper-skill authority drift is routed to #5366.

## Review Result

Revision: Some("git-blake3:7c3e1e0e86a4ca982231ce91c39073530c5408e6:a2cb6e69050799dbf591108612b6e2233eab2fded3106d8f99ab41e8bfd2fcdf")

Reviewer: Some("bounded-subagent-review-5335")

Result: pass
