# Validation Planning Prompt

Template: 1.0.0

Issue: 492

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/492/design.md

Diagram: .csdlc/prepared/issues/492/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-gcp-org-billing-packet",
    "proof_role": "Proves #492 design packet readiness, #491 terminal dependency gate, owned-path boundaries, scoped-policy and corporate-ownership invariants, unchanged POC boundary, and lifecycle distinction between pre-bind packet proof and future implementation/live readback proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "prebind-gcp-org-readback-static",
    "proof_role": "Proves the GCP-C readback entrypoint has a static non-credentialed mode and reports cloud-mutation and credential-retention posture without requiring GCP credentials.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh",
      "--lane=static"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "prebind-review-readiness",
    "proof_role": "Proves #492 has issue-owned executable packet validators before design approval; this does not claim final implementation exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "gcp-c-organization-static",
    "proof_role": "After bind, verifies implemented organization/billing Terraform, runbook, proof packet, scoped-policy invariant, corporate group ownership, labels, budgets/export, and unchanged POC boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh",
      "--phase=postbind"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #492 is bound and implementation creates the complete organization/billing baseline and evidence surfaces."
  },
  {
    "lane": "gcp-c-readback",
    "proof_role": "After reviewed implementation and approved GCP context, reconciles live GCP organization/billing readbacks against the baseline with redacted retained evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh",
      "--lane=inventory-readonly"
    ],
    "parallel_group": "gcp-readonly",
    "defer_reason": "Deferred until #492 is bound, reviewed baseline exists, and approved GCP read-only context is selected without exposing credential contents."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh`
- `bash .csdlc/prepared/issues/492/run-gcp-c-readbacks.sh --lane=static`
- `bash .csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh`
- `bash .csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh --phase=postbind`
- `bash .csdlc/prepared/issues/492/run-gcp-c-readbacks.sh --lane=inventory-readonly`

## Failure Semantics

Fail closed if scoped policy impact cannot be proven, individual-only ownership remains, cost attribution is absent, existing POC resources would change without explicit admission, live GCP readback would require credential disclosure, or downstream GCP-D/GCP-E/XCL work would be absorbed.

## Handoff

Retain typed evidence before convergence.
