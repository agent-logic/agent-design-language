# Structured Output Record

Template: 1.0.0

Issue: 100

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recovered and verified the complete 34-file Medium launch package in the approved Agent Logic company Drive folder: 10 canonical articles, 10 editorial reviews, 10 source packets, and 4 series records. The original issue folder is obsolete and inaccessible to the company credential. Three title-only remote corrections were explicitly operator-authorized for What is ADL?; there were no other overwrites, no deletions, and no sharing changes.

## Artifacts

- .csdlc/evidence/100/recovery-manifest.json
- .csdlc/evidence/100/recovery-report.md
- .csdlc/evidence/100/company-drive-upload-receipt.json
- .csdlc/evidence/100/company-drive-support-upload-receipt.json
- .csdlc/evidence/100/company-drive-title-correction-receipt.json
- .csdlc/prepared/issues/100/validate-recovery.rb

## Execution

- Corrected the article 1, editorial review, and source packet headings to the exact operator-approved title What is ADL?.
- Recovered and verified 34 publication files under the approved company credential and folder 1hCVwqDLetD9Q8tWEDB8e3nTYzvI1Q-rd.
- Retained exact source provenance, content digests, Drive URLs, destination reconciliation, and remote mutation totals.
- Added a focused fail-closed validator for source-at-revision provenance, package completeness, Drive receipts, and title-correction authority.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/100/validate-recovery.rb"
    ],
    "purpose": "Prove exact source provenance, the 34-file package, approved company destination, Drive readbacks, and three authorized title-only corrections.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/100/recovery-manifest.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify whitespace and patch hygiene for the bounded issue diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/100/recovery-report.md"
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
