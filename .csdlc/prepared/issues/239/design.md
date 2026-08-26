# Issue #239 Design: Terminal Envelope Metadata-Only Head Reconciliation

## Objective

Make retained terminal validation accept the same typed review/publication
metadata-only head progression that pre-merge finish already accepts, while
continuing to reject substantive drift.

## Owned Paths

- `csdlc-v2/src/finish.rs`
- `csdlc-v2/src/bin/csdlc-finish.rs`
- `csdlc-v2/src/cleanup.rs`
- `csdlc-v2/tests/gate_finish.rs`
- `.csdlc/issues/239/`
- `.csdlc/prepared/issues/239/`
- `.csdlc/evidence/239/`

Every other path is read-only input.

## Defect

`execute_finish` accepts an exact PR head after automatic metadata-only review
reconciliation. The derived terminal envelope stores that final head SHA.
`envelope_matches_record`, however, requires the final head's clean revision to
equal the earlier `publication.revision`. A valid publication metadata commit
therefore makes the command reject its own retained envelope.

## Design

Add a root-aware envelope matcher and route both cached validation and cleanup
compatibility through it; never infer the repository from cwd or a potentially
pruned `record.worktree`. Retain the exact-only matcher for compatibility where
repository evidence is intentionally unavailable.

Retain all canonical identity, generation, digest, issue, PR, repository, and
publication checks. For a live PR terminal envelope, accept either:

1. exact equality between `publication.revision` and the terminal head's clean
   revision; or
2. a repository-grounded proof through `git::metadata_only_changed_paths` that
   the change from the publication commit to the terminal head is limited to
   the automatic metadata-only allowlist already enforced by review policy.

The reconciliation must derive both commits from the revision/head values,
verify ancestry, inspect the exact changed paths, and fail closed on malformed
revisions, missing commits, non-ancestry, or any substantive path.

## Regression

Add a focused `gate_finish` test with the PR #238 topology:

- reviewed substantive commit;
- publication revision at the clean pre-publication commit;
- later typed review/publication metadata-only commit as terminal head;
- retained envelope validation succeeds;
- a later substantive source commit is rejected;
- a malformed publication revision is rejected; and
- a metadata-only but non-ancestor commit is rejected.

## Validation

Run only the focused `gate_finish` test target and diff hygiene locally. After
merge, run `csdlc-finish --validate-cached-issue 5835` against current `main`.

## Rollback

Revert the focused `finish.rs` reconciliation, both root-aware call-site edits
in `src/bin/csdlc-finish.rs` and `src/cleanup.rs`, and the `gate_finish`
regression as one atomic slice. Existing exact-equality behavior remains the
fallback and no caller may remain bound to the reverted root-aware API.

## Non-Goals

- Rewriting issue #5835 cards or terminal cache.
- Weakening exact-head review, publication, or canonical digest checks.
- Changing GitHub merge or issue-closure behavior.
