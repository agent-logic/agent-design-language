# Structured Output Record

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented config-driven dynamic Ollama agent admission, lifecycle management, checkpointing, freeze-dried migration and rehydration through csmctl and Runtime v3.

## Artifacts

- .csdlc/prepared/issues/602/design.md
- .csdlc/prepared/issues/602/diagram.mmd
- .csdlc/prepared/issues/602/validate-focused.sh
- infra/runtime-v3/agents/ember.axioma.yaml

## Execution

- Added authenticated csmctl agent add --config plus list, get, checkpoint, dehydrate, migrate, rehydrate, and remove commands.
- Separated canonical two-part agent identity and display name from provider, model, endpoint, and first-class office configuration.
- Added durable Runtime dynamic-agent state, real Ollama execution, concurrent health, checkpoints with conversation continuity, two-phase migration, and legacy persisted-store compatibility.
- Updated the Runtime route inventory, Observatory OpenAPI contract, portable example configuration, and focused tests.

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
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
