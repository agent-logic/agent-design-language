# Structured Output Record

Template: 1.0.0

Issue: 285

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

#285 reconciles ADR 0068 birthday-to-governance handoff evidence issue-locally: #5839 has terminal handoff evidence, while #5836 lacks current derived-terminal authority and remains a residual WP-18 evidence gap.

## Artifacts

- .csdlc/evidence/285/evidence-manifest.json
- .csdlc/evidence/285/live-observations.json
- .csdlc/evidence/285/adr0068-birthday-governance-handoff-reconciliation.md
- .csdlc/evidence/285/validate_adr0068_birthday_governance_handoff_evidence.sh
- .csdlc/prepared/issues/285/validate_adr0068_birthday_governance_handoff_evidence.sh
- .csdlc/issues/285

## Execution

- Added .csdlc/evidence/285/evidence-manifest.json with #5839 terminal handoff identity, #5836 non-terminal current-main retained state, residual gaps, and non-claims.
- Added .csdlc/evidence/285/live-observations.json with PR #289 merged state, current-repo #5836 absence, and retained #5836 lifecycle state.
- Added .csdlc/evidence/285/adr0068-birthday-governance-handoff-reconciliation.md summarizing terminal WP-19 evidence, non-terminal WP-18 evidence, and #207/#288 boundaries.
- Added focused issue-owned validator and prepared mirror for ADR 0068 handoff evidence reconciliation.
- Repaired #285 VPP typed validation lane to use the stable generated owner binary with worktree-relative --root .

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff whitespace checks.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/evidence/285/validate_adr0068_birthday_governance_handoff_evidence.sh"
    ],
    "purpose": "Run the #285 focused reconciliation validator.",
    "outcome": "passed",
    "evidence_ref": "focused-adr0068-birthday-governance-handoff-evidence.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "285"
    ],
    "purpose": "Run typed C-SDLC issue validation.",
    "outcome": "passed",
    "evidence_ref": "typed-issue-validation.log"
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
