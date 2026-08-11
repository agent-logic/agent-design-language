# Structured Output Record

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved the PR #230 hosted-CI regressions and the subsequently exercised retained-validator provenance defect. Production test initialization supplies the mandatory isolated private continuity listener, exact five-participant registry, dedicated server/client EKUs, trust roots, and SPKI pins. Nextest uses an exact 56-case filter, explicit standalone config binding, and a five-second result=fail loaded-host EOF bound. The proof validator now parses only canonical typed v2 Option review fields, binds them to the canonical index review, validates exact git-blake3 shape and ancestry, and self-tests malformed, absent, bare, trailing, truncated, and wrong-shape provenance rejection. After incorporating current origin/main, fresh v4 evidence at 146040c47fd64365d2c7eb0670de211855c4156a retains Runtime 21/21 and kernel 35/35 in two concurrent plus two isolated waves with zero LEAK, production ACIP 2/2, both strict Clippy lanes, selector contract, exact 56/64/12 parity, and diff hygiene. Fresh independent rereview remains required before republishing PR #230.

## Artifacts

- adl/.config/nextest.toml
- adl-runtime-kernel/tests/production_acip_wss.rs
- adl-runtime-kernel/tests/support/runtime_init.rs
- .csdlc/prepared/issues/208/verify-nextest-workspace-contract.rb
- .csdlc/prepared/issues/208/produce-proof-receipt.rb
- .csdlc/prepared/issues/208/validate-proof-receipt.rb
- .csdlc/evidence/208/v4/execution-proof.json

## Execution

- Extended the production runtime-init fixture with an isolated private continuity listener, exact five-participant bound, dedicated ServerAuth and ClientAuth credentials, trust roots, SPKI pins, and actionable readiness diagnostics.
- Replaced absent binary-name nextest selectors with the exact ordered 56-case filter and explicitly bound all standalone proof lanes to the tracked fail-on-leak policy.
- Expanded retained proof with production ACIP 2/2 and the workspace/slow-proof selector contract while retaining four zero-LEAK full waves per package.
- Repaired the final receipt validator to parse canonical typed v2 Some review fields, compare them with canonical index review truth, and use the embedded review commit for source ancestry and protected-file checks.
- Added fail-closed parser regressions for absent, bare, trailing, unterminated, truncated, and wrong-shape review provenance, then incorporated current origin/main before regenerating proof.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--config-file",
      "adl/.config/nextest.toml",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "kernel_continuity_client",
      "--no-tests=fail"
    ],
    "purpose": "Prove the exact 21-case Runtime target in two concurrent and two isolated retained waves.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/runtime-nextest*.stderr.log: 21/21 in all four waves, zero LEAK"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--config-file",
      "adl/.config/nextest.toml",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "kernel_continuity_control",
      "--no-tests=fail"
    ],
    "purpose": "Prove the exact 35-case kernel target in two concurrent and two isolated retained waves.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/kernel-nextest*.stderr.log: 35/35 in all four waves, zero LEAK"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--config-file",
      "adl/.config/nextest.toml",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "production_acip_wss",
      "--no-tests=fail"
    ],
    "purpose": "Reproduce the hosted production ACIP lane with mandatory private continuity initialization.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/production-acip-nextest.stderr.log: 2/2 passed"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/208/verify-nextest-workspace-contract.rb"
    ],
    "purpose": "Reject absent binary-name selectors and require exact fail-closed standalone policy binding.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/nextest-workspace-contract.stdout.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-guardian",
      "--test",
      "kernel_continuity_client",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across the Runtime, Guardian, client target, and teardown assertion.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/runtime-clippy.stderr.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-kernel",
      "--test",
      "kernel_continuity_control",
      "--test",
      "production_acip_wss",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across kernel effects, persistence, private server, fixture, and tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/kernel-clippy.stderr.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/208/verify-diff-hygiene.rb"
    ],
    "purpose": "Verify exact execution-base-to-source diff hygiene and current-main ancestry.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/diff-hygiene.stdout.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/208/produce-proof-receipt.rb"
    ],
    "purpose": "Produce exact 56-case, 64-boundary, 12-lifecycle evidence with hosted regressions and zero-LEAK lanes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/208/v4/execution-proof.json"
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
