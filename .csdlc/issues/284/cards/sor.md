# Structured Output Record

Template: 1.0.0

Issue: 284

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

#284 records issue-local ADR 0066 Guardian authority evidence reconciliation from retained #5878 and #194 inputs, including live-vs-retained legacy PR #140 drift, residual gaps, and non-claims; it does not claim #142 completion, ADR acceptance, #207 closeout, or #288 serialization.

## Artifacts

- .csdlc/evidence/284/evidence-manifest.json
- .csdlc/evidence/284/live-observations.json
- .csdlc/evidence/284/adr0066-guardian-authority-reconciliation.md
- .csdlc/evidence/284/validate_adr0066_guardian_authority_evidence.sh
- .csdlc/prepared/issues/284/validate_adr0066_guardian_authority_evidence.sh
- .csdlc/issues/284

## Execution

- Added .csdlc/evidence/284/evidence-manifest.json with retained #5878 terminal cache, #5878 execution proof, #194 private qualification summary, #194 preflight input hashes, classifications, residual gaps, and non-claims.
- Added .csdlc/evidence/284/live-observations.json with current #142/#194/#397 and legacy PR #140 observations plus retained #5878 terminal-cache identity.
- Added .csdlc/evidence/284/adr0066-guardian-authority-reconciliation.md with issue-local ADR 0066 evidence classification, residual gaps, #142/#207/#288 non-claims, and live-vs-retained PR #140 drift truth.
- Repaired .csdlc/evidence/284/validate_adr0066_guardian_authority_evidence.sh and the prepared mirror so linked-worktree terminal-cache lookup uses the canonical Git-common cache and validates the current retained-terminal schema.
- Repaired #284 VPP lane truth so the focused validator is issue-owned and typed validation uses the stable C-SDLC v2 owner binary with explicit worktree root.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run Git diff whitespace hygiene.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/evidence/284/validate_adr0066_guardian_authority_evidence.sh"
    ],
    "purpose": "Run the #284 focused reconciliation validator.",
    "outcome": "passed",
    "evidence_ref": "focused-adr0066-guardian-authority-evidence.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-284-adr0066-guardian-authority-evidence-reconciliation",
      "issue",
      "--issue",
      "284"
    ],
    "purpose": "Run typed C-SDLC v2 issue validation.",
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
