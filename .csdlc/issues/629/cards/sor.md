# Structured Output Record

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Implemented V3-H.3 GitHub, PR-state, review, and publication construction routes under the single non-authoritative csdlc v3 binary, with synthetic receipts and ambiguous closing references failing closed before #505.

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
- Tightened publication body relation parsing so `Closes #6290` no longer satisfies the required `Closes #629` relation.
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
    "purpose": "Prove the current v3 Rust source is formatted after tightening publication relation parsing.",
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
      "remote_publication_commands"
    ],
    "purpose": "Prove remote publication planning rejects ambiguous closing references such as Closes #6290.",
    "outcome": "passed",
    "evidence_ref": "console: 5 passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove local whitespace hygiene after the relation-boundary fix.",
    "outcome": "passed",
    "evidence_ref": "console: no output"
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
