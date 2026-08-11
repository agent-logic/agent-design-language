# Structured Output Record

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved all eight actionable findings from the final independent review: the production Polis runtime now owns and invokes the sealed #210 and #204 ports; #204 decisions use a distinct boot-trusted key; the five live participants execute durable quiesce and reverse resume effects; retained generations accept exact retries only; restart revalidates signed catalogs, opaque handles, cleanup and chunk receipts, trusted generations, decisions, and active-target state; server effects remain cancellation/deadline guarded through commit; the durable server journal uses descriptor-relative no-follow lock and state I/O; and tests perform real TLS exporter, client-EKU, unknown-CA, activation-authority, and restart-tamper behavior. Fresh v4 evidence at af8571a0b62abd26caa8e6962bbe55f100f58762 proves 21/21 Runtime tests, 35/35 kernel tests, both strict Clippy lanes, exact 56/64/12 parity, no LEAK, clean exact-range diff, and current origin/main ancestry. Distinct exact-head rereview remains pending; publication and merge remain untouched.

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
- adl-runtime/tests/kernel_continuity_client.rs
- infra/runtime-v3/runtime-init.toml
- .csdlc/prepared/issues/208/produce-proof-receipt.rb
- .csdlc/prepared/issues/208/validate-proof-receipt.rb
- .csdlc/evidence/208/v4/execution-proof.json

## Execution

- Made ProductionPolisRuntime the production owner and caller of sealed #210 source/transfer and #204 activation/resume/discard continuity effects instead of retaining a Guardian status-only capability.
- Added a required migration-decision public key, key id, and generation that must be distinct from operation-permit and continuity-signing trust, and verify finalized #204 decisions only against that boot-trusted key set.
- Connected ingress, accepted-prefix, governance, all ten operational factories, and reasoning mutation to live quiesce/resume effects with durable participant-start, prepare, reverse-resume, and aggregate receipts.
- Bound retained source generations to predecessor, accepted prefix, topology, and config so only an exact retry is accepted and every conflict is rejected.
- Revalidated target catalog signatures and trusted generations, opaque handle and cleanup bindings, predecessor-linked chunk receipts and staged bytes, possession/discard/activation receipts, full #204 decisions, and active-target records on restart.
- Applied cancellation and remaining absolute-deadline guards at server admission, effect execution, filesystem commit boundaries, response completion, and client transport operations.
- Moved durable server journal lock and canonical control-state I/O to descriptor-relative O_NOFOLLOW operations rooted in an opened trusted directory handle.
- Replaced source-marker laundering with live TLS 1.3 exporter mismatch rejection, real invalid-client-EKU and unknown-CA handshake failures, independent permit-versus-#204 activation authority, real five-participant rollback, and restart-tamper tests.

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
    "purpose": "Prove the production Runtime continuity owner and live private TLS boundary.",
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
    "purpose": "Prove live five-participant continuity effects, exact retries, restart verification, activation authority, and filesystem safety.",
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
    "purpose": "Reject warnings across kernel production effects, persistence, private server, and behavior tests.",
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
    "purpose": "Produce exact canonical behavior evidence with 56/64/12 parity, no LEAK, strict lanes, and current-main ancestry.",
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
