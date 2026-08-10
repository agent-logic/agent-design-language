# Structured Output Record

Template: 1.0.0

Issue: 101

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a fail-closed GitHub lifecycle route guardrail with synchronized root and boundary policy, a credential-free connector-403 fixture, installer-owner coverage, typed default-token issue reconciliation, and focused resolver regression proof.

## Artifacts

- AGENTS.md
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
- csdlc-v2/tests/gate_github_route_policy.rs
- csdlc-v2/tests/fixtures/github_connector_403.json
- .csdlc/evidence/101

## Execution

- Named csdlc-github-issue, csdlc-github-pr, csdlc-publish, and csdlc-finish as the sole covered GitHub lifecycle owners in root and boundary policy.
- Prohibited connector and raw-gh lifecycle writes and made missing owner binaries a fail-closed condition without fallback authority.
- Added a focused policy drift test, connector-403 classification fixture, verified-installation publication-owner assertion, and direct shared-token precedence and redaction regression test without changing resolver implementation.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "0608764d902b02eb2965002168ae210059866e8e",
      "33fad0d3bc70c8701a811670d8254bdef374289b"
    ],
    "purpose": "Check the exact issue base through corrected implementation head for whitespace and patch errors.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/101/diff-hygiene-exact.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_route_policy"
    ],
    "purpose": "Run the corrected dedicated issue #101 Rust integration target with installer-owner coverage.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/101/github-route-policy-review-fix.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-github-issue",
      "run",
      "--request",
      "/Volumes/FastWork/csdlc-101-default-token-read.json"
    ],
    "purpose": "Read back issue #101 and reconcile its typed creation operation marker through the approved default token resolver without retaining token contents.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/101/typed-default-token-issue-read.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_route_policy"
    ],
    "purpose": "Prove the authoritative boundary lists the dedicated route-policy target and the target enforces that hook alongside all issue guardrails.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/101/github-route-proof-hook-review-fix.log"
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
