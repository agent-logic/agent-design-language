# v0.92.1 Decisions

| ID | Decision | Status | Gate |
| --- | --- | --- | --- |
| D-01 | Combine corporate/IP transfer, C-SDLC v3, and distributed Runtime qualification in v0.92.1. | Accepted by operator | Setup |
| D-02 | Keep all three implementation lanes independent. | Accepted by operator | All planning |
| D-03 | Treat v0.92.5 as a superseded routing draft, preserving its infrastructure content. | Accepted by operator | Corporate lane |
| D-04 | Require qualified counsel for final transfer instruments and legal sufficiency. | Accepted by operator | Corporate release |
| D-05 | Preserve PR #77's reviewed C-SDLC v3 architecture without silent revision. | Accepted by operator | V3-01 |
| D-06 | Use the Rust construction spike as a promote/reimplement and estimate gate. | Planned confirmation | V3-02 |
| D-07 | Use one v3 binary and one operator skill. | Planned confirmation | V3-03 |
| D-08 | Use `state.json` as the sole aggregate commit point. | Planned confirmation | V3-06/V3-08 |
| D-09 | Use branch/worktree ownership, not claims or heartbeats. | Planned confirmation | V3-10A |
| D-10 | Keep watch foreground-only with structured cancellation. | Planned confirmation | V3-14 |
| D-11 | Approve the per-platform commit matrix and Windows mutation posture. | Required after V3-02 | Hard gate for V3-08 |
| D-12 | Never permit v2 and v3 mutation authority for the same issue simultaneously. | Accepted invariant | V3-16 |
| D-13 | Defer v2 retirement to V3-R01 after the rollback window. | Accepted invariant | Post-release |
| D-14 | Require terminal #142 and WP-04.16 production proof before live Runtime qualification. | Accepted invariant | DRT-03/04 |
| D-15 | Use exactly two live Runtime windows and derive all later synthesis offline. | Planned | DRT-03/04 |
