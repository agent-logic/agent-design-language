# Issue 5826 Design: Stable Name And Identity Root

## Outcome And Sources

Define the WP-09 identity record from `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`, the candidate birthday record in `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`, and current Runtime v3 lineage/private-state authority in `adl-runtime-kernel/src/identity_memory.rs` and `adl-runtime-kernel/src/private_state.rs`. Retained Runtime v2 lineage is compatibility evidence only.

## Owned Paths

- `adl-runtime-kernel/src/birthday_identity.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/fixtures/birthday_identity/authority_tests.rs`
- `adl-runtime-kernel/tests/fixtures/birthday_identity`
- `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`
- `.csdlc/prepared/issues/5826/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5826/produce-native-receipt.rb`
- `.csdlc/evidence/5826`
- `.github/workflows/wp09-native-birthday-identity.yml`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-birthday-kernel-registration-v1",
    "paths": [
      "adl-runtime-kernel/src/lib.rs"
    ],
    "issues": [
      5825,
      5826,
      5827,
      5828,
      5829,
      5830,
      5831,
      5833
    ],
    "order": [
      5825,
      5826,
      5827,
      5828,
      5829,
      5830,
      5831,
      5833
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-identity-feature-doc-v1",
    "paths": [
      "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"
    ],
    "issues": [
      5826,
      5827
    ],
    "order": [
      5826,
      5827
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5826-identity-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"
    ],
    "issues": [
      5826,
      5843
    ],
    "order": [
      5826,
      5843
    ]
  }
]
```

## Contract

Stable name is a label bound to an identity root, never the root itself. Aliases are ordered, provenance-bearing additions and cannot silently replace the root. A Birthday Identity may be constructed or revalidated only with an opaque verified-evidence capability produced from the existing Runtime v3 authorities: `verify_binding` plus `MemoryLedger::restore` must authenticate the `IdentityBinding` and signed `MemoryCheckpoint`, while `PrivateStateLineage::append` plus `project_private_state` must authenticate the accepted private-state record, lineage position, signer, generation, sanctuary policy, principal, and projection bytes. Trust-policy establishment is crate-private so external callers cannot nominate self-consistent attacker roots; evidence verification consumes the already-provisioned opaque policy capability. The Birthday layer must not duplicate those cryptographic authorities or accept caller-supplied visibility/redaction booleans as proof.

The verified capability binds the candidate's citizen/runtime/continuity identity, origin reference, signed checkpoint head, private-state record hash, governed projection digest, signer, and generation. Construction fails closed for invented provenance, wrong continuity heads, forged or mismatched bindings, stale/wrong signers or generations, projection tampering, and raw-private artifacts mislabeled as reviewer-visible. Identity creation also rejects empty or ambiguous roots, duplicate/conflicting aliases, path-unsafe references, and unsupported root bases.

## Dependencies And Invariants

WP-08/#5825 must be terminal before implementation; prior citizen-state lineage remains authoritative substrate. `adl-runtime-kernel/src/identity_memory.rs` and `adl-runtime-kernel/src/private_state.rs` are read-only authority inputs, not code owned by WP-09. Serialization and identity-root derivation are deterministic. Raw private state is never required for review, and a display name, boot admission, wake, snapshot, or copied state cannot establish identity alone.

## Validation

The exact `birthday_identity` integration-test target must run a nonzero test count covering a real signed identity binding, signed memory checkpoint, accepted private-state lineage, governed projection, canonical records, deterministic ordering, missing roots, alias collision, invented provenance, forged or mismatched bindings, stale/wrong signer and generation, substituted continuity heads, projection tampering, and raw-private disclosure mislabeled reviewer-visible. The issue-local producer must run that target on native GitHub Actions macOS and Linux jobs at exact candidate HEAD and retain a hashed source manifest that includes `birthday_identity.rs`, `identity_memory.rs`, and `private_state.rs`, a complete nextest log, machine-derived negative cases, and a canonical semantic-output artifact. The independent validator recomputes those files and producer digest, parses the positive test count, verifies workflow/run/job identity, and requires byte-identical semantic outputs; ancestral SHA equivalence is forbidden.

## Rollback

Remove only the WP-09 identity record module, registration, integration test,
fixtures, and owned feature-document edits. Preserve prior lineage primitives,
WP-08 outputs, rejected identity records, and native receipt evidence; rollback
must not rename an identity root or rewrite continuity history.

## Non-Goals

This issue does not prove multi-cycle continuity, migration, citizenship, reputation, legal personhood, or the birthday event.
