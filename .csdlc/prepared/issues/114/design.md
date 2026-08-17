# Issue #114 Design: Durable Governed Conversation History

## Outcome And Boundary

Issue #114 adds a Runtime-owned durable record of the public-safe conversation
contract supplied by #111 and reauthorizes every read and lifecycle operation
through the Layer 8 authority supplied by #112. Authorized operators can reopen
a conversation, page and search their visible turns, distinguish delivery and
response outcomes, export an authorized snapshot, request deletion, and verify
continuity through stable identities and receipts across browser and Runtime
restart.

The store is not agent memory, execution restore authority, an audit-log
replacement, or a browser transcript cache. It never persists private
cognition, provider payloads, credentials, policy internals, signing material,
or conversations outside the current principal's authority.

## Dependency And Serial Gates

- #111 must be terminal through a merged PR, and its merge revision must be an
  ancestor of the #114 execution base. It supplies canonical conversation,
  turn, sequence, correlation, causation, delivery, and response identities.
- #112 must then be terminal through a merged PR, and its merge revision must
  also be an ancestor of the #114 execution base. It supplies authenticated
  Layer 8 principals, action capabilities, current-policy decisions,
  revocation, public-safe refusals, and redacted audit linkage.
- #110 remains the open umbrella and sequencing authority. It need not close
  before #114, but any changed ordering or scope requires typed replanning.
- #83 is a transitive read-only baseline through #111 and #112. #114 performs
  no #83 mutation, lifecycle transition, card edit, or artifact rewrite.
- Binding, product edits, product validation, review, publication, and
  closeout stop until both direct dependency merges are terminal and ancestral.

## Exact Affected-Area Ownership

Exclusive issue-owned product paths after all serial gates pass:

- `adl-runtime/src/conversation_history.rs`
- `adl-runtime/tests/conversation_history.rs`
- `adl/tests/conversation_history_runtime_api.rs`
- `adl/tools/validate_v092_html_observatory_history.mjs`
- `docs/milestones/v0.92/features/DURABLE_CONVERSATION_HISTORY.md`

Narrow shared integration paths, editable only after the dependency gate and a
typed replan if their merged shapes differ:

- `adl-runtime/src/lib.rs`, module export only;
- `adl/src/csm_runtime_api.rs`, authorized history, search, export, deletion,
  and receipt endpoints only;
- `docs/api/runtime-v3/v1/openapi.json`, exact history contract only;
- `docs/api/runtime-v3/v1/observatory.openapi.json`, exact Observatory history
  projection only;
- `demos/html-observatory/app.js`, Runtime-backed paging, search, receipt, and
  cache-revocation behavior only.

Read-only inputs include the merged #111 session and turn contract, merged #112
authority and audit contract, `adl-runtime/src/runtime_api_auth.rs`,
`adl-runtime/src/continuity_history.rs`, and all #83 lifecycle artifacts. The
new store is independent from checkpoint and lifelog databases so conversation
history can never become execution restore authority. Every path not listed
above is read-only. Preparation owns only `.csdlc/issues/114` and
`.csdlc/prepared/issues/114`.

## Canonical Storage Contract

The Runtime opens one canonical absolute node-local `redb` database through a
private path derived from Runtime configuration. Callers cannot supply a path,
file handle, table name, or alternate store. The database has versioned tables
for conversation metadata, immutable turn envelopes, current outcome records,
idempotency keys, retention/deletion state, receipt chain entries, and schema
metadata. Keys use canonical binary encodings with explicit upper bounds.

Conversation metadata binds conversation id, Polis, participants, creation
time, latest committed sequence, retention class, policy epoch, schema version,
and a digest of the public-safe projection. Each turn binds the exact #111 turn,
sender, recipient, sequence, correlation, causation, bounded public content,
delivery state, response reference, timestamps, and predecessor receipt digest.
Outcome transitions are monotonic and closed; pending may become delivered,
responded, refused, failed, timed out, or cancelled, but a terminal outcome
cannot be rewritten into a different terminal truth.

One write transaction atomically validates authorization context, appends an
exact next turn or monotonic outcome, updates the conversation watermark,
records the idempotency result, and appends a receipt-chain entry. Exact replay
of the same canonical request returns the retained receipt. Conflicting reuse,
sequence gap, reorder, duplicate sequence with different content, stale policy
binding, overflow, or N+1 bounded input fails before mutation.

## Read, Search, Export, Retention, And Deletion

Every list, page, search, export, and receipt lookup carries a fresh #112
authorization decision bound to principal, action, conversation scope, policy
epoch, query digest, cursor, and deadline. Browser state and a previously
allowed page never grant a later read. Revocation or expiry invalidates the
next request, and the Observatory clears rendered and local cached transcript
material for the affected scope.

Pagination uses an opaque authenticated cursor that binds store generation,
conversation, principal projection, policy epoch, sort direction, filter,
snapshot high-watermark, and expiry. It returns a stable bounded snapshot with
no duplicates or skips while later turns append. A stale, tampered, expired, or
cross-principal cursor fails closed. Search operates only on the authorized
public-safe normalized projection, with bounded query length, result count,
scan work, and deadline. It is not full-text search over agent-private state.

Export creates a bounded canonical manifest plus authorized records and receipt
digests. It is reauthorized at start and completion, records the exact policy
and high-watermark, and never includes forbidden fields. Retention is explicit
per allowed policy class and defaults to a bounded duration. Expiry creates a
durable lifecycle decision before physical reclamation.

Deletion is a monotonic tombstone protocol. The transaction records authority,
scope, cutoff, policy epoch, request id, previous receipt digest, and a deletion
receipt before records become unavailable. Reads deny tombstoned data
immediately. Compaction later removes only exact tombstoned generations and
retains the minimal redacted audit and receipt evidence required by policy.
Exact retry returns the retained deletion receipt. Partial deletion, hidden
surviving indexes, export residue, or deletion without authority fails closed.

## Schema Migration Contract

The database records a store schema version, compatible reader floor, migration
generation, source digest, target digest, and migration receipt. Opening an
unknown newer schema, unsupported older schema, missing metadata, or ambiguous
migration state leaves history unavailable without modifying bytes.

A migration is offline for writes and proceeds under one exclusive store lock:

1. validate the entire source schema and bounded record counts;
2. persist a migration intent binding source generation and digests;
3. copy and transform into new versioned tables without changing old tables;
4. validate record counts, ordering, identities, receipt chains, redaction, and
   deterministic target digest;
5. atomically publish the target generation and migration receipt; and
6. retain the source generation until the configured rollback window closes.

Restart before target publication resumes or discards only the incomplete
target after verifying the intent. Restart after publication opens only the
published target and reconciles any owed receipt or source cleanup. Rollback is
allowed only before new-version writes and only to the exact retained source
generation. Downgrade after new-version writes, lossy transformation, silent
field defaulting, or in-place mutation is denied.

## Recovery And Corruption Contract

On startup, recovery verifies schema metadata, published generation, table and
record bounds, conversation watermarks, contiguous sequences, idempotency
bindings, outcome monotonicity, retention/deletion state, and every receipt-chain
link before serving history. Recovery never invents a turn, fills a gap,
reorders records, trusts an index over primary records, or uses conversation
history to restore agent execution.

An interrupted transaction relies on `redb` atomicity and then revalidates the
published generation. A persisted operation whose external reply was lost is
resolved cache-first through its idempotency key and retained receipt. Index
drift is rebuilt only from validated authorized public-safe primary records
under a new index generation. Primary corruption, digest mismatch, chain break,
watermark disagreement, unknown schema, or ambiguous generation quarantines the
affected store or conversation and emits a bounded public-safe unavailable
state. No partial history is presented as complete.

Disk-full, read-only media, lock contention, cancellation, deadline expiry, and
shutdown before the first durable effect return no-effect failure. After an
atomic commit, retry returns the retained result. Shutdown drains bounded work
or stops admission; it does not acknowledge uncommitted history.

## Failure Cases And Exact Proof Denominator

The focused proof denominator is exactly forty-two named cases:

`append_first_turn`, `append_ordered_turn`, `outcome_monotonic`,
`restart_continuity`, `browser_reconnect_page`, `exact_duplicate_cached`,
`conflicting_duplicate_denied`, `sequence_gap_denied`, `reorder_denied`,
`terminal_rewrite_denied`, `unauthorized_read_denied`,
`revoked_read_denied`, `expired_identity_denied`, `policy_epoch_drift_denied`,
`cross_polis_denied`, `cross_principal_cursor_denied`,
`tampered_cursor_denied`, `stale_cursor_denied`, `stable_snapshot_paging`,
`bounded_search`, `search_private_state_absent`, `bounded_export`,
`export_reauthorization_denied`, `retention_expiry`, `deletion_tombstone`,
`deletion_exact_retry`, `deletion_residue_absent`, `partial_write_recovery`,
`reply_loss_cached`, `disk_full_no_false_success`, `read_only_no_effect`,
`lock_contention_bounded`, `corrupt_record_quarantined`,
`receipt_chain_break_quarantined`, `watermark_drift_quarantined`,
`unknown_newer_schema_denied`, `unsupported_older_schema_denied`,
`migration_resume_before_publish`, `migration_reopen_after_publish`,
`lossy_migration_denied`, `rollback_after_new_write_denied`, and
`forbidden_field_redaction`.

The test and browser receipt producer must emit one marker for each exact name
in this order. Validators reject missing, extra, duplicate, renamed,
reordered, or nonpassing cases. No case may define a hidden subdenominator.

## Observatory Contract

The Observatory requests history only through canonical Runtime endpoints. It
renders stable turn and receipt identity, outcome, timestamp, retryability, and
public-safe refusal or unavailability. It never treats a local cache as
authority, reconstructs missing turns, scrapes provider transcripts, or keeps
revoked content visible. Paging, search, export, deletion, reconnect, stale
cursor, migration unavailable, and corruption quarantine states are explicit
and keyboard accessible.

## Rollback

Rollback disables history APIs and Observatory history controls while leaving
canonical #111 live sessions and #112 authorization fail closed. It preserves
the exact database generation and receipts for an authorized later recovery;
it never converts history into browser ownership, checkpoint authority, a
provider transcript, or an ungoverned fallback store.

## Execution Handoff

The executor must reread #110, #111, #112, and #114 through the typed GitHub
owner, prove both dependency issues terminal with merged revisions ancestral to
the selected base, inspect their actual changed paths for ownership drift, and
rerun typed validation and doctor. Any contract or path mismatch requires typed
SPP/VPP replanning before `csdlc-bind`. This preparation commit is not product
implementation, review, publication, or completion authority.

## Non-Goals

- Global memory search, private agent state, provider transcript scraping, or
  execution checkpoint/lifelog authority.
- Indefinite retention, silent deletion, browser-owned history, or ungoverned
  export.
- Implementing or mutating #83, #110, #111, #112, or sibling WP-18C issues.
- Rooms, roster/presence, attention inbox, final hardening, Unity, AWS, public
  deployment, model/provider work, publication, merge, or closeout.
