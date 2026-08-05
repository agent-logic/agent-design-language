# Structured Task Prompt

Template: 1.0.0

Issue: 5802

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Bind approved credentials, run the existing full recursive mirror, repair only observed in-scope defects, independently verify the complete Drive projection, and enable the schedule only after acceptance.

## Deliverables

- Successful native recursive execute report
- Exact inventory and readback acceptance evidence
- Verified 16-file CodeFriend Drive subtree
- Truthful schedule enablement decision
- Bounded code or runbook repair only if live execution exposes a defect

## Acceptance

1. AC-1: Current Drive .adl/docs/TBD is recursively populated and listable
2. AC-2: All 16 CodeFriend files exist exactly once at the expected path and match repository bytes
3. AC-3: Every generator-selected Markdown file verifies by parent, identity, MIME, listing, and exact content digest
4. AC-4: The native report is recursive_live with zero skipped or unverified results
5. AC-5: No remote deletion, unrelated move, duplicate sibling, or credential exposure occurs
6. AC-6: The automation remains paused on failure and is enabled only after full acceptance

## Dependencies

- Issue #5802 is open
- PR #5626 recursive implementation is present on current main
- An operator-approved Google credential source is available outside the repository
- The Google Drive connector can independently list and fetch the bounded mirror

## Inputs

- Issue #5802
- Issue #5587 and PR #5626
- adl/src/adl_gws_context_mirror.rs
- adl/src/adl_gws_drive_sync.rs
- adl/src/adl_gws_native.rs
- adl/src/bin/demo_adl_gws_context_mirror.rs
- docs/tooling/ADL_GOOGLE_DRIVE_CONTEXT_MIRROR_RUNBOOK.md
- .adl/docs/TBD/codefriend_ai

## Non Goals

- Do not make Drive canonical
- Do not import Drive edits into the repository
- Do not delete remote files
- Do not broaden Google permissions
- Do not use AWS or Spot
