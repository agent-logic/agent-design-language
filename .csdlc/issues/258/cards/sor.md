# Structured Output Record

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired #258 after the prior source-inclusion fixture could forge authority access by giving the production capability a private trait-object seal and exposing the legitimate integration-test token only behind the non-default internal-test-fixtures feature. The ordinary no-feature external surface cannot import or construct the token, while the feature-gated runtime transport target executes all 14 tests. Cargo metadata now explicitly requires the fixture feature for that integration target, keeping default all-target and coverage discovery valid.

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

- Replaced structurally reproducible store-access seals with private trait-object seals for certificate, lease, and fencing capabilities.
- Exposed TEST_* access tokens only under cfg(test) or the non-default internal-test-fixtures feature; the ordinary dependency surface remains denied.
- Reworked the external denial proof to invoke rustc against the Cargo fingerprint-proven no-feature adl_runtime rlib, avoiding nested Cargo cache authority and proving import, construction, unit-transmute, and zeroed-forgery denial.
- Restored distributed_runtime_transport to the production crate surface under internal-test-fixtures and executed all 14 tests with zero ignored.
- Declared distributed_runtime_transport as a required-features integration target so default all-target and coverage discovery do not compile a feature-only harness.
- Preserved #203 as frozen and did not absorb #203/#259 transport architecture scope.

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
    "purpose": "Prove the focused authority boundary and no-feature external import/construction/forgery denial.",
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
    "purpose": "Execute the complete feature-gated runtime transport integration target.",
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
    "purpose": "Compile-check every runtime target with the internal fixture feature enabled.",
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
    "purpose": "Strict-lint the focused authority and external denial proof.",
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
    "purpose": "Strict-lint the complete feature-gated runtime transport target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/cargo-clippy-distributed-runtime-transport.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Exercise the hosted coverage-impact contract, including default all-target discovery.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/test-check-coverage-impact.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Exercise the deterministic mechanical coverage fallout classifier.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/test-mechanical-coverage-fallout.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch-hygiene defects.",
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
    "purpose": "Validate canonical #258 lifecycle and card truth after the r4 repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r4/csdlc-validate.log"
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
