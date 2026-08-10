# Issue 90 design

## Decision

Add one typed `csdlc-issue migrate-code-repository` operation for legacy issue
records that were bound before `code_repository` became part of the canonical
record. The operation may fill an absent field; it may not retarget an existing
identity or weaken publication checks.

The migration is a recovery transaction, not a general record editor. It is
authorized only from the issue's registered branch and canonical worktree, and
only when every effective `origin` fetch and push URL resolves to the requested
GitHub repository.

## Typed contract

The versioned request contains:

- issue number
- requested `code_repository`
- expected record generation and digest
- actor and reason

The report contains the issue, phase, canonical branch and worktree, adopted
repository identity, whether a mutation occurred, and the resulting generation
and digest. Errors remain typed and machine-readable through the existing v2
error envelope.

The CLI surface is:

```text
csdlc-issue --root <repo> migrate-code-repository --request <request.json>
```

## Authorization sequence

The operation acquires the shared binding lock and then the issue lock, matching
the established bind lock order. It then proves, in order:

1. the request schema, issue number, actor, reason, repository identity,
   generation, and digest are valid;
2. the record phase is `bound`, `implemented`, or `reviewed`;
3. the record has an exact branch and canonical worktree registration;
4. the invocation is running in that canonical worktree on that branch;
5. tracked and untracked worktree state is clean;
6. every effective `origin` fetch and push endpoint resolves to one GitHub
   repository identity;
7. that identity equals the requested `code_repository`;
8. the record's `code_repository` is absent.

Immediately before commit, while both locks remain held, it repeats the CAS,
topology, current branch, cleanliness, and normalized remote-identity checks.

Missing or ambiguous topology, a wrong or divergent origin, a dirty worktree,
stale CAS, an unsupported phase, or any existing identity fails before mutation.
Validation checks request CAS before the existing-field guard. Therefore an
exact retry of a successful request deterministically returns `stale_digest`;
generation and audit evidence remain unchanged. A new request using current
CAS against any present field returns `invalid_transition`, whether the present
identity matches or conflicts, because this operation only fills an absent
field.

## Atomic mutation

After authorization, one locked store transaction:

- sets only `code_repository` in semantic lifecycle state;
- advances record generation and canonical digest;
- mechanically advances each card identity generation and regenerates its
  projection digest without changing semantic card content;
- appends one audit event plus a versioned
  `csdlc.code_repository_migration_evidence.v1` payload containing the issue,
  actor, reason, pre-generation and pre-digest, prior absent value, normalized
  requested/fetch/push identities, phase, branch, canonical worktree, and a
  boolean clean-worktree result.

Semantic card content and all lifecycle fields other than code repository,
generation, digest, and the new audit evidence remain unchanged. In particular,
phase, review assignment/result, reviewed revision, publication evidence,
readiness, and terminal truth are preserved byte for byte. Raw remote URLs are
never retained because they may contain credentials. A failed write leaves the
prior record authoritative through the existing atomic store replacement
mechanism.

## Publication boundary

Migration grants no publication authority. A migrated `reviewed` record must
pass the existing `csdlc-publish` checks unchanged, including exact-head review,
qualified issue linkage, issue/code repository identity, branch identity, clean
worktree, and remote readback. The implementation will reuse existing Git and
repository-identity helpers rather than introduce a weaker parallel parser.

## Validation

Focused tests cover:

- successful migration in `bound`, `implemented`, and `reviewed` phases;
- exact preservation of review and lifecycle truth;
- publication preflight after reviewed migration;
- wrong, non-GitHub, missing, and divergent fetch/push origin identities;
- wrong worktree or branch, incomplete topology, and dirty state;
- unsupported phases, stale generation/digest, and an existing identity;
- deterministic retry behavior with one audit event;
- CLI request parsing, schema exposure, installed-command smoke, formatting,
  and warning-denied Clippy.

Build artifacts and Rust caches are placed on `/Volumes/FastWork`; that host
path is execution policy and is not embedded in portable repository contracts.

## Operator fallback

Until the typed command is installed, operators stop at the publication error
and retain the bound worktree unchanged. They must not hand-edit `state.json`,
rewrite cards, retarget remotes, or bypass publication identity checks.

## Non-goals

- changing `issue_repository`
- replacing a present `code_repository`
- repairing arbitrary branch, worktree, or remote topology
- migrating initialized, published, merge-ready, merged, or closed-out records
- refreshing review or granting publication authority
- providing a general administrative state mutation command
