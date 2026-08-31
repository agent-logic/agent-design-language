# Structured Output Record

Template: 1.0.0

Issue: 570

Repository: agent-logic/agent-design-language

Card: sor

Status: ready

## Summary

Repaired PR #584 documentation findings by replacing instructional legacy architecture lifecycle text with typed v2 lifecycle guidance and strengthening stale-route validation.

## Artifacts

- docs/architecture/ADL_ARCHITECTURE.md
- .csdlc/prepared/issues/570/validate-docs-routes.sh
- .csdlc/issues/570

## Execution

- Updated docs/architecture/ADL_ARCHITECTURE.md so the active task-bundle lifecycle uses typed C-SDLC v2 bind, review/publish, finish, and cleanup instead of pr run, pr finish, and pr closeout.
- Corrected the canonical architecture six-card invariant to include VPP: SIP, STP, SPP, VPP, SRP, and SOR.
- Strengthened validate-docs-routes.sh to reject instructional list-item legacy routes even when the same line lacks words such as current, default, or use.
- Retained historical legacy-route mentions only where clearly marked as retired/non-executable evidence.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/570/validate-docs-routes.sh"
    ],
    "purpose": "Docs route scan including instructional legacy-route rejection and VPP lifecycle invariant.",
    "outcome": "passed",
    "evidence_ref": "local stdout: docs route scan passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/570/validate-skill-guidance.sh"
    ],
    "purpose": "Skill guidance scan after stale-route repair.",
    "outcome": "passed",
    "evidence_ref": "local stdout: skill guidance scan passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/570/validate-authority-boundary.sh"
    ],
    "purpose": "Authority boundary scan preserving v2-live/v3-construction split.",
    "outcome": "passed",
    "evidence_ref": "local stdout: authority boundary scan passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Exact-range diff hygiene for PR #584 remediation.",
    "outcome": "passed",
    "evidence_ref": "local command produced no findings"
  },
  {
    "command": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "570"
    ],
    "purpose": "Typed C-SDLC issue validation after recovery and doc repairs.",
    "outcome": "passed",
    "evidence_ref": "generation 23 phase implemented status pass before this SOR replacement"
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
