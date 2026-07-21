# Structured Output Record

Template: 1.0.0

Issue: 5615

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Separate lifecycle metadata from standalone C-SDLC v2 Rust proof, require the selected lane in stable adl-ci, preserve coverage truth, and route Cargo state to a validated external root.

## Artifacts

- .github/workflows/ci.yaml
- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/ci_path_policy.sh
- adl/tools/run_cargo_validation.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_run_cargo_validation.sh
- adl/tools/test_select_validation_lanes.sh

## Execution

- Split .csdlc metadata and csdlc-v2 Rust selectors in validation-manager authority
- Add an explicit standalone C-SDLC v2 selector output and hosted test/format/strict-Clippy job
- Fail closed on malformed selector output and required-but-skipped standalone proof
- Preserve Runtime v3 focused routing when issue lifecycle metadata is present
- Add an external Cargo build-root wrapper with FastWork fallback and compatibility symlink

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh",
      "&&",
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh",
      "&&",
      "bash",
      "adl/tools/test_run_cargo_validation.sh"
    ],
    "purpose": "Prove metadata-only, standalone C-SDLC v2, Runtime-plus-lifecycle, stable aggregate, selector, and external build-root behavior.",
    "outcome": "passed",
    "evidence_ref": "issue-5615:focused-ci-routing-and-wrapper-contracts:pass"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5615/run_csdlc_v2_standalone.sh"
    ],
    "purpose": "Prove all C-SDLC v2 targets with locked tests, formatting, and strict Clippy while Cargo state remains on the external SSD.",
    "outcome": "passed",
    "evidence_ref": "issue-5615:csdlc-v2-standalone-fastwork:test-fmt-clippy-pass"
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
