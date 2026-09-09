# v0.92.2 Work Breakdown Structure

Status: planned. Uncreated rows use number-free planning identifiers; three admitted rows reuse existing issues. No GitHub issues are created by this document.

| WP | Work track | Primary outcome | Depends on |
|---|---|---|---|
| WP-01 | Milestone opening | Validate package and create the authorized execution wave | none |
| CF-SHELL | Product shell | Setup, onboarding, run controls, status, artifact browser | WP-01 |
| CF-ADAPTER | Adapter v2 | Portable local/GitHub/CI repository ingestion | WP-01 |
| CF-EVIDENCE | Evidence core | Stable identity, provenance, redaction, retention, shared finding/run contract and consumer fixtures | CF-ADAPTER |
| CF-COG | Architecture cognition | Dependency, boundary, coupling, drift, blast-radius, quanta, ADR analysis | CF-EVIDENCE |
| CF-GOV | Executable governance | Fitness functions and CI integration | CF-EVIDENCE |
| CF-REVIEW | Review engine | Four perspectives, synthesis, remediation, test planning | CF-EVIDENCE |
| CF-MEMORY | Longitudinal memory | Second-run comparison and compatibility handling | CF-EVIDENCE |
| CF-UX | Publication and outputs | Human approval, claims/non-claims, manifests, Markdown/HTML/PDF | CF-SHELL, CF-EVIDENCE |
| CF-PROOF | Docs, fixtures, and proof repos | Examples, fixtures, ADL self-review, external OSS proof | CF-COG, CF-GOV, CF-REVIEW, CF-MEMORY, CF-UX |
| CF-INTEGRATE | Beta 1 integration | One coherent product path and acceptance packet | all other CF tracks, PLAT-PROVIDER, PLAT-MEMORY |
| PLAT-PROVIDER | Provider configuration | Separate provider behavior from editable endpoint/profile data | WP-01; merged v0.92.1 issue #622 |
| PLAT-MLX | MLX/Metal provider | Add one bounded Apple MLX/Metal adapter over the canonical provider-definition contract | PLAT-PROVIDER |
| PLAT-UTS | UTS productization | Standardize and package the UTS contract for supported consumers | WP-01 |
| PLAT-RUST | Rust reduction | One measured behavior-preserving refactoring slice | WP-01 |
| OPS-AWS | AWS inventory maintenance | Refresh SCR, S3, model, and staleness deltas from completed #484 ownership-inventory authority | WP-01; completed #484 baseline |
| PUB-MEDIUM | Medium preparation | Prepare one v0.92.2 Medium article packet without publishing | WP-01 |
| PUB-CSDLC | C-SDLC paper preparation | Advance one C-SDLC paper packet without submission | WP-01 |
| PLAT-MEMORY | Memory Palace integration | Deliver the next bounded production Runtime/CodeFriend Memory Palace slice | CF-EVIDENCE, CF-MEMORY |
| SPEC-RETEST | Speculative decoding requalification | Retest the retained prototype and issue an evidence-backed keep, repair, or retire decision | WP-01 |
| OBS-LIVE (#720) | Existing Observatory issue | Remove retained-mode hazards from live product | Existing live UI; no WP-01 wait |
| SIM-UMBRELLA | C-SDLC first sprint | Coordinate the first C-SDLC simplification sprint | Completes after SIM-07; coordination opens before SIM-01 |
| SIM-01 | C-SDLC first sprint | Read-only diagnostics and baseline journeys | Own readiness; parallel Runtime; no WP-01 wait |
| SIM-02 | C-SDLC first sprint | One current installed command contract | SIM-01 |
| SIM-03 | C-SDLC first sprint | Typed evidence and intent-oriented commands | SIM-02 |
| SIM-04 | C-SDLC first sprint | One semantic issue transaction owner | SIM-03 |
| SIM-05 | C-SDLC first sprint | Derived cards and precise evidence invalidation | SIM-04 |
| SIM-06 | C-SDLC first sprint | Conversion rehearsal and writer fencing | SIM-05 |
| SIM-07 | C-SDLC first sprint | Independent qualification and authorized transition | SIM-06 |
| TAIL-01..10 | Canonical release tail | Quality through ceremony in standard order | TAIL-01 converges integration plus all admitted supporting/existing tracks |

## Parallelism

CF-SHELL, CF-ADAPTER, PLAT-UTS, PLAT-RUST, OPS-AWS, PUB-MEDIUM, PUB-CSDLC, and SPEC-RETEST can start independently after WP-01. PLAT-PROVIDER also requires merged v0.92.1 issue #622; PLAT-MLX follows PLAT-PROVIDER. After the evidence contract merges, cognition, governance, review and publication can advance in parallel; Memory Palace integration additionally requires CF-MEMORY. The SIM sprint has its own serial implementation order, beginning first alongside Runtime; product integration and the release tail retain their stated convergence gates.

## Work-Package Rule

The milestone inventory reconciles one bounded issue per expanded row. WP-01 opens the CodeFriend wave; SIM-UMBRELLA coordinates its independently launched eight-issue sprint. Reuse #720; consume #717 and #718 as v0.92.1 predecessors; do not recreate completed #620 or predecessor #439. Resolve the separate WP-01 conductor first; it cannot create itself. The denominator is 39 rows: the original 30 planning rows, one admitted existing issue and the eight-issue SIM sprint. With WP-01 resolved, 37 rows remain prospective creations and one reuses an existing issue. Eight prospective rows belong to the dedicated SIM sprint launch rather than WP-01 creation. Creation requires separate operator authority and is outside #523.

## Immediate existing work

#717 and #718 execute under v0.92.1 and are consumed here only after merge. #720 must not delay #512. Other backlog issues are not admitted by this package.
