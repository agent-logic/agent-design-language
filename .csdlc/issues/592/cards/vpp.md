# Validation Planning Prompt

Template: 1.0.0

Issue: 592

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/592/design.md

Diagram: .csdlc/prepared/issues/592/diagram.mmd

## Selected Lanes

[
  {
    "lane": "dependency-terminal-readback",
    "proof_role": "Use typed C-SDLC readback to prove #528 is terminal before any execution bind.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-github-issue",
      "run",
      "--request",
      ".csdlc/prepared/issues/592/read-528-request.json"
    ],
    "parallel_group": "preflight",
    "defer_reason": "#528 was non-terminal during issue-start canary."
  },
  {
    "lane": "vertex-config-docs",
    "proof_role": "Verify provider project location model and credential sourcing are explicit and redacted.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1600,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/592/validate-vertex-config-docs.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": "Implementation is gated by #528."
  },
  {
    "lane": "runtime-provider-proof",
    "proof_role": "Prove Polis uses the configured Vertex AI provider route without mocks or secret output.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/592/validate-runtime-vertex-ai.sh"
    ],
    "parallel_group": "runtime",
    "defer_reason": "Implementation is gated by #528 and later explicit live-call authorization if needed."
  },
  {
    "lane": "tooling-canary",
    "proof_role": "Retain real issue create read bootstrap validate and doctor evidence plus defects.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/592/validate-tooling-canary.sh"
    ],
    "parallel_group": "tooling",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `.adl/bin/csdlc-v2/csdlc-github-issue run --request .csdlc/prepared/issues/592/read-528-request.json`
- `bash .csdlc/prepared/issues/592/validate-vertex-config-docs.sh`
- `bash .csdlc/prepared/issues/592/validate-runtime-vertex-ai.sh`
- `bash .csdlc/prepared/issues/592/validate-tooling-canary.sh`

## Failure Semantics

Fail closed on non-terminal #528, credential exposure, ambient GCP defaults, mock-provider acceptance, raw gh dependency, or unrecorded tooling defects.

## Handoff

Retain typed evidence before convergence.
