# Structured Output Record

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented provider-neutral model-backed resident Shepherds with canonical per-resident routing, shared inference-readiness gating, automatic recovery, canonical request validation, and exact Wuji restart acceptance.

## Artifacts

- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/resident_shepherd.rs
- adl-runtime-kernel/src/shepherd.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/shepherd.rs
- .csdlc/prepared/issues/640/validate-model-backed-shepherd.sh
- .csdlc/evidence/640/model-backed-shepherd.log
- .csdlc/evidence/640/wuji-shepherd-acceptance.log

## Execution

- Route governed requests by canonical resident identity while retaining the configured primary as the compatibility default.
- Share model readiness between preload recovery and provider inference so loading and degraded residents fail closed without affecting Runtime availability.
- Reuse canonical Shepherd envelope, identifier, prompt-size, and NUL validation.
- Advertise ready only after an internal OperationalAdapter inference probe succeeds.
- Prove multi-resident routing, exact restart, automatic qwen3:8b residency, governed inference, and readiness/feed consistency.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh"
    ],
    "purpose": "Prove five nonzero focused configuration, two-resident routing, canonical validation, model-health recovery, roster consistency, formatting, and diff-hygiene behaviors.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/model-backed-shepherd.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh",
      "--live-wuji"
    ],
    "purpose": "Prove the exact Wuji candidate restarts to a new PID, automatically preloads qwen3:8b, passes governed inference, and reports consistent readiness/feed truth.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/wuji-shepherd-acceptance.log"
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
    "purpose": "Reject warning regressions on the changed Runtime production targets.",
    "outcome": "passed",
    "evidence_ref": "Local strict Clippy exited zero before exact-head evidence capture."
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed whitespace in current issue changes.",
    "outcome": "passed",
    "evidence_ref": "Local diff hygiene exited zero after retained evidence generation."
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
