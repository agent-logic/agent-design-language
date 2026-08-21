# Structured Output Record

Template: 1.0.0

Issue: 309

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Removed 20 reviewed dead or superseded Rust files across independently reversible Bands A and B, totaling 11,571 deleted physical lines from the pinned 485-file/265,633-line baseline. Retained Runtime v2, #414 continuity, policy authority, current providers, ACC/capability surfaces, supported CLI routes, and every candidate lacking complete deletion proof.

## Artifacts

- adl/src/lib.rs
- adl/src/gws_live_test_support.rs
- adl/tools/run_pr_fast_test_lane.sh
- adl/tools/test_run_pr_fast_test_lane.sh
- .csdlc/prepared/issues/309/refresh_crate_reference_edges.py
- .csdlc/prepared/issues/309/refresh_dead_code_band.py
- .csdlc/prepared/issues/309/run_gemini_dead_code_audit.py
- .csdlc/prepared/issues/309/validate_reduction_inventory.py
- .csdlc/prepared/issues/309/validate_rollback_proof.py
- .csdlc/evidence/309/baseline-manifest.json
- .csdlc/evidence/309/reference-edge-manifest.json
- .csdlc/evidence/309/disposition-manifest.json
- .csdlc/evidence/309/reduction-report.json
- .csdlc/evidence/309/rollback-proof.json
- .csdlc/evidence/309/gemini-dead-code-audit.md

## Execution

- Recomputed the pinned baseline and 2,216 normalized reference edges, with every deleted target and deleted-source edge explicitly removal-disposed.
- Kept Band A's two retired evaluation modules and added Band B's pre-v0.92 skill schema, speculative-decoding prototype, retired GWS demo implementations, local-Gemma evaluator, and UTS/ACC benchmark-only cluster.
- Restored policy_authority byte-for-byte after the advisory audit proposed it incorrectly; retained cognitive-transition, AWS #268, Runtime v2/#414, active provider, capability, ACC, current demo, and supported CLI surfaces.
- Normalized Band B into one independently reversible commit and proved exact Git tree restoration and reapplication without unrelated path changes.
- Made PR-fast Rust routing deletion-aware and added a regression fixture proving deleted Rust paths select a focused lane instead of the forbidden full fallback.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/309/validate_reduction_inventory.py",
      "--root",
      "."
    ],
    "purpose": "Recompute the exact baseline, reference, disposition, historical-retirement, candidate-diff, and reduction denominator.",
    "outcome": "passed",
    "evidence_ref": "485 files; 265633 baseline lines; 2216 edges; 20 deleted files; 11571 deleted lines"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/309/validate_rollback_proof.py",
      "--root",
      "."
    ],
    "purpose": "Prove exact per-band Git revert and reapply topology, trees, and path isolation.",
    "outcome": "passed",
    "evidence_ref": "2 bands; 0 errors"
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
    "purpose": "Protect #414 resident dehydration, restore, admission, and useful-continuation behavior.",
    "outcome": "passed",
    "evidence_ref": "6 passed; 0 failed"
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
    "purpose": "Protect signed Runtime-kernel continuity behavior consumed by #414.",
    "outcome": "passed",
    "evidence_ref": "8 passed; 0 failed"
  },
  {
    "command": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all-targets"
    ],
    "purpose": "Compile every ADL target after removing the orphan clusters.",
    "outcome": "passed",
    "evidence_ref": "all targets compiled"
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
    "purpose": "Reject warnings and residual references across every ADL target.",
    "outcome": "passed",
    "evidence_ref": "strict Clippy passed"
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
    "purpose": "Verify canonical Rust formatting.",
    "outcome": "passed",
    "evidence_ref": "format check passed"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_owner_binary_install.sh"
    ],
    "purpose": "Prove supported clean owner-binary installation remains intact.",
    "outcome": "passed",
    "evidence_ref": "owner binary stable install: ok"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_test_lane.sh"
    ],
    "purpose": "Prove deleted Rust paths trigger focused PR-fast routing.",
    "outcome": "passed",
    "evidence_ref": "PASS test_run_pr_fast_test_lane"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch hygiene defects.",
    "outcome": "passed",
    "evidence_ref": "no findings"
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
