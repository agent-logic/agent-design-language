# Structured Output Record

Template: 1.0.0

Issue: 617

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Runtime roster and detail projections expose authoritative canonical agent names, and the repaired retained proof now covers configured Shepherd construction plus the dynamic lifecycle, restart, persisted-state, checkpoint, and freeze-dried compatibility path.

## Artifacts

- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/agent_roster.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- infra/runtime-v3/runtime-init.toml
- .csdlc/evidence/617/remediation/canonical-agent-name-remediation.log

## Execution

- Carry dynamic AgentAdmissionRequest.name unchanged through AgentSample, AgentRuntimeEvidence, roster, and detail projections.
- Require and validate resident_shepherd.name in Runtime init configuration and route production construction through a directly tested config-to-feed helper.
- Add the required outbound name field to AgentRosterEntry and OpenAPI while retaining deserialization compatibility for previously persisted v1 entries.
- Keep checkpoint samples on a distinct compatibility schema so roster naming does not alter checkpoint or freeze-dried digest semantics.
- Expand the issue validator to retain both the four canonical/configuration/OpenAPI cases and the complete dynamic lifecycle compatibility case.

## Validation

[
  {
    "command": [
      "/bin/bash",
      ".csdlc/prepared/issues/617/validate-canonical-agent-name.sh"
    ],
    "purpose": "Prove canonical-name configuration/projection/OpenAPI behavior, production Shepherd construction, and the dynamic lifecycle compatibility path.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/617/remediation/canonical-agent-name-remediation.log"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration",
      "--test",
      "agent_roster",
      "--test",
      "control",
      "--test",
      "observatory",
      "--test",
      "openapi_contract",
      "--no-tests=fail"
    ],
    "purpose": "Prove the complete five-surface Runtime API regression denominator.",
    "outcome": "passed",
    "evidence_ref": "local-nextest:e0391facf18f6182155acb0833701c7eab2e42ba:94-passed"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--",
      "--check"
    ],
    "purpose": "Prove Rust formatting after review remediation.",
    "outcome": "passed",
    "evidence_ref": "local-command:issue-617:fmt-pass"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove branch diff whitespace hygiene after review remediation.",
    "outcome": "passed",
    "evidence_ref": "local-command:issue-617:diff-check-pass"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
