# Structured Output Record

Template: 1.0.0

Issue: 617

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Runtime roster and detail projections now expose authoritative canonical agent names while preserving operational IDs, display names, offices, legacy persisted reads, and checkpoint digest semantics.

## Artifacts

- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- infra/runtime-v3/runtime-init.toml

## Execution

- Carry dynamic AgentAdmissionRequest.name unchanged through AgentSample, AgentRuntimeEvidence, roster, and detail projections.
- Require and validate resident_shepherd.name in Runtime init configuration and pass it into the production Shepherd population feed.
- Add the required outbound name field to AgentRosterEntry and OpenAPI while retaining deserialization compatibility for previously persisted v1 entries.
- Keep checkpoint samples on a distinct compatibility schema so roster naming does not alter checkpoint or freeze-dried digest semantics.

## Validation

[
  {
    "command": [
      "/bin/bash",
      "/Volumes/FastWork/adl-worktrees/adl-issue-617-runtime-api-canonical-agent-names/.csdlc/prepared/issues/617/validate-canonical-agent-name.sh"
    ],
    "purpose": "Issue #617 focused canonical-agent-name implementation validation",
    "outcome": "passed",
    "evidence_ref": "canonical-agent-name.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
