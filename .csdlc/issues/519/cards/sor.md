# Structured Output Record

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Final publication-candidate packet bound to merged #518 / PR753: immutable reviewed source, published head and merged content agree. Preserve 791-document handoff, artifact hashes, closing linkage and explicit non-release boundary. Final independent review follows.

## Artifacts

- docs/milestones/v0.92.1/evidence/release/tail-03/candidate.json
- docs/milestones/v0.92.1/evidence/release/tail-03/README.md
- docs/milestones/v0.92.1/evidence/release/tail-03/source-pr.json
- .csdlc/prepared/issues/519/validate-publication-candidate.rb
- docs/milestones/v0.92.1/evidence/release/tail-03/candidate.json
- docs/milestones/v0.92.1/evidence/release/tail-03/README.md
- docs/milestones/v0.92.1/evidence/release/tail-03/source-pr.json
- .csdlc/prepared/issues/519/validate-publication-candidate.rb
- docs/milestones/v0.92.1/evidence/release/tail-03/candidate.json
- docs/milestones/v0.92.1/evidence/release/tail-03/README.md
- docs/milestones/v0.92.1/evidence/release/tail-03/source-pr.json
- .csdlc/prepared/issues/519/validate-publication-candidate.rb
- docs/milestones/v0.92.1/evidence/release/tail-03/candidate.json
- docs/milestones/v0.92.1/evidence/release/tail-03/README.md
- docs/milestones/v0.92.1/evidence/release/tail-03/source-pr.json
- .csdlc/prepared/issues/519/validate-publication-candidate.rb
- docs/milestones/v0.92.1/evidence/release/tail-03/README.md
- docs/milestones/v0.92.1/evidence/release/tail-03/candidate.json
- docs/milestones/v0.92.1/evidence/release/tail-03/source-pr.json
- .csdlc/prepared/issues/519/validate-publication-candidate.rb

## Execution

- Created provisional candidate binding reviewed #518 source, 13 source artifacts and 775 document hashes.
- Implemented preparation and final validation modes, source review/linkage checks and complete manifest-referenced redaction screening.
- Preparation self-test passed three negative fixtures; final mode intentionally blocks while source PR is open.
- Independent preparation review requested; final typed review and publication remain pending.
- Operator-authorized early start and bounded typed v2 authorization recorded.
- Created immutable source candidate and explicit #518/#519 closing relationships.
- Implemented hash, review-scope, linkage, redaction and merge-content checks.
- Independent preparation review passed; no merge, tag, release or issue-close mutation.
- Updated immutable source hashes and saved live PR observation.
- Distinguished historical quality exceptions from completed accounting dispositions.
- Original preparation validator independently reviewed; refreshed content requires final review after predecessor merge.
- Updated immutable source hashes and saved live PR observation.
- Distinguished historical quality exceptions from completed accounting dispositions.
- Original preparation validator independently reviewed; refreshed content requires final review after predecessor merge.
- Freeze final #518 source and authenticated live PR read observation after its merge.
- Verify all document/artifact hashes, reviewed scope and merge content on canonical main.
- Enforce exact closing relationships and publication-packet plus manifest-document redaction; reject corrupt and premature acceptance fixtures.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/519/validate-publication-candidate.rb",
      "--redaction"
    ],
    "purpose": "Prove credentials, private payloads, and machine-local paths are absent.",
    "outcome": "passed",
    "evidence_ref": "artifact-redaction.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove exact packet diff hygiene before review.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/519/validate-publication-candidate.rb",
      "--exact-head"
    ],
    "purpose": "Prove the packet and artifact digests bind the exact reviewed candidate revision.",
    "outcome": "passed",
    "evidence_ref": "exact-head.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/519/validate-publication-candidate.rb",
      "--self-test"
    ],
    "purpose": "Reject corrupt hashes, closing linkage, repository/head/merge, premature acceptance and sensitive text.",
    "outcome": "passed",
    "evidence_ref": "negative-fixtures.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/519/validate-publication-candidate.rb",
      "--linkage"
    ],
    "purpose": "Prove exact and unambiguous publication and closing relationships.",
    "outcome": "passed",
    "evidence_ref": "publication-linkage.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
