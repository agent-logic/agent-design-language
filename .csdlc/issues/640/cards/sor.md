# Structured Output Record

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented provider-backed resident Shepherds with shared trusted-clock readiness, governed inference gating, automatic model preload recovery, and race-free Wuji restart acceptance.

## Artifacts

- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/resident_shepherd.rs
- adl-runtime-kernel/src/shepherd.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/shepherd.rs
- .csdlc/prepared/issues/640/validate-model-backed-shepherd.sh
- .csdlc/evidence/640/model-backed-shepherd.log
- .csdlc/evidence/640/wuji-shepherd-acceptance.log

## Execution

- Route governed Shepherd requests to the configured resident identity and provider model.
- Require a successful provider preload and governed inference probe before advertising the resident Shepherd as ready.
- Compare Shepherd admission freshness with the same trusted Runtime clock that creates and refreshes admission evidence.
- Wait until both public and private Runtime listeners are released before acceptance restarts the permanent launchd service.
- Preserve Runtime availability while provider failures leave only the affected Shepherd degraded.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh"
    ],
    "purpose": "Prove five nonzero focused configuration, multi-resident routing, canonical validation, model-health recovery, roster consistency, formatting, and diff-hygiene behaviors.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/model-backed-shepherd.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "shepherd_readiness_tests",
      "--lib"
    ],
    "purpose": "Prove readiness uses the same trusted clock as admission and still fails closed after heartbeat loss.",
    "outcome": "passed",
    "evidence_ref": "2 focused readiness tests passed at source 15cc7e9660ae84c9b85ed4aa528fc27fff154185"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bins",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings on changed Runtime production targets.",
    "outcome": "passed",
    "evidence_ref": "Strict Clippy exited zero after the trusted-clock repair."
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh",
      "--live-wuji"
    ],
    "purpose": "Prove exact candidate 843df6a967db3f91712be2280f4a602fd9b422b9 restarts cleanly on Wuji, preloads qwen3:8b, passes governed inference, and reports consistent readiness/feed truth.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/wuji-shepherd-acceptance.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed whitespace in current issue changes.",
    "outcome": "passed",
    "evidence_ref": "No output at source 80918b5ea117250e7b7322f1264237ab094b1788."
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

- Do not bind execution until #617/#636 is merged into the selected base.
