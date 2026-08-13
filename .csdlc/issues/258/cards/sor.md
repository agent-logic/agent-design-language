# Structured Output Record

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #258 as the first #203 split slice: sealed raw certificate, lease, and fencing store access behind explicit authority/test access tokens; added authority-bound store adapter facade and expanded published receipt view; preserved the in-scope published-store mutation classifier/view fix; removed earlier over-scope transport authorization seams; repaired the r2 MaybeUninit token-forgery issue; and repaired the r3 stale transport helper regression. Candidate 2d5f1d94 was invalid because distributed_runtime_transport still used a panic sentinel instead of an executable certificate-store fixture. The current repair keeps test access unavailable to ordinary dev-profile dependents, uses a test-internal source fixture capability for the large runtime transport integration target, validates sealed raw access with private magic, and proves ordinary external unit/zeroed forgeries fail to compile.

## Artifacts

- .csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-test-distributed-runtime-transport.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-test-distributed-identity-lease-authority.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-test-distributed-guardian-no-run.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-check-adl-runtime.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-clippy-distributed-identity-lease-authority.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-clippy-distributed-runtime-transport.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r3/hosted-equivalent-check-coverage-impact.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r3/test-mechanical-coverage-fallout.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r3/git-diff-check.log

## Execution

- Changed CertificateStoreAccess, LeaseStoreAccess, and FencingStoreAccess validation from pointer-identity-only seals to private sealed-magic capabilities so a source-included test fixture can exercise integration tests while ordinary external unit/zeroed forgeries remain compile-denied.
- Replaced the distributed_runtime_transport panic sentinel with a test-internal certificate-store source fixture capability bridge and restored all seven previously ignored large runtime transport tests to executable status.
- Confirmed distributed_runtime_transport now runs the full target with 14 passed, 0 failed, and 0 ignored, including the seven runtime transport tests requested by Planning.
- Retained the focused external rustc proof denying TEST_* fixture imports, unsafe transmute(()) unit forging, and zeroed MaybeUninit forging for certificate, lease, and fencing access capability types.
- Retained the #258 authority-store boundary and coverage-classifier scope; #203/#259 transport architecture remains frozen/non-goal, with no new production transport authorization seams added in this repair.
- Used repo-local TMPDIR for hosted-equivalent preflight evidence and did not use /private/tmp.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_transport",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Exercise all runtime transport integration tests with the test-internal source fixture and prove the previous panic sentinel/ignored-test regression is gone.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-test-distributed-runtime-transport.log"
  },
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
    "purpose": "Exercise the focused #258 authority-store boundary and external compile-fail authority proof, including TEST_* import denial, unit transmute denial, and zeroed MaybeUninit denial.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-test-distributed-identity-lease-authority.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_guardian",
      "--no-run"
    ],
    "purpose": "Compile guard for distributed_guardian after the raw-store token seal repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-test-distributed-guardian-no-run.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Compile-check the runtime crate after the executable fixture and sealed-token repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-check-adl-runtime.log"
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
    "purpose": "Strict-lint the focused #258 authority-store boundary test target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-clippy-distributed-identity-lease-authority.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_transport",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict-lint the restored executable runtime transport target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/cargo-clippy-distributed-runtime-transport.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Hosted-equivalent coverage-impact preflight for the #258 coverage classifier and current changed-source selection.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/hosted-equivalent-check-coverage-impact.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Retain the deterministic mechanical coverage fallout policy proof for the #258 coverage classifier.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/test-mechanical-coverage-fallout.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch hygiene errors across the r3 repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/git-diff-check.log"
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
    "purpose": "Validate typed #258 lifecycle truth after restoring executable runtime transport tests with the test-internal source fixture.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r3/csdlc-validate.log"
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
