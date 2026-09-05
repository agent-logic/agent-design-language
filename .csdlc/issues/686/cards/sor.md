# Structured Output Record

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and remediated the #686 Runtime v3 configuration-generation handoff after exact-head review recovery. The change creates immutable configuration-generation receipts, validates and activates the active generation reference before Runtime service mutation, carries generation and receipt digest through CSM, Guardian child launch, kernel startup, readiness, and status, reconciles interrupted reload transactions before configuration-generation preflight can reject stranded active/ref mismatches, provisions reload candidate receipts into the active init generation store, and adds deterministic regression coverage without starting live Runtime/cloud services.

## Artifacts

- adl-runtime-kernel/src/config_generation.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl/tests/csm_runtime_v3_generation.rs
- .csdlc/prepared/issues/686/issue_686_validate_config_generation_handoff.py
- .csdlc/evidence/686/issue686-csm-runtime-v3-unit.log sha256=3d7fb46156c73d54af51d19918025559ebfd290302e4f0630f336e4cbc67f79a
- .csdlc/evidence/686/issue686-denominator.log sha256=b467b0b8e13867cd4f44f24c1a2ad8ece84ac570f3df5982aeca2b9ec0577c28
- .csdlc/evidence/686/issue686-diff-check.log sha256=d78b6d79e6f264c5eee5cef5bc64aaa79f9b3fd789e7036445c0bf53a6c7b46b
- .csdlc/evidence/686/issue686-fmt-check.log sha256=c648f002a6d761d6291b5515b0e101ed62facfcfbe1e01d9cdc0f33d12502a77
- .csdlc/evidence/686/issue686-focused-generation.log sha256=540d66b054769b0f5654e55ee7b3036c5f312810a329b247c84938b11a31c76d
- .csdlc/evidence/686/issue686-strict-clippy.log sha256=da84eda3e79cb154f553c0b502f175d46d2be262f71c5ce0b92abb76ef0ae6f6

## Execution

- Added Runtime kernel configuration-generation receipt primitives for immutable receipt construction, digest validation, active-reference activation, and active-generation validation.
- Threaded the active configuration generation and receipt digest through CSM Runtime v3 start/reload/status, Guardian child launch environment propagation, kernel startup, and readiness reporting.
- Hardened reload recovery so active configuration-generation references are backed up, restored, and committed together with init file replacement, and so interrupted reload reconciliation runs before active configuration-generation preflight.
- Provisioned reload candidate configuration-generation receipts into the active init generation store so candidates from a separate directory can become valid active generations after activation.
- Added focused deterministic integration and unit coverage for receipt immutability, secret-path reference redaction, pre-activation non-authority, pointer mismatch rejection, candidate readiness without activation, candidate failure restoration, malformed receipt rejection, cross-binary rejection, cross-directory candidate receipt activation, and pre-reconcile active/ref mismatch recovery.
- Kept scope local to Runtime v3 configuration-generation handoff surfaces and did not perform any live Runtime restart, cloud, paid, credential, or deployment action.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/686/issue_686_validate_config_generation_handoff.py"
    ],
    "purpose": "Prove the issue-owned #686 static denominator for configuration-generation handoff coverage.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-denominator.log sha256=b467b0b8e13867cd4f44f24c1a2ad8ece84ac570f3df5982aeca2b9ec0577c28 head=c41a48ee406ed74e2489b995b5f7eb5778f7fcc7 status=0"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "csm_runtime_v3_cmd"
    ],
    "purpose": "Focused csm_runtime_v3_cmd unit proof including interrupted reload reconciliation before configuration-generation preflight.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-csm-runtime-v3-unit.log sha256=3d7fb46156c73d54af51d19918025559ebfd290302e4f0630f336e4cbc67f79a head=c41a48ee406ed74e2489b995b5f7eb5778f7fcc7 status=0"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "csm_runtime_v3_generation"
    ],
    "purpose": "Focused Runtime v3 configuration-generation integration regression suite including active-store candidate receipt validation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-focused-generation.log sha256=540d66b054769b0f5654e55ee7b3036c5f312810a329b247c84938b11a31c76d head=c41a48ee406ed74e2489b995b5f7eb5778f7fcc7 status=0"
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
    "purpose": "Strict Clippy over all ADL Cargo targets with warnings denied.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-strict-clippy.log sha256=da84eda3e79cb154f553c0b502f175d46d2be262f71c5ce0b92abb76ef0ae6f6 head=c41a48ee406ed74e2489b995b5f7eb5778f7fcc7 status=0"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--check"
    ],
    "purpose": "Rust formatting check for the ADL Cargo workspace.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-fmt-check.log sha256=c648f002a6d761d6291b5515b0e101ed62facfcfbe1e01d9cdc0f33d12502a77 head=c41a48ee406ed74e2489b995b5f7eb5778f7fcc7 status=0"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Whitespace and patch hygiene check.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-diff-check.log sha256=d78b6d79e6f264c5eee5cef5bc64aaa79f9b3fd789e7036445c0bf53a6c7b46b head=c41a48ee406ed74e2489b995b5f7eb5778f7fcc7 status=0"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
