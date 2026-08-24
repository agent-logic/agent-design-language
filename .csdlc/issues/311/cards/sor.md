# Structured Output Record

Template: 1.0.0

Issue: 311

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Executed the canonical v0.92 quality gate over 13 indexed feature contracts and 20 supporting AEE critical paths; structural validation passed, all 33 rows remain release blockers, downstream unlock is false, and the accepted-row path now fails closed on canonical GitHub, Git, typed-terminal, review, check, implementation-path, and proof-artifact authority.

## Artifacts

- .csdlc/prepared/issues/311/validate-quality-gate.rb
- .csdlc/prepared/issues/311/test-validate-quality-gate.rb
- .csdlc/evidence/311/validation.json
- docs/reviews/v0.92/quality-gate-311/feature-completion-matrix.json
- docs/reviews/v0.92/quality-gate-311/quality-gate-record.json
- docs/reviews/v0.92/quality-gate-311/blocker-report.md
- docs/milestones/v0.92/QUALITY_GATE_v0.92.md
- docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md

## Execution

- Added a deterministic denominator generator and fail-closed matrix validator for issue #311.
- Authenticated accepted rows against stable typed terminal validation, retained digest-bound cache/review/proof artifacts, exact implementation paths, Git topology, and live GitHub closing/check truth.
- Expanded the negative suite to 21 cases, including a valid canonical control and independent stale-head, non-ancestral, fabricated-check, malformed-cache, cache-digest, cross-repository, missing-platform, prohibited-authority, review-digest, and implementation-path failures.
- Generated the 33-row matrix, gate record, findings-first blocker report, and retained validation receipt.
- Updated only the approved v0.92 quality-gate and execution-readiness documents; coverage remains read-only input.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/311/validate-quality-gate.rb",
      "matrix"
    ],
    "purpose": "Validate the exact 33-row denominator and fail-closed dispositions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/311/semantic-quality-matrix.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/311/test-validate-quality-gate.rb"
    ],
    "purpose": "Prove a valid canonical accepted control and reject 21 incomplete or forged authority cases.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/311/quality-negative-suite.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove exact candidate diff hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/311/docs-schema-diff.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
