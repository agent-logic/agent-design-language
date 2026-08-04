# Structured Task Prompt

Template: 1.0.0

Issue: 5348

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare lifecycle packet only; release ceremony is future execution work.

## Deliverables

- generated six-card preparation packet
- concise design and diagram
- focused typed doctor validation

## Acceptance

1. AC-1: All six #5348 cards, design, and diagram are issue-specific, typed C-SDLC v2 generated, digest-consistent, and doctor-clean.
2. AC-2: #5348 execution remains blocked until WP-22 #5359 is observed live-merged and the observed merge SHA is an ancestor of the exact #5348 execution base.
3. AC-3: Preparation validation is focused on csdlc-doctor, request-driven csdlc-validate, and git diff --check, with no ceremony execution.
4. AC-4: Preparation does not publish, open a PR, tag, merge, touch main, use /private/tmp, touch #5357 remediation, or mutate any version:v0.92 issue.
5. AC-5: Future ceremony execution reconciles release evidence, tag/release notes, GitHub issue and PR state, cards, milestone docs, and v0.92 handoff truth without hidden repair work.

## Dependencies

- WP-22 #5359 live merged into origin/main
- #5359 observed merge SHA ancestral to exact #5348 execution base

## Inputs

- docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- issue #5348

## Non Goals

- implementation or remediation during ceremony
- release tagging during preparation
- PR publication
- receipt-gated execution
