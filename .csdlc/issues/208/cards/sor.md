# Structured Output Record

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved the two actionable findings from the independent review of #208 at 218a31cf. Completed operation retries now validate the exact canonical command value/digest and accepted prefix before consulting cached responses, so any conflicting same-kind retry fails without kernel contact. The succession retry explicitly releases and reacquires its only case-owned OS resource before evidence emission, and the retained producer now runs two complete Runtime-plus-kernel waves concurrently and rejects any LEAK sentinel. Fresh v4 evidence at abe0f10889667906d6031ab770d3e05651a2ed5f proves repeated concurrent 21/21 Runtime and 35/35 kernel lanes with zero LEAK, both strict Clippy lanes, exact 56/64/12 parity, current-main ancestry, and clean exact-range diff. Distinct exact-head rereview remains pending; publication and merge remain untouched.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/continuity_control.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/reasoning.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/kernel_continuity_control.rs
- adl-runtime-kernel/tests/support/runtime_init.rs
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/guardian.rs
- adl-runtime/src/kernel_continuity_client.rs
- adl-runtime/tests/kernel_continuity_client.rs
- infra/runtime-v3/runtime-init.toml
- .csdlc/prepared/issues/208/produce-proof-receipt.rb
- .csdlc/prepared/issues/208/validate-proof-receipt.rb
- .csdlc/evidence/208/v4/execution-proof.json

## Execution

- Moved journal reserve and exact retry comparison ahead of completed-response lookup so command JCS digest/value and accepted prefix always fail closed before a cached result can return.
- Added production regressions that stop the private listener, prove an exact completed checkpoint retry remains cache-served, then reject conflicting completed checkpoint, range-read, stage, chunk-write, and target-validation retries with ConflictingRetry and no possible kernel contact.
- Made certificate_succession_retry explicitly drop and successfully reacquire its exclusive journal descriptor before receipt emission, proving the case retains no task, subprocess, or OS lock into process teardown.
- Strengthened the proof producer and validator to retain two repeated concurrent full Runtime-plus-kernel nextest waves, require 21/21 and 35/35 in each wave, and reject every LEAK sentinel.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "kernel_continuity_client",
      "--no-tests=fail"
    ],
    "purpose": "Prove production completed-cache retry validation, private TLS-to-kernel behavior, restart custody, and the exact Runtime denominator in both retained concurrent waves.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/runtime-nextest.stderr.log and runtime-nextest-repeat.stderr.log: 21 passed each, zero LEAK"
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
      "kernel_continuity_control",
      "--no-tests=fail"
    ],
    "purpose": "Prove live participant continuity, restart reconciliation, exact retries, filesystem safety, and the exact kernel denominator in both retained concurrent waves.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/kernel-nextest.stderr.log and kernel-nextest-repeat.stderr.log: 35 passed each, zero LEAK"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-guardian",
      "--test",
      "kernel_continuity_client",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across the production Runtime, Guardian, exact completed-cache validation, and behavioral tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/runtime-clippy.stderr.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-kernel",
      "--test",
      "kernel_continuity_control",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across kernel effects, persistence, private server, and behavior tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/kernel-clippy.stderr.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/208/verify-diff-hygiene.rb"
    ],
    "purpose": "Verify exact execution-base-to-source diff hygiene and current-main ancestry.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/diff-hygiene.stdout.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/208/produce-proof-receipt.rb"
    ],
    "purpose": "Produce fresh canonical behavior evidence with exact 56/64/12 parity and two retained concurrent zero-LEAK full waves.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/execution-proof.json"
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

- none
