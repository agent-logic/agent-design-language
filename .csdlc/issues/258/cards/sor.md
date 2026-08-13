# Structured Output Record

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #258 as the first #203 split slice: sealed raw certificate, lease, and fencing store access behind explicit authority/test access tokens; added authority-bound store adapter facade and expanded published receipt view; preserved the in-scope published-store mutation classifier/view fix; removed earlier over-scope transport authorization seams; and repaired the post-publication stale helper regression correctly after candidate 986f6c01c still allowed ordinary integration tests to forge access with MaybeUninit. CertificateStoreAccess, LeaseStoreAccess, and FencingStoreAccess now carry private static seal identities that raw methods validate before opening, mutating, or authorizing. Ordinary integration tests no longer contain stale unsafe helper forgeries, and the focused external compile-fail proof covers both unit transmute and zeroed MaybeUninit attempts.

## Artifacts

- .csdlc/evidence/258/postpub-stale-helper-repair-r2/test-check-coverage-impact.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-test-distributed-identity-lease-authority.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-test-distributed-guardian-no-run.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-test-distributed-runtime-transport-no-run.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-check-adl-runtime.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-clippy-distributed-identity-lease-authority.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-clippy-distributed-guardian.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-clippy-distributed-runtime-transport.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/test-mechanical-coverage-fallout.log
- .csdlc/evidence/258/postpub-stale-helper-repair-r2/git-diff-check.log

## Execution

- Changed CertificateStoreAccess, LeaseStoreAccess, and FencingStoreAccess from byte-pattern capabilities to private-static-seal capabilities and added raw access validation to certificate open/activate/authorize/revoke, lease new/apply/authorize_mutation, and fencing create/open/commit/authorize_active_lease.
- Removed the stale unsafe LeaseStoreAccess helper from distributed_guardian.rs by using a test-internal source fixture for the crate-private TEST_LEASE_STORE_ACCESS.
- Removed the stale unsafe CertificateStoreAccess helper body from distributed_runtime_transport.rs so the large transport target remains compile-clean without a token-forging setup helper in ordinary integration code.
- Extended the focused external rustc proof to deny TEST_* fixture imports, unsafe transmute(()) unit forging, and zeroed MaybeUninit forging for certificate, lease, and fencing access capability types.
- Confirmed production transport/core authorization sites remain bound to AUTHORITY_BOUND_CERTIFICATE_ACCESS and did not reopen #259 transport scope.
- Retained repo-local TMPDIR for hosted-equivalent preflight evidence and did not use /private/tmp.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Hosted-equivalent coverage-impact preflight contract that previously failed on stale compiled unsafe helper fallout.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/test-check-coverage-impact.log"
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
    "purpose": "Exercise the focused #258 authority-store boundary and external compile-fail authority proof, including zeroed MaybeUninit denial.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-test-distributed-identity-lease-authority.log"
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
    "purpose": "Compile-only guard for the stale LeaseStoreAccess helper fallout in distributed_guardian.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-test-distributed-guardian-no-run.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_transport",
      "--no-run"
    ],
    "purpose": "Compile-only guard for the stale CertificateStoreAccess helper fallout in distributed_runtime_transport.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-test-distributed-runtime-transport-no-run.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Compile-check the runtime crate after the private-static-seal access repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-check-adl-runtime.log"
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
    "purpose": "Strict-lint the focused #258 authority-store boundary test target after the r2 repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-clippy-distributed-identity-lease-authority.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_guardian",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict-lint the repaired distributed_guardian helper target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-clippy-distributed-guardian.log"
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
    "purpose": "Strict-lint the repaired distributed_runtime_transport helper target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/cargo-clippy-distributed-runtime-transport.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Retain the PR-fast deterministic mechanical coverage fallout proof for the #258 coverage classifier.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/test-mechanical-coverage-fallout.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch hygiene errors across the r2 repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/git-diff-check.log"
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
    "purpose": "Validate typed #258 lifecycle truth after the private-static-seal r2 SOR update.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/postpub-stale-helper-repair-r2/csdlc-validate.log"
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
