# Structured Output Record

Template: 1.0.0

Issue: 5591

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented canonical Runtime v3 ingress, deterministic execution and replay continuity, and graceful terminal serialization without Runtime v2 or AWS.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/tests/control.rs

## Execution

- Route signed Execute capability through bounded canonical Runtime v3 components and deterministic results
- Persist signed checkpoints and restore fresh-runtime sequence and duplicate-safe replay continuity
- Close admission and drain accepted work before terminal checkpoint serialization
- Keep governed provider and cloud-bridge operations behind permit-authoritative operational APIs
- Prevent pressure recovery from reopening admission after a signed terminal action is reserved
- Keep unsupported and failed dispatch retries deterministic without advancing accepted state

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "assembly",
      "--test",
      "control",
      "--test",
      "operations"
    ],
    "purpose": "Prove governed routing, terminal-pressure race safety, delayed-work checkpoint ordering, deterministic retries, and focused Runtime v3 regressions; assembly 4, control 21, operations 9, ingress atomic admission, and focused process pressure/signed shutdown yielded 36 unique passing tests",
    "outcome": "passed",
    "evidence_ref": "subagent:019f8277-0050-7db0-a96b-05593df9c703@6f19349e6d6227c362f5d73dce2c977aab41c1db:36-focused-tests:/Volumes/FastWork/adl-5591-rereview-target"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the Parity-A implementation and test targets are warning-free under strict Clippy",
    "outcome": "passed",
    "evidence_ref": "owner:019f8189-8205-7dd3-bbe2-f7f4dddd098a@6f19349e6d6227c362f5d73dce2c977aab41c1db:/Volumes/FastWork/adl-5591/runtime-target"
  },
  {
    "command": [
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5591/integrated-focused cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test assembly --test control --test operations",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5591/integrated-full cargo test --manifest-path adl-runtime-kernel/Cargo.toml",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5591/integrated-clippy cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets --all-features -- -D warnings",
      "ADL_HORUST_BIN=/Volumes/FastWork/adl-5591/horust-install/bin/horust CARGO_TARGET_DIR=/Volumes/FastWork/adl-5591/integrated-horust cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test guardian_soak horust_restarts_once_and_restores_continuity -- --ignored --exact",
      "ADL_HORUST_BIN=/Volumes/FastWork/adl-5591/horust-install/bin/horust CARGO_TARGET_DIR=/Volumes/FastWork/adl-5591/integrated-horust cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test guardian_soak horust_allowlists_child_environment -- --ignored --exact",
      "cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --all -- --check",
      "cargo tree --locked --manifest-path adl-runtime-kernel/Cargo.toml",
      "bash adl/tools/report_runtime_v3_loc.sh",
      "git diff --check origin/main...HEAD"
    ],
    "purpose": "Prove the integrated Parity-A head: 34 focused and 187 complete-suite tests, strict all-target/all-feature lint, Horust restart/continuity and environment isolation, formatting and diff hygiene, locked COTS inventory, and exact budget truth. The report records 12,683 physical lines and 195 tests: 683 lines above the 12,000 reviewed target but within the explicit 20,000 exception ceiling, requiring review disposition.",
    "outcome": "passed",
    "evidence_ref": "owner:019f8362-357c-7621-95b7-afb314e0c61d@740a65ba2c60d06915ce8c6e08b8b5756d245ccc:/Volumes/FastWork/adl-5591/integrated-*"
  },
  {
    "command": [
      "bash -n adl/tools/report_runtime_v3_loc.sh",
      "bash adl/tools/report_runtime_v3_loc.sh",
      "git diff --check"
    ],
    "purpose": "Record 12,683 physical source lines as exactly +474 over the pinned #5336 baseline of 12,209 and +683 over the 12,000 target. The 20,000 hard safety ceiling does not authorize an exception. Exact review found the +474 functional delta necessary and non-duplicative: signed ingress +323, control/admission +76, assembly +45, continuity +21, and terminal/misc wiring +9; identified safe consolidation cannot recover the delta without removing acceptance behavior.",
    "outcome": "passed",
    "evidence_ref": "subagent:/root/review_5591_exact_head@be9daf2ea:P1-budget-authority-correction"
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
