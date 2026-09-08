# Structured Output Record

Template: 1.0.0

Issue: 263

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Prepared The Cognitive Stack directory-submission runbooks and operator preflight for Apple Podcasts, Spotify for Creators, Amazon Music for Podcasters, and YouTube RSS ingestion without mutating provider accounts, submitting the show, or claiming public launch.

## Artifacts

- docs/milestones/v0.92.1/review/podcast_directory_263/README.md
- docs/milestones/v0.92.1/review/podcast_directory_263/operator-preflight.md
- docs/milestones/v0.92.1/review/podcast_directory_263/provider-runbooks.md
- docs/milestones/v0.92.1/review/podcast_directory_263/submission-ledger.schema.json
- .csdlc/prepared/issues/263/bootstrap-request.json
- .csdlc/prepared/issues/263/design.md
- .csdlc/prepared/issues/263/diagram.mmd
- .csdlc/prepared/issues/263/design-approval.json
- .csdlc/prepared/issues/263/advance-ready-after-working-name.json
- .csdlc/prepared/issues/263/readiness-affected-edit.json
- .csdlc/prepared/issues/263/readiness-lanes-edit.json
- .csdlc/prepared/issues/263/validate-directory-runbooks.rb
- .csdlc/prepared/issues/263/working-name-assumption.json

## Execution

- Added a bounded directory-submission packet for Issue #263 under docs/milestones/v0.92.1/review/podcast_directory_263.
- Documented operator-only account, ownership, verification, irreversible submission, and ledger steps for Apple Podcasts, Spotify for Creators, Amazon Music for Podcasters, and YouTube RSS ingestion.
- Added a redaction-safe operator preflight that names required account-side decisions without collecting credentials, mailbox codes, platform account data, or private receipts.
- Added a submission-ledger JSON schema for #264 handoff so future submission evidence can be recorded without retaining secrets or verification material.
- Added a deterministic validator covering current official-source sampling date, The Cognitive Stack working name, production feed identity, provider census, non-submission boundaries, and ledger-schema shape.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/263/validate-directory-runbooks.rb"
    ],
    "purpose": "Validate The Cognitive Stack provider runbooks, production feed identity, official-source sampling date, non-submission boundary, and redacted ledger handoff shape.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/263/directory-runbooks-validation.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Reject whitespace errors and conflict-marker residue across the exact reviewable #263 candidate diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/263/diff-check.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-finish",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-263-podcast-directory-runbooks-exec",
      "--validate-cached-issue",
      "261"
    ],
    "purpose": "Prove #263 executed after canonical terminal truth for podcast identity issue #261.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/263/dependency-261-terminal-validation.json"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-finish",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-263-podcast-directory-runbooks-exec",
      "--validate-cached-issue",
      "262"
    ],
    "purpose": "Prove #263 executed after canonical terminal truth for podcast hosting issue #262.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/263/dependency-262-terminal-validation.json"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
