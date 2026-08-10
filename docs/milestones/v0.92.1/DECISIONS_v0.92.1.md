# v0.92.1 Decisions

## C-SDLC v3 Architecture Decisions

These eleven decisions preserve the mandatory sequence in the accepted C-SDLC
v3 architecture. They are implementation gates, not substitutes for the
milestone decisions below.

| Source ID | Decision | Status | Gate |
| --- | --- | --- | --- |
| V3-D01 | Approve the shared v3 product and command contract. | Required | V3-01 |
| V3-D02 | Approve the Rust construction-spike measurements and pass/fail thresholds. | Required after measurement | V3-02 |
| V3-D03 | Approve one binary and one operator skill. | Required | V3-03 |
| V3-D04 | Approve the `App` dependency-container boundary. | Required | V3-04 |
| V3-D05 | Approve `state.json` as the sole typed aggregate and commit point. | Required | V3-06/V3-08 |
| V3-D06 | Approve direct flags plus optional typed `--input`. | Required | V3-03/V3-10A/V3-10B |
| V3-D07 | Approve branch/worktree topology rather than claims and heartbeat authority. | Required | V3-10A |
| V3-D08 | Approve explicit foreground `pr watch` with structured cancellation. | Required | V3-14 |
| V3-D09 | Approve no initial extension system beyond repository-declared PVF runners. | Required | V3-11A/V3-11B |
| V3-D10 | Decide whether `finish` can ever own an explicitly authorized merge. | Required | V3-15 |
| V3-D11 | Approve the per-platform commit matrix and whether Windows mutation support ships initially or remains fail-closed read-only pending equivalent proof. | Required after V3-02 | Hard gate for V3-08 |

Validation must compare this complete ordered set with the accepted source
architecture. The presence of `V3-D11` alone is not sufficient.

## Milestone Decisions

| ID | Decision | Status | Gate |
| --- | --- | --- | --- |
| M-01 | Combine corporate/IP transfer, C-SDLC v3, and distributed Runtime qualification in v0.92.1. | Accepted by operator | Setup |
| M-02 | Keep all three implementation lanes independent. | Accepted by operator | All planning |
| M-03 | Treat v0.92.5 as a superseded routing draft, preserving its infrastructure content. | Accepted by operator | Corporate lane |
| M-04 | Require qualified counsel for final transfer instruments and legal sufficiency. | Accepted by operator | Corporate release |
| M-05 | Preserve PR #77's reviewed C-SDLC v3 architecture without silent revision. | Accepted by operator | V3-01 |
| M-06 | Never permit v2 and v3 mutation authority for the same issue simultaneously. | Accepted invariant | V3-16 |
| M-07 | Defer v2 retirement to V3-R01 after the rollback window. | Accepted invariant | Post-release |
| M-08 | Require terminal #142 and WP-04.16 production proof before live Runtime qualification. | Accepted invariant | DRT-03/04 |
| M-09 | Use exactly two live Runtime windows and derive all later synthesis offline. | Planned | DRT-03/04 |
