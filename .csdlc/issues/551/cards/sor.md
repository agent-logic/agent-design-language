# Structured Output Record

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a validated Runtime-owned Polis identity and endpoint projection whose complete presentation snapshot hot-loads atomically without Runtime restart, with feed-owned HTML rendering and last-known-good rejection behavior.

## Artifacts

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/observatory.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- adl-runtime-kernel/tests/guardian_soak.rs
- adl-runtime-kernel/tests/support/runtime_init.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- infra/runtime-v3/runtime-init.toml
- demos/html-observatory/index.html
- demos/html-observatory/app.js
- demos/html-observatory/tests/polis_identity.test.mjs
- .csdlc/evidence/551/html-polis-node.tap

## Execution

- Added the required validated Polis identity section and production configuration values.
- Advanced the Runtime Observatory feed to v3 with an explicit redacted Polis identity object while preserving v1 and v2 compatibility constants.
- Unified Polis identity, Runtime API base, and Observatory origin policy under one RuntimePresentationState lock so every declared parameter hot-loads as one snapshot without restart.
- Rejected invalid reloads before publication and retained the complete last-known-good snapshot with bounded redacted diagnostics.
- Replaced the HTML deployment-name constant with feed-owned rendering and exact nonzero Node tests.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/551/validate-runtime-polis.sh"
    ],
    "purpose": "Prove exact nonzero configuration, full hot-reload control, v3 feed, OpenAPI, and bounded redacted diagnostic targets.",
    "outcome": "passed",
    "evidence_ref": "Local focused run: configuration 3/3, control 1/1, observatory 1/1, openapi_contract 1/1, binary diagnostic 1/1."
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration",
      "--test",
      "control",
      "--test",
      "observatory",
      "--test",
      "openapi_contract",
      "--no-tests=fail"
    ],
    "purpose": "Prove the complete affected integration-test denominator remains green.",
    "outcome": "passed",
    "evidence_ref": "Local nextest run: 76/76 passed."
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "guardian_soak",
      "--no-tests=fail",
      "-E",
      "test(signed_https_wss_shutdown_checkpoints_and_forgery_cannot_stop_the_process)"
    ],
    "purpose": "Prove the production schema-v3 guardian fixture remains green.",
    "outcome": "passed",
    "evidence_ref": "Local nextest run: 1/1 passed."
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/551/validate-html-polis.sh"
    ],
    "purpose": "Prove exact feed-owned HTML projection and reject a zero-test result.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/551/html-polis-node.tap; 3/3 passed."
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Prove Rust formatting.",
    "outcome": "passed",
    "evidence_ref": "Local command exited zero."
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "Local command exited zero."
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
