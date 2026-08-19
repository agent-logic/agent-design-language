# Structured Planning Prompt

Template: 1.0.0

Issue: 415

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and bind #415, implement labeled atomic builder diagnostics and minimal runner retention, prove missing-tool and compatibility behavior, review, publish, shepherd, merge when authorized by the requested terminal outcome, finish, and clean.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Bind #415 and confirm exact diagnostics and cleanup boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Implement labeled atomic diagnostics and runner retention wiring.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Add missing-tool, success-compatibility, evidence, and exact-scope proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Obtain fresh review, publish, shepherd, merge, finish, and clean when green.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No paid cloud execution.
- No provider fallback or GPU path.
- No #268 or #269 lifecycle writes.
- Exact-owner cleanup remains unchanged.
- Durable evidence is redacted and repo-relative.

## Risks

- A compound shell still hides the exact failing executable.
- Early exit occurs before durable evidence publication.
- Diagnostic retention leaks secrets or machine-local paths.
- Test doubles prove markers rather than executing captured nonzero-command and cleanup-compatible behavior.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/415/design.md

Digest: 464d54f4988eaaeebc78a4cbb18f5d5b67ffe3766b4e68de6725f4e50b264c4d

## Diagram

.csdlc/prepared/issues/415/diagram.mmd

Digest: 89620afbe9a97d515c3628f684de26709cd53559a5c839ed26ea5df0d30bf463

## Stop Conditions

- Any step would launch AWS or mutate provider resources.
- Any step would mutate #268 or #269 lifecycle state.
- Exact output cannot be retained without leaking sensitive data.
- Typed validation, review, CI, or terminal ancestry fails.

## Handoff

Proceed only after doctor readiness.
