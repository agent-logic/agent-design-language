# #516 Release-tail admission with folded gap analysis

## Goal

Decide whether the converged v0.92.1 candidate may enter the release tail by
proving that the complete planned denominator is reviewed, merged, ancestral,
implemented on real production paths, and free of unresolved material gaps.

## Design

The issue produces one admission packet rather than a second review ceremony.
It derives the expected denominator from the canonical v0.92.1 issue catalog,
execution specifications, live issue acceptance criteria, and retained
predecessor dependencies. It compares that baseline with merged code, runtime
call paths, tests, review records, documentation, PR state, and terminal
receipts.

Each mismatch is recorded with severity, evidence, uncertainty, disposition,
and an existing owner. Findings are classified as release blockers, durable
proof gaps, routed work, stale release-readiness documentation, or non-blocking
quality concerns. Absence of evidence is never converted into a passing claim.

The final admission record indexes exact revisions and artifacts and fails
closed when the denominator is incomplete, ancestry disagrees, a required
behavior is a stub or test-only path, a P0/P1 gap remains unresolved, or a
material gap has no owner.

## Outputs

- `docs/milestones/v0.92.1/evidence/integration/release-tail-admission.json`
- `docs/milestones/v0.92.1/evidence/integration/gap_analysis_report.json`
- `docs/milestones/v0.92.1/evidence/integration/gap_analysis_report.md`
- updated `docs/milestones/v0.92.1/DEMO_MATRIX_v0.92.1.md` when observed demo
  evidence differs from the current matrix

## Boundary

#516 identifies, classifies, and routes gaps. It does not implement child
repairs, create duplicate issues, approve the release, or turn missing evidence
into success.

