# Local governed evidence

Issue #881 adds `adl codefriend evidence admit-local`, which acquires the exact
scoped Git snapshot through the local production adapter and admits it into an
operator-local store. Repository content remains inert, untrusted evidence.
Admission is not a completed review or publication.

```
adl codefriend evidence admit-local --checkout CHECKOUT --repository https://host/owner/repo --revision FULL_SHA --scope scope.json --store STORE --retention-seconds 86400
adl codefriend evidence admit-packet --input packet.json --store STORE --retention-seconds 86400
adl codefriend evidence read --store STORE --packet-id PACKET_ID
adl codefriend evidence delete --store STORE --packet-id PACKET_ID
```

Store paths must be outside the source checkout and contain no symlink or parent
traversal. A new or empty directory may become a store; existing nonempty directories
require the store marker. Existing directory permissions are preserved. New directories
and records use owner-only permissions on Unix. An OS lock serializes operations;
concurrent callers fail with `store_busy` rather than reading a partial write.

The validated packet is retained as one immutable admission record. Its per-object
evidence identity binds canonical repository, exact revision, relative path, Git
blob and content digest. It excludes retention policy, clock, display text and
finding identity. Only included objects get evidence IDs; omitted/missing objects,
unsupported analysis, source scope and license/context text stay explicit in the
packet. The shared #878 JSON/TOML scanner rejects known unsafe retained content
before any temporary or final write. Unsafe acquisition objects are omitted whole.
This is bounded recognition, not universal secret detection or prompt-injection
prevention. No repository text invokes tools or changes authority.

Admission records bind the packet, derived evidence metadata, redaction policy,
original admission time and retention deadline. A separate create-only digest anchor
rejects altered provenance or policy even if the record's outer digest is recomputed.
Identical repeated packets with identical policy return their original admission and
deadline; changing policy is a collision, never an implicit renewal. Retention is
1 second through 365 days. The production clock is sampled on every operation;
proofs inject a controlled clock and advance it on an already-open handle.

Writes validate first, sync a temporary file, create the final link without replacing
an existing record, then sync the directory. Reopen discards interrupted temporary
files and orphan anchors. Expiry is enforced lazily on open/read/admission: it writes
and syncs a content-free tombstone before removing the raw admission record. Explicit
delete follows the same path. Reopen finishes any interrupted unlink; expired/deleted
packets cannot be re-admitted to that store. Tombstones retain packet ID, admission
digest, deletion time and reason, plus integrity digest. Anchors contain only digests.
Deleted raw content is not retained in these records. Secure erasure of filesystem
snapshots, backups, SSD blocks or another process's prior copy is outside this contract.

This is a local operator-controlled store. Hashes verify internal consistency; they
are not signatures or proof that a blob belongs to an authenticated commit. An actor
who can rewrite every record, anchor and tombstone can replace the store's history.
The path checks and lock handle ordinary callers, not a hostile filesystem administrator
racing syscalls. A caller that opens a library store controls its clock; the CLI uses
the system clock and offers no clock override.

Shared contracts are `codefriend.contracts.v1` in `evidence::contracts`. A run binds
repository/revision/scope, exact packet/admission provenance, inclusions/exclusions,
lane versions, credential-free route, completion and failures. Complete, incomplete,
failed, cancelled and withheld remain distinct. Stable finding identity uses repository,
perspective, rule and semantic anchor; mutable title, line and revision are not identity.
Evidence references, severity rationale, confidence/unknown, inference and limitations
are required. Duplicate finding IDs fail closed even if their prose happens to agree.

Comparison records validate explicit delta assertions; they do not perform matching.
Resolved/added/changed/unchanged require compatible completed coverage, same repository,
scope, lane versions and provider route. Partial/narrower runs cannot prove resolution.
Not-comparable carries an explicit reason. Publication records bind exact run, finding
set, artifact manifest, renderer versions, scope, target, claims and nonclaims. An
approved record with a changed binding is rejected; withheld and invalidated states
remain representable. An approval field is a contract record, not an operator
authentication mechanism or a publishing implementation.

Review, memory and renderer consumers must reuse `consume` and the versioned fixtures
in `adl/tests/fixtures/codefriend/evidence/`; their algorithms remain successor work.
The runtime lane test target `codefriend_evidence` executes CLI admission/readback,
identity relocation, deletion/expiry, crash cleanup, tamper/secret negatives and shared
consumer conformance. Tests are deterministic bounded CPU/disk local integration,
required before consumer execution; no model/cloud call is part of this proof.
