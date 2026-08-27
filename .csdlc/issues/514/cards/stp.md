# Structured Task Prompt

Template: 1.0.0

Issue: 514

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one shared provider-profile contract; provider-specific checks are evidence inputs.

## Deliverables

- One shared provider inference-profile contract with deterministic Ollama materialization.
- Bounded validation evidence
- Exact-head review receipt

## Acceptance

1. AC-1: Profiles bind provider model and bounded parameters
2. AC-2: Invalid profiles fail before activation
3. AC-3: Last-known-good state is retained
4. AC-4: Credentials prompts and private payloads are excluded

## Dependencies

- #480 WP-01 merged opening gate

## Inputs

- agent-logic/agent-design-language#514
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#PROV-A
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md

## Non Goals

- MLX or Metal provider
- OCI packaging
