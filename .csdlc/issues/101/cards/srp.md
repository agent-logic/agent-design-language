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
    "fix_revision": "git-blake3:fb2705e1ad3d019118611a76643b3e5e7bdd0b7c:5678fe75e1cfa4eb11811e2dbada73e0424a8bbce1af684dd302bf285a2e3a48",
    "route": null
  },
  {
    "id": "F-101-2",
    "severity": "p2",
    "summary": "The default HOME token-file fallback was created but never exercised by the regression test.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:fb2705e1ad3d019118611a76643b3e5e7bdd0b7c:5678fe75e1cfa4eb11811e2dbada73e0424a8bbce1af684dd302bf285a2e3a48",
    "route": null
  },
  {
    "id": "F-101-3",
    "severity": "p3",
    "summary": "The focused installation gate checked only one of the four declared lifecycle route owners.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:fb2705e1ad3d019118611a76643b3e5e7bdd0b7c:5678fe75e1cfa4eb11811e2dbada73e0424a8bbce1af684dd302bf285a2e3a48",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:fb2705e1ad3d019118611a76643b3e5e7bdd0b7c:5678fe75e1cfa4eb11811e2dbada73e0424a8bbce1af684dd302bf285a2e3a48")

Reviewer: Some("codex-subagent:019fe91f-8fd7-75b3-9eb1-c398807c839d")

Result: pass
