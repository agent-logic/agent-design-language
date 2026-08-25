# v0.92.2 Quality Gate

Status: planned release gate.

Beta 1 cannot enter publication finalization until all required rows below have current, retained evidence against the integrated candidate.

## Required Gates

1. Canonical planning and issue-routing validation passes.
2. Local, GitHub, and CI ingestion conform to the same portable packet contract.
3. Stable identity, provenance integrity, redaction negatives, and retention behavior pass.
4. Architecture findings trace to evidence and report confidence or unknowns.
5. Fitness functions are deterministic and distinguish machine invariants from human judgment.
6. All four review perspectives remain attributed through synthesis.
7. Remediation and test plans are bounded and do not mutate source.
8. Second-run comparison classifies added, resolved, and changed findings correctly.
9. Publication requires human approval and Markdown/HTML/PDF outputs preserve claim parity.
10. Privacy/legal checks and artifact manifests pass.
11. ADL self-review and the bounded external OSS proof are retained and independently reviewable.
12. Integrated success and failure-path demonstrations pass with no unresolved P1 finding.

## PVF Posture

Each new test is classified at authoring time by lane, proof role, determinism, resource profile, and release-gate status. Focused deterministic checks run first; paid, long, or external-resource proofs are isolated and cannot be replaced by self-declared receipts.

## Gate Result

The result is `pass`, `fail`, or `not proven`. Partial success is recorded by track and cannot be promoted to a milestone pass.
