# Structured Intent Prompt

Template: 1.0.0

Issue: 745

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Complete source-grounded v0.92.1 ADR curation and document native-v3 defect #744.

## Required Outcome

Four new Proposed ADRs, one Deferred ADR update, complete eight-topic mapping, reviewed and merged documentation PR.

## Scope

- docs/architecture/adr/0072-csdlc-v3-native-authority.md
- docs/architecture/adr/0073-validated-configuration-snapshot-reload.md
- docs/architecture/adr/0074-runtime-generation-source-ownership.md
- docs/architecture/adr/0075-provider-profile-and-shadow-authority.md
- docs/architecture/adr/0069-observatory-governed-runtime-consumer-boundary.md
- docs/architecture/adr/README.md
- docs/milestones/v0.92.1/ADR_PLAN_v0.92.1.md
- .csdlc/prepared/issues/745
- .csdlc/issues/745
- .csdlc/evidence/745

## Authority

- Operator explicitly authorized bounded typed v2 recovery after native-v3 issue-create failure.
- Native v3 remains default; #744 owns retained-intent repair.
- ADR publication does not grant architecture acceptance or release admission.

## Assumptions

- none

## Operator Constraints

- Preserve the native v3 intent and concurrent work.
- No runtime, cloud, provider, or credential-value changes.
