# Structured Review Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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
.csdlc/prepared/issues/602

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
    "fix_revision": "git-blake3:d34fa0eff9e3f8e3086c4e16b5e3c09d2738db46:072cbdd32707a7e4f5f505684eda18fc9679945675c926ce70ce82200f9b0e3e",
    "route": null
  },
  {
    "id": "602-review-office-compatibility",
    "severity": "p2",
    "summary": "Office needed first-class persistence with an explicit non-conflicting legacy role compatibility boundary.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d34fa0eff9e3f8e3086c4e16b5e3c09d2738db46:072cbdd32707a7e4f5f505684eda18fc9679945675c926ce70ce82200f9b0e3e",
    "route": null
  },
  {
    "id": "602-review-portable-live-config",
    "severity": "p2",
    "summary": "The live validator originally generated a machine-local retained config with textual substitution.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d34fa0eff9e3f8e3086c4e16b5e3c09d2738db46:072cbdd32707a7e4f5f505684eda18fc9679945675c926ce70ce82200f9b0e3e",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live Wuji deployment is deferred until after PR publication; the reviewed focused local proof is complete.

## Review Result

Revision: Some("git-blake3:d34fa0eff9e3f8e3086c4e16b5e3c09d2738db46:072cbdd32707a7e4f5f505684eda18fc9679945675c926ce70ce82200f9b0e3e")

Reviewer: Some("codex-subagent:issue_602_review")

Result: pass
