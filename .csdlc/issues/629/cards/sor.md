# Structured Output Record

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the V3-H.3 GitHub, PR-state, review, and publication command surfaces as non-authoritative v3 construction routes, with the GitHub/readback and publication-authority routes intentionally fail-closed until production typed review and authenticated GitHub adapter ingestion exists.

## Artifacts

- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/main.rs
- csdlc-v3/tests/command_manifest.rs
- csdlc-v3/tests/remote_publication_commands.rs
- csdlc-v3/tests/real_issue_canary.rs
- docs/csdlc-v3/v3-command-manifest.json
- .csdlc/prepared/issues/629/design.md
- .csdlc/prepared/issues/629/diagram.mmd
- .csdlc/prepared/issues/629/validate-v3-h3-github-publication.sh
- .csdlc/issues/629

## Execution

- Implemented #629-owned route handlers for github, github-issue, github-pr, pr-state, publish, and review under the single v3 csdlc binary.
- Kept every #629 route non-authoritative before #505 cutover; GitHub/readback and publication routes return fail-closed findings when authority would depend on caller-provided receipts.
- Added fail-closed detection for caller-forged GitHub adapter readbacks and caller-attested typed review/publication receipts.
- Kept credential-name redaction in route reports.
- Preserved the v3 command manifest truth that #629 GitHub/publication routes are fail_closed/not_live construction routes, not operational authority.
- Added and updated focused command-manifest, remote-publication, and real-issue canary tests for the fail-closed boundary.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--check"
    ],
    "purpose": "Prove the current v3 Rust source is formatted after the fail-closed remote/publication repairs.",
    "outcome": "passed",
    "evidence_ref": "console: no output"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "remote_publication_commands",
      "--test",
      "real_issue_canary"
    ],
    "purpose": "Prove the #629 remote/publication command routes and real-issue canaries reject caller-forged authority while preserving non-authoritative planning output.",
    "outcome": "passed",
    "evidence_ref": "console: 11 passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/629/validate-v3-h3-github-publication.sh",
      "all"
    ],
    "purpose": "Run the #629 issue-owned validator for route surface, manifest, and fail-closed behavior.",
    "outcome": "passed",
    "evidence_ref": "console: v3 h3 github/publication validator: pass"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy across csdlc-v3 targets after the fail-closed authority changes.",
    "outcome": "passed",
    "evidence_ref": "console: Finished dev profile"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove local whitespace hygiene before committing the SOR truth repair.",
    "outcome": "passed",
    "evidence_ref": "console: no output"
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
