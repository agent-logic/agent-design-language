# Structured Intent Prompt

Template: 1.0.0

Issue: 5802

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Populate and verify the complete current Drive mirror for docs and .adl/docs/TBD, including the CodeFriend corpus.

## Required Outcome

The configured Drive mirror reports recursive_live and every generator-selected Markdown file verifies at its repository-relative path with exact bytes.

## Scope

- adl/src/adl_gws_context_mirror.rs
- adl/src/adl_gws_drive_sync.rs
- adl/src/adl_gws_native.rs
- adl/src/bin/demo_adl_gws_context_mirror.rs
- docs/tooling/ADL_GOOGLE_DRIVE_CONTEXT_MIRROR_RUNBOOK.md
- docs/reviews/v0.92/google-drive-context-mirror-5802
- Configured ADL Google Drive root and seed folder
- Scheduled sync-adl-google-drive-context-mirror automation

## Authority

- The repository is canonical and Drive is a read-oriented mirror
- Issue #5802 owns the full recursive deployment acceptance missing from #5587
- Drive writes are create-or-update only; deletion and unrelated moves are forbidden
- Credential material remains outside the repository and outside retained evidence

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- Issue-bound worktree only
- No AWS or Spot
- One bounded subagent review before PR publication
- Keep the automation paused until complete acceptance
