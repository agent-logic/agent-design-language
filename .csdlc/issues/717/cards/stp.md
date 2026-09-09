---
issue_card_schema: adl.issue.v1
wp: "RT-ORIENT"
slug: "polis-capability-orientation"
title: "[v0.92.1][Runtime] Teach admitted agents about Polis modules and capabilities"
labels:
  - "track:roadmap"
issue_number: 717
generated_at: "2026-09-09T18:30:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.4"
required_outcome_type:
  - "runtime_behavior_docs_and_focused_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/717"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Operator-promoted v0.92.1 bugfix lane; #718 is promoted separately and is not implemented here."
pr_start:
  enabled: true
  slug: "polis-capability-orientation"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: 2026-09-09T18:30:00Z

# Structured Task Prompt

## Summary

Expand and mechanically validate the agent Welcome Package against canonical Runtime capability identifiers.

## Goal

Agents understand what the Polis can provide and how governed access works before their first turn.

## Required Outcome

A compact capability map is delivered with exact version/digest provenance and fails validation when its canonical inventory drifts.

## Deliverables

- Expanded Welcome Package
- Canonical inventory validation
- Negative drift tests
- Milestone planning reconciliation
- Focused validation proof

## Acceptance Criteria

- Required capability families and Freedom Gate are present.
- Runtime contract use and four-stage availability/authority distinctions are explicit.
- Missing, stale, duplicate, and invented identifiers fail.
- First-turn injection and provenance remain unchanged.

## Repo Inputs

- adl-runtime-kernel/src/operations.rs and assembly.rs
- docs/architecture/RUNTIME_V3_OPERATIONAL_COMPONENTS.md
- docs/explainers/ACIP.md, UTS_AND_ACC.md, and AEE.md
- Existing orientation implementation

## Dependencies

- #708/#709 are complete.
- No external dependency blocks deterministic implementation.

## Target Files / Surfaces

- Welcome Package prose
- Orientation resource validation and unit tests
- Bounded milestone planning truth

## Validation Plan

Focused Runtime unit tests, relevant first-turn tests, format/diff checks, and independent bounded review.

## Demo Expectations

Deterministic local proof only; no live model or cloud call.

## Non-goals

- Granting capabilities
- Changing service behavior
- Implementing #718
- Live deployment

## Issue-Graph Notes

- Build on #708/#709 versioned orientation delivery.
- Reconcile v0.92.2 planning removal and v0.92.1 admission.
- Avoid collision with #718 canonical-name routing.

## Notes

Do not create a second operational service registry; derive the operational subset from REQUIRED_OPERATIONAL_ADAPTERS and validate the documented broader capability families explicitly.

## Tooling Notes

Use native C-SDLC v3, the bound FastWork worktree, typed card editors, focused proof, and pre-PR subagent review.
