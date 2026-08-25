# Design — v0.92.1

## Topology

`#432 + reviewed planning-package merge -> WP-01 eligible -> operator declares v0.92.1 ready -> operator creates number-free WP-01 -> {Corporate and IP, C-SDLC v3, Distributed Runtime, Podcast, hot reload, Observatory, Runtime v2/v3 decoupling, provider profiles}`; `DRT-C -> DRT-D`; `PROV-A -> PROV-B`. Closed #431 is provenance only and cannot open the wave.

The roots are parallel after the shared opening gates except for the explicit DRT-D and PROV-B edges. Cross-lane dependencies must be explicit issue edges, never assumed from document order.

Within those lanes, #251 TLS 1.2, #122 public exposure, #345 GPU Shepherd hardening, OBS-A, and #84 preparation may proceed concurrently. #84 final proof joins #251 and #122; OBS-B joins OBS-A and #84; GPU-backed distributed qualification consumes #345.

## Lane contracts

- Corporate and IP produces reviewed transfer, rights, account, and governance records without exposing private material.
- C-SDLC v3 consumes the tracked architecture source and delivers typed slices with migration and rollback proof.
- Distributed multi-agent Runtime uses authentic resident agents, governed UTS work, continuity, and evidence-bound qualification.
- Podcast separates operator decisions and external publication authority from repository preparation.
- Axum configuration hot reload uses parse-validate-swap, last-known-good retention, debounced watching, and observable failure without process restart.
- Observatory redesign consumes stable Runtime authority APIs, renders no invented data, and includes keyboard, screen-reader, reduced-motion, empty, degraded, and recovery states.
- Runtime decoupling assigns every v2/v3 source, manifest, import, export, test, and compatibility surface to exactly one authority, with executable migration and rollback proof.
- Provider profiles give all tools one bounded provider contract; Ollama materialization is deterministic and shadow output never acquires mutation authority.
- GCP qualification replays the six-resident contract as a provider-portability sidecar after DRT-C; it neither replaces AWS qualification nor authorizes issue #269.

INT-01 consumes all named roots. Historical #188 informs convergence and quality admission, #190 informs successor planning, and #189 is reserved for the final ceremony.

## Rebaseline rule

Runtime v4 is not silently included. If it changes authority APIs used by Distributed Runtime or Observatory work, affected issues stop, record the exact incompatibility, and replan against the new canonical interface.

## Successor rule

CodeFriend Beta 1 is planned for v0.92.2. v0.92.1 may produce prerequisites, but it must not claim the beta itself.
