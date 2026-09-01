# Structured Output Record

Template: 1.0.0

Issue: 261

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Pre-execution output record.

## Artifacts

- none

## Execution

- none

## Validation

[
  {
    "command": [
      "python3",
      "docs/milestones/v0.92/review/podcast_identity_261/validate_identity_packet.py"
    ],
    "purpose": "Prove the candidate show-identity packet schema, artwork metadata, ownership allocation, name research digest, scope allowlist, and pending external-gate truth after the working-title refresh.",
    "outcome": "passed",
    "evidence_ref": "local terminal transcript 2026-08-28: candidate mode passed with artwork_sha256 e142182ecefa06b34256d7ceeededfb3c3418c1f66e9a57750d3ed21d8d2fc8d and external gates pending."
  },
  {
    "command": [
      "python3",
      "docs/milestones/v0.92/review/podcast_identity_261/validate_identity_packet.py",
      "--redaction-only"
    ],
    "purpose": "Prove the refreshed identity packet retains no credential, token, private mailbox content, recovery code, verification code, private key material, or unbounded retained authority text.",
    "outcome": "passed",
    "evidence_ref": "local terminal transcript 2026-08-28: redaction-only mode passed for exact #261 packet scope after The Cognitive Stack refresh."
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
