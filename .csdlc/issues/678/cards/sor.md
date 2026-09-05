# Structured Output Record

Template: 1.0.0

Issue: 678

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Implemented installer-managed stable CSM routing so .adl/bin/csm execs the active Runtime v3 generation's CSM instead of remaining an independent stale binary.

## Artifacts

- adl/tools/runtime_v3_generation.py
- adl/tools/install_runtime_v3_generation.sh
- adl/tools/test_runtime_v3_generation_install.sh
- .csdlc/prepared/issues/678/validate-stable-csm-route.sh
- .adl/docs/TBD/resilience/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md

## Execution

- Added atomic stable CSM launcher installation in adl/tools/runtime_v3_generation.py before generation activation and rollback symlink switches.
- Strengthened the Runtime v3 generation installer fixture to prove stable route parity with current/bin/csm, stale stable binary repair, rollback route switching, and missing active-generation CSM failure.
- Updated the issue-owned validation wrapper to retain the focused proof log under .csdlc/evidence/678.
- Added the hidden operator note for the stable CSM route contract under .adl/docs/TBD/resilience/.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/678/validate-stable-csm-route.sh"
    ],
    "purpose": "Runtime v3 generation installer stable CSM route fixture",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/678/runtime-v3-generation-install.log; runtime v3 generation installer: PASS"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Git diff hygiene proof",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/678/diff-check.log; clean"
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
