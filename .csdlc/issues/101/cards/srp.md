# Structured Review Prompt

Template: 1.0.0

Issue: 101

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

AGENTS.md
docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
csdlc-v2/tests/gate_github_route_policy.rs
csdlc-v2/tests/fixtures/github_connector_403.json
.csdlc/issues/101
.csdlc/prepared/issues/101
.csdlc/evidence/101

## Prompts

- Can any covered write still be routed through the connector or raw gh?
- Do missing owner binaries fail closed?
- Will material root/boundary drift fail the focused test?
- Does the 403 fixture distinguish integration authorization from token failure without secrets?
- Did the diff avoid token resolver implementation changes and issue #100?

## Findings

[
  {
    "id": "F-101-1",
    "severity": "p2",
    "summary": "Connector-403 regression was self-asserting instead of coupled to the incident fields and authoritative policy.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:3a6913d5774edd645c2ed0872154afc409953478:cf57e0e2cd38aa8882c4927553b26cd9c07cf76f430437f365bb0af8433d522a",
    "route": null
  },
  {
    "id": "F-101-2",
    "severity": "p2",
    "summary": "The default HOME token-file fallback was created but never exercised by the regression test.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:3a6913d5774edd645c2ed0872154afc409953478:cf57e0e2cd38aa8882c4927553b26cd9c07cf76f430437f365bb0af8433d522a",
    "route": null
  },
  {
    "id": "F-101-3",
    "severity": "p3",
    "summary": "The focused installation gate checked only one of the four declared lifecycle route owners.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:3a6913d5774edd645c2ed0872154afc409953478:cf57e0e2cd38aa8882c4927553b26cd9c07cf76f430437f365bb0af8433d522a",
    "route": null
  },
  {
    "id": "F-101-4",
    "severity": "p2",
    "summary": "Strict Clippy rejected a needless borrow in the focused route-policy test.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:3a6913d5774edd645c2ed0872154afc409953478:cf57e0e2cd38aa8882c4927553b26cd9c07cf76f430437f365bb0af8433d522a",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:3a6913d5774edd645c2ed0872154afc409953478:cf57e0e2cd38aa8882c4927553b26cd9c07cf76f430437f365bb0af8433d522a")

Reviewer: Some("codex-subagent:019fe91f-8fd7-75b3-9eb1-c398807c839d")

Result: pass
