# Structured Review Prompt

Template: 1.0.0

Issue: 101

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:a5f698b581105b58129a4a406bc10f62747babb1:8e84d77b3ec8df8ed477cf46bb49f539a4dc1a75e68a1fce2018c3c3fd6413df")

Reviewer: Some("openai-codex:independent-issue-101-final-review")

Result: pass
