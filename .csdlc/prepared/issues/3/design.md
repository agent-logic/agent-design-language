# Issue 3 Design: Canonical PRs Closing Preserved Legacy Issues

## Outcome

Complete and harden the typed C-SDLC v2 split-authority publication path so a
pull request in `agent-logic/agent-design-language` can close an issue retained
in `danielbaustin/agent-design-language` without allowing repository, remote,
head, linkage, review, or terminal-state substitution.

## Current Baseline

The merged #5901 readiness repair already introduced separate issue and code
repository identities in publication intent and finish, qualified closing
linkage, canonical PR reconciliation, and a successful live canary. Issue #3
owns the remaining production-hardening and contract work; it does not repeat
that landed implementation.

## Implementation Boundary

- Verify both the configured fetch URL and the effective push URL for the
  selected Git remote against the canonical code repository before any push.
- Reconcile every page of matching open pull requests and fail closed unless
  the result is unambiguous.
- Preserve same-repository publication behavior and exact-head review guards.
- Add focused regression tests for split authority, push-URL substitution,
  pagination/ambiguity helpers, fork/base/head drift, and qualified linkage.
- Document the two-repository request, publication, and finish contract in the
  typed operator skills and GitHub-client boundary documentation.
- Retain a canary evidence packet grounded in canonical PR #5 and legacy issue
  #5901 without performing any new legacy repository code mutation.

## Invariants

- The issue repository remains the identity in the canonical issue record.
- The code repository controls the Git remote, pushed branch, PR base/head,
  observed PR, and publication evidence.
- Split-authority PR bodies require a qualified GitHub closing reference.
- Same-repository requests remain backward compatible.
- Review revision, issue generation/digest, branch, head SHA, and live closing
  relationship remain fail-closed.
- No AWS or remote-builder work is in scope.

## Proof

Focused Rust tests cover publication preparation, remote verification,
pagination/ambiguity, GitHub normalization, finish linkage, and public schemas.
The retained canary packet verifies the already-merged canonical PR and closed
legacy issue using live GitHub identities and ancestry. Exact-head independent
review must resolve every actionable finding before publication.

## Rollback

Revert only issue #3 changes. The already-landed conservative split-authority
baseline remains available; same-repository publication is unchanged. Do not
push to, rewrite, or otherwise mutate the preserved legacy code repository.
