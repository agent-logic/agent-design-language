---
issue_card_schema: adl.issue.v1
wp: "1006"
slug: "v0922-native-coordination-completion"
title: "[v0.92.2][C-SDLC] Support native completion closure for coordination issues"
labels:
  - "track:roadmap"
issue_number: 1006
generated_at: "2026-09-16T02:45:19.992791+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "0.92.2"
required_outcome_type:
  - "tested_native_tooling_behavior"
repo_inputs:
  - ".git/csdlc-v3/local/invocations/worker10-1006/source-issue.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Standalone tooling repair prompted by completed Sprint #929; #1003 parallel preparation must not cause shared-source ownership collision."
pr_start:
  enabled: true
  slug: "v0922-native-coordination-completion"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-16T02:45:19.992791+00:00

# Structured Task Prompt

## Summary

Provide native typed completion closure for coordination-only issues with explicit operator authority, verified child completion evidence and authenticated immutable reconciliation.

## Goal

Provide native typed completion closure for coordination-only issues with explicit operator authority, verified child completion evidence and authenticated immutable reconciliation.

## Required Outcome

Provide native typed completion closure for coordination-only issues with explicit operator authority, verified child completion evidence and authenticated immutable reconciliation.

## Deliverables

Typed coordination completion request/authorization contract; bounded verified child-evidence checks; authenticated mutation/reconciliation; native finish compatibility; focused regression and installed fixture proof; operator docs.

## Acceptance Criteria

1. A coordination-only issue with explicit operator authority, exact identity/state and current completed-child evidence closes with completed reason and native finish reconciles it.
2. Missing or stale evidence, unfinished children, wrong issue identity/state and ordinary implementation issues fail closed before mutation.
3. Existing duplicate, superseded and no-op administrative closure retains its semantics.
4. Installed native command fixtures prove the complete close/readback/finish sequence without live GitHub writes; receipts bind exact operation and observed state.
5. Stdout remains JSON; stderr and durable evidence exclude credentials and sensitive payloads.

## Repo Inputs

.git/csdlc-v3/local/invocations/worker10-1006/source-issue.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; current remote/terminal code; retained #929 independent review and blocked audit in root Git metadata

## Dependencies

No additional implementation prerequisite identified; current native authority and existing terminal/remote owners must remain valid. Coordinate overlapping remote owner changes with #1003 before implementation.

## Target Files / Surfaces

csdlc-v3/src/commands/remote/mod.rs; csdlc-v3/src/commands/terminal.rs; bounded remote/terminal owner tests and installed command fixtures; docs/csdlc-v3 completion-route guidance

## Validation Plan

Small deterministic C-SDLC owner regression tests and installed CLI fixture proof with controlled GitHub adapter. Cover success followed by finish; wrong identity/state, stale/missing evidence, unfinished children, replay/reconciliation and existing administrative routes. Run strict Clippy and fmt on touched crate; native six-card validate. Hosted required checks remain separately pending.

## Demo Expectations

Installed native CLI fixture proof is required; no live provider or GitHub mutation demo.

## Non-goals

No broad lifecycle redesign, no generic raw-gh bypass, no closing unrelated issues, no weakening ordinary implementation or unfinished-child gates, no live closure during fixture validation.

## Issue-Graph Notes

No new dependency graph or sibling scope added.

## Notes

Coordination classification and evidence freshness must be verified, not self-certified. Prevent stale state/replay, identity substitution and implicit ordinary-issue completion. #929 workaround is historical evidence, not authority for new bypasses.

## Tooling Notes

Native v3 bound implementation authorized. Build isolated candidate only; shared stable binary remains unchanged.
