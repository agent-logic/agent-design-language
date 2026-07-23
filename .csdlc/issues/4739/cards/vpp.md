# Validation Planning Prompt

Template: 1.0.0

Issue: 4739

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/4739/design.md

Diagram: .csdlc/prepared/issues/4739/diagram.mmd

## Selected Lanes

[
  {
    "lane": "unity-mcp-alignment-contract",
    "proof_role": "Prove repository shell contract and focused static integration for the alignment surface",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_contract.sh"
    ],
    "parallel_group": "unity-mcp-static",
    "defer_reason": null
  },
  {
    "lane": "unity-mcp-diff-hygiene",
    "proof_role": "Prove bounded text and script hygiene for the issue-owned diff",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "unity-mcp-static",
    "defer_reason": null
  },
  {
    "lane": "unity-mcp-live-read-only",
    "proof_role": "Retain exact live project and endpoint alignment plus one read-only MCP result",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "node",
      "unity-mcp-cli",
      "status",
      "observatory-project"
    ],
    "parallel_group": "unity-mcp-live",
    "defer_reason": "Replace with the repository-owned alignment probe argv after S1 creates the issue-owned command; run only when the intended Unity project is available."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_v0916_unity_observatory_contract.sh`
- `git diff --check`
- `node unity-mcp-cli status observatory-project`

## Failure Semantics

Fail closed on project or endpoint ambiguity, fixed-port assumptions, cloud fallback, missing read-only proof, secret exposure, broad process scans, adjacent Unity scope, or unsupported readiness claims.

## Handoff

Retain typed evidence before convergence.
