# Structured Task Prompt

Template: 1.0.0

Issue: 592

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly one future Polis GCP Vertex AI configuration task plus pre-cutover tooling canary evidence.

## Deliverables

- Explicit Vertex AI provider configuration plan
- Redacted credential sourcing documentation
- Runtime/provider validation plan
- Failure-mode classification
- Real-issue tooling canary evidence

## Acceptance

1. AC-1: #528 is terminal before execution begins
2. AC-2: Polis GCP configuration uses Vertex AI through explicit provider configuration, not ambient defaults
3. AC-3: Vertex AI project location model and credential sourcing are documented and redacted
4. AC-4: Runtime/provider validation proves the configured route without printing or committing secrets
5. AC-5: Failure modes distinguish missing credentials disabled Vertex APIs project/location mismatch quota/auth errors and model errors
6. AC-6: Evidence captures the pre-cutover tooling canary defects found while creating and bootstrapping this real issue

## Dependencies

- #528 terminal

## Inputs

- agent-logic/agent-design-language#592
- agent-logic/agent-design-language#528
- adl-runtime-kernel/**
- infra/runtime-v3/**
- docs/runtime/**

## Non Goals

- Implement #528
- Live paid GCP calls without explicit authorization
- C-SDLC v2/v3 authority cutover
- Mock provider acceptance evidence
