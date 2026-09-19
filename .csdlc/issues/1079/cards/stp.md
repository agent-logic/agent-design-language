---
issue_card_schema: adl.issue.v1
wp: "1079"
slug: "deepseek-openrouter-reasoning"
title: "[v0.92.2][Runtime][OpenRouter] Honor reasoning effort for DeepSeek review turns"
labels:
  - "track:roadmap"
issue_number: 1079
generated_at: "2026-09-18T23:46:34Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "runtime_provider_behavior_and_live_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/1079"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "Follow-on to merged provider lifecycle and hot-reload work; issue-local fix discovered during resident qualification."
pr_start:
  enabled: true
  slug: "deepseek-openrouter-reasoning"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-18T23:46:34Z

# Structured Task Prompt

## Summary

Add OpenRouter reasoning controls, a separate bounded DeepSeek profile, typed Runtime provider failures, and installed full-review/A2A proof.

## Goal

Make DeepSeek V4 Flash responsive and diagnostically useful in Runtime without altering Nexus or Nemotron.

## Required Outcome

Normalized low reasoning reaches OpenRouter for DeepSeek, contradictory controls fail before dispatch, public conversation results retain safe provider failure categories, and installed proof completes the required review and A2A path.

## Deliverables

OpenRouter reasoning serialization and validation; request-capture and invalid-config coverage; Runtime conversation failure projection coverage; provider docs; separate operational DeepSeek definition; redacted live proof.

## Acceptance Criteria

All acceptance criteria from issue #1079, including full issue review, governed A2A continuation, Nexus/Nemotron preservation, and five typed provider failure classes.

## Repo Inputs

Issue #1079 and the canonical files listed in frontmatter; merged issues #854, #855, #876, and #901 provide constraints.

## Dependencies

Merged provider-neutral lifecycle and provider-definition hot loading. No unresolved code dependency.

## Target Files / Surfaces

The seven canonical tracked files plus local installed Runtime provider/admission state and repo-local retained proof.

## Validation Plan

Focused tests first; provider-core suite and Clippy; Runtime focused conversation regression; native card validation; exact-head independent review; installed full-review and A2A proof.

## Demo Expectations

One full current issue body and one governed A2A continuation through installed DeepSeek V4 Flash; retain redacted controls/status/timing only.

## Non-goals

No shared OpenRouter profile widening, recurring probe, other resident model change, cloud billing/configuration change, or unrelated baseline Runtime repair.

## Issue-Graph Notes

Issue #1079 closes only this provider-control and Runtime qualification defect.

## Notes

OpenRouter may return long hidden reasoning; low effort, 8192 output tokens, and 180-second timeout bound the DeepSeek-specific profile.

## Tooling Notes

Use native C-SDLC v3, the bound FastWork worktree, typed card edits, exact-head review, and installed local proof.
