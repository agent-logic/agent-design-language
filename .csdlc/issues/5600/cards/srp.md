# Structured Review Prompt

Template: 1.0.0

Issue: 5600

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2
.csdlc/prepared/issues/5600/design.md
.csdlc/prepared/issues/5600/diagram.mmd
.csdlc/prepared/issues/5600/bootstrap.json
.csdlc/prepared/issues/5600/validate-local.json
.csdlc/evidence/5600

## Prompts

- Does every required planning collection have an explicit card-owned typed replacement operation?
- Can any failed operation change a card, generation, digest, projection, or audit event?
- Does acceptance coverage reject stale, missing, duplicate, and extra identifiers across STP, SPP, and VPP?
- Does the #5337 fixture prove a real preparation-to-implementation conversion without direct card edits?
- Are phase authorization and existing serialized operation compatibility preserved?

## Findings

[
  {
    "id": "F-5600-1",
    "severity": "p1",
    "summary": "Acceptance-set cardinality cannot change atomically across STP, SPP, and VPP.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5600-2",
    "severity": "p2",
    "summary": "Operator-constraint and acceptance-criteria replacements are not Bound-only.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5600-3",
    "severity": "p2",
    "summary": "The #5337 real JSON CLI proof covers only SRP review prompts.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5600-4",
    "severity": "p3",
    "summary": "The issue design assigns dependencies, repository inputs, and non-goals to the wrong card.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:16a916638428b519abe22ee665717ba8e097ddde:b9637c5a6bc99eead9886d42e18730890446f0dab92cb786f512d6f932d4767f")

Reviewer: Some("subagent:codex-exec-5600")

Result: changes_required
