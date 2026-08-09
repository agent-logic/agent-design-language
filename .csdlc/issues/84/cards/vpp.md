# Validation Planning Prompt

Template: 1.0.0

Issue: 84

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/84/design.md

Diagram: .csdlc/prepared/issues/84/diagram.mmd

## Selected Lanes

[
  {
    "lane": "unity-runtime-v3-contract-tests",
    "proof_role": "Prove schema compatibility, ordering, cursor continuity, authorization refusal, and explicit failure-state logic.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3500,
    "argv": [
      "bash",
      "adl/tools/validate_v092_unity_observatory_live.sh",
      "--contract-tests-only"
    ],
    "parallel_group": "unity-contract",
    "defer_reason": "This contract runner and its Unity test target are issue #84 implementation deliverables; the lane becomes mandatory after those targets are created."
  },
  {
    "lane": "unity-observatory-live-native",
    "proof_role": "Prove the real Unity Editor/player against Runtime v3 HTTPS/WSS, including controls, refusal, disconnect, and reconnect.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_unity_observatory_live.sh",
      "--live"
    ],
    "parallel_group": "live-unity",
    "defer_reason": "This live runner is an issue #84 implementation deliverable and requires the approved Unity Editor/player plus the exact live Runtime candidate."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed changes and path-boundary drift.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/validate_v092_unity_observatory_live.sh --contract-tests-only`
- `bash adl/tools/validate_v092_unity_observatory_live.sh --live`
- `git diff --check`

## Failure Semantics

Fail closed into an explicit read-only, stale, denied, version-mismatch, or unavailable state; never synthesize live success or a Unity-only compatibility path.

## Handoff

Retain typed evidence before convergence.
