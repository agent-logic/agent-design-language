# Distributed Test Plan Consultation

## Boundary

Issue `#146` retains ownership of v0.92.1 planning. The task named `ADL V2 V0.91.8 (2)`, which owns active `#142` implementation work, reviewed only the proposed qualification boundary. It made no `#146` edits and took no v0.92.1 execution ownership.

## Review Scope

The consultation reviewed DRT-01 through DRT-07 for topology, sequencing, proof quality, failure injection, determinism and replay, security boundaries, resource cleanup, and duplication with `#142`.

## Findings And Disposition

| Finding | Disposition |
|---|---|
| Qualification must consume the exact merged `#142` revision and passing retained production proof. | Added as a hard external dependency and quality gate. |
| The local proof must explicitly demonstrate `3 -> 2 -> 1` voter behavior. | Added to DRT-03, the demo matrix, and the Runtime resilience gate. |
| The old Observatory lease must expire before a successor binds. | Added to DRT-03 proof and Runtime feature requirements. |
| Hybrid proof must use private authenticated transport, separate AZs, independent snapshots, Wuji isolation, and AWS-only continuity. | Added to DRT-04 and the hybrid demo contract. |
| Replay must bind exact commands, terms, committed indexes, receipts, source revisions, and model digests. | Added to DRT-07 and the Runtime feature contract. |
| Security must cover key separation, non-voting Shepherd identity, stale lease/fence denial, cross-polis replay, and pre-auth REST/WSS disclosure. | Added to DRT-05 and the Runtime security gate. |
| Cleanup must run after every failed phase, not only final success. | Added to DRT-03, DRT-04, DRT-07, demos, and release gates. |

## Result

The lanes remain qualification-only and do not duplicate `#142` implementation. Live DRT-03 and later work remains blocked until `#142` is terminal with exact merged ancestry and retained production Guardian, API, WSS, and WP-04.16 proof.
