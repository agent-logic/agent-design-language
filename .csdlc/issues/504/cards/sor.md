# Structured Output Record

Template: 1.0.0

Issue: 504

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired PR #588 remote-delivery findings by narrowing delivery, publication, finish, and cleanup authority boundaries; preserving whitespace-normalized reviewer independence; proving Part-Of delivery end-to-end; and implementing safe cleanup removal/outcome distinctions while keeping V3-E non-authoritative construction work.

## Artifacts

- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/commands/remote/tests.rs
- csdlc-v3/src/publication/mod.rs
- csdlc-v3/src/review/mod.rs
- .csdlc/prepared/issues/504/validate-remote-workflow.rb
- .csdlc/issues/504

## Execution

- Changed the remote delivery boundary so deliver(), Verified::new(), receipt minting, and RemoteDeliveryInput construction are crate-scoped rather than public caller-forgeable APIs.
- Made PublicationAuthorization fields crate-scoped so external callers cannot construct review authorization directly.
- Made publish(), derive_finish(), and execute_cleanup_removal() crate-scoped so external callers cannot bypass review/publication/finish authority or invoke cleanup removal directly.
- Added receipt subject-digest binding so verified observations must match the subject they claim to authorize before source-specific delivery gates can consume them.
- Kept reviewer independence normalized on both sides before case-insensitive comparison so whitespace variants of the same principal cannot self-authorize publication.
- Kept Part-Of publication on the advertised end-to-end delivery path: deliver() now returns CheckpointCompleted without terminal cleanup authority for checkpoint work.
- Implemented cleanup removal execution with filesystem removal after terminal, receipt, preview, clean, non-live, canonical-path, and registration gates; removed, already-removed, unregistered, path-mismatch, dirty, and live outcomes remain distinct.
- Moved remote delivery end-to-end tests into the remote module so authority fixture minting is no longer exposed as an integration-test/public API surface.
- Strengthened the #504 validator so it checks the sealed delivery/publication/finish/cleanup authority API, end-to-end Part-Of proof, whitespace self-review proof, cleanup execution proof, and already-removed cleanup gate proof.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/504/validate-remote-workflow.rb"
    ],
    "purpose": "Issue-owned V3-E validator including sealed remote authority API and regression-proof denominator checks.",
    "outcome": "passed",
    "evidence_ref": "local stdout: status pass, checked sealed_remote_authority_api"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets"
    ],
    "purpose": "Rust all-target regression coverage for remote delivery, foundation, local commands, and transactions.",
    "outcome": "passed",
    "evidence_ref": "local stdout: 19 lib tests, 11 foundation tests, 6 local command tests, 20 transaction tests passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Rust warning-free lint gate for the touched construction crate.",
    "outcome": "passed",
    "evidence_ref": "local stdout: Finished dev profile without warnings"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "504"
    ],
    "purpose": "Typed C-SDLC issue validation after remote authority repairs.",
    "outcome": "passed",
    "evidence_ref": "generation 15 phase implemented status pass before this SOR replacement"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Exact-range diff hygiene for the PR branch.",
    "outcome": "passed",
    "evidence_ref": "local stdout: no findings"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
