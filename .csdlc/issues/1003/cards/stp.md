---
issue_card_schema: adl.issue.v1
wp: "1003"
slug: "scope-rebind-validator-recovery"
title: "[v0.92.2][C-SDLC] Restore rebind and validator replacement after scope amendments"
labels:
  - "track:roadmap"
issue_number: 1003
generated_at: "2026-09-16T02:43:15.759077+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "behavioral_fix"
repo_inputs:
  - ".git/csdlc-v3/local/invocations/worker10-1003/source-issue.json"
canonical_files: []
demo_required: yes
demo_names: []
issue_graph_notes:
  - "Standalone tooling defect; #1006 is a separate fix."
pr_start:
  enabled: true
  slug: "scope-rebind-validator-recovery"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-16T02:43:15.759077+00:00

# Structured Task Prompt

## Summary

Restore guarded rebind after scope amendments and permit replacing an inadmissible validator before proof.

## Goal

Restore guarded rebind after scope amendments and permit replacing an inadmissible validator before proof.

## Required Outcome

A scope-rewound issue rebinds in its registered checkout at unchanged HEAD; changed HEAD refreshes binding without running old validators; typed validator replacement succeeds before proof; stale authority, branch/worktree mismatch, stale requests and pending recovery still fail closed; evidence invalidation remains explicit.

## Deliverables

Reproduce both #1003 failure paths in focused tests. Inspect semantic bind operation identity and admission after scope_acceptance. Add the smallest explicit rebind/head-refresh path preserving registered topology and version guards. Permit typed validator replacement after the guarded refresh without executing retained validators. Test negative ownership/authority/recovery and evidence invalidation cases; run installed candidate scenarios, independent review and CI.

## Acceptance Criteria

A scope-rewound issue rebinds in its registered checkout at unchanged HEAD; changed HEAD refreshes binding without running old validators; typed validator replacement succeeds before proof; stale authority, branch/worktree mismatch, stale requests and pending recovery still fail closed; evidence invalidation remains explicit.

## Repo Inputs

Issue #1003 and current native intent/semantic binding, edit, proof, recovery and transaction contracts.

## Dependencies

No unmerged implementation dependency; #970 is the reproducer, not a mutation target.

## Target Files / Surfaces

csdlc-v3/src/application/intent/{context,local}.rs; semantic binding owner; focused intent/semantic regression tests; docs/csdlc-v3/INTENT_COMMANDS.md

## Validation Plan

Focused Rust intent and semantic-owner tests, installed candidate rebind and validator replacement fixtures, negative identity/admission cases, cargo fmt and strict Clippy, then required GitHub CI tests and coverage. PVF deterministic local contract/regression proof; small CPU/local Git and isolated synthetic transport; required gate; no provider/cloud.

## Demo Expectations

Installed isolated candidate demonstrates both rebind paths and validator replacement without running inadmissible validator.

## Non-goals

No raw state edits, automatic phase advancement, weaker validator admission, fallback authority or changes to #970.

## Issue-Graph Notes

Standalone tooling defect; #1006 is a separate fix.

## Notes

Preparation only; no implementation or proof claimed. Preserve all authority/version/topology guards. No arbitrary execution time or token limits.

## Tooling Notes

Use installed native v3 typed routes; build isolated candidate for proof; do not replace shared binary.
