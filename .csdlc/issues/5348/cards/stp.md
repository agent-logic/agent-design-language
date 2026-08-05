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

1. AC-1: WP-22 PR #5811 merge 703ee31f2c02bb6c8fda7d6bc51ff7963075132e is ancestral to the exact WP-23 worktree revision.
2. AC-2: Release plan, final release notes, checklist, readiness, proof coverage, demo matrix, canonical inventory, and ceremony packet agree on completed release-tail truth and explicit v0.92 non-claims.
3. AC-3: #5809 supplemental evidence pins the WP-21 publication base, reviewed head, squash merge, and tree equivalence without rewriting execution-time evidence.
4. AC-4: Focused Markdown/path checks, JSON and YAML parsing, git diff --check, and release-script check-only preflight pass; no Rust build, Clippy, coverage, or broad tests run for this docs-only issue.
5. AC-5: One bounded exact-head review passes before publication, and the PR body closes #5348 and #5809.
6. AC-6: After merge, adl/tools/release_ceremony.sh creates and pushes tag v0.91.8, publishes the GitHub release from final notes, and live verification proves the tag target and release state.
7. AC-7: Sprint umbrella #5595 closes only after #5348, #5809, the tag, and the published release are verified.

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
