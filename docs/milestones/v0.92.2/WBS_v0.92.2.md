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
| TAIL-01..10 | Canonical release tail | Quality through ceremony in standard order | CF-INTEGRATE |

## Parallelism

CF-SHELL and CF-ADAPTER can start together. After the evidence contract stabilizes, cognition, governance, review, memory, and publication can advance in parallel. Only integration and the release tail are deliberately serial.

## Work-Package Rule

WP-01 may create fewer implementation issues than one per row when ownership and proof remain coherent, but it must not create a junkyard issue spanning unrelated product boundaries.
