# Structured Output Record

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented config-driven dynamic Ollama agent admission and lifecycle management through csmctl and proved the exact candidate live on Wuji.

## Artifacts

- .csdlc/prepared/issues/602/design.md
- .csdlc/prepared/issues/602/diagram.mmd
- .csdlc/prepared/issues/602/validate-focused.sh
- infra/runtime-v3/agents/ember.axioma.yaml
- .csdlc/prepared/issues/602/design.md
- .csdlc/prepared/issues/602/diagram.mmd
- .csdlc/prepared/issues/602/validate-focused.sh
- .csdlc/evidence/602/live-wuji-acceptance.md
- infra/runtime-v3/agents/ember.axioma.yaml

## Execution

- Added authenticated csmctl agent add --config plus list, get, checkpoint, dehydrate, migrate, rehydrate, and remove commands.
- Separated canonical two-part agent identity and display name from provider, model, endpoint, and first-class office configuration.
- Added durable Runtime dynamic-agent state, real Ollama execution, concurrent health, checkpoints with conversation continuity, two-phase migration, and legacy persisted-store compatibility.
- Updated the Runtime route inventory, Observatory OpenAPI contract, portable example configuration, and focused tests.
- Added authenticated csmctl agent add --config plus list, get, checkpoint, dehydrate, migrate, rehydrate, and remove commands.
- Separated canonical two-part agent identity and display name from provider, model, endpoint, and first-class office configuration; persisted records reject simultaneous current office and legacy role authority.
- Added durable Runtime dynamic-agent state, governed Ollama execution, health, checkpoints with conversation continuity, two-phase migration, and unambiguous legacy persisted-store compatibility.
- Rebased issue 602 onto current main so PR 614 contains only issue 602 work.

## Validation

[
  {
    "command": [
      "/bin/bash",
      "/Volumes/FastWork/adl-worktrees/adl-issue-602-runtime-csmctl-agent-add/.csdlc/prepared/issues/602/validate-focused.sh"
    ],
    "purpose": "Issue 602 focused implementation validation",
    "outcome": "passed",
    "evidence_ref": "focused-agent-lifecycle.log"
  },
  {
    "command": [
      "bash .csdlc/prepared/issues/602/validate-focused.sh",
      "exact-candidate csmctl add, duplicate, checkpoint, migrate, rehydrate, authenticated WSS inference, clean restart, and roster verification on Wuji"
    ],
    "purpose": "Prove issue 602 focused contracts and the required live Wuji lifecycle and gemma4:e4b-mlx inference acceptance.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/602/live-wuji-acceptance.md"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
