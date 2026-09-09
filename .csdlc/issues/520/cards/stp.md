---
issue_card_schema: adl.issue.v1
wp: "TAIL-04"
slug: "tail-04-internal-review"
title: "[v0.92.1][TAIL-04] Internal review"
labels:
  - "track:roadmap"
issue_number: 520
generated_at: "2026-09-09T19:19:55Z"
card_status: "review_in_progress"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "complete_findings_first_internal_review_packet"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/520"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "TAIL-04 rerun is gated by merged #718/PR #809 and #758/PR #805; remediation remains owned by #522."
pr_start:
  enabled: true
  slug: "tail-04-internal-review"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-09T19:19:55Z

# Structured Task Prompt

## Summary

Prepare and execute one complete internal review rerun on the exact post-#718/post-#758 v0.92.1 candidate.

## Goal

Produce a trustworthy, complete, exact-revision findings register for the final v0.92.1 candidate.

## Required Outcome

Every declared review denominator is complete, every row is dispositioned, every mandatory specialist lane runs against the same candidate, and synthesis retains all supported findings and limitations.

## Deliverables

- Exact candidate and gate manifest
- Complete path, issue/PR, acceptance, documentation, demo, and evidence denominators
- Findings-first specialist reports
- Canonical finding register and synthesis
- Validated exact-head review packet

## Acceptance Criteria

- Inventory and disposition every changed production file, proof file, canonical document, milestone issue/PR, and acceptance surface.
- Bind every finding to exact evidence, severity, candidate revision, impact, source lane, and owner.
- Identify partial, inert, unreachable, unproven, and documentation-only outcomes.
- Credit no sampled, empty, zero-test, or CI-only lane as passing proof.
- Validate packet manifests, counts, redaction, portability, and exact-head identity.

## Repo Inputs

- agent-logic/agent-design-language#520
- issues #718 and #758 plus PRs #809 and #805
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
- .csdlc/prepared/issues/520/internal-review-plan.md
- docs/tooling/OPUS_REVIEW_RUNBOOK.md

## Dependencies

- #718/PR #809 merged and is present in the frozen candidate.
- #758/PR #805 merged and is present in the frozen candidate.
- Exact fetched origin/main fb6cbc7f619daa54f901fd2d12f480add682ace3 is the sole reviewed candidate.

## Target Files / Surfaces

- docs/milestones/v0.92.1/evidence/release/tail-04/**
- .csdlc/issues/520/**
- .csdlc/prepared/issues/520/**

## Validation Plan

Freeze exact fetched origin/main after both gates, rebuild complete denominators, rerun all mandatory specialist lanes, synthesize without dropping findings, run the production validator and negative fixtures, then obtain independent exact-head packet review.

## Demo Expectations

No live provider or cloud demo. Review evidence must be deterministic, candidate-bound, and complete.

## Non-goals

- Do not fix product findings inside #520; route them through #522.
- Do not perform the external review owned by #521.
- Do not approve the release, merge product work, deploy, restart Runtime, or spend cloud/provider funds.

## Issue-Graph Notes

- #718 and #758 gates are satisfied in the frozen candidate.
- #522 owns the remediation wave through #814-#821.
- #521 owns external review.

## Notes

Execution is blocked until #758/PR #805 is merged. The candidate must be fetched origin/main, equal the reviewed revision, and contain merge commits for both #718 and #758.

## Tooling Notes

Use native C-SDLC v3, the bound FastWork worktree, complete findings-first sprint-review lanes, candidate-bound evidence, and independent exact-head review before publication.
