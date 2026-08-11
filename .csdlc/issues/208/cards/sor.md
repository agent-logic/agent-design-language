# Structured Output Record

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved the remaining deterministic zero-LEAK finding from the independent review of #208 at ce606701. Systematic audit showed the moving nextest label occurred on resource-free synchronous cases and therefore did not originate in the production client, journals, TLS listeners, Tokio tasks, locks, or temporary roots. The shared Runtime harness now fails every case if any live or unreaped child process remains after explicit fixture closure, while a target-local 500 ms leak-as-failure window accounts for loaded-runner EOF observation latency without suppressing genuine inherited-handle leaks. Fresh v4 evidence at b0252da2a0e3f160ddddfa1815fcfea72ac75e8f retains two isolated and two concurrent full waves for both packages: Runtime 21/21 four times and kernel 35/35 four times, all zero-LEAK, plus both strict Clippy lanes, exact 56/64/12 parity, current-main ancestry, and clean exact-range diff. Distinct exact-head rereview remains pending; publication and merge remain untouched.

## Artifacts

- adl/.config/nextest.toml
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

- Audited shared Runtime harness and production client/journal teardown across descriptors, locks, TLS connections, listener tasks, current-thread runtimes, temporary roots, Drop ordering, and subprocess creation; the moving label reached cases that create none of those resources.
- Added one shared bounded waitpid(WNOHANG) teardown assertion after explicit temporary-root closure so every Runtime case fails on any live or unreaped direct child instead of relying on nextest timing.
- Added target-local 500 ms leak-as-failure overrides only for the two issue #208 test binaries, retaining detection of any genuine inherited stdout/stderr handle while removing the repo's below-default 100 ms loaded-runner classification race.
- Expanded the exact producer and validator to retain two isolated and two concurrent complete nextest waves for each package, require every exact 21/21 and 35/35 denominator, and reject any LEAK or LEAK-FAIL sentinel.

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
    "purpose": "Prove all 21 Runtime cases, shared child-process teardown assertion, and target-local fail-closed leak classification in two isolated and two concurrent retained waves.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/runtime-nextest*.stderr.log: 21 passed in all four waves, zero LEAK"
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
    "purpose": "Prove all 35 kernel cases and target-local fail-closed leak classification in two isolated and two concurrent retained waves.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/kernel-nextest*.stderr.log: 35 passed in all four waves, zero LEAK"
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
    "purpose": "Reject warnings across the production Runtime, Guardian, completed-cache validation, and shared teardown assertion.",
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
    "purpose": "Produce exact canonical 56/64/12 behavior evidence and eight retained isolated/concurrent zero-LEAK full lanes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/execution-proof.json"
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
