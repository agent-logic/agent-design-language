# Structured Planning Prompt

Template: 1.0.0

Issue: 84

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add one native adapter and shared-contract projection, feed the existing Unity views with truthful connection state, prove authorization and reconnect behavior, and preserve native evidence.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement the shared-contract-derived compatibility resource and native HTTPS/WSS adapter.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind existing Unity views and controls to adapter state and real authorized behavior without visual redesign.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused contract tests and bounded reconnect coverage for ordering, cursor continuity, and authority invariants.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run live Unity Editor/player proof against Runtime v3 and review exact-head evidence within the strict path boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Unity has no independent Runtime schema
- No fixture is labeled live
- Read access never implies write authority
- Reconnect never duplicates events or escalates authority
- No private state or secrets enter Unity assets or evidence
- The approved Unity design remains intact

## Risks

- The Unity compatibility resource could drift into a schema fork
- Main-thread Unity lifecycle behavior could race reconnect callbacks
- Reconnect could duplicate events or replay commands
- Native proof could accidentally exercise a fixture or stale Runtime
- Secrets could leak into Unity logs or serialized assets

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/84/design.md

Digest: 7480307c05a79039b81fcff080ca59d789a39094644e68b68007a2668cf9ebd3

## Diagram

.csdlc/prepared/issues/84/diagram.mmd

Digest: ffb2173997cfba8bfa0981ad03c80801f9bea50147db6e328f69657a8a2d1768

## Stop Conditions

- The implementation requires changing Runtime, HTML, or shared coordinator paths
- The live endpoint does not match the approved Runtime v3 contract
- Required secrets would enter assets, builds, logs, screenshots, or repository files
- The approved Unity native environment is unavailable for final live proof
- Issue #5836 remains open when final implementation credit is requested

## Handoff

Proceed only after doctor readiness.
