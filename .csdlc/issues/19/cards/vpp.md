# Validation Planning Prompt

Template: 1.0.0

Issue: 19

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/19/design.md

Diagram: .csdlc/prepared/issues/19/diagram.mmd

## Selected Lanes

[
  {
    "lane": "podcast-preview-source-contract",
    "proof_role": "Validate the podcast page, preview references, feed, artwork, and smoke-audio source packet before deployment",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "python3",
      "adl/tools/validate_podcast_launch_packet.py",
      "demos/podcast",
      "docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json",
      "--preview-root",
      "demos/_preview/podcast"
    ],
    "parallel_group": "preview-source",
    "defer_reason": null
  },
  {
    "lane": "podcast-preview-live-https",
    "proof_role": "Verify the deployed preview route returns the expected page over HTTPS",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "curl",
      "--fail",
      "--silent",
      "--show-error",
      "https://agent-logic.ai/_preview/podcast/"
    ],
    "parallel_group": "preview-live",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `python3 adl/tools/validate_podcast_launch_packet.py demos/podcast docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json --preview-root demos/_preview/podcast`
- `curl --fail --silent --show-error https://agent-logic.ai/_preview/podcast/`

## Failure Semantics

Fail closed on wrong-account identity, ambiguous resources, production-route mutation, digest mismatch, missing assets, sensitive evidence, or any EC2 requirement; do not widen deployment scope.

## Handoff

Retain typed evidence before convergence.
