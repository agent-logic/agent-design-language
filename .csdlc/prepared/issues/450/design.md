# Issue #450 Design — Memory Palace Production Authority Convergence

## Decision and ownership boundary

`adl-runtime-kernel::memory_palace` becomes the sole production authority for Memory Palace admission, topology, working-set selection, canonicalization, and packet validation. `adl-runtime` owns the durable single-writer service around that authority. `adl::memory_palace` remains a compatibility consumer: it validates a Runtime-produced kernel packet and deterministically projects the existing legacy context artifact without deriving independent writable truth.

#450 does not edit `adl/src/long_lived_agent.rs`, `adl/src/long_lived_agent/tests.rs`, `adl/src/resident_tool_execution.rs`, `adl/src/lib.rs`, or `adl-runtime/src/resident_agent.rs`. Those #446-owned paths are outside the write set. If implementation cannot satisfy #450 without one of them, execution stops pending sequencing after #446.

## Production authority minting and fixed trust roots

Opaque verified tokens remain kernel-only. #450 extends trusted Runtime bootstrap, not the per-candidate request, with `BirthdayAuthorityBootstrap`: fixed identity/private/continuity verifying-key registries, required key ids and generations, sanctuary/projection policy, continuity signer id, and service-schema requirements. `LiveBindings` receives this bootstrap authority alongside the existing permit keys. `build_live_assembly` establishes the private identity policy once, captures the fixed continuity trust material in a private `RuntimeMemoryPalaceProvisioner`, and fails assembly construction if the bootstrap authority is absent or invalid. `LiveAssembly` exposes no policy mutator or trust-root accessor.

The Runtime-facing provision method accepts only candidate evidence:

- `BirthdayIdentityRecord` and `BirthdayEvidence`;
- `BirthdayContinuityRecord` and its ordered cycle evidence/manifests;
- Runtime-governed redaction-policy digest and trace authority reference.

Inside the kernel, the provisioner verifies candidate evidence against the policies and key registries fixed at assembly bootstrap. After verified identity exists, it establishes continuity policy only from the provisioner's captured trusted continuity keys/signer plus assembly-owned topology/config hashes and that verified identity; none of those values come from the candidate request. It then calls `verify_birthday_evidence`, `validate_birthday_identity_record`, `verify_birthday_cycles`, and `verify_birthday_continuity_record`, proves common identity/checkpoint lineage, validates trace/redaction inputs, and returns opaque `VerifiedMemoryPalaceAuthority`. A caller may construct a separate Runtime with a different bootstrap configuration, but cannot substitute roots on an already constructed production assembly. Production service tests enter through this fixed-bootstrap path; test-only `authority_tests` helpers are forbidden from the proving lane. If trusted bootstrap material is unavailable without fabrication, the STP stop condition fires and implementation does not claim AC2/AC6.

The Runtime Memory Palace service accepts that token plus normalized ObsMem records, calls the kernel adapter/builder/validator, and emits the validated kernel packet. `adl-runtime` does not reimplement packet fields or trust decisions.

## Dependency and consumer boundary

- `adl-runtime-kernel`: trust verification, opaque authority, packet construction, validation.
- `adl-runtime`: serialized durable service, checkpoint journal, context cache, recovery.
- `adl`: read-only validation and exact legacy projection. A bounded direct kernel dependency is allowed; the dependency remains acyclic.

## Serialized single-writer commit protocol

The service owns one configured repo-relative directory and an exclusive advisory writer lock acquired before reading predecessor state and held through directory synchronization:

```text
memory-palace/
  writer.lock
  journal/<20-digit-generation>.json
  generations/<20-digit-generation>/packet.json
  generations/<20-digit-generation>/context-cache.json
  generations/<20-digit-generation>/checkpoint.json
  latest.json
```

`journal/` is an append-only committed-generation ledger and supplies the rollback floor; `latest.json` is the selection pointer, not the sole rollback detector. Under the writer lock, recovery validates every journal entry from generation 1 through the highest committed entry, including predecessor digest linkage and its referenced artifacts. The highest valid journal entry is authoritative. A pointer higher than or differently bound from that head fails closed. A missing or lower pointer is rejected for consumption, classified as an interrupted pointer commit, and atomically reconstructed to the validated journal head before any context is exposed. Directory scanning never promotes a candidate.

For generation N the service writes packet, cache, and checkpoint into a new generation directory using create-new temporary files, fsyncs each file, renames each file, fsyncs the generation directory, writes and fsyncs the create-new journal entry, fsyncs `journal/`, then atomically replaces and fsyncs `latest.json`, and finally fsyncs the state directory. Only a journaled generation is committed. Two writers cannot select the same predecessor because selection and commit occur under the exclusive lock.

An unjournaled generation directory is an interrupted candidate, never authority. Recovery permits removal/replacement only after validating that it has no journal entry and its complete digests equal the deterministic retry; otherwise it emits reconciliation-required and stops. A journal entry without complete matching artifacts, a journal gap/fork, duplicate generation, pointer ahead/different from the journal head, or conflicting candidate is terminal corruption requiring explicit reconciliation. A pointer behind the fully validated journal head is never accepted as authority; it is safely repaired from the rollback floor as described above. Generation 1 is permitted only with an empty valid journal and no pointer.

## Independent generations and exact bindings

`memory_palace_generation` advances exactly once per committed packet. `birthday_continuity_generation` is copied from verified continuity and may stay fixed across multiple Memory Palace generations. It may advance only when the new verified cycle list extends the prior verified list as an exact prefix and yields the successor continuity head. It may never decrease or change identity lineage. The checkpoint independently binds both generations, identity root/record digest, continuity head/record digest, topology, working set, source references, canonical-input digest, packet digest, trace reference, and redaction-policy digest.

## Context cache

`adl.memory_palace.context_cache.v1` is a deterministic JCS serialization of the validated kernel packet's exact ordered working set plus identity, continuity, both generations, packet/canonical digests, and source packet reference. Its digest is checkpoint-bound. Load reconstructs it from the packet and requires byte equality. It is never writable memory authority.

## Compatibility contract

The kernel packet and the legacy resident artifact remain deliberately distinct serialized contracts:

- #450 intentionally versions the kernel authority packet to `adl.memory_palace.context_packet.v2`. In addition to the selected working set and overflow summaries, v2 contains one canonically ordered immutable `record_index` entry for every admitted input record: record/run/workflow ids, citations, temporal anchor, room/anchor ids, visibility, selected/excluded status, disposition reason, and record digest. Packet and canonical digests cover the index. Runtime persists only this v2 authority packet; kernel v1 remains readable solely as explicit rejected migration evidence because it cannot support a total legacy projection.
- `build_context_from_agent_memory` keeps its current `Result<Option<adl::memory_palace::MemoryPalaceContextPacket>>` signature and continues emitting `adl.memory_palace_context.v1`.
- The adapter validates the referenced kernel v2 packet, then projects legacy `cycle_id`, `input_ref`, nested topology, selected/excluded working set, citations, staleness, and legacy digest using the complete `record_index`. It does not return the kernel type, re-read legacy input, or select/rank records.
- The legacy artifact adds no authority: its source reference and digest must resolve to the validated kernel packet, and tests compare every projected field to that packet. Existing consumers and artifact filenames remain unchanged.
- Legacy input packets are rejected as authority because they lack verified identity/continuity/trace/redaction bindings. No configuration still returns `None`.

Thus AC5 preserves the public return type and artifact schema while AC3 proves semantic and digest agreement with the sole kernel authority.

## Restart rejection

Restart validates the lock-protected journal chain, rollback floor, pointer, checkpoint, kernel packet, reconstructed cache, and current opaque authority before exposing context. It rejects a stale pointer for consumption and repairs it only from a complete valid journal head; it rejects pointer-ahead/different bindings, journal rollback/gaps/forks, duplicates, missing artifacts, forged digests, incompatible schema, identity drift, non-prefix continuity, trace/redaction drift, and canonical mismatch.

## Exact nonzero proof denominator

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test memory_palace`
- `cargo test --manifest-path adl-runtime/Cargo.toml --lib`
- `cargo test --manifest-path adl/Cargo.toml --test memory_palace_tests`

The issue-owned `resident_memory` library tests must call the raw production provisioner and cover concurrent writers, two Memory Palace generations under one birthday generation, verified continuity advance, restart, cache consumption, interrupted candidates, pointer rollback, journal gap/fork, duplicate/conflicting generation, file/directory durability error propagation, forged digests, and incompatible schemas. The `adl` integration target must execute the unchanged public hook's compatibility path with Runtime-produced fixtures and prove exact projection, no-config behavior, and legacy-authority rejection. It also contains assertion-based evidence tests that fail unless canonical feature/evidence surfaces identify exactly one kernel production authority, record every old-surface disposition, cite retained passing evidence, and contain no ambiguous legacy-authority claim. Each named Cargo target must report at least one executed test; match-only census commands are non-proving.

## Migration disposition

Retain kernel packet types/build/validation; add raw production provisioner and opaque authority; add Runtime durable service and integration target; reduce resident derivation to validated legacy projection; retain the old fixture only as rejection/compatibility evidence; update feature/evidence truth only after all proving lanes pass.

## Non-goals and stop conditions

No new trust root, signing-key exposure, directory-scan authority, broad ObsMem rewrite, subjective-memory claim, Runtime v4 redesign, or #446 mutation. Stop if production trust inputs would be fabricated, durability cannot be expressed with the platform's supported primitives, compatibility needs a #446-owned edit, or an asserted proving target runs zero tests.
