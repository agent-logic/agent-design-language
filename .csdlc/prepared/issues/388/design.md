# Design: implemented-phase card truth repair route

## Problem

Implemented C-SDLC issues can reach a truthful product/proof state while older card fields remain stale. Existing implemented-phase repair operations cover some SIP/STP/SPP/SOR surfaces, but #114 exposes gaps:

- SPP summary repair still requires a reviewed/published/merge-ready recovery transition.
- VPP summary and failure-policy have no implemented-phase repair route.
- SOR follow-up text has no implemented-phase repair route.

## Approach

Add bounded semantic operations that are authorized only in `Implemented` phase after a current review recovery and cleared downstream truth:

- repair SPP summary after current review recovery even when the issue has only a bound-to-implemented transition.
- repair VPP summary/failure-policy as explicit validation-truth fields.
- replace SOR follow-ups as execution-truth cleanup, including replacing the vector with `[]` to remove all stale follow-ups while rejecting blank entries in any non-empty replacement.

Each operation remains CAS-guarded, actor/reason required, issue-local, audit-appending, and refuses active review assignment/review/publication/readiness/terminal truth.

## Proof

Focused `csdlc-v2` regression covers a #114-like sequence:

1. bootstrap/bind/implemented,
2. assign/recover review,
3. apply each repair, including empty-vector SOR follow-up removal,
4. prove audit provenance and only expected fields change,
5. fail closed for stale CAS, active assignment, terminal truth, wrong card/field, empty required text, and blank SOR follow-up entries.

## Non-goals

No generic implemented-phase `set_field`; no product mutation; no publication/review weakening.
