# Issue 259 design

## Purpose

Bind governed Runtime transport certificate and authority flows to the terminal #258 authority-store boundary.

## Source truth

- Live issue: #259, `[v0.92][WP-04.16b3.1a][203.b] Bind governed transport to authority adapters`.
- Parent: #203 coordination/final integration.
- Required prerequisite: terminal, reconciled, and ancestral #258 at merge `193f77d24a693f955a2fcf3bdfc759ad1db8aff4`.
- Preserved parent prerequisites: #191, #201, #202, #199, and #200.

## Scope

- Route governed transport certificate and authority flows through #258 authority-bound store adapters.
- Prove governed transport authorization uses authority-bound certificate handles rather than raw-store bypasses.
- Keep the change limited to governed transport and directly coupled transport tests.

## Non-goals

- No migration, recovery, placement, projection, resource-weather, snapshot-catalog, capability-advertisement, or peripheral Runtime caller migration; those belong to #260.
- No parent #203 integration, publication, or closeout.
- No #205 Shepherd/Observatory serving-eligibility work.
- No cleanup or mutation of preserved #203 worktrees.

## Validation shape

- Focused governed transport tests must prove the positive authority-bound path.
- Negative tests must reject any raw-store or caller-nominated bypass for transport certificate authority.
- Strict Runtime clippy remains the local quality gate before publication.

## Review boundary

Fresh exact-head review should inspect only #259 governed transport authority binding, #258 dependency consumption, #260/#203 non-absorption, and validation evidence.
