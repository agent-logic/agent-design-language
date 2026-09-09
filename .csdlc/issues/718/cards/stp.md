---
issue_card_schema: adl.issue.v1
wp: "RUNTIME-A2A-NAME"
slug: "a2a-canonical-name-routing"
title: "[v0.92.1][Runtime] Route agent-to-agent communication by canonical agent name"
labels:
  - "track:roadmap"
issue_number: 718
generated_at: "2026-09-09T18:20:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.4"
required_outcome_type:
  - "runtime_behavior_api_and_focused_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/718"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "Standalone v0.92.1 canonical-name routing bugfix; builds on merged governed A2A and #717 orientation."
pr_start:
  enabled: true
  slug: "a2a-canonical-name-routing"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: 2026-09-09T18:20:00Z

# Structured Task Prompt

## Summary

Replace public A2A model/admission-ID addressing with unique canonical-name routing through the live roster.

## Goal

Agents communicate using civic identities such as ember.axioma regardless of their provider, model, host, or lifecycle movement.

## Required Outcome

Canonical names are the public address; Runtime resolves once to internal identity before governed dispatch, and every public/history surface reports names consistently.

## Deliverables

- Canonical-name resolver at the A2A boundary
- Typed denial and legacy compatibility behavior
- Provider prompt/tool and API schema updates
- History and Observatory projection updates
- Migration/rehydration preservation tests
- Pairwise governed A2A proof

## Acceptance Criteria

- Every ordinary resident pair can communicate by canonical name.
- Ambiguous, unknown, malformed, missing, and self targets fail explicitly.
- Internal IDs remain authorization/audit keys and are not public addresses.
- Model replacement and lifecycle movement do not alter addresses.
- No Shepherd-only branch exists.

## Repo Inputs

- adl-runtime-kernel Runtime control, roster, assembly, history, and lifecycle code
- Existing A2A/ACIP contracts and OpenAPI schemas
- Observatory projections and Welcome Package

## Dependencies

- Governed A2A initiation and close-loop behavior are merged.
- #717 capability orientation is merged.
- No external service is required for deterministic implementation proof.

## Target Files / Surfaces

- Runtime canonical roster lookup and governed dispatch
- Public action/tool schemas and prompts
- History/API/Observatory projections
- Lifecycle and compatibility tests

## Validation Plan

Focused Runtime tests covering success, all denials, legacy input, history, migration/rehydration, prompts, schemas, and full resident-pair routing; format/diff checks and independent review.

## Demo Expectations

Exercise canonical names through the production conversation-to-A2A path, not a helper-only test.

## Non-goals

- Distributed transport
- Provider/model changes
- Shepherd-specific routing
- Identity-authority redesign

## Issue-Graph Notes

- Preserve current signed A2A and closed-loop behavior.
- Preserve immutable internal identity and legacy-record readability.
- Do not absorb distributed routing or provider replacement.

## Notes

Resolve canonical names against one authoritative snapshot and keep compatibility narrow enough that internal IDs cannot leak back into new prompts or public output.

## Tooling Notes

Use native C-SDLC v3, the bound FastWork worktree, focused Runtime proof, and pre-PR subagent review.
