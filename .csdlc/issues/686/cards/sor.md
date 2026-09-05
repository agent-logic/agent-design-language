# Structured Output Record

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recovered and remediated #686 after exact-head review found that configuration-generation identity used only init content SHA-256 while immutable receipt bytes also included compatible_binary_generation. The fix makes the immutable configuration-generation identity include both content_sha256 and compatible_binary_generation, while retaining content_sha256 as a separate receipt field. Focused regression coverage now proves an unchanged init can provision and activate runtime-generation-one, provision the identical init for runtime-generation-two, activate and validate generation two, and roll back to the generation-one receipt.

## Artifacts

- adl-runtime-kernel/src/config_generation.rs
- adl/tests/csm_runtime_v3_generation.rs
- .csdlc/prepared/issues/686/issue_686_validate_config_generation_handoff.py
- .csdlc/evidence/686/issue686-csm-runtime-v3-unit.log sha256=799396c961bfaae0beb581bb121edb55a108d1f24885457ffaeccc81cd5679fd
- .csdlc/evidence/686/issue686-denominator.log sha256=58228c3d075a16922e7d3a20250d75d9d53d6e0e6bfc9e626fe6b0703d359995
- .csdlc/evidence/686/issue686-diff-check.log sha256=34940f5adc404d49647969ab0511c866df5b4074fa9c60b137990fd02d118cdc
- .csdlc/evidence/686/issue686-fmt-check.log sha256=317426c7ee5b6ccc749a44e0d911b69b2c88a0e9bda3bfe1f2fe70681817e23a
- .csdlc/evidence/686/issue686-focused-generation.log sha256=5612ab4cf6cbb1da5eb6a269d88096ad6fa7ea05563343b5f86fe29635c2d778
- .csdlc/evidence/686/issue686-guardian-cli-bounds.log sha256=0afe4379b7b0a6a9a2e7f3014037b6e1eb2cc60ae7f6549284b5f7e8bc856455
- .csdlc/evidence/686/issue686-guardian-cli-portable-child.log sha256=42cfc6e9ba8cdd52b557c42a9fc91f9696a0c158bca67ab91ee0bb2944f45e40
- .csdlc/evidence/686/issue686-strict-clippy.log sha256=b7725162ff33f7068a0ba17409f4169680e6398f5c890db900faf2b8bb892658

## Execution

- Preserved typed review-failure truth by recovering #686 from published gen38 back to implemented gen39 before source edits.
- Changed `adl-runtime-kernel/src/config_generation.rs` so `ConfigGenerationReceipt.generation` is a SHA-256 digest over the receipt schema, retained `content_sha256`, and `compatible_binary_generation`, instead of aliasing generation to the init content hash.
- Kept `content_sha256` in the receipt as the stable configuration-content digest for audit and comparison.
- Added `unchanged_config_can_upgrade_and_roll_back_binary_generation` to prove identical init content can create distinct immutable receipts for two Runtime binary generations, validate generation two after upgrade, and re-activate/validate generation one for rollback.
- Updated the issue-owned #686 denominator to require the binary-generation-bound identity helper and the upgrade/rollback regression anchors.
- Re-ran the #686 denominator, focused csm_runtime_v3_cmd unit lane, focused Runtime v3 generation integration suite, both hosted-runtime regression tests, strict Clippy, all-workspace fmt check, and diff-check at source head 9c5cf807b219fcf170e07d68dd8a796ed8c82031.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/686/issue_686_validate_config_generation_handoff.py"
    ],
    "purpose": "Prove the issue-owned #686 static denominator now requires binary-generation-bound configuration identity and upgrade/rollback regression anchors.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-denominator.log sha256=58228c3d075a16922e7d3a20250d75d9d53d6e0e6bfc9e626fe6b0703d359995 head=9c5cf807b219fcf170e07d68dd8a796ed8c82031 status=0"
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
    "purpose": "Focused csm_runtime_v3_cmd unit proof for Runtime v3 command behavior touched by the configuration-generation handoff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-csm-runtime-v3-unit.log sha256=799396c961bfaae0beb581bb121edb55a108d1f24885457ffaeccc81cd5679fd head=9c5cf807b219fcf170e07d68dd8a796ed8c82031 status=0"
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
    "purpose": "Focused Runtime v3 configuration-generation integration regression suite, including unchanged-config binary upgrade and rollback.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-focused-generation.log sha256=5612ab4cf6cbb1da5eb6a269d88096ad6fa7ea05563343b5f86fe29635c2d778 head=9c5cf807b219fcf170e07d68dd8a796ed8c82031 status=0"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "guardian_cli",
      "guardian_cli_reports_successful_portable_child_as_json",
      "--",
      "--nocapture"
    ],
    "purpose": "Hosted adl-coverage-runtime regression for Guardian portable-child JSON success with valid config-generation authority.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-guardian-cli-portable-child.log sha256=42cfc6e9ba8cdd52b557c42a9fc91f9696a0c158bca67ab91ee0bb2944f45e40 head=9c5cf807b219fcf170e07d68dd8a796ed8c82031 status=0"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_guardian_lifecycle",
      "guardian_cli_rejects_oversized_durations_before_spawning_the_kernel",
      "--",
      "--nocapture"
    ],
    "purpose": "Hosted adl-coverage-runtime regression for Guardian oversized-duration rejection before marker-child spawn.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-guardian-cli-bounds.log sha256=0afe4379b7b0a6a9a2e7f3014037b6e1eb2cc60ae7f6549284b5f7e8bc856455 head=9c5cf807b219fcf170e07d68dd8a796ed8c82031 status=0"
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
    "evidence_ref": ".csdlc/evidence/686/issue686-strict-clippy.log sha256=b7725162ff33f7068a0ba17409f4169680e6398f5c890db900faf2b8bb892658 head=9c5cf807b219fcf170e07d68dd8a796ed8c82031 status=0"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all",
      "--check"
    ],
    "purpose": "Hosted-equivalent Rust formatting check for all ADL Cargo packages and local path dependencies.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-fmt-check.log sha256=317426c7ee5b6ccc749a44e0d911b69b2c88a0e9bda3bfe1f2fe70681817e23a head=9c5cf807b219fcf170e07d68dd8a796ed8c82031 status=0"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Whitespace and patch hygiene check.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/686/issue686-diff-check.log sha256=34940f5adc404d49647969ab0511c866df5b4074fa9c60b137990fd02d118cdc head=9c5cf807b219fcf170e07d68dd8a796ed8c82031 status=0"
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
