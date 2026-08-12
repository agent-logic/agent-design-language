# Structured Output Record

Template: 1.0.0

Issue: 5911

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Enforced the canonical FastWork worktree parent in typed binding, archived 6,454 inactive Codex transcript files totaling 131,750,498,984 bytes to FastWork, verified equal source/archive SHA-256 manifests and an independent archive-side check, and retained all source files pending separate deletion approval.

## Artifacts

- .adl/worktree-policy.json
- adl/tools/archive_codex_sessions_to_fastwork.sh
- adl/tools/test_archive_codex_sessions_to_fastwork.sh
- /Volumes/FastWork/adl-archives/codex-sessions/issue-5911-20260811/summary.json
- /Volumes/FastWork/adl-archives/codex-sessions/issue-5911-20260811/manifest.sha256
- /Volumes/FastWork/adl-archives/codex-sessions/issue-5911-20260811/deletion-proposal.json

## Execution

- Added tracked .adl/worktree-policy.json with /Volumes/FastWork/adl-worktrees as the mandatory parent.
- Made canonical ADL binding fail closed when policy is missing or a new worktree resolves outside FastWork while preserving idempotent legacy issue-local binds.
- Added a non-destructive archive/verification tool and focused tests including symlink-escape refusal.
- Archived the stable older-than-one-day transcript set; no source transcript or existing worktree was deleted.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "fastwork_policy"
    ],
    "purpose": "Prove allowed FastWork placement, outside-parent refusal, and missing-policy refusal.",
    "outcome": "passed",
    "evidence_ref": "local issue worktree output: 3 passed, 0 failed"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_archive_codex_sessions_to_fastwork.sh"
    ],
    "purpose": "Prove manifest generation, source preservation, path refusal, and symlink-escape refusal.",
    "outcome": "passed",
    "evidence_ref": "local issue worktree output: PASS archive_codex_sessions_to_fastwork"
  },
  {
    "command": [
      "adl/tools/archive_codex_sessions_to_fastwork.sh",
      "--verify-only",
      "/Volumes/FastWork/adl-archives/codex-sessions/issue-5911-20260811/manifest.sha256"
    ],
    "purpose": "Independently verify all 6,454 archived transcript digests on FastWork.",
    "outcome": "passed",
    "evidence_ref": "/Volumes/FastWork/adl-archives/codex-sessions/issue-5911-20260811/summary.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict lint proof for the changed v2 lifecycle library.",
    "outcome": "passed",
    "evidence_ref": "local issue worktree output"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_cargo_validation.sh",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Prove the complete C-SDLC v2 standalone surface, including case-insensitive FastWork enforcement and every linked-worktree fixture repaired after the first hosted failure.",
    "outcome": "passed",
    "evidence_ref": "GitHub Actions run 31561244618, job csdlc-v2-standalone, exact head 5a2444dbf03f5b315285722689fb8eeb8fb6469e"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
