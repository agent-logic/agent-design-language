# Structured Output Record

Template: 1.0.0

Issue: 47

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented typed exact, intentional-broad, and invalid Rust test selector classification so misleading free substrings before or after the libtest separator fail before execution while explicit Cargo target boundaries, Cargo toolchain and global-option prefixes, and truthful broad lanes remain supported.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/tests/validation_selectors.rs
- csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md

## Execution

- Added strum-backed RustTestSelectorPosture classification and actionable fail-closed validation at the typed VPP lane boundary.
- Recognized --doc and cargo +toolchain test target routes while rejecting post-separator substring fan-out without a Cargo target boundary.
- Parsed supported Cargo global options before the test subcommand and rejected ambiguous pre-subcommand shapes instead of bypassing validation.
- Added an issue-owned regression target proving exact, broad, missing-name, conflicting-selector, nonzero schema, and unrelated-integration exclusion behavior.
- Updated active csdlc-v2 validation skill guidance with exact target examples and the intentional broad-lane distinction.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "validation_lane_rejects_free_rust_test_substring_before_execution"
    ],
    "purpose": "Prove the typed VPP validation boundary rejects a misleading free substring while accepting exact and intentional broad routes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/issues/47/cards/sor.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "validation_selectors"
    ],
    "purpose": "Prove selector classification and exact nonzero schema selection without launching estimation_contracts.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2/tests/validation_selectors.rs"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "schema::tests"
    ],
    "purpose": "Prove the exact schema library lane selects its three intended unit tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/issues/47/cards/sor.md"
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
    "purpose": "Prove all C-SDLC v2 targets remain warning-free after the selector change.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/issues/47/cards/sor.md"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--",
      "--check"
    ],
    "purpose": "Prove Rust formatting on the changed crate.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/issues/47/cards/sor.md"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace errors in the bounded issue diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/issues/47/cards/sor.md"
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
