# Structured Output Record

Template: 1.0.0

Issue: 5865

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Bind each WP-04.02 Transport authorization to the authenticated TLS leaf identity and prove the complete wrong-domain, unrelated-key, stale-replay, and concurrent-stream denial surface.

## Artifacts

- .csdlc/evidence/5865/execution-proof.json
- .csdlc/evidence/5865/distributed-transport.stdout.log
- .csdlc/evidence/5865/negative-cases.json

## Execution

- Require the exact active signed Transport authority certificate when constructing transport authorization and bind its holder, trust domain, generation, certificate identity, and Ed25519 subject public key.
- Parse the authenticated TLS leaf Ed25519 subject key and reject any authority whose trust domain or cryptographic subject does not match the peer binding before network use.
- Add negative tests for wrong-domain authority, unrelated TLS subject keys, replay-window boundary staleness, and the configured unidirectional stream ceiling while preserving single-provider AWS-LC Rustls operation.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_transport",
      "--no-tests=fail"
    ],
    "purpose": "Prove 14 focused authenticated transport, authority-binding, replay-window, stream-bound, cancellation, and framing behaviors.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5865/exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5865/validate-proof-receipt.rb"
    ],
    "purpose": "Recompute the two-revision source, evidence, command, artifact, and negative-case bindings.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5865/exact-revision-proof-receipt.log"
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

- none
