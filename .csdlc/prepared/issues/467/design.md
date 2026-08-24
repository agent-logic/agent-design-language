# Issue 467 Design

## Intent

Repair the v0.92 WP-22 quality gate after #311/PR #466 proved the denominator shape but published a vacuous all-blocked matrix. The corrective gate must discover and hydrate every canonical row, grant release credit only where exact evidence satisfies the contract, and classify remaining blockers with concrete cause.

## Approach

The corrective implementation keeps #311 immutable and writes a new #467 packet. The generator derives the same 13 feature plus 20 critical-path denominator from canonical v0.92 sources, then hydrates each row from explicit row evidence definitions and independently validates that every row is accepted or classified. Acceptance requires exact repository, owner, implementation path, reviewed head, PR head, merge ancestry, required checks, proof artifacts, claim boundary, and typed or approved recordless terminal authority. Non-accepted rows use a taxonomy that separates missing implementation, missing proof, stale or non-ancestral authority, unresolved evidence mapping, and explicit planned/deferred milestone authority.

The validator recomputes the denominator and row identities, rejects missing, duplicate, extra, ambiguous, unclassified, cross-row, and cross-repository substitutions, and refuses an uninvestigated 100% blocked publication. An all-blocked matrix remains possible only when every row carries independently verified concrete blockers.

## Boundaries

This issue does not implement product features, waive missing proof, rewrite #311/PR #466, mutate #312, or depend on administrative closeout work. It only repairs the quality-gate evidence hydration semantics, retained corrective packet, and directly supporting docs/tests.

## Validation

Run the focused matrix generator/validator, the adversarial suite, diff hygiene, typed C-SDLC validation/doctor, and one fresh exact-head review before publication.
