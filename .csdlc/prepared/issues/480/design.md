# WP-01 Execution-Wave Creation Design

## Purpose

Issue #480 is the v0.92.1 opening conductor. It produces one immutable live creation receipt for the exact 45 number-free child slots defined by the merged milestone package. It does not implement child work.

## Authorities

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml`
- `docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml`
- `docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md`
- `docs/milestones/v0.92.1/WP_EXECUTION_READINESS_v0.92.1.md`
- live issue #480 and merged prerequisite #432

The merged documents are immutable input for one run. A digest change stops execution and requires review.

## Creation protocol

1. Parse exactly 45 creation-owned child IDs in canonical catalog order.
2. Materialize each issue body from its issue-level execution specification, exact title, dependency IDs, retained predecessor references, owned paths, acceptance criteria, PVF lanes, stop conditions, and non-goals. The irreversible full title is exactly `[v0.92.1][<PLANNED-ID>] <wave title>`; title fragments from the wave are never submitted directly.
3. Before each mutation, scan live milestone issues and the retained partial receipt. Reject duplicate IDs, operation keys, conflicting canonical titles, or ambiguous mappings.
4. Resolve dependencies only to verified existing issues or previously created child IDs. Never guess a number.
5. Before mutation, generate an exact routing table for all 45 IDs. Every new issue has `version:v0.92.1`, `track:roadmap`, `type:task`, milestone 1, and exactly one area label derived by a closed mapping: corporate and AWS/GCP/XCL/DRT/PROV/HOT use `area:runtime` except corporate custody/diligence use `area:security`; RUST/DEC use `area:architecture`; V3 uses `area:csdlc`; OBS uses `area:observatory`; INT and TAIL-01/06 use `area:quality`; TAIL-02/03/07/08 use `area:docs`; TAIL-04/05/09 use `area:review`; TAIL-10 uses `area:release`. Any unmapped or multiply mapped ID stops before creation.
6. Create through `csdlc-github-issue` with operation key `v0921-wp01:<planning-digest>:<planned-id>:create`. The key and request fingerprint bind repository, action, full title, exact body digest, sorted label set, milestone, and planned ID; replay is accepted only when live readback matches that fingerprint exactly.
7. Before every external mutation, create and fsync a mode-0600 intent receipt at `docs/milestones/v0.92.1/evidence/wp-01/operations/<sequence>-<id>-intent.json`. Immediately after the typed result, create and fsync the paired observed receipt binding issue number, full title, sorted labels, milestone, body/spec digest, dependencies, operation key, observed state, and live response digest. Existing receipt paths are never overwritten.
8. Recovery classifies each operation as absent, intent-only, or observed. Intent-only recovery performs live operation-key/title search before any retry; exactly one matching live issue is adopted only after full fingerprint readback, zero matches permits the same idempotent request, and multiple/conflicting matches stop for operator disposition.
9. On interruption, reconstruct live truth and resume at the first absent ID. Existing verified rows are never recreated or renumbered.
10. After the final child, perform an independent live 45-of-45 readback and write the final immutable receipt.

## Existing issue reconciliation

#51, #84, #122, #251, #261-#264, #342, and #345 are inputs and dependencies, not creation slots. WP-01 verifies their live identity without creating replacements. Where routing is stale, it uses `csdlc-github-issue` `issue_update` with operation key `v0921-wp01:<planning-digest>:existing-<number>:route`, a pre-mutation intent receipt, exact title/label/milestone request fingerprint, immediate live readback, and paired observed receipt under the same crash-recovery rules as creation.

The exact target rows are closed and replace the complete title/label/milestone routing fields:

| Issue | Exact final title | Exact sorted labels | Milestone |
|---|---|---|---|
| #84 | `[v0.92.1][Observatory] Complete live Unity Observatory Runtime v3 integration` | `area:observatory`, `track:roadmap`, `type:task`, `version:v0.92.1` | 1 / `v0.92.1` |
| #122 | `[v0.92.1][Observatory] Deploy public exposure with Route53 and ACM` (unchanged) | `area:observatory`, `track:roadmap`, `type:task`, `version:v0.92.1` | 1 / `v0.92.1` |
| #251 | `[v0.92.1][Runtime] Support TLS 1.2 on public Axum HTTPS/WSS for Unity` | `area:runtime`, `track:roadmap`, `type:bug`, `version:v0.92.1` | 1 / `v0.92.1` |
| #345 | `[v0.92.1][Sidecar] Harden and retain the AWS GPU Shepherd proof runner` | `area:runtime`, `track:roadmap`, `type:task`, `version:v0.92.1` | 1 / `v0.92.1` |

#51/#261-#264/#342 are read back and retained unchanged unless a later separately reviewed canonical contract explicitly authorizes mutation. #269 remains excluded.

`partial-receipt.json` is a deterministic atomically replaced index derived only from the create-only intent/observed operation files. It contains the ordered verified prefix, journal-root digest, next absent ID, and any intent-only recovery classification; it is a convenience/restart index, never authority over the immutable operation receipts.

## Failure and rollback

Issue creation is externally durable and cannot be rolled back by deleting or recycling issue numbers. Any mismatch stops the wave, retains the partial receipt, and requires operator-reviewed recovery. Recovery is forward-only from verified live state.

## Proof

- exact 45-ID ordered denominator and complete issue-level specification
- duplicate and conflicting-title denial before mutation
- exact dependency resolution and no unresolved numeric references
- stable operation-key replay denial
- partial-failure resume without duplicate creation
- exact labels, milestone, title, and body/spec readback
- final 45-of-45 live receipt with no missing or extra child
- independent exact-head review before publication

## Boundaries

No child implementation, no issue deletion, no number recycling, no v0.93 activation, no tag/release mutation, and no dependency on asynchronous finish or cleanup.
