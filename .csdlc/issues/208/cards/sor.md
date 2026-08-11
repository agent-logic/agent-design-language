# Structured Output Record

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved the three actionable findings from the independent review of #208. Participant resume and final admission effects now remain cancellation- and absolute-deadline-bound while awaiting live locks; Guardian bootstrap reconstructs cleanup-permit custody solely from canonical ordered StageTarget, ActivateTarget, and DiscardTarget history and rejects missing, extra, conflicting, or corrupt custody; and fixture teardown explicitly proves zero residue. Fresh v4 evidence at 0840e1517b430a21069c29d7fc5e485e7a6b5959 proves 21/21 Runtime tests, repeated 35/35 kernel tests, repeated isolated and concurrent zero-LEAK lanes, both strict Clippy lanes, exact 56/64/12 parity, current-main ancestry, and clean exact-range diff. Distinct exact-head rereview remains pending; publication and merge remain untouched.

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

- Bounded every live operation-factory resume and the reasoning/final-ingress resume path by the remaining absolute deadline and cancellation selection, dropping pending lock/effect futures at the boundary so they cannot reopen later.
- Added during-await cancellation and expiry regressions that retain the admission lock across the boundary, then prove releasing it cannot cause a late reopen.
- Upgraded the Guardian client journal to retain each exact canonical command and reconstruct expected cleanup-permit custody from contiguous ordered accepted history, validating operation keys, command digests, completed response digests, stage creation, activation, and discard before accepting persisted custody.
- Added restart corruption proof for semantically missing permits, self-consistent extra permits, corrupt and aliased permits, and resurrection of custody already consumed by terminal activation history.
- Made both behavior harnesses explicitly close their temporary roots after all fixture-owned resources drop, surfacing any teardown residue immediately and proving repeated isolated and concurrent full lanes without LEAK.

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
    "purpose": "Prove the production Runtime continuity owner, private TLS-to-kernel effects, reconstructed cleanup custody, restart corruption rejection, explicit teardown, and exact denominator.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/runtime-nextest.stderr.log: 21 passed"
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
    "purpose": "Prove live participant continuity, in-flight bounded resume, restart reconciliation, exact retries, filesystem safety, explicit teardown, and exact denominator.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/kernel-nextest.stderr.log: 35 passed"
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
    "purpose": "Reject warnings across the production Runtime, Guardian, sealed role ports, journal reconstruction, and behavioral TLS tests.",
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
    "purpose": "Reject warnings across kernel effects, bounded resume, persistence, private server, and behavior tests.",
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
    "purpose": "Produce fresh canonical behavior evidence with exact 56/64/12 parity and no LEAK sentinel.",
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
