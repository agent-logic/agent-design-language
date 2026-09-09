---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.09"
slug: "greeting-recovery-local-model-boundary"
title: "[v0.92.1][TAIL-06.09][runtime] Repair greeting recovery identity and local-model boundary"
labels:
  - "track:roadmap"
issue_number: 814
generated_at: "2026-09-09"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "runtime bug repair"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/814"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Child of #522; resolves #520 findings D520-RUNTIME-001, D520-RUNTIME-002, and D520-SEC-004."
pr_start:
  enabled: true
  slug: "greeting-recovery-local-model-boundary"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-09

# Structured Task Prompt

## Summary

Close the three Runtime/security findings without widening the provider or deployment architecture.

## Goal

Make admission greeting retries crash-safe and identity-stable, and make local-model proof structurally localhost-only.

## Required Outcome

Bounded recovery cannot over-dispatch or poison durable state; one logical key spans retries; unsafe URL forms fail closed.

## Deliverables

Narrow Runtime control repair, narrow local-model boundary repair, and deterministic regression coverage.

## Acceptance Criteria

AC-1 final-attempt crash recovery is bounded and loadable; AC-2 one logical work key persists across retry surfaces; AC-3 unsafe local-model URL forms and redirects are rejected; AC-4 focused validation and exact-head review pass.

## Repo Inputs

Issue #814, parent #522, review #520, adl-runtime-kernel/src/control.rs, adl-runtime/tests/shepherd_local_model.rs.

## Dependencies

#718 and #758 are merged; #520 review candidate is frozen at fb6cbc7f619daa54f901fd2d12f480add682ace3.

## Target Files / Surfaces

adl-runtime-kernel/src/control.rs; adl-runtime/tests/shepherd_local_model.rs; tightly coupled Runtime tests only.

## Validation Plan

Focused Rust tests for both defects, Runtime owner lane, cargo fmt, git diff --check, then independent exact-head review.

## Demo Expectations

No demo; deterministic regression coverage is mandatory.

## Non-goals

No provider selection changes, no deployment architecture work, no broad Runtime refactor.

## Issue-Graph Notes

Parent remediation #522; source review #520; this issue owns only three named findings.

## Notes

Retry-state semantics and URL authority parsing are security-sensitive.

## Tooling Notes

Use native C-SDLC v3, bound FastWork worktree, and the Runtime owner validation lane.
