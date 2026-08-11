# Structured Output Record

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved the production authorized-learner findings on exact current-main ancestry: every factory requires one reconstructed shared admission/exclusion authority, sessions bind exact TLS direction and authority generations, successor flips revoke retained routes and clones, and real OpenRaft learner catch-up plus later replication runs through the factory and Quinn server. Exact fresh review and publication remain pending.

## Artifacts

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/learner_transport.rs
- adl-runtime/src/distributed/learner_transport/tests.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/distributed/transport.rs
- adl-runtime/tests/distributed_runtime_transport.rs
- .csdlc/prepared/issues/202/produce-proof-receipt.rb
- .csdlc/prepared/issues/202/validate-proof-receipt.rb
- .csdlc/evidence/202/v2/execution-proof.json

## Execution

- Removed production allow-all endorsement and ordinary-session APIs; ProductionLearnerAuthority now reconstructs admission and exclusion before exposure and owns exclusion-aware endorsement.
- Made ProductionLearnerAuthority mandatory for SecurePolisNetworkFactory construction and consulted it for ordinary pending/install/revalidation, learner install/server sessions, exclusion activation, and successor-flip fencing.
- Bound learner effects to trust domain, polis, exact voter and learner Raft/node/Guardian identities, TLS direction, authorized socket address, certificate and boot generations, full authority voter cut, operation and predecessor digests, deadline, sequence, kind, role, and payload.
- Preserved token previous_operation_sha256, required exact predecessor staging, and made the atomic successor flip remove routes so retained EstablishedLearnerSession clones reject without manual close or cross-generation reuse.
- Replaced the disconnected four-node proof with SecurePolisNetworkFactory install/request over live Quinn into serve_authorized_learner_connection and a real fourth PolisRaft added by add_learner(4,true), proving forced snapshot catch-up, later append replication, unchanged voters, and denied vote/generic/unknown messages.
- Strengthened live stale-boot, wrong-address, reversed-direction, production exclusion endorsement, and failed/committed admission plus exclusion restart regressions while retaining the exact 36 private and 13 public case denominator.
- Versioned repaired proof as immutable v2 evidence, bound exact 2afa820c current-main ancestry, fifteen behavior assertions, mandatory production authority ownership, runtime integration compile, and strict library/public Clippy.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "learner_transport::tests",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Run the exact behavior-bound private authorized learner lane.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v2/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authorized_learner_transport",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run the exact public canonical Membership artifact boundary.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v2/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_transport",
      "--no-run"
    ],
    "purpose": "Compile the full runtime transport integration surface against mandatory shared authority construction.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v2/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy across the production library.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v2/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authorized_learner_transport",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy across the exact public learner target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v2/execution-proof.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/202/produce-proof-receipt.rb"
    ],
    "purpose": "Produce immutable exact-source issue proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v2/execution-proof.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/202/validate-proof-receipt.rb"
    ],
    "purpose": "Validate current-main ancestry, immutable evidence, protected source, exact denominators, and behavior assertions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/prepared/issues/202/validate-proof-receipt.rb"
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
