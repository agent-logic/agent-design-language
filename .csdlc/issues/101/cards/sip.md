# Structured Intent Prompt

Template: 1.0.0

Issue: 101

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the repo-native Rust GitHub owner route unmistakable and mechanically reject policy drift that could authorize connector or raw-gh lifecycle writes.

## Required Outcome

Root policy and focused deterministic proof identify the sole Rust owners for issue, PR-state, publication, and finish operations; connector 403 remains an integration authorization failure and never becomes token-failure or fallback authority.

## Scope

- AGENTS.md
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
- csdlc-v2/tests/gate_github_route_policy.rs
- csdlc-v2/tests/fixtures/github_connector_403.json
- .csdlc/issues/101
- .csdlc/prepared/issues/101
- .csdlc/evidence/101

## Authority

- csdlc-github-issue owns covered issue create, read, update, comment, and close operations.
- csdlc-github-pr owns direct PR-state observation.
- csdlc-publish owns PR publication and csdlc-finish owns terminal delivery.
- The GitHub connector is read-only for this workflow and raw gh is not lifecycle authority.
- Missing owner binaries fail closed and do not authorize fallback.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only.
- Use FastWork for the bound worktree and build artifacts.
- Do not use the GitHub connector or raw gh for lifecycle writes.
- Preserve shared token resolver precedence and redaction behavior.
- Do not touch issue #100.
- Do not merge the publication PR.
