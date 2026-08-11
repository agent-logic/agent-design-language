# Structured Output Record

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved all five actionable findings from the latest independent #208 review. Guardian restart now securely restores exact TargetCleanupPermit custody and rejects missing, corrupt, conflicting, or relabeled journal state; retained source state rejects root-generation relabel; source resume and every participant/admission reopen effect remain behind one absolute-deadline and cancellation guard; a real ProductionPolisRuntime crosses the private TLS listener into actual source_checkpoint_210 and activate_target_204 kernel effects across restart; and deterministic test teardown removes the prior no-LEAK race. Fresh v4 evidence at 8ad54ad7c40dd3553c0fc2e00463b2c2b18d1d30 proves 21/21 Runtime tests, repeated 35/35 kernel tests without LEAK, both strict Clippy lanes, exact 56/64/12 parity, and clean exact-range diff. Distinct exact-head rereview remains pending; publication and merge remain untouched.

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

- Made ProductionPolisRuntime the production owner and caller of sealed #210 source/transfer and #204 activation/resume/discard continuity effects, with a distinct boot-trusted migration-decision key.
- Connected ingress, accepted-prefix, governance, all operational factories, and reasoning mutation to durable live quiesce/checkpoint/reverse-resume effects, retaining exact retry and fail-closed conflict semantics.
- Persisted TargetCleanupPermit custody in the Guardian client journal, atomically reconciled it with stage, root, catalog, channel, activation, and discard state on restart, and rejected missing, corrupt, rollback, alias, and conflict forms.
- Persisted and compared retained source root_generation so restart cannot relabel a durable source generation.
- Threaded one absolute deadline and cancellation guard through ResumeSource, every participant reopen effect, and final admission reopen so no effect commits after the boundary.
- Kept restart reconciliation exact for signed catalogs, opaque handles, cleanup and chunk receipts, trusted generations, decisions, and active-target state while using descriptor-relative no-follow durable I/O.
- Added a real production Runtime through private TLS into actual source_checkpoint_210 and activate_target_204 kernel effects, including restart restoration and corruption/conflict rejection.
- Moved kernel evidence scanning after all fixture-owned resources drop, eliminating the discard_exact_retry/general teardown LEAK race without sleeps or hidden routing policy.

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
    "purpose": "Prove the production Runtime continuity owner, real private TLS-to-kernel effects, durable cleanup custody, and exact denominator.",
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
    "purpose": "Prove live participant continuity, bounded resume, restart reconciliation, exact retries, filesystem safety, and deterministic no-LEAK teardown.",
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
    "purpose": "Reject warnings across the production Runtime, Guardian, sealed role ports, and behavioral TLS tests.",
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
    "purpose": "Verify exact execution-base-to-source diff hygiene and ancestry.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/diff-hygiene.stdout.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/208/produce-proof-receipt.rb"
    ],
    "purpose": "Produce fresh canonical behavior evidence with exact 56/64/12 parity and no LEAK.",
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
