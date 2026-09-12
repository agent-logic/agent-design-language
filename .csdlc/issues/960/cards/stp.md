---
issue_card_schema: adl.issue.v1
wp: "960"
slug: "960-shutdown-barrier"
title: "Fix Runtime shutdown barrier acknowledgment race (v0.92.2)"
labels:
  - "track:roadmap"
issue_number: 960
generated_at: "2026-09-12T05:59:59.590095+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "runtime regression repair"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/960"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Part of #928; separate repair to unblock #957."
pr_start:
  enabled: true
  slug: "960-shutdown-barrier"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12T05:59:59.590095+00:00

# Structured Task Prompt

## Summary

Fix event-specific Runtime shutdown sink acknowledgment under concurrent heartbeats; preserve fail-closed sink errors and retain daemon exit/stderr in smoke failures. Separate regression #960, part of #928, diagnosed while PR #957 was red; original CI cause remains unproven.

## Goal

Fix event-specific Runtime shutdown sink acknowledgment under concurrent heartbeats; preserve fail-closed sink errors and retain daemon exit/stderr in smoke failures. Separate regression #960, part of #928, diagnosed while PR #957 was red; original CI cause remains unproven.

## Required Outcome

Fix event-specific Runtime shutdown sink acknowledgment under concurrent heartbeats; preserve fail-closed sink errors and retain daemon exit/stderr in smoke failures. Separate regression #960, part of #928, diagnosed while PR #957 was red; original CI cause remains unproven.

## Deliverables

Bounded Runtime fix, focused proof and child failure diagnostics; preserve old CI evidence.

## Acceptance Criteria

Exact barrier acknowledgment survives subsequent heartbeat updates; required sink write failures still reject shutdown; deterministic concurrent-writer and failure injection tests plus actual local CLI notice/disposition proof; stderr/exit retained. No sleeps, longer timeouts, weakened assertions or CodeFriend changes.

## Repo Inputs

Issue #960; .adl/960-diagnosis/shutdown-diagnostic/ISSUE.md and reproduction.json; original CI log preserved separately.

## Dependencies

No implementation dependency; separate runtime regression from #880.

## Target Files / Surfaces

adl/src/long_lived_agent.rs; adl/src/cli/observability.rs (also included by adl/src/observability.rs); adl/tests/cli_smoke/agent.rs; focused proof under .csdlc/evidence/960.

## Validation Plan

Exact barrier acknowledgment survives subsequent heartbeat updates; required sink write failures still reject shutdown; deterministic concurrent-writer and failure injection tests plus actual local CLI notice/disposition proof; stderr/exit retained. No sleeps, longer timeouts, weakened assertions or CodeFriend changes.

## Demo Expectations

Actual local CLI shutdown with notice and disposition; no paid calls.

## Non-goals

No CodeFriend changes, paid calls, longer waits or weaker assertions; no claim that original CI cause is proven.

## Issue-Graph Notes

Part of #928; independent from CodeFriend #880.

## Notes

Supported heartbeat race proven; original CI cause unknown. Local sink acknowledgment is not remote OTLP delivery.

## Tooling Notes

Native v3 only; bound worktree; independent review before publication.
