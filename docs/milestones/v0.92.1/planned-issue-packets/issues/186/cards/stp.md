# Structured Task Prompt

Template: 1.0.0

Issue: 186

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only DRT-06 within its exact owned paths and authority boundary.

## Deliverables

- Observatory coherent-cut and causal-trace proof packet.
- Stale ownership, stale read, split-view, redaction, and singleton negative evidence.

## Acceptance

1. Exactly one Observatory owns the quorum lease at any instant and successor binding follows old-lease expiry.
2. Every displayed operation correlates agent, node, polis, identity, authority, trace, term, commit index, and state revision.
3. Partitions and leadership changes cannot present stale authority as current or combine an incoherent cut.
4. Secrets, credentials, private legal data, and unredacted provider payloads never appear in retained or visible evidence.

## Dependencies

- DRT-03: issue #183
- DRT-04: issue #184

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-06
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Implementing new Observatory features
- Using screenshots as sole proof
- Allowing multiple active owners
