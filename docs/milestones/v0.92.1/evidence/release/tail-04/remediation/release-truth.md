## Outcome

Refresh exact-candidate release truth and repair malformed or mislabeled retained evidence without rewriting immutable historical claims.

## Parent and findings

- Parent remediation issue: #522
- Source review: #520 at `fb6cbc7f619daa54f901fd2d12f480add682ace3`
- Findings: `D520-V3F-001`, `D520-REL-001`, `D520-DOC-003`, `D520-DOC-004`, `D520-EVID-001`, `D520-EVID-002`

## Acceptance criteria

- V3-F exact-head mapping and locked suite are re-proved at the actual remediation candidate.
- The canonical current-status projection is regenerated from live issue/PR truth and candidate bytes; its validator passes from a clean checkout.
- #519 terminal truth is reconciled through the authorized terminal projection/receipt path without rewriting immutable post-merge card history.
- All fourteen malformed ownership links are corrected and a negative fixture rejects link-tail corruption.
- GCP-E `.json` readbacks are emitted as JSON.
- Empty failed-command artifacts become structured failure envelopes or are truthfully renamed and dispositioned; no empty `.json` remains in the reviewed set.

## Owned paths

- `docs/milestones/v0.92.1/evidence/release/current-status/**`
- `docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/**`
- `docs/milestones/v0.92.1/FEATURE_PROOF_COVERAGE_v0.92.1.md`
- `docs/milestones/v0.92.1/evidence/cloud/gcp-e/**`
- narrowly required terminal projection and retained evidence manifests

## Validation

- V3-F current validator and locked suite.
- Current-status validator against the exact candidate.
- Link-corruption negative fixture.
- JSON parse validation across every affected retained artifact.

## Non-goals

- Claiming that the separate 198-row retained-product-proof denominator is closed.
- Re-running paid cloud workloads.
