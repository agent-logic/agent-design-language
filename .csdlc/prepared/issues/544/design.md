# Issue 544 Design: Primary Checkout Bootstrap Guard

Status: proposed for independent design review

## Problem

`csdlc-issue create` can currently initialize C-SDLC state from the repository's primary checkout. In ADL, the primary checkout is reserved for inspection and should stay clean on `main`; bootstrap state created there pollutes shared surfaces such as `.csdlc/issues`, `.csdlc/prepared`, and `.csdlc/locks`.

## Design

Add a pre-write guard to the native initialization route in `csdlc-v2/src/lifecycle.rs`. The guard runs at the beginning of `initialize_issue`, before the binding lock, request-created design or diagram files, `.csdlc` issue records, prepared surfaces, or issue locks can be created.

The guard uses Git topology authority rather than branch-name heuristics:

- read `git worktree list --porcelain` for the invocation repository;
- take the first `worktree` entry as Git's primary checkout;
- canonicalize the invocation root and primary checkout;
- verify both paths share the same `--git-common-dir`;
- reject only when the invocation root is the primary checkout.

Every topology ambiguity is fail-closed. If Git worktree listing fails, no
primary entry is present, the primary path cannot be canonicalized, the
invocation or primary common Git directory cannot be resolved, or the common
directories do not match, initialization returns a typed `UnsafeCheckout` error
before any initialization write. This makes "not sure" equivalent to "do not
bootstrap here."

This preserves idempotent initialization in isolated staging checkouts and allows non-primary worktrees that share the same common Git directory. Existing bind policy remains unchanged: canonical issue execution still binds to a FastWork child worktree through `csdlc-bind`.

## Acceptance Mapping

- AC-1: topology detection comes from Git worktree metadata and common-dir verification.
- AC-2: the guard runs before all initialization writes.
- AC-3: non-primary checkouts sharing the common Git dir remain allowed.
- AC-4: existing initialized-record reconciliation remains handled by `bootstrap_issue`.
- AC-5: focused lifecycle tests cover primary rejection, zero residue, isolated success, fail-closed ambiguous topology, and bind policy.
- AC-6: operator docs document inspection-only primary checkout and isolated staging bootstrap.

## Non-Goals

- No migration or cleanup of existing records.
- No GitHub issue semantic changes.
- No merge, finish, or cleanup authority.
- No weakening of FastWork bind policy.

## Validation

Focused Rust tests in the C-SDLC v2 lifecycle surface should prove:

- a primary checkout invocation fails with `UnsafeCheckout`;
- the failure leaves no design, diagram, issue, prepared, or lock residue;
- a non-primary checkout initialized from the same Git common dir succeeds;
- ambiguous topology, including missing primary entry, canonicalization failure, and common-dir mismatch/error, fails before issue-scoped residue;
- bind worktree policy still rejects non-FastWork targets and allows canonical FastWork targets as before.
