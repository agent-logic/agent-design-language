# Structured Output Record

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Repaired #258 with concrete private non-zero-sized certificate, lease, and fencing access seals validated by exact static pointer identity plus private magic. The legitimate runtime transport integration token remains available only under the non-default internal-test-fixtures feature, whose target is explicitly feature-gated. The no-feature external denial proof rejects the prior fat-trait-pointer attack and selects only the newest no-feature rlib, failing if that artifact predates current Cargo.toml or source inputs. A direct runtime regression now copies each exact magic value, transmutes its same-layout thin reference into the public access type, and proves every raw-store entrypoint rejects it before mutation.

## Artifacts

- .csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-test-distributed-identity-lease-authority.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-test-distributed-runtime-transport.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-check-adl-runtime-all-targets.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-clippy-distributed-identity-lease-authority.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-clippy-distributed-runtime-transport.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r4/test-check-coverage-impact.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r4/test-mechanical-coverage-fallout.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r4/git-diff-check.log

## Execution

- Replaced forgeable trait-object seals with private 32-byte concrete seals and exact known-static pointer identity checks.
- Added external compile denial for the exact compatible fat-trait-pointer transmute attack reported by review.
- Made no-feature artifact selection deterministic by newest modification time and rejected artifacts older than any current manifest/source input.
- Added a direct runtime denial for copied-magic same-layout thin-reference forgeries across certificate, lease, and fencing public raw-store entrypoints.
- Retained non-default internal-test-fixtures and required-features gating for the 14-test runtime transport integration target.
- Preserved frozen #203 and did not absorb #203/#259 transport architecture scope.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_identity_lease_authority",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Prove focused authority boundaries and current no-feature external denial, including the reported fat-pointer attack.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-test-distributed-identity-lease-authority.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_runtime_transport",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Execute the full gated runtime transport integration target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-test-distributed-runtime-transport.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--all-targets",
      "--features",
      "internal-test-fixtures"
    ],
    "purpose": "Compile-check all runtime targets with the internal fixture feature.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-check-adl-runtime-all-targets.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_identity_lease_authority",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict-lint the focused authority proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-clippy-distributed-identity-lease-authority.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_runtime_transport",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict-lint the gated runtime transport target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-clippy-distributed-runtime-transport.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Exercise hosted coverage-impact contract and default all-target discovery.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/test-check-coverage-impact.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Exercise mechanical coverage fallout policy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/test-mechanical-coverage-fallout.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject patch hygiene defects.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/git-diff-check.log"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-258-authority-store-boundary",
      "issue",
      "--issue",
      "258"
    ],
    "purpose": "Validate canonical lifecycle truth after r5 repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/csdlc-validate.log"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
