# Structured Task Prompt

Template: 1.0.0

Issue: 101

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the #101 route-selection guardrail, focused policy drift proof, and connector-403 regression fixture.

## Deliverables

- Explicit root route prohibition and owner mapping.
- Focused drift test comparing root policy with the client-boundary document.
- Connector-403 incident fixture and assertions that reject token-failure and fallback-authorized classifications.
- Focused validation and exact-head independent review evidence.
- One ready, non-merged PR closing #101 when eventually merged.

## Acceptance

1. AC-1: Root AGENTS explicitly prohibits connector and raw-gh covered lifecycle writes.
2. AC-2: Root guidance names csdlc-github-issue, csdlc-github-pr, csdlc-publish, and csdlc-finish as sole owners.
3. AC-3: Missing owner binaries fail closed without fallback.
4. AC-4: A focused deterministic fitness test detects root and boundary-document drift.
5. AC-5: A fixture proves connector 403 is integration authorization failure, not token failure or fallback authorization.
6. AC-6: Existing token precedence and redaction behavior remain unchanged.
7. AC-7: Issue creation through the typed owner and approved default resolver remains proven without exposing secrets.

## Dependencies

- C-SDLC v2 Rust GitHub owner binaries
- existing shared GitHub token resolver
- root AGENTS policy

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/101
- AGENTS.md
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
- csdlc-v2/src/github.rs
- csdlc-v2/src/github_token.rs
- csdlc-v2/tests/gate_github_actions.rs

## Non Goals

- Changing token discovery, precedence, propagation, or redaction.
- Authorizing connector writes or raw-gh fallback.
- Changing GitHub App permissions.
- Touching issue #100.
- Broad C-SDLC lifecycle refactoring.
