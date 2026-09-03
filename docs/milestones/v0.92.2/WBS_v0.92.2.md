# v0.92.2 Work Breakdown Structure

Status: planned. IDs are number-free planning identifiers; no GitHub issues are created by this document.

| WP | Work track | Primary outcome | Depends on |
|---|---|---|---|
| WP-01 | Milestone opening | Validate package and create the authorized execution wave | none |
| CF-SHELL | Product shell | Setup, onboarding, run controls, status, artifact browser | WP-01 |
| CF-ADAPTER | Adapter v2 | Portable local/GitHub/CI repository ingestion | WP-01 |
| CF-EVIDENCE | Evidence core | Stable identity, provenance, redaction, retention | CF-ADAPTER |
| CF-COG | Architecture cognition | Dependency, boundary, coupling, drift, blast-radius, quanta, ADR analysis | CF-EVIDENCE |
| CF-GOV | Executable governance | Fitness functions and CI integration | CF-EVIDENCE |
| CF-REVIEW | Review engine | Four perspectives, synthesis, remediation, test planning | CF-EVIDENCE |
| CF-MEMORY | Longitudinal memory | Second-run comparison and compatibility handling | CF-EVIDENCE |
| CF-UX | Publication and outputs | Human approval, claims/non-claims, manifests, Markdown/HTML/PDF | CF-SHELL, CF-EVIDENCE |
| CF-PROOF | Docs, fixtures, and proof repos | Examples, fixtures, ADL self-review, external OSS proof | CF-COG, CF-GOV, CF-REVIEW, CF-MEMORY, CF-UX |
| CF-INTEGRATE | Beta 1 integration | One coherent product path and acceptance packet | all CF tracks |
| PLAT-PROVIDER | Provider configuration | Separate provider behavior from editable endpoint/profile data | WP-01; merged v0.92.1 issue #622 |
| PLAT-MLX | MLX/Metal provider | Add one bounded Apple MLX/Metal adapter over the canonical provider-definition contract | PLAT-PROVIDER |
| PLAT-UTS | UTS productization | Standardize and package the UTS contract for supported consumers | WP-01 |
| PLAT-RUST | Rust reduction | One measured behavior-preserving refactoring slice | WP-01 |
| OPS-AWS | AWS inventory maintenance | Refresh SCR, S3, model, and staleness deltas from completed #484 ownership-inventory authority | WP-01; completed #484 baseline |
| PUB-MEDIUM | Medium preparation | Prepare one v0.92.2 Medium article packet without publishing | WP-01 |
| PUB-CSDLC | C-SDLC paper preparation | Advance one C-SDLC paper packet without submission | WP-01 |
| PLAT-MEMORY | Memory Palace integration | Deliver the next bounded production Runtime/CodeFriend Memory Palace slice | CF-EVIDENCE, CF-MEMORY |
| SPEC-RETEST | Speculative decoding requalification | Retest the retained prototype and issue an evidence-backed keep, repair, or retire decision | WP-01 |
| TAIL-01..10 | Canonical release tail | Quality through ceremony in standard order | CF-INTEGRATE |

## Parallelism

CF-SHELL, CF-ADAPTER, PLAT-UTS, PLAT-RUST, OPS-AWS, PUB-MEDIUM, PUB-CSDLC, and SPEC-RETEST can start independently after WP-01. PLAT-PROVIDER also requires merged v0.92.1 issue #622; PLAT-MLX follows PLAT-PROVIDER. After the evidence contract stabilizes, cognition, governance, review, Memory Palace integration, and publication can advance in parallel. Only integration and the release tail are deliberately serial.

## Work-Package Rule

WP-01 creates exactly one bounded issue per row. Each issue owns one concrete result and its proof; rows are never combined merely to reduce issue count.
