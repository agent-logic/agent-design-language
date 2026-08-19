# Validation Planning Prompt

Template: 1.0.0

Issue: 426

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/426/design.md

Diagram: .csdlc/prepared/issues/426/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csmctl-linux-lifecycle",
    "proof_role": "Prove Darwin routing, Linux start/status/restart/stop, unsupported-platform refusal, foreign and stale process-identity denial, and Linux documentation coverage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_csmctl_linux_backend.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "gemini-exact-head-review",
    "proof_role": "Obtain the required hosted Gemini exact-head findings-first review independently from the deterministic shell proof.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 6000,
    "argv": [
      ".adl/bin/adl-provider-adapter",
      "--request",
      ".csdlc/evidence/426/gemini-review/request.json",
      "--out",
      ".csdlc/evidence/426/gemini-review/result.json",
      "--log",
      ".csdlc/evidence/426/gemini-review/run.jsonl"
    ],
    "parallel_group": "hosted-review",
    "defer_reason": "Run after the final substantive commit with the approved hosted Gemini credential; typed csdlc-review remains lifecycle authority."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_csmctl_linux_backend.sh`
- `.adl/bin/adl-provider-adapter --request .csdlc/evidence/426/gemini-review/request.json --out .csdlc/evidence/426/gemini-review/result.json --log .csdlc/evidence/426/gemini-review/run.jsonl`

## Failure Semantics

Fail closed on unsupported OS, ambiguous PID ownership, readiness failure, or review findings.

## Handoff

Retain typed evidence before convergence.
