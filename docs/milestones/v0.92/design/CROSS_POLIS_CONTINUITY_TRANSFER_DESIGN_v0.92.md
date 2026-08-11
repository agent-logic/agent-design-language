# Cross-Polis Continuity Transfer Design

## Purpose

This WP-17 design turns the landed v0.92 birthday proof set into a deterministic
input-classification contract for future movement work. It is deliberately a
design artifact: it specifies verification and rejection semantics but no
runtime transfer protocol.

## Sources Of Truth

- `adl-runtime-kernel/src/birthday_identity.rs`
- `adl-runtime-kernel/src/birthday_continuity.rs`
- `adl-runtime-kernel/src/memory_palace.rs`
- `adl-runtime-kernel/src/capability_envelope.rs`
- `adl-runtime-kernel/src/cognitive_profile.rs`
- `adl-runtime-kernel/src/adaptive_learning.rs`
- `adl-runtime-kernel/src/acip.rs`
- `adl-runtime-kernel/src/birth_witness.rs`
- `docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json`

The review inventory supplies exact merged/reviewed proof references. Runtime
types supply field semantics. Neither source gives WP-17 operational migration
authority.

## Model

A `ContinuityTransferProposal` is conceptual input to later work:

```text
source_repository
source_revision
artifact_schema
artifact_path
artifact_sha256
identity_root
continuity_head
authority_context_sha256
redaction_policy_sha256
requested_disposition
```

Every field in the proposal is an untrusted claim. The verifier obtains
repository, revision, authority-context, and redaction-policy truth from the
trusted anchor set below and requires exact equality before evaluating artifact
bytes. A proposal cannot add, rotate, or replace an anchor.

The proposal contains references only. It cannot contain raw memory, private
state, credentials, signing keys, mutable provider sessions, checkpoint bytes,
or runtime grant handles.

The conceptual result is one of:

```text
candidate(reference_digest)
local_only(reason)
defer(required_authority)
quarantine(reason, competing_digests)
reject(reason)
```

No result activates the referenced state in a target polis.

## Trusted Anchor Set

The v0.92 WP-17 verifier is provisioned with, rather than caller-supplied:

- canonical code repository `agent-logic/agent-design-language` and issue
  repository `danielbaustin/agent-design-language`;
- accepted merge commits and reviewed evidence entries from the digest-pinned
  WP-16 manifest and Sprint 4 terminal review named in
  `.csdlc/evidence/5835/dependency-authority.json`;
- schema-specific authority-context and redaction-policy digests resolved from
  the accepted canonical record at the accepted revision; and
- replacement ACIP authority `agent-logic/agent-design-language#209`, PR `#215`,
  merge `a77519c3fca9f64752af41c9a2ebd396468891f7`, plus the digest-pinned local and
  native manifests under `.csdlc/evidence/209/`.

The verifier rejects a repository, revision, signer/authority context, or
redaction policy that is absent from those anchors. It also rejects an anchor
rotation unless a separately reviewed authority update adds the predecessor,
successor, effective boundary, and authorization proof to the trusted set.
Internal consistency, matching caller-provided digests, public schemas, and a
valid signature under a caller-nominated key are insufficient.

## Verification Order

Order is part of the contract. A future implementation must not skip ahead to
governance or transport when lineage is unresolved.

1. **Shape:** accept only a known schema and repository-relative path.
2. **Trusted source:** resolve repository and accepted revision from the
   provisioned anchor set; compare proposal claims and reject any mismatch.
3. **Trusted policy:** resolve signer/authority context and redaction policy
   from the anchored canonical record or registry; reject caller nomination.
4. **Bytes:** recompute the artifact digest at the accepted source revision.
5. **Canonical record:** recompute the schema's own record/packet digest.
6. **Identity:** bind identity root to the verified identity-record digest.
7. **Continuity:** validate predecessor, ordered cycles, authority context, and
   continuity head; require the proposed head to match.
8. **Review authority:** for a work-package proof, verify exact reviewed head,
   merged PR, closed issue, and merge ancestry.
9. **Privacy:** reduce the artifact to the row-authorized public or redacted
   projection before crossing an authority boundary.
10. **Transport gate:** require authenticated source/target identities,
   confidentiality where needed, freshness, replay isolation, authorization,
   and revocation. Otherwise return `defer`.
11. **Governance gate:** require v0.93 policy for standing, rights, duties,
   institutional recognition, or acceptance of learned effects. Otherwise
   return `defer`.
12. **Decision:** emit only a digest-bound disposition and audit reference.

## Conflict Semantics

Conflicts fail closed:

- A different identity root for the same stable name is an identity conflict,
  not an alias update.
- Two different canonical identity records for one root are quarantined.
- Two valid continuity heads with no authority-approved successor relation are
  quarantined. Wall-clock recency does not select a winner.
- A witness-set or receipt mismatch is rejected; a receipt is never used to
  repair its own authority chain.
- An adaptive-learning history with a missing predecessor, divergent sequence,
  or rollback mismatch is quarantined or rejected, never copied into service.
- A carrier trace proves communication behavior only. It cannot resolve an
  identity, continuity, capability, or governance conflict.

## Copy Semantics

Copying is not transfer. A copied snapshot or state blob has no admissible
lineage merely because hashes match. The target must validate an approved
predecessor relation and the current authority context. Until future WP-04 and
governance contracts define and authorize an operational transition, copied
state remains `reject`.

This rule also applies to mutable reasoning graphs, provider session state,
replay tables, cached working sets, runtime grants, and checkpoints.

## Privacy And Redaction

The source authority applies redaction before a reference crosses the boundary.
The target receives no implicit retrieval right. Public schema catalogs are
decode aids, not content authorization. A target that cannot validate a
redaction-policy digest or governed projection must reject the input.

Permitted projections are limited to:

- identity fields explicitly visible in the governed identity projection;
- digest-bound continuity metadata and cycle references;
- public or redacted Memory Palace citations without raw payloads;
- bounded capability identifiers, limits, denials, and provenance digests;
- the cognitive-profile public projection and its nonclaims;
- bounded learning decision/rationale metadata without mutable state;
- public witness summaries, receipt caveats, and public evidence references;
- WP-16 public projections that remain narrower than retained proof.

## WP-04 Boundary

WP-17 does not define or implement:

- snapshot/chunk transport or storage replication;
- target preparation, fencing, handoff, resume, rollback, or recovery;
- distributed consensus, lease, placement, or polis discovery;
- network protocols, cross-polis authentication, encryption, key custody,
  rotation, revocation, or replay stores;
- production observability or migration orchestration.

Those mechanics remain WP-04 work and must consume this classification only as
a reviewed design input. WP-04 cannot reinterpret a `candidate` disposition as
authorization to activate state.

## v0.93 Boundary

v0.93 may decide how verified identity and continuity evidence affects
citizenship, standing, rights, duties, social contract, or institutional
review. WP-17 supplies no default. Missing governance authority produces
`defer`, never implicit acceptance.

Capability grants are non-operative on arrival. Cognitive-profile fields are
not reputation. Adaptive-learning history is not a right to import learned
effects. A citizen-facing receipt is not citizenship.

## Determinism

For identical source bytes, source revision, authority context, redaction
policy, transport-policy state, and governance-policy state, classification
must be identical. Ordering, display name, wall clock, local path, host name,
or provider session state cannot change the result.

## Rollback

Rollback restores only the WP-17-owned feature and design documents to their
previous revision. The handoff is read-only and is not part of rollback. It
retains `.csdlc/evidence/5835/rejected-transfer-matrix.json`, the validation
logs, and `.csdlc/evidence/5835/rollback-proof.json`. It never rewrites identity
roots, continuity heads, child proofs, review records, or downstream governance
decisions.

## Review Questions

1. Does every matrix row bind to a landed schema or exact proof surface?
2. Can any copied, ambiguous, private, or self-authorized state reach
   `candidate`?
3. Are WP-04 mechanics and v0.93 governance unambiguously downstream?
4. Does any public-schema or receipt language accidentally grant content or
   standing authority?
5. Can the positive and negative validators be reproduced without chat state?
