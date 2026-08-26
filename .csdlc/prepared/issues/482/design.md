# Issue 482 design: CORP-A

## Outcome

Produce one accepted critical-asset schedule for the complete asset denominator.

## Authority and scope

This issue owns only the declared paths below. It does not authorize adjacent sprint work,
cloud/provider mutation, credential disclosure, legal advice, or lifecycle work for another issue.

- `docs/operations/corporate/asset-register/**`
- `docs/milestones/v0.92.1/evidence/corporate/corp-a/**`
- `.csdlc/prepared/issues/482`

## Execution shape

1. Reconcile dependencies and freeze the exact issue-local denominator.
2. Produce one accepted critical-asset schedule with provenance, ownership, licensing, trademark, and assignment dispositions.
3. Run the planned PVF lanes and retain bounded, redacted evidence.
4. Obtain exact-head review and stop before publication unless separately authorized.

## Invariants

- Issue completion is exactly acceptance of the one critical-asset schedule; source-specific checks are evidence inputs, not separately closeable results.
- The asset validator proves every critical asset appears exactly once with an accepted disposition and redacted receipt.
- Private credentials, legal instruments, auth codes, recovery factors, and provider secrets stay outside Git.
- Any operator-only mutation requires explicit bounded authorization at execution time.

## Stop conditions

- Unknown critical-asset ownership
- Counsel or corporate authority is missing
- Private material would enter Git
