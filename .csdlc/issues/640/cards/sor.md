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
- Decode bounded HTTP/1.1 content-length and chunked response bodies through one shared provider response path for Ollama and private OpenAI-compatible gateways.
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
    "purpose": "Prove eight nonzero focused configuration, provider dispatch including a valid chunked private gateway response, governed inference, lifetime recovery, canonical identity, truthful roster counts, readiness consistency, formatting, and diff-hygiene behaviors.",
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
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "vertex_ai",
      "--lib"
    ],
    "purpose": "Prove the post-merge conflict resolution preserves mainline Vertex AI admission and fail-closed invocation alongside #640 local provider routes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/model-backed-shepherd.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh",
      "--live-wuji"
    ],
    "purpose": "Prove clean exact reviewed commit 670ea79b87ea80f736cd9d6d3718d39661c13c2a, installed kernel digest 80ea61fc51a3651b93f6a4e8be332f72d4a8248da398a2d4c6754064ca82c6a6, and installed Guardian digest ae9ff7e5b622c29feef63fc99187f28c66225b862a92538f860e53ae54ee7e11 restart to new PIDs, preload qwen3:8b, pass governed inference, and report certificate-verified local and AWS readiness.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/640/wuji-shepherd-acceptance.log"
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

- Do not bind execution until #617/#636 is merged into the selected base.
