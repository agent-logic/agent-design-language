---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.23"
slug: "legacy-ready-intent-recovery"
title: "[v0.92.1][TAIL-06.23][tooling] Recover retained ready intents without target identity"
labels:
  - "track:roadmap"
issue_number: 843
generated_at: "2026-09-11T15:33:13Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "tooling defect repair"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/843"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Additional remediation for D520-V3F-001 under parent #522."
pr_start:
  enabled: true
  slug: "legacy-ready-intent-recovery"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T15:33:13Z

# Structured Task Prompt

## Summary

Resolve and durably retain authenticated exact PR identity before retrying a pre-target ready intent.

## Goal

Recover retained legacy pull-request-ready intents that lack target identity without ambiguous replay.

## Required Outcome

A matching legacy intent reaches guarded ready reconciliation; mismatched or uncertain state fails closed before mutation.

## Deliverables

Narrow recovery fix, regression tests, issue proof, and refreshed #835 handoff.

## Acceptance Criteria

Automatic matching legacy recovery; mismatch rejection; idempotent repeated recovery; focused V3-F proof and exact-head review.

## Repo Inputs

Issue #843; parent #522; #835 current-candidate finding; merged #824/PR #830 implementation.

## Dependencies

Parent #522 and merged #824; #835 proof remains failed pending this repair.

## Target Files / Surfaces

csdlc-v3 remote mutation state machine, focused tests, and issue #843 proof packet.

## Validation Plan

Focused remote tests, exact legacy-intent regression, V3-F current-source validator, fmt, clippy, diff hygiene, and independent exact-head review.

## Demo Expectations

Deterministic fake-transport proof only.

## Non-goals

No broad GitHub mutation redesign, raw gh lifecycle write, or stale #840 ready claim.

## Issue-Graph Notes

#835 current-candidate proof exposed the gap after PR #840 merged.

## Notes

Never dispatch until exact authenticated target identity is durably retained.

## Tooling Notes

Native v3; fake transport for mutation proof.
