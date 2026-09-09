---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.02"
slug: "admission-a2a-outbox"
title: "[v0.92.1][TAIL-06.02][runtime] Persist and recover admission-triggered A2A initiation"
labels:
  - "track:roadmap"
issue_number: 758
generated_at: "2026-09-09T00:00:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.4"
required_outcome_type:
  - "runtime_behavior_and_recovery_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/758"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "P1 remediation R520-002 under #522, sourced from review #520."
pr_start:
  enabled: true
  slug: "admission-a2a-outbox"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: 2026-09-09T00:00:00Z

# Structured Task Prompt

## Summary

Replace the unowned post-admission spawn with a durable, bounded, recoverable A2A greeting obligation.

## Goal

Make autonomous admission greetings survive failure and restart without duplication.

## Required Outcome

Successful admission atomically creates or confirms durable greeting intent, a bounded worker owns delivery and recovery, and the Runtime exposes its terminal disposition.

## Deliverables

- Durable initiation intent/outbox record
- Idempotent bounded worker and recovery scan
- Stable ledger idempotency binding
- Health/evidence projection
- Deterministic tests and real-agent demo proof

## Acceptance Criteria

- Intent exists before successful admission completes.
- Startup and repeated admission recover pending intent.
- Refusal/interruption are retryable under bounded policy.
- Duplicate greetings are suppressed after uncertain completion.
- Pending, retrying, completed, and terminal-failure states are observable.
- Autonomous demo succeeds without an operator prompt.

## Repo Inputs

- adl-runtime-kernel/src/control.rs admission and A2A paths
- Existing runtime durable-state and conversation-ledger abstractions
- Internal review #520 specialist evidence

## Dependencies

- Parent #522 and review #520 provide the remediation authority and evidence.
- Integrate #759 health-task isolation before final validation.
- Base the durable admission aggregate and recovery transaction on #757 crash-consistent removal so greeting persistence cannot regress removal fencing.
- Preserve existing A2A transport and ledger authority contracts.

## Target Files / Surfaces

- adl-runtime-kernel/src/control.rs
- Narrowly coupled persistence, health, tests, demo, and PVF manifest surfaces only

## Validation Plan

Focused Runtime unit/integration recovery tests; Runtime owner lane; exact-head independent review; real-agent autonomous greeting demo.

## Demo Expectations

A real admitted agent is greeted autonomously after a forced first-attempt failure and Runtime restart, with exactly one ledger-visible greeting outcome.

## Non-goals

- Exactly-once delivery across providers
- General workflow orchestration
- Unrelated dynamic-agent lifecycle changes

## Issue-Graph Notes

- Parent remediation ledger: #522
- Source review: #520
- Deduplicates A520-ARCH-002 and C520-CODE-002 into one owned runtime repair.

## Notes

The repair must order persistence before acknowledgement, bind retries to a stable key, and avoid retry storms or duplicate ledger turns.

## Tooling Notes

Use typed C-SDLC v3, a bound FastWork worktree, focused Runtime validation, pre-PR subagent review, native review/publication, and truthful finish/clean.
