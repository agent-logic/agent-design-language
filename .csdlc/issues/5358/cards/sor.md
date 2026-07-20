# Structured Output Record

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

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
      "csdlc-validate --request .csdlc/prepared/issues/5358/validation-request.json",
      "csdlc-doctor --repo . --issue 5358",
      "git diff --check"
    ],
    "purpose": "Validate the typed #5358 six-card bundle, retained design integrity, bound issue state, and issue-local patch hygiene only.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5358/preparation-validation"
  },
  {
    "command": [
      "csdlc-validate --request .csdlc/prepared/issues/5358/validation-request.json",
      "csdlc-doctor --repo . --issue 5358",
      "git status --short -- .csdlc/issues/5358 .csdlc/prepared/issues/5358 .csdlc/evidence/5358"
    ],
    "purpose": "Validate typed record/card integrity and explicitly enumerate the full untracked #5358 review scope. This replaces the earlier git diff --check evidence, which did not cover untracked artifacts.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5358/preparation-validation"
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
