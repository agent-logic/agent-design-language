# Gate 10D2 bounded deletion design

## Boundary

Apply only an exact, approved D1 deletion manifest in small independently
reviewed PR slices. The issue remains unbound until eligibility is true and the
operator explicitly approves deletion.

## Wave

1. Revalidate D1 inputs and approval against the candidate revision.
2. Delete one bounded obsolete owner surface.
3. Prove v2 independently and verify protected rollback/importer paths remain.
4. Review the exact revision and merge only when green.
5. Recompute removed/retained LoC and test counts after each slice.

## Invariants

Useful code may remain with owner and justification. The 90 percent deletion
goal is a reviewable target, not a code-removal command or completion cap.
Measured results below the target require explicit retained-surface review and
approval; useful or necessary code is never removed solely to improve the
percentage. Rollback and importer paths belong to separate dated issues.

## Non-goals

No early sunset, unrelated ADL/Runtime cleanup, or deletion outside the
approved manifest.
