# Validation Planning Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/122/design.md

Diagram: .csdlc/prepared/issues/122/diagram.mmd

## Selected Lanes

[
  {
    "lane": "public-exposure-policy",
    "proof_role": "Exact issue-owned local policy target for serial gates, non-gating topology, resource allowlist, forbidden compute, canonical hostnames, origin policy, revision binding, redaction, ownership, rollback, cleanup, and review prerequisites.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/validate_public_observatory_exposure.sh",
      "--local-only"
    ],
    "parallel_group": "local",
    "defer_reason": "Issue-owned target is intentionally created only after the distributed Runtime is terminal and separate operator AWS authorization exists."
  },
  {
    "lane": "public-runtime-gateway-policy",
    "proof_role": "Exact issue-owned local gateway target for HTTPS, WSS, authentication, CORS, origin, rate-limit, redaction, health, and revision contracts.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/validate_public_runtime_gateway.sh",
      "--local-only"
    ],
    "parallel_group": "local",
    "defer_reason": "Issue-owned target is intentionally created only after the distributed Runtime is terminal and separate operator AWS authorization exists."
  },
  {
    "lane": "authorized-live-public-proof",
    "proof_role": "Operator-authorized live proof of DNS, ACM, HTTPS, WSS, exact revision parity, business-account ownership, rollback, cleanup, and absence of forbidden compute.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "bash",
      "adl/tools/validate_public_observatory_exposure.sh",
      "--live"
    ],
    "parallel_group": "live",
    "defer_reason": "Live AWS proof is prohibited until distributed Runtime terminal state and separate operator authorization; this preparation performs no AWS action."
  },
  {
    "lane": "issue-diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts in the eventual issue implementation.",
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
    "parallel_group": "static",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/validate_public_observatory_exposure.sh --local-only`
- `bash adl/tools/validate_public_runtime_gateway.sh --local-only`
- `bash adl/tools/validate_public_observatory_exposure.sh --live`
- `git diff --check`

## Failure Semantics

Fail closed on an open serial gate, absent operator authorization, wrong account, forbidden compute, revision drift, invalid browser trust, permissive origin or authentication policy, unbounded traffic, disclosure, ownership ambiguity, incomplete rollback or cleanup, failed proof, or unresolved exact-head finding.

## Handoff

Retain typed evidence before convergence.
