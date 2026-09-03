# Structured Output Record

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented provider-backed resident Shepherds with provider-neutral private gateway support, tested lifetime recovery, presentation-safe continuity identity, shared readiness truth, and exact-binary Wuji restart proof.

## Artifacts

- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/resident_shepherd.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/shepherd.rs
- .csdlc/prepared/issues/640/validate-model-backed-shepherd.sh
- .csdlc/evidence/640/model-backed-shepherd.log
- .csdlc/evidence/640/wuji-shepherd-acceptance.log

## Execution

- Support compiled Ollama and private OpenAI-compatible provider adapters without serializing provider credentials into Runtime configuration or API output.
- Run provider preload plus governed inference through one reusable lifetime recovery controller that retries transient failure without terminating or globally blocking the Runtime.
- Exclude resident Shepherd presentation labels from continuity identity for both single and multi-Shepherd configurations while retaining canonical stateful bindings.
- Bind Wuji acceptance to the exact checked-out release build and installed canonical kernel and Guardian executables, then prove a controlled Guardian restart restores a new Runtime process without letting the validator terminate the live service.
- Retain one canonical deployed Runtime kernel binary and report Beacon Axioma backed by qwen3:8b with consistent readiness and Observatory truth.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh"
    ],
    "purpose": "Prove eight nonzero focused configuration, provider dispatch, governed inference, lifetime recovery, canonical identity, truthful roster counts, readiness consistency, formatting, and diff-hygiene behaviors.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/model-backed-shepherd.log"
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
      "--tests",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across changed Runtime production and test targets.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/model-backed-shepherd.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh",
      "--live-wuji"
    ],
    "purpose": "Prove clean exact implementation commit 58fbeaa41795b9bfcf2a903fb4a3f3225c68f8a6, installed kernel digest 4259f2d0441fecac0890bfab3b0f246bac16695101aa8b3bd1203ebbf6ed5ae4, and installed Guardian digest 94074672f547f6adecbe46d8f580f0e63e588b2989358050854f1075055d5074 restart to new PIDs, preload qwen3:8b, pass governed inference, and report certificate-verified local and AWS readiness.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/wuji-shepherd-acceptance.log"
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
