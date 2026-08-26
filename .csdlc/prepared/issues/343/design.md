# Issue #343 Design — Demonstration, Handoff, and Publication Sprint Closeout

## Purpose

#343 is a coordination and review umbrella. It does not implement child product
work. It closes only after the current-repository WP-18 and WP-18B children
have truthful terminal outcomes, the historical WP-17 and WP-19 outcomes have
been revalidated, and one exact evidence-based sprint review passes.

## Canonical Graph

- Current WP-18 authority: #256.
- Current WP-18B authority: #341.
- Historical WP-17 and WP-19 authorities remain read-only inputs and must be
  revalidated rather than replayed or reimplemented.
- #414 is a prerequisite consumed by #256 and #341; #343 does not duplicate its
  continuity implementation or proof.
- #342 is out of this sprint denominator.
- #340 and the deferred Unity #84/#251 track are not #343 children.
- The next release-tail handoff is #307/#308 and is not executed by #343.

## Readiness State

Preparation and design review may complete while #256 and #341 remain open.
Binding, evidence reconciliation, review execution, and terminal closeout stay
blocked until both children are terminal, canonical, and ancestral to the exact
candidate base. A terminal cache alone is insufficient when its canonical or
ancestry check fails.

## Owned Surfaces

- `.csdlc/issues/343/**`
- `.csdlc/prepared/issues/343/**`
- `.csdlc/evidence/343/**`
- `docs/milestones/v0.92/review/sprint_343/**`

All child implementation, demo, provider, Runtime, Observatory, AWS, website,
and release-tail paths are read-only inputs.

## Review Packet

The final packet inventories each required child or historical authority with:

- issue and PR identity;
- exact reviewed and merged revisions;
- terminal cache generation and digest;
- canonical-match and merge-ancestry result;
- validation and review denominators;
- demo path and retained artifact digests;
- publication/non-publication classification;
- residual risks and non-claims.

The packet must reject missing, stale, noncanonical, nonancestral, fixture-only,
or receipt-only evidence. It must not convert an optional or deferred stream
into a release claim.

## Exit Contract

1. #256 is terminal, canonical, ancestral, and its real birthday demo uses the
   governed Observatory and habitable Runtime path.
2. #341 is terminal, canonical, ancestral, and its provider-neutral matrix has
   the declared real-provider positive and failure denominators.
3. Historical WP-17 and WP-19 terminal evidence is identified and validated
   without creating new implementation authority.
4. Claimed demonstrations resolve to real repository/runtime paths with exact
   revisions and artifact digests.
5. Release-truth, redaction, credentials, public/private, and non-publication
   boundaries are reconciled.
6. One exact-head sprint review has no unresolved actionable finding.
7. The closeout records #307/#308 as the next handoff without executing them.

## Failure Policy

Fail closed on open or nonterminal child authority, stale or noncanonical
terminal cache, nonancestral merge, ambiguous historical authority, missing
demo artifact, fixture substitution, secret/private evidence retention,
unsupported publication claim, or unresolved review finding.

The issue-owned readiness validator uses separate terminal and packet modes.
The terminal mode executes the typed canonical terminal-cache validator for
every current and historical authority, requires the candidate base to equal
the immutable current HEAD, and binds exact generation/digest, PR, reviewed and
merged revisions, merge ancestry, real demo paths, retained artifact digests,
and historical evidence to that output. The packet mode additionally binds the
sprint packet digest, release truth, redaction, credential/private-evidence
absence, excluded issues, handoff issues, and a fresh-session review artifact
to the exact candidate revision and packet digest. A separate exact
scope validator inventories committed, staged, unstaged, and untracked paths and
checks the actual candidate content for diff hygiene.

## Non-Goals

- Implementing or repairing #256, #341, #340, #342, #84, #251, #307, or #308.
- Running AWS, providers, demos, publication, or release deployment.
- Reopening or recreating historical WP-17/WP-19 implementation.
- Treating #342 or deferred Unity work as a sprint gate.
- Claiming v0.92 release completion from #343 alone.
