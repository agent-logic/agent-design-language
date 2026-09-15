# Structured Output Record

Template: 1.0.0

Issue: 3

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Hardened canonical C-SDLC v2 publication for preserved legacy issue authority and retained a causal split-authority closure canary.

## Artifacts

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-finish/SKILL.md
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
- .csdlc/prepared/issues/3/split-authority-canary.json
- .csdlc/prepared/issues/3/validate-split-authority-canary.rb

## Execution

- Verified every configured fetch and effective push URL for the publication remote against the canonical code repository before pushing.
- Reconciled all pages of matching pull requests and rejected ambiguous publication identity.
- Added focused remote-substitution and ambiguity regression tests while preserving same-repository compatibility.
- Documented split issue and code repository authority for typed publication and finish.
- Retained GitHub timeline evidence proving canonical PR #4 causally closed legacy issue #5901, while identifying later PR #5 as non-causal reconciliation evidence.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--target-dir",
      "../adl-builds/csdlc-v2-issue-3",
      "--bin",
      "csdlc-publish",
      "--test",
      "gate5",
      "--test",
      "gate6",
      "--test",
      "gate_finish"
    ],
    "purpose": "Run the focused C-SDLC v2 publication and finish regression surfaces.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/3/csdlc-publication-focused.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/3/validate-split-authority-canary.rb"
    ],
    "purpose": "Validate canonical PR #4 causal closure, later PR #5 non-causality, and preserved legacy issue #5901 against retained GitHub evidence.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/3/canary/split-authority-causal-canary.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--target-dir",
      "../adl-builds/csdlc-v2-issue-3",
      "--bin",
      "csdlc-publish",
      "--tests",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the changed publication binary and its tests remain warning-free.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/3/clippy/csdlc-publication-clippy.log"
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
