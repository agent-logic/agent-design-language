# Issue 523 design — successor planning refresh

## Result

One reviewed v0.92.2 CodeFriend Beta 1 planning package, refreshed from the
post-#522 v0.92.1 truth. This issue changes planning only and creates no
successor execution issues.

## Inputs and authority

The execution session freezes the merged #522 `main` revision, enumerates the
canonical v0.92.1 results and residuals, then reconciles them against
`docs/milestones/v0.92.2/**`, `docs/planning/ADL_FEATURE_LIST.md`, and relevant
TBD sources. Each item receives one disposition: delivered, deferred, retained
existing issue, or planned successor unit.

## Execution shape

1. Build the exact source/disposition denominator.
2. Repair only inconsistent successor planning and feature-routing surfaces.
3. Run the focused v0.92.2 validator and diff hygiene.
4. Obtain independent exact-head review and publish one PR closing #523.

## Guardrails

- Do not open v0.92.2 issues.
- Do not promote deferred work without explicit authority.
- Do not claim planned work is implemented.
- Stop if #522 is not a reviewed merge or ownership cannot be reconciled.
