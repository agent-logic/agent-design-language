# Structured Output Record

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Forward-recovered the authoritative bound #112 branch to the decomposed Layer 8 signed authority core and remediated exact-head review findings: identity precheck refusals now enter the hash-chained audit path, focused tests now cover the claimed core refusal/signature/audit matrix, and the stale executable preparation validator was removed from current artifacts.

## Artifacts

- adl-runtime-kernel/src/layer8_authority/audit.rs
- adl-runtime-kernel/src/layer8_authority/mod.rs
- adl-runtime-kernel/tests/layer8_authority.rs
- .csdlc/prepared/issues/112/validate-preparation.rb
- .csdlc/evidence/112/layer8-authority-core-tests-forward.log
- .csdlc/evidence/112/layer8-authority-core-fmt-forward.log
- .csdlc/evidence/112/layer8-authority-core-clippy-forward.log
- .csdlc/evidence/112/layer8-authority-core-diff-check-forward.log

## Execution

- Cleared stale review assignment through typed csdlc-review recover after exact-head review findings.
- Routed identity mismatch, stale sender generation, revoked sender, and expired sender precheck refusals through Layer8AuthorityStore so redacted hash-chain audit records are appended before the public refusal returns.
- Preserved fail-closed public refusal semantics while hashing correlation IDs and omitting raw secret/content/provider/private-cognition fields from audit records.
- Expanded focused runtime-kernel tests from one integration test to four tests covering replay and known-recipient behavior, audited identity precheck refusals, revoked/stale capability, unavailable policy, corrupt audit state, non-canonical payload rejection, request tamper, and acknowledgement substitution.
- Removed .csdlc/prepared/issues/112/validate-preparation.rb from the current prepared artifacts because it was a stale ready-phase/API/UI validator and contradicted the forward-recovered core-only implemented state.
- Kept #265 Runtime ingress, #270 served acknowledgement/API protocol, #271 Observatory UI, durable history, rooms, roster, presence, #114/#115/#117, and sibling work out of the current net product diff.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "layer8_authority",
      "--",
      "--nocapture"
    ],
    "purpose": "Run the focused Layer 8 authority core integration target covering audited precheck refusals, capability/policy/audit failure, replay, signature/payload tamper, acknowledgement substitution, and known-recipient behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/112/layer8-authority-core-tests-forward.log (4 passed)"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Verify runtime-kernel Rust formatting for the review-fix diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/112/layer8-authority-core-fmt-forward.log (empty output; exit 0)"
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
    "purpose": "Enforce strict warning-free clippy for the runtime-kernel crate and tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/112/layer8-authority-core-clippy-forward.log (exit 0)"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify whitespace and conflict-marker hygiene for the review-fix diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/112/layer8-authority-core-diff-check-forward.log (empty output; exit 0)"
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
