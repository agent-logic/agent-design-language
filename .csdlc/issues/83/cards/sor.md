# Structured Output Record

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and integrated the live HTML Observatory Runtime v3 client and shared signed ACIP identity-message path for Layer 8 and agent communication, including durable replay continuity, browser keylessness, recipient-signed acknowledgements, qualified-time freshness, and attempt-local cross-carrier reservation truth.

## Artifacts

- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/tests/production_acip_wss.rs
- demos/html-observatory/app.js
- adl/tools/validate_v092_html_observatory_live.mjs
- docs/api/runtime-v3/v1/observatory.openapi.json
- /Volumes/FastWork/adl-issue-83-826e378f2-v2/evidence/observatory-layer8-chat-826e378f270e-22a51f53-1964-4ebf-b7af-21b1b9e61ae3-report.json

## Execution

- Integrated the HTML Observatory with the live Runtime roster, qualified capture time, authenticated Layer 8 chat, recipient signature verification, reconnect, and fail-closed states.
- Implemented one JCS-canonical signed identity-message contract for Layer 8 and agent-to-agent communication with durable sender and acknowledgement replay watermarks.
- Provisioned distinct external Runtime communication identities without browser or repository private-key material.
- Returned attempt-local canonical-ingress reservation disposition so concurrent carriers cannot claim another submission's replay watermark.
- Corrected the VPP continuity lane to run the implemented exact restart test and removed the false #5836 publication gate.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run the full runtime-kernel unit denominator including the deterministic cross-carrier reservation regression.",
    "outcome": "passed",
    "evidence_ref": "exact-head local output: 47 passed at 826e378f270ea36edef4183430baea7c9ec5eb7e"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "assembly",
      "communication_replay_and_ack_sequences_survive_process_cycles_without_snapshot_restore",
      "--",
      "--exact"
    ],
    "purpose": "Run the corrected exact acknowledgement restart-continuity lane.",
    "outcome": "passed",
    "evidence_ref": "exact-head local output: 1 passed at 826e378f270ea36edef4183430baea7c9ec5eb7e"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "production_acip_wss",
      "production_binary_acip_wss_produces_observed_receipt",
      "--",
      "--exact"
    ],
    "purpose": "Prove the production binary ACIP WebSocket path after reservation-disposition remediation.",
    "outcome": "passed",
    "evidence_ref": "exact-head local output: 1 passed at 826e378f270ea36edef4183430baea7c9ec5eb7e"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bins",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings in the changed Runtime library and binary surfaces.",
    "outcome": "passed",
    "evidence_ref": "exact-head local output at 826e378f270ea36edef4183430baea7c9ec5eb7e"
  },
  {
    "command": [
      "node",
      "adl/tools/validate_v092_html_observatory_live.mjs"
    ],
    "purpose": "Run the exact committed Runtime and Observatory through trusted public TLS in managed Chrome, including restart and replay continuity.",
    "outcome": "passed",
    "evidence_ref": "/Volumes/FastWork/adl-issue-83-826e378f2-v2/evidence/observatory-layer8-chat-826e378f270e-22a51f53-1964-4ebf-b7af-21b1b9e61ae3-report.json"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "83"
    ],
    "purpose": "Validate typed issue structure and card projections.",
    "outcome": "passed",
    "evidence_ref": "csdlc.doctor.report.v1 generation 66"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed diff output.",
    "outcome": "passed",
    "evidence_ref": "clean exact-head remediation diff"
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
