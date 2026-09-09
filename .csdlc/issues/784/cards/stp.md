---
issue_card_schema: adl.issue.v1
wp: "RUNTIME-A2A"
slug: "runtime-a2a-closed-loop"
title: "[v0.92.1][Runtime] Return completed A2A replies to the initiating agent"
labels:
  - "track:roadmap"
issue_number: 784
generated_at: "2026-09-09T16:40:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.4"
required_outcome_type:
  - "runtime_behavior_and_focused_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/784"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "Standalone v0.92.1 close-loop defect discovered by live cooperative testing."
pr_start:
  enabled: true
  slug: "runtime-a2a-closed-loop"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: 2026-09-09T16:40:00Z

# Structured Task Prompt

## Summary

Feed governed peer terminal results back into the initiating agent's subsequent provider context.

## Goal

Close the A2A reasoning loop without operator relay.

## Required Outcome

The initiator can consume and synthesize a peer's completed reply or typed failure on its next turn.

## Deliverables

- Causal prior-A2A result projection
- Provider prompt integration
- Deterministic delegate/reply/synthesize test
- Focused regression proof

## Acceptance Criteria

- Next initiating turn contains the peer terminal result once.
- Causal IDs and status remain explicit.
- Restart/checkpoint history remains compatible.
- All agents use the same path.

## Repo Inputs

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/assembly.rs
- Existing A2A conversation tests

## Dependencies

- Existing A2A dispatch and terminal-history implementation is authoritative.
- No external issue blocks this repair.

## Target Files / Surfaces

- Runtime control and provider prompt assembly
- Focused A2A tests only

## Validation Plan

Focused Runtime unit test plus adjacent A2A/conversation tests, formatting, diff hygiene, and bounded review.

## Demo Expectations

A resident initiator must summarize a peer result without operator relay.

## Non-goals

- Naming and routing changes
- Distributed transport
- Shepherd-only behavior

## Issue-Graph Notes

- Preserve #713 transcript/history behavior.
- Keep #718 canonical names out of scope.

## Notes

Avoid duplicate prompt entries and leaking provider-private payloads.

## Tooling Notes

Use native C-SDLC v3, this bound FastWork worktree, focused proof, and pre-PR subagent review.
