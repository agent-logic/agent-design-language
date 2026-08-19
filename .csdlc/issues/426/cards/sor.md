# Structured Output Record

Template: 1.0.0

Issue: 426

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented Linux process service control for CSMctl while preserving Darwin launchd behavior and continuity-safe shutdown.

## Artifacts

- CSMctl
- docs/tooling/START_CSM_RUNBOOK.md
- adl/tools/test_csmctl_linux_backend.sh
- .csdlc/evidence/426

## Execution

- Added explicit Darwin launchd and Linux process service backends to CSMctl.
- Added PID ownership validation, stale and foreign PID refusal, and bounded continuity-safe TERM shutdown on Linux.
- Added focused lifecycle, platform-routing, ownership-denial, and shell-syntax proof.
- Documented supported Linux Runtime control and macOS-only Observatory boundaries.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff hygiene check.",
    "outcome": "passed",
    "evidence_ref": "426-diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_csmctl_linux_backend.sh"
    ],
    "purpose": "Run the focused CSMctl Linux backend validator.",
    "outcome": "passed",
    "evidence_ref": "426-linux-lifecycle.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
