# Structured Task Prompt

Template: 1.0.0

Issue: 182

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only DRT-02 within its exact owned paths and authority boundary.

## Deliverables

- Versioned positive and negative ACIP conformance vector corpus.
- Deterministic producer and independent replay verifier with exact digest contract.

## Acceptance

1. Canonical encode-decode-reencode is byte-stable for every supported message family.
2. Identity, authority, permit, causation, correlation, sequence, term, and polis bindings reject every declared mutation.
3. Duplicate, reordered, stale, malformed, unsigned, wrong-domain, and cross-polis messages produce typed deterministic outcomes.
4. Independent replay from retained inputs reproduces the exact committed outcome and digest.

## Dependencies

- DRT-01: issue #181

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-02
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Provisioning a distributed cluster
- Replacing ACIP implementation
- Accepting hard-coded assertion labels as producer proof
