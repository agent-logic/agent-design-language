# Structured Task Prompt

Template: 1.0.0

Issue: 511

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly one Observatory experience-design contract and its validation plan.

## Deliverables

- Stable per-view information contract
- Empty degraded recovery and revoked-state matrix
- Keyboard and screen-reader flow specification
- Runtime-field census with source references
- Reviewed OBS-A evidence packet
- .csdlc/prepared/issues/511/validate-obs-a-contract.sh
- .csdlc/prepared/issues/511/validate-obs-a-states.sh
- .csdlc/prepared/issues/511/validate-obs-a-accessibility.sh
- .csdlc/prepared/issues/511/validate-obs-a-runtime-fields.sh

## Acceptance

1. AC-1: Every view has a stable information contract
2. AC-2: Empty degraded recovery and revoked states are designed
3. AC-3: Keyboard and screen-reader flows are specified
4. AC-4: No invented Runtime field is introduced
5. AC-5: One-command pre-cutover canary passes with v2 authority and v3 local non-authority evidence

## Dependencies

- none

## Inputs

- agent-logic/agent-design-language#511
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#OBS-A
- demos/html-observatory/**
- docs/api/runtime-v3/**
- adl-runtime-kernel/**

## Non Goals

- Production implementation
- Unity TLS work
- Runtime API mutation
- Public exposure
