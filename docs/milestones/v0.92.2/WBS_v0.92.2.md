# v0.92.2 Work Breakdown Structure

Status: planned. Uncreated rows use number-free planning identifiers; #720 is the sole reused existing issue. No GitHub issues are created by this document.

| WP | Work track | Primary outcome | Depends on |
|---|---|---|---|
| WP-01 | Milestone opening | Validate package and create the authorized execution wave | none |
| CF-SHELL | Product shell | One operable CodeFriend shell | WP-01 |
| CF-ADAPTER | Adapter v2 | Portable local/GitHub/CI repository ingestion | WP-01 |
| CF-EVIDENCE | Evidence core | Stable identity, provenance, redaction, retention, shared finding/run contract and consumer fixtures | CF-ADAPTER |
| CF-COG | Architecture cognition | Dependency, boundary, coupling, drift, blast-radius, quanta, ADR analysis | CF-EVIDENCE |
| CF-GOV | Executable governance | Fitness functions and CI integration | CF-EVIDENCE |
| CF-REVIEW | Review engine | One complete multi-perspective review packet | CF-EVIDENCE |
| CF-MEMORY | Longitudinal memory | Second-run comparison and compatibility handling | CF-EVIDENCE |
| CF-UX | Publication and outputs | One governed publication bundle | CF-SHELL, CF-EVIDENCE |
| CF-PROOF | Qualification evidence | One Beta 1 qualification evidence packet | CF-COG, CF-GOV, CF-REVIEW, CF-MEMORY, CF-UX |
| CF-INTEGRATE | Beta 1 integration | One coherent product path and acceptance packet | all other CF tracks, PLAT-PROVIDER, PLAT-MEMORY |
| PLAT-PROVIDER | Provider configuration | Separate provider behavior from editable endpoint/profile data | WP-01; merged v0.92.1 issue #622 |
| PLAT-MLX | MLX/Metal provider | Add one bounded Apple MLX/Metal adapter over the canonical provider-definition contract | PLAT-PROVIDER |
| PLAT-PAIR | NVIDIA PAIR experiment | Determine whether PAIR improves bounded multi-node local inference without becoming a provider type | PLAT-PROVIDER |
| PLAT-UTS | UTS productization | Standardize and package the UTS contract for supported consumers | WP-01 |
| PLAT-RUST | Rust reduction | One measured behavior-preserving refactoring slice | WP-01 |
| OPS-AWS | AWS inventory maintenance | Refresh SCR, S3, model, and staleness deltas from completed #484 ownership-inventory authority | WP-01; completed #484 baseline |
| OPS-GCP | GCP move-in planning | Produce one apply-ready company GCP move-in execution packet | WP-01; merged v0.92.1 GCP foundations |
| PUB-MEDIUM | Medium preparation | Prepare one v0.92.2 Medium article packet without publishing | WP-01 |
| PUB-CSDLC | C-SDLC paper preparation | Advance one C-SDLC paper packet without submission | WP-01 |
| PLAT-MEMORY | Memory Palace integration | Deliver the next bounded production Runtime/CodeFriend Memory Palace slice | CF-EVIDENCE, CF-MEMORY |
| SPEC-RETEST | Speculative decoding requalification | Retest the retained prototype and issue an evidence-backed keep, repair, or retire decision | WP-01 |
| OBS-LIVE (#720) | Existing Observatory issue | Remove retained-mode hazards from live product | Existing live UI; no WP-01 wait |
| OBS-S3 | Observatory deployment sidecar | Apply the existing #679 S3/CloudFront Terraform and verify the deployed Observatory | WP-01, OBS-LIVE; completed #679 / merged PR #685 |
| ARCH-ADR | Milestone architecture decisions | Generate and reconcile the source-grounded ADR set required by v0.92.2 | WP-01 |
| SIM-UMBRELLA | C-SDLC first sprint | Produce one sprint scorecard | Completes after SIM-09; coordination opens before SIM-01 |
| SIM-01 | C-SDLC first sprint | Make diagnostics observably read-only | Own readiness; parallel Runtime; no WP-01 wait |
| SIM-02 | C-SDLC first sprint | One current installed command contract | SIM-01 |
| SIM-03 | C-SDLC first sprint | Deliver one intent-oriented CLI | SIM-02 |
| SIM-04 | C-SDLC first sprint | One semantic issue transaction owner | SIM-03 |
| SIM-05 | C-SDLC first sprint | Derived cards and precise evidence invalidation | SIM-04 |
| SIM-06 | C-SDLC first sprint | Deliver one safe conversion rehearsal | SIM-05 |
| SIM-07 | C-SDLC first sprint | Produce one independent transition qualification | SIM-06 |
| SIM-08 | C-SDLC first sprint | Produce one transition operations packet | SIM-07 |
| SIM-09 | C-SDLC first sprint | Run one authorized consecutive-issue pilot | SIM-08; separate activation authority |
| TAIL-01..10 | Canonical release tail | Quality through ceremony in standard order | TAIL-01 converges integration plus the declared release-gating support set; OBS-S3 and ARCH-ADR are excluded |

## Parallelism

CF-SHELL, CF-ADAPTER, PLAT-UTS, PLAT-RUST, OPS-AWS, OPS-GCP, PUB-MEDIUM, PUB-CSDLC, SPEC-RETEST, and ARCH-ADR can start independently after WP-01. PLAT-PROVIDER also requires merged v0.92.1 issue #622; PLAT-MLX and PLAT-PAIR follow PLAT-PROVIDER. OBS-S3 starts after WP-01 creates its issue and OBS-LIVE is complete, consuming the completed #679/PR #685 deployment design rather than redesigning it. After the evidence contract merges, cognition, governance, review and publication can advance in parallel; Memory Palace integration additionally requires CF-MEMORY. The SIM sprint has its own serial implementation order, beginning first alongside Runtime; product integration and the release tail retain their stated convergence gates.

## Work-Package Rule

For the three broadest product rows, cohesion is explicit: CF-EVIDENCE owns one admissible-evidence contract whose identity, provenance, redaction, retention, and consumer clauses cannot ship independently; CF-COG owns one reporter whose analysis facets share one evidence-to-explanation contract; CF-PROOF owns one release-gate packet whose documentation and two repository runs are required evidence sections, not separately accepted products. SIM-08 and SIM-09 are separate because operations preparation and live pilot exposure remain independently valuable and separately authorized.

The milestone inventory reconciles one bounded issue per expanded row. Every row must produce one named primary result. Supporting code, documentation, fixtures, and tests may travel with that result only when they are necessary to implement or prove it; independently useful results require separate rows before issue creation. WP-01 opens the CodeFriend wave; SIM-UMBRELLA coordinates its independently launched ten-issue sprint. Reuse #720; consume #717 and #718 as v0.92.1 predecessors; do not recreate completed #620 or predecessor #439. Resolve the separate WP-01 conductor first; it cannot create itself. The denominator is 45 rows: the prior 43-row inventory plus `OBS-S3` and `ARCH-ADR`. With WP-01 resolved, 43 rows remain prospective creations and one reuses an existing issue. Ten prospective rows belong to the dedicated SIM sprint launch rather than WP-01 creation. Creation requires separate operator authority and is outside #525.

## Immediate existing work

#717 and #718 are closed merged v0.92.1 predecessors and are consumed here as completed inputs. #720 remains independently admitted under its existing authority. Other backlog issues are not admitted by this package.
