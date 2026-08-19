# Structured Output Record

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Resolved all four generation-38 production review findings on exact current origin/main ancestry. A distinct learner-owned factory now holds its own durable transport instance and live boot custody; pending exclusion governs recovery admission and route retention; removal binds target membership and deadline with cache-first exact retry; and immutable v9 proof requires a fully clean worktree plus exact current origin/main. The STP v6 evidence entry remains immutable historical design-time input because typed lifecycle policy forbids STP mutation after implemented; current proof authority is the v9 VPP, SOR, SRP, and evidence packet. The hosted CI repair maps executable transport subtree paths to the correct coverage candidate and reinstalls a rotated route after the intentional authority-cut drain.

## Artifacts

- .csdlc/evidence/202/v9/execution-proof.json
- .csdlc/prepared/issues/202/produce-proof-receipt.rb
- .csdlc/prepared/issues/202/validate-proof-receipt.rb
- adl-runtime/src/distributed/transport/governed/learner_transport.rs
- adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
- adl-runtime/src/distributed/transport/governed/polis_runtime.rs
- adl-runtime/tests/distributed_runtime_transport.rs
- adl/tools/check_coverage_impact.sh
- adl/tools/test_check_coverage_impact.sh

## Execution

- Added SecureLearnerNetworkFactory as a learner-owned production ingress path with distinct durable ProductionLearnerAuthority, TransportAuthorityOwner, transport instance, peer pins, and nonextractable LearnerBootAttestationCustody.
- Proved the voter and learner factories complete a production handshake over Quinn and replicate snapshot plus append traffic into a real fourth PolisRaft while the learner-owned expiry writer waits through the actual Raft effect and response.
- Made pending exclusion govern admission currentness, session currentness, voter-side route installation, learner-side ingress, and recovery of the same stable Raft id under a new node and Guardian identity with a higher committed index.
- Bound removal activation and durable recovery state to the exact target-membership digest and live authorization deadline while serving exact retries from durable cache before re-decoding an expired result.
- Drained learner routes denied by exclusion and retained transition serialization across ordinary and learner dispatch locks.
- Rebased onto exact origin/main 1567469e395f9a6ea6c2e736366a8008f5ee1e06 and introduced immutable v9 evidence with no temporary-directory or machine-local cleanliness exceptions.
- Recorded v9 as current validation and review authority while preserving the typed lifecycle's immutable implemented-phase STP history.
- Mapped executable distributed transport subtree files to the runtime transport coverage candidate while retaining the dependency-free shim classification.
- Updated the rotation regression to install the peer as a fresh route after replace_authority_cut intentionally drains stale boot and certificate bindings.
- Hardened the receipt validator to require the exact command-key set and byte-exact argv for every retained validation lane.
- Expanded the focused coverage contract to prove nested governed transport mapping and zero risk filter for the dependency-free shim.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/202/produce-proof-receipt.rb"
    ],
    "purpose": "Produce exact clean-source v9 evidence for 36 semantic cases, 42 private runner tests, 13 public tests, 31 named behavior assertions, three integration compiles, and strict library and public Clippy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v9/execution-proof.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/202/validate-proof-receipt.rb"
    ],
    "purpose": "Validate immutable v9 protected-source parity, exact current origin/main ancestry, proof denominators, command streams, and absence of post-proof drift.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v9/execution-proof.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Prove executable transport subtree coverage mapping and policy routing.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v9/execution-proof.json"
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
      "route_replacement_retries_exact_sequence_after_peer_restart_and_certificate_rotation",
      "--",
      "--exact",
      "--nocapture"
    ],
    "purpose": "Prove fresh route installation after intentional authority-cut drain.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/202/v9/execution-proof.json"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
