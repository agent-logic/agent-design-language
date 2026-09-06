# Structured Output Record

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired the Sprint 5/6 cutover remediation branch so PR #615 visibly closes #596, preserves #505/#534 as non-closing links, restores v3 separation by removing net C-SDLC v2 source/test mutations, and records v3 replacement/canary evidence without claiming #505 cutover.

## Artifacts

- csdlc-v3/tests/real_issue_canary.rs
- docs/csdlc-v3/full-replacement-denominator.json
- .csdlc/evidence/604/full-cycle-defects-tail.md
- .csdlc/issues/596
- .csdlc/prepared/issues/596
- .csdlc/prepared/issues/596/validate-remediation-regression.sh

## Execution

- Removed the net csdlc-v2 source/test implementation delta from the remediation branch; v2 remains live authority until #505 and is not patched by this v3 cutover-remediation PR.
- Updated #596 SIP, STP, SPP, and VPP through typed C-SDLC edits so scope, outcome, deliverables, acceptance criteria, affected areas, invariants, stop conditions, and validation lanes match the v3-separate scope.
- Strengthened the issue-owned validator to fail if the PR diff contains any csdlc-v2 source/test mutation, and to consume a fresh typed PR #615 readback before accepting visible Closes #596 and non-closing issue 505/issue 534 references.
- Narrowed the real-issue v3 canaries so #596 identity is read from the real issue cards and #4646 terminal state is derived from the observed real issue phase before exercising v3 durable-storage behavior.
- Retained the v3 full replacement denominator and real-issue canary evidence as pre-cutover proof surfaces; v3 remains non-authoritative until #505 cutover.
- Kept duplicate-publication, PR #612 supersession, and the SRP prompt-edit recovery-window limitation as captured defects for the replacement/cutover lane rather than repairing them by mutating v2 source.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/596/validate-remediation-regression.sh"
    ],
    "purpose": "Prove #596 card presence, fresh typed PR #615 live readback linkage, portable owner-lane sources, and no net csdlc-v2 diff.",
    "outcome": "passed",
    "evidence_ref": "local run after live PR readback validator repair; exit 0; validator consumes fresh csdlc-github-pr state output"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "real_issue_canary"
    ],
    "purpose": "Exercise C-SDLC v3 against real issue records while preserving non-authoritative pre-cutover status and deriving identity/state from tracked issue data.",
    "outcome": "passed",
    "evidence_ref": "4 passed; 0 failed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "CI parity/no-regression check proving the PR no longer trips csdlc-v2 standalone lint while not claiming v2 implementation changes.",
    "outcome": "passed",
    "evidence_ref": "strict Clippy passed after restoring csdlc-v2 source/test files to origin/main"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run branch diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "no findings"
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

- DEFECT-019 records that v2 recovery-window card editing is brittle; v3 must provide one deterministic lifecycle-truth repair route before cutover.
