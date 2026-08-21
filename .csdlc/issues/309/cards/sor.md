# Structured Output Record

Template: 1.0.0

Issue: 309

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remove two superseded evaluation-only Rust modules after a complete 485-file disposition census and exact per-band rollback rehearsal.

## Artifacts

- adl/src/lib.rs
- .csdlc/evidence/309/baseline-manifest.json
- .csdlc/evidence/309/reference-edge-manifest.json
- .csdlc/evidence/309/disposition-manifest.json
- .csdlc/evidence/309/reduction-report.json
- .csdlc/evidence/309/rollback-proof.json

## Execution

- Captured the immutable 485-file 265633-line baseline and 1742 active reference edges
- Removed the retired DSpark and provider-native comparison evaluation modules plus their public exports after #5347 had already retired their executable demos
- Proved Band A restores and reapplies exact Git trees without unrelated path changes

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "e926e3bca0ab1981d77b4658d2feb4059bdf33a6...HEAD"
    ],
    "purpose": "Diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "diff_hygiene.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/309/validate_reduction_inventory.py"
    ],
    "purpose": "Reduction inventory proof.",
    "outcome": "passed",
    "evidence_ref": "reduction_inventory.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "resident_shepherd_spot_continuity",
      "--",
      "--nocapture"
    ],
    "purpose": "ADL continuity proof.",
    "outcome": "passed",
    "evidence_ref": "resident_continuity_adl.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "live_continuity",
      "--",
      "--nocapture"
    ],
    "purpose": "Kernel continuity proof.",
    "outcome": "passed",
    "evidence_ref": "resident_continuity_kernel.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/309/validate_rollback_proof.py"
    ],
    "purpose": "Rollback proof.",
    "outcome": "passed",
    "evidence_ref": "rollback_proof.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--",
      "--check"
    ],
    "purpose": "Rust formatting.",
    "outcome": "passed",
    "evidence_ref": "rust_format.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict Clippy.",
    "outcome": "passed",
    "evidence_ref": "strict_clippy.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_owner_binary_install.sh"
    ],
    "purpose": "Clean-install owner proof.",
    "outcome": "passed",
    "evidence_ref": "supported_cli_clean_install.log"
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
