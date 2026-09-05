# Structured Output Record

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and remediated the #686 Runtime v3 configuration-generation handoff after exact-head review recovery, current-main resync, and r8 denominator-proof correction. The change creates immutable configuration-generation receipts, validates and activates the active generation reference before Runtime service mutation, carries generation and receipt digest through CSM, Guardian child launch, kernel startup, readiness, and status, reconciles interrupted reload transactions before configuration-generation preflight can reject stranded active/ref mismatches, provisions reload candidate receipts into the active init generation store, and tightens the issue-owned denominator script so it checks named implementation, ordering, propagation, and regression-test anchors instead of loose token presence.

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
- .csdlc/evidence/686/issue686-csm-runtime-v3-unit.log sha256=ae664e073b399e5c6d4592b434f89efa7fd8c1c71b58178871ee6979a17aff32
- .csdlc/evidence/686/issue686-denominator.log sha256=bb1c2325224c0219dd4e68c76e0835b0f529aabdb90dbfca9db2df53585d85ce
- .csdlc/evidence/686/issue686-diff-check.log sha256=c78ae230003ac1b3b218409622ca313b6eea006352da204a670361145f02d80f
- .csdlc/evidence/686/issue686-fmt-check.log sha256=897f3f04eaf71a3ac03d330fd679ea8e41e0c7f49ae5afce4b0f30c574d90fd7
- .csdlc/evidence/686/issue686-focused-generation.log sha256=5074f5410731b0024c94386ab3818a9725648df498fdc808bbeac3d66445f688
- .csdlc/evidence/686/issue686-strict-clippy.log sha256=fcaa1d19936fb0caf537a6ac20f4488835b3220116f47c5a0f29147cf5b18b7f

## Execution

- Added Runtime kernel configuration-generation receipt primitives for immutable receipt construction, digest validation, active-reference activation, and active-generation validation.
- Threaded the active configuration generation and receipt digest through CSM Runtime v3 start/reload/status, Guardian child launch environment propagation, kernel startup, and readiness reporting.
- Hardened reload recovery so active configuration-generation references are backed up, restored, and committed together with init file replacement, and so interrupted reload reconciliation runs before active configuration-generation preflight.
- Provisioned reload candidate configuration-generation receipts into the active init generation store so candidates from a separate directory can become valid active generations after activation.
- Merged current origin/main into the issue branch, preserving mainline Runtime v3 command changes and #686's configuration-generation handoff invariants.
- Remediated r8-p2-denominator-token-presence-overclaim by expanding the denominator script to verify concrete receipt, active-reference, CSM ordering, Guardian/kernel propagation, readiness, active-store candidate, and regression-test anchors.
- Added focused deterministic integration and unit coverage for receipt immutability, secret-path reference redaction, pre-activation non-authority, pointer mismatch rejection, candidate readiness without activation, candidate failure restoration, malformed receipt rejection, cross-binary rejection, cross-directory candidate receipt activation, and pre-reconcile active/ref mismatch recovery.
- Kept scope local to Runtime v3 configuration-generation handoff surfaces and did not perform any live Runtime restart, cloud, paid, credential, or deployment action.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/686/issue_686_validate_config_generation_handoff.py"
    ],
    "purpose": "Prove the issue-owned #686 static denominator checks concrete configuration-generation handoff, recovery-ordering, propagation, active-store candidate, and regression-test anchors.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-denominator.log sha256=bb1c2325224c0219dd4e68c76e0835b0f529aabdb90dbfca9db2df53585d85ce head=c0bf5380ac027fa31621bec4762cd24ca1af040a status=0"
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
    "evidence_ref": ".csdlc/evidence/686/issue686-csm-runtime-v3-unit.log sha256=ae664e073b399e5c6d4592b434f89efa7fd8c1c71b58178871ee6979a17aff32 head=c0bf5380ac027fa31621bec4762cd24ca1af040a status=0"
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
    "evidence_ref": ".csdlc/evidence/686/issue686-focused-generation.log sha256=5074f5410731b0024c94386ab3818a9725648df498fdc808bbeac3d66445f688 head=c0bf5380ac027fa31621bec4762cd24ca1af040a status=0"
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
    "evidence_ref": ".csdlc/evidence/686/issue686-strict-clippy.log sha256=fcaa1d19936fb0caf537a6ac20f4488835b3220116f47c5a0f29147cf5b18b7f head=c0bf5380ac027fa31621bec4762cd24ca1af040a status=0"
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
    "evidence_ref": ".csdlc/evidence/686/issue686-fmt-check.log sha256=897f3f04eaf71a3ac03d330fd679ea8e41e0c7f49ae5afce4b0f30c574d90fd7 head=c0bf5380ac027fa31621bec4762cd24ca1af040a status=0"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Whitespace and patch hygiene check.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-diff-check.log sha256=c78ae230003ac1b3b218409622ca313b6eea006352da204a670361145f02d80f head=c0bf5380ac027fa31621bec4762cd24ca1af040a status=0"
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
