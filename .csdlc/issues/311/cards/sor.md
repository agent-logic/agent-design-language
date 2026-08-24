# Structured Output Record

Template: 1.0.0

Issue: 311

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Executed the canonical v0.92 quality gate over 13 indexed feature contracts and 20 supporting critical paths. Structural and authority validation pass, all 33 rows remain truthful release blockers, downstream unlock is false, and accepted-row validation now consumes stable typed authority, reviewed Git blobs, live GitHub closing/check truth, and the active main-protection ruleset.

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

- Removed caller-controlled GitHub and C-SDLC executable hooks from the production acceptance path; stable C-SDLC owners derive from the Git common directory and GitHub observations use the authenticated API directly.
- Derived canonical required checks from the live active main-protection ruleset instead of an implementation-owned or row-selected list.
- Required review authority from the exact typed issue index and required proof blobs to be committed at the PR head, retained unchanged, included in review scope, and bound to passed typed SOR validation lanes.
- Added atomic cross-validation of the matrix, gate record, validation receipt, lane logs, artifact digests, blocker report, counts, result, and downstream unlock semantics.
- Expanded the fail-closed suite to 36 cases around a real canonical #451/PR #459 control, including authority substitution, live closing/check/ruleset failures, typed-terminal mutations, proof/review mutations, and exact Git topology.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/311/validate-quality-gate.rb",
      "matrix"
    ],
    "purpose": "Validate the exact 33-row denominator plus the complete retained packet atomically.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/311/semantic-quality-matrix.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/311/test-validate-quality-gate.rb"
    ],
    "purpose": "Run the real canonical accepted control and reject 36 incomplete, stale, forged, or authority-substituted cases.",
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
