# Structured Review Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl/src/cli/csmctl_cmd.rs
docs/api/runtime-v3/v1/observatory.openapi.json
infra/runtime-v3/agents/ember.axioma.yaml
.csdlc/issues/602

## Prompts

- Can any unauthorized or conflicting request mutate durable or live roster state?
- Can persistence and in-memory roster truth split after any modeled failure?
- Does restart reload preserve exact admission and reject corrupt state?
- Does csmctl keep credentials out of argv output errors and persisted state?
- Does the live proof preserve Shepherd and avoid init mutation or restart for first add?

## Findings

[
  {
    "id": "602-review-name-authority",
    "severity": "p1",
    "summary": "Canonical agent naming was not originally enforced at Runtime admission and rehydration authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:a8de57cd3190a6193fee1cb22eb5ba244e9b1cfe:901281288bbf6e2ae9bfa0cfdc746524b1121bbb3255a994cfbb5683f24189d7",
    "route": null
  },
  {
    "id": "602-review-office-compatibility",
    "severity": "p2",
    "summary": "Office needed first-class persistence with an explicit non-conflicting legacy role compatibility boundary.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:a8de57cd3190a6193fee1cb22eb5ba244e9b1cfe:901281288bbf6e2ae9bfa0cfdc746524b1121bbb3255a994cfbb5683f24189d7",
    "route": null
  },
  {
    "id": "602-review-portable-live-config",
    "severity": "p2",
    "summary": "The live validator originally generated a machine-local retained config with textual substitution.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:a8de57cd3190a6193fee1cb22eb5ba244e9b1cfe:901281288bbf6e2ae9bfa0cfdc746524b1121bbb3255a994cfbb5683f24189d7",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live Wuji deployment is deferred until after PR publication; the reviewed focused local proof is complete.

## Review Result

Revision: Some("git-blake3:a8de57cd3190a6193fee1cb22eb5ba244e9b1cfe:901281288bbf6e2ae9bfa0cfdc746524b1121bbb3255a994cfbb5683f24189d7")

Reviewer: Some("codex-subagent:issue_602_review")

Result: pass
