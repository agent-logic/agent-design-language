# Structured Output Record

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired the Sprint 5/6 cutover remediation branch after merged PR #597 left issue #596 open and after real-issue testing exposed missing local lifecycle adoption and branch-portability defects.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v3/tests/real_issue_canary.rs
- docs/csdlc-v3/full-replacement-denominator.json
- .csdlc/evidence/604/full-cycle-defects-tail.md
- .csdlc/issues/596
- .csdlc/prepared/issues/596

## Execution

- Added a safe typed C-SDLC v2 path to reapprove ready issue design references only when the SPP and VPP authored design and diagram refs match the canonical record paths and only the content digests changed.
- Added a safe typed bind-adoption path for an already-advanced ready issue running in its exact current FastWork remediation branch/worktree, rejecting topology mismatch and later lifecycle evidence.
- Fixed terminal materialization after duplicate publication so a historical merged terminal PR can supersede stale open publication state without accepting caller-forged terminal or issue identity.
- Brought the v3 full replacement denominator and real-issue canaries onto the remediation branch, proving v3 remains incomplete for cutover while exercising current #596 and terminal #4646 records.
- Bound issue #596 through typed lifecycle state instead of leaving the PR-closing/follow-up path detached from local C-SDLC authority.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run exact branch diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/596/validate-remediation-regression.sh"
    ],
    "purpose": "Run the issue-owned remediation validator.",
    "outcome": "passed",
    "evidence_ref": "issue-596-remediation-regression.log"
  },
  {
    "command": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "issue",
      "--issue",
      "596"
    ],
    "purpose": "Validate the current #596 C-SDLC record.",
    "outcome": "passed",
    "evidence_ref": "issue-596-typed-validation.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "prebind_contract_repair_is_exact_atomic_and_fail_closed",
      "--test",
      "gate2"
    ],
    "purpose": "Run the focused gate2 prebind repair regression.",
    "outcome": "passed",
    "evidence_ref": "v2-prebind-contract-regression.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "ready_unbound",
      "--lib"
    ],
    "purpose": "Run focused C-SDLC v2 ready and bind adoption regressions.",
    "outcome": "passed",
    "evidence_ref": "v2-ready-bind-regression.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "terminal_materialization_policy_tests",
      "--lib"
    ],
    "purpose": "Run focused C-SDLC v2 terminal materialization unit tests.",
    "outcome": "passed",
    "evidence_ref": "v2-terminal-materialization-regression.log"
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
    "purpose": "Run C-SDLC v3 real issue canaries.",
    "outcome": "passed",
    "evidence_ref": "v3-real-issue-canaries.log"
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
