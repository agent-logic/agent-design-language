# Structured Output Record

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Runtime v3 now creates a configured model-backed resident Shepherd, preloads and keeps its local model resident, reports provider/model health through the roster, and isolates provider degradation from global Runtime readiness.

## Artifacts

- adl-runtime-kernel/src/resident_shepherd.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- infra/runtime-v3/runtime-init.toml
- .csdlc/prepared/issues/640/wuji-acceptance.json

## Execution

- Add validated one-or-many resident Shepherd provider, model, endpoint, and preload configuration.
- Route governed Shepherd reasoning through the configured Ollama model while retaining native admission authority.
- Preload and continuously recover resident Shepherd models without terminating or globally degrading the Runtime.
- Expose non-secret provider/model identity and consistent model_loading, ready, and degraded roster health.
- Deploy and restart the exact candidate on Wuji with qwen3:8b resident forever.

## Validation

[
  {
    "command": [
      "/bin/bash",
      "/Volumes/FastWork/adl-worktrees/adl-issue-640-runtime-model-backed-resident-shepherd/.csdlc/prepared/issues/640/validate-model-backed-shepherd.sh"
    ],
    "purpose": "Issue #640 focused deterministic implementation validation",
    "outcome": "passed",
    "evidence_ref": "model-backed-shepherd.log"
  },
  {
    "command": [
      "/bin/bash",
      "/Volumes/FastWork/adl-worktrees/adl-issue-640-runtime-model-backed-resident-shepherd/.csdlc/prepared/issues/640/validate-model-backed-shepherd.sh",
      "--live-wuji"
    ],
    "purpose": "Issue #640 bounded live Wuji acceptance",
    "outcome": "passed",
    "evidence_ref": "wuji-shepherd-acceptance.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- Do not bind execution until #617/#636 is merged into the selected base.
