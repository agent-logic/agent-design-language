# Issue 298: Anchored Classification and Resumable Recovery Ledger

## Purpose and boundary

Issue #298 implements only the non-destructive authority half of the #297 graph. It classifies a failed projection namespace, archives the rejected projection without deleting it, builds a verified recovery-owned canonical candidate, installs that candidate, and resumes the same operation after interruption. Destructive archive cleanup belongs to #299. The exhaustive cross-engine qualification matrix belongs to #300.

The parent #297 r1 commit `2d84616d5b309f0f4bd8d1a21dfc82bf907a8812` is unreviewed input only. No lifecycle record, validation claim, review assignment, or cleanup implementation transfers from it.

## Namespaces and serialization

For target issue `N`, the owner holds the existing issue lock for the full classify or recover operation and mutates only handle-relative names below `.csdlc/issues`:

- `N/`: canonical projection.
- `.N.backup`: prior projection from an interrupted ordinary commit.
- `.N.rollback-preserved`: verifier-rejected projection retained as evidence.
- `.N.recovery/<operation-id>/`: private mode-0700 recovery attempt.
- `.N.recovery/<operation-id>/rejected/`: archived rejected evidence.
- `.N.recovery/<operation-id>/candidate/`: unpublished recovery-owned canonical candidate.
- `.N.recovery/<operation-id>/displaced/`: verified prior canonical retained after atomic install.

The recovery root and attempt directory must be same-owner, non-symlink directories on the issue-store mount. Receipt files are exclusive-create, no-follow, regular, link-count-one, owner-only mode 0600. Unexpected names, wrong type/owner/mode/mount, or multiple active attempts are unsafe. Receipt sequence numbers and state names are deterministic; an existing same-name receipt is accepted only when its complete bytes and digest exactly match the expected immutable receipt.

## Anchored per-node observation

Classification opens the `.csdlc/issues` directory and candidate roots without following symlinks, then walks children relative to retained directory handles. It never authorizes a later mutation from a string path alone. Each directory and regular file records:

- relative byte-safe path and node type;
- device and handle-derived mount identity (`statx` mount ID on Linux; `fstatfs` filesystem ID plus device on macOS/BSD; fail closed when unavailable);
- inode, ctime seconds/nanoseconds, link count, uid, gid, and mode;
- size and BLAKE3 digest for regular files; and
- parent identity and sorted child-name set for directories.

Every node must match the issue-store owner, remain on the candidate root device and mount, and be non-group/world-writable. Regular files require link count one. Symlinks, sockets, devices, FIFOs, unsupported types, repeated `(mount, device, inode)` identities, mount transitions, non-UTF-8 paths unsupported by the record format, and containment escape fail closed. Two complete enumerations through retained handles must match before classification returns. Immediately before and after any recovery rename or exchange, retained identities, manifests, and parents are revalidated.

A projection is verified only when its `index.json` parses, names target issue `N`, its record digest recomputes, all six card values/rendered projections agree with record identity/generation, and authored artifacts satisfy existing store validation. Invalid embedded generation/digest never grants authority.

## Tagged CAS and classification

`classify_preserved_projection` accepts actor, reason, issue, and exactly one anchor:

- `verified_canonical { generation, record_digest }`: canonical is verified and matches.
- `expected_canonical_absent { backup_generation, backup_record_digest }`: canonical is absent and exactly one verified backup matches.
- `exact_observed_invalid { canonical_root_identity, canonical_manifest_digest, backup_generation, backup_record_digest }`: canonical is the exact observed invalid tree and exactly one verified backup matches.

Classification inventories canonical, backup, rollback-preserved, and recovery attempts under the lock, emits exact observations plus a self-digest, and performs no mutation. Dispositions are:

- `clean`: matching verified canonical, no preserved/backup/active attempt.
- `recoverable`: one verified prior source, one distinct rejected evidence source, and unambiguous failed-operation lineage.
- `already_recovered`: canonical audit and final immutable receipt agree for the requested operation/evidence.
- `ambiguous`: insufficient lineage, multiple plausible sources/attempts, or conflicting exact states.
- `unsafe`: topology, identity, type, containment, ownership, permission, projection, or receipt corruption.

Failed-operation lineage is explicit: recovery requires the ordinary commit's preserved-state marker or equivalent typed receipt to bind prior canonical identity/digest, rejected identity/manifest, and failure boundary. The mere presence of `.backup` or `.rollback-preserved` is not lineage.

`PREPARED` commits exactly one immutable anchor-specific transition plan before any namespace mutation:

- `verified_canonical`: canonical is the verified prior source and rollback-preserved is the rejected source. Archive rollback-preserved to `rejected`, construct the candidate from canonical, atomically exchange candidate and canonical, then move the exchanged prior canonical from the candidate name to `displaced`.
- `expected_canonical_absent`: canonical is required absent, backup is the verified prior source, and rollback-preserved is the rejected source. Archive rollback-preserved to `rejected`, construct the candidate from backup, move backup without replacement to `displaced`, revalidate canonical absence, then install candidate at canonical with a no-replace handle-relative rename.
- `exact_observed_invalid`: canonical is the exact invalid rejected source and backup is the verified prior source. Archive canonical without replacement to `rejected`, construct the candidate from backup, move backup without replacement to `displaced`, revalidate canonical absence, then install candidate at canonical with a no-replace handle-relative rename.

Each plan fixes the exact source and destination identities, required-absent names, operation ordering, supported atomic primitive, and pre/post parent manifests for every step. A restart may execute or adopt only the transition plan committed by `PREPARED`; a different anchor, source role, order, or namespace state is unsafe. No plan deletes or overwrites a source.

## Immutable recovery ledger

Every state receipt includes schema, issue, operation id, monotonic sequence, previous receipt digest, classify digest, anchor, actor/reason, branch/worktree identity, exact source/destination names and identities, expected pre-state, intended post-state, and state-specific manifests. The receipt is written to a temporary regular file inside the private attempt, fsynced, linked/renamed without replacement to its final deterministic name, and followed by attempt-directory fsync. Receipt temporary names are never adopted without a prior intent naming their exact identity.

Candidate nodes use a separate operation-owned temporary-node protocol. `CANDIDATE_PLAN` commits a deterministic, collision-resistant temporary basename and final relative name for every node. The private attempt directory, operation id, node ordinal, required-absent temporary and final names, expected parent identity and manifest, node type, initial attributes, final content digest, final metadata, and permitted parent-manifest delta are fixed before creation. A temporary node is never canonical candidate content until its exact identity and completed state have durable receipts and a no-replace publish transition installs it at the planned final name.

The candidate's typed recovery audit event contains no resulting record digest. It binds the prior generation and record digest, operation id, classify digest, anchor-specific transition-plan digest, and complete candidate-manifest commitment. The candidate record digest is computed only after that non-circular audit payload is fixed. `CANDIDATE_VERIFIED` records the resulting generation and record digest, and later install, canonical-verification, and final receipts bind that digest.

The state sequence is:

1. `PREPARED`: exact classify receipt, failed-operation lineage, registered topology, all retained candidate identities, required-absent attempt children, and one exact anchor-specific transition plan.
2. `ARCHIVE_INTENT`: exact rejected source selected by the transition plan and required-absent `rejected` destination.
3. `REJECTED_ARCHIVED`: destination contains the exact source root/tree and both parents were fsynced.
4. `CANDIDATE_PLAN`: verified prior projection, complete sorted node construction plan, non-circular recovery audit payload and candidate-manifest commitment, expected new generation and resulting record digest, and required-absent `candidate` root/nodes.
5. For each planned directory/file, `NODE_CREATE_INTENT` durably records the operation-owned temporary name, required-absent temporary and final names, exact parent pre-manifest, permitted one-name post-manifest, type, and initial attributes. Exclusive no-follow creation is followed immediately by `NODE_CREATED_IDENTITY`, which binds the new handle-derived identity and observed one-name parent delta. If a crash lands between create and that identity receipt, restart may adopt only the sole node at the intent-named temporary name after two matching retained-handle observations prove the planned type/initial attributes, exact parent pre/post delta, required-absent final name, and absence of every unplanned child; it then writes `NODE_CREATED_IDENTITY`. Any mismatch is unsafe and retained.
6. `NODE_WRITE_INTENT` binds the created identity, complete expected bytes or directory metadata, final BLAKE3 digest, exact metadata, and zero-or-current verified prefix length. A regular-file restart may continue only when the retained identity is unchanged and its current bytes are an exact prefix of the committed bytes; it appends only the committed suffix through the retained handle. Full content and metadata completion is recorded by `NODE_WRITE_COMPLETED`; inconsistent, oversized, non-prefix, or identity-drifted content is unsafe and retained. Directories record their completed metadata through the same intent/completion pair.
7. `NODE_FSYNC_INTENT` precedes node fsync and `NODE_FSYNC_COMPLETED` records the unchanged identity/content/metadata after fsync. `NODE_PARENT_FSYNC_INTENT` precedes parent fsync and `NODE_PARENT_FSYNC_COMPLETED` records the exact parent manifest after fsync. Restart after either fsync but before its completion receipt repeats that idempotent fsync through the retained handle and then records completion; it never infers durability from content bytes.
8. `NODE_PUBLISH_INTENT` binds the completed temporary identity, required-absent final name, and exact pre/post parent manifests. It publishes with a handle-relative no-replace rename. Exact temporary-present/final-absent order performs the rename; temporary-absent/final-present with the exact recorded identity adopts it after repeating parent fsync; every other order is unsafe. `NODE_CREATED` completes the node only after publish identity, final content/metadata, and parent durability revalidate. Directories are planned parents-first and files are written through retained handles.
9. `CANDIDATE_VERIFIED`: every planned node has an exact created receipt; two anchored walks match; record/cards/authored artifacts validate; exactly one typed recovery audit event binds operation id, classify digest, prior source, archived rejected manifest, actor/reason, prior generation/digest, transition-plan digest, and candidate-manifest commitment; the receipt binds the resulting generation and record digest.
10. For backup-source plans, `DISPLACE_INTENT` and `PRIOR_DISPLACED` move the exact verified backup without replacement to `displaced`, fsync both parents, and revalidate the required-absent canonical. For `verified_canonical`, displacement remains after exchange as steps 13-14.
11. `INSTALL_INTENT` records the plan-selected primitive and exact pre/post order. `verified_canonical` requires the exact verified prior canonical and proven no-follow atomic exchange. Absent-canonical and invalid-canonical plans require canonical absent after their recorded archive/displacement steps and use a no-replace handle-relative candidate-to-canonical rename. Platforms lacking the selected proven primitive fail closed before `INSTALL_INTENT`; both parents are fsynced after installation.
12. `CANONICAL_INSTALLED`: exact post-exchange identities/manifests and parent durability.
13. For `verified_canonical`, `DISPLACE_INTENT` records the exact exchanged prior canonical at the candidate name and required-absent `displaced` destination.
14. For `verified_canonical`, `PRIOR_DISPLACED` adopts or performs the no-replace rename, records the exact identity at `displaced`, and fsyncs both parents. Backup-source plans must already have their matching completion from step 10.
15. `CANONICAL_VERIFIED`: installed canonical record/cards/audit/generation/digest/manifests reread through retained handles and exactly match the plan.
16. `RECOVERED`: final result digest binds `CANONICAL_VERIFIED`, archive, displaced prior, and full ledger head.

No state rewrites canonical audit after installation. The audit is part of the complete candidate before exchange. The rejected archive and displaced verified prior are retained; #298 never removes them.

## Deterministic restart table

Restart opens and validates the immutable chain before considering namespace state. Missing sequence, broken hash link, duplicate state, unexpected name, malformed receipt, or identity drift is unsafe.

- Intent absent: execute only the next state after revalidating the preceding completion.
- Rename intent present, completion absent: exact source plus absent destination permits the no-replace rename; absent source plus destination containing the exact recorded source identity/tree adopts the completed rename after repeating parent fsync; both present, both absent, or replacement identity is ambiguous.
- Node-create intent present, created-identity absent: absent temporary and final names permit exclusive creation. Exactly one node at the operation-owned temporary name may be adopted only from the intent's exact type, initial attributes, parent-manifest delta, required-absent final name, and two matching retained-handle observations; restart then records its identity. No other present node is adopted.
- Created identity plus write intent: exact committed prefix permits appending only the committed suffix; exact completed bytes and metadata permit recording write completion. Non-prefix bytes, excess length, metadata drift, aliasing, or identity drift are unsafe. Restart never truncates, overwrites, replaces, or guesses partial content.
- Node- or parent-fsync intent without completion: exact identity/content/metadata repeats the recorded fsync through retained handles and records completion. A completion receipt is required; bytes alone never prove durability.
- Node-publish intent without completion follows the rename rule for the exact temporary and final names. No-replace publication or exact post-order adoption is permitted; collisions are unsafe and retained.
- Install intent present, completion absent: for `verified_canonical`, exact pre-order permits the atomic exchange and exact post-order permits adoption after repeating both parent fsyncs; for absent-canonical and invalid-canonical plans, exact absent canonical plus exact candidate permits the no-replace install rename and exact installed canonical plus absent candidate permits adoption after repeating parent fsyncs. Any third order fails closed. Installation is never inferred from bytes.
- Displace intent follows the rename table.
- `CANONICAL_INSTALLED` without later receipts resumes displacement and verification.
- `CANONICAL_VERIFIED` without `RECOVERED` writes only the final receipt.
- `RECOVERED` returns the same result only when canonical embedded audit, archive, displaced tree, classify digest, and ledger head still agree.

A crash before or after every receipt creation, temporary-node create, content or metadata write, node fsync, parent fsync, no-replace node publish, archive rename, exchange, displacement, and verification therefore resumes the same operation from exact intent/identity/completion truth or fails closed on observed drift without deleting, replacing, or guessing evidence. A different operation id while an incomplete attempt exists is rejected.

## Ordinary commit gate

Ordinary issue-store commit remains fail-closed while a rollback-preserved namespace or incomplete/ambiguous recovery attempt exists. It may proceed after a complete matching `RECOVERED` receipt and canonical embedded recovery audit agree. The commit must revalidate that pair under the issue lock; it must not remove the recovery attempt, rejected archive, or displaced prior projection.

## Child-scoped proof

Issue #298 focused tests exercise production classify/recover code with deterministic failpoints across all recovery-only states above:

- clean, recoverable, already-recovered, ambiguous, and unsafe classification;
- each tagged anchor, stale anchor, failed-operation-lineage mismatch, wrong issue/repository/branch/worktree;
- archive, candidate temporary-node create, created-identity receipt, write intent/completion, file fsync intent/completion, parent fsync intent/completion, no-replace node publish, exchange, displacement, verification, and final receipt interruptions before and after each boundary;
- exact partial-prefix continuation, completed-write adoption, repeated fsync, and published-node adoption for the same operation, plus fail-closed non-prefix, collision, replacement, alias, and identity-drift cases;
- symlink/special/hardlink/repeated-inode/cross-mount/wrong-owner-or-mode/corrupt-card/namespace mismatch and collision/replacement races;
- same-operation idempotency and different-operation rejection;
- rejected and displaced evidence preservation; and
- successful later ordinary typed commit plus initialized/ready and #291 regression.

Tests use issue-owned deterministic failpoint injection immediately around production boundaries. They do not use sleeps, wall-clock widening, or mock receipt strings as authority. Cleanup capture/removal/placeholder behavior is excluded and belongs to #299; the integrated exhaustive matrix belongs to #300.

## Non-goals

- No archive deletion, recursive removal, cleanup namespace, tombstone, or placeholder disposal.
- No #291, #294, #296, #299, or #300 implementation/card mutation.
- No merge, parent closeout, or claim that child completion releases #296.
