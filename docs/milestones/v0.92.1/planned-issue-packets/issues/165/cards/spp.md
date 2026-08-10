# Structured Planning Prompt

Template: 1.0.0

Issue: 165

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement one invocation-scoped App with reviewed narrow service traits, lazy credential-bearing adapters, cancellation-safe single-flight initialization, typed configuration/errors, and strict stdout/stderr redaction.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the Git, FileSystem, ProcessRunner, reviewer-identity, cancellation, configuration, error, and observability interfaces at the required checkpoint.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement production and strict-fake App constructors with no mutable global locator and lazy expensive or credential-bearing services.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement synchronous once-only result caching and asynchronous single-flight success/error caching with cancellation reset and bounded cooldown retry.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Wire root signal cancellation, child/task teardown, exit 130, and no-detached-work guarantees across supported consoles and operating systems.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Enforce machine stdout, diagnostic stderr, typed errors, secret/path redaction, and durable-output hygiene.",
    "acceptance_ids": [
      "AC-11",
      "AC-12"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run deterministic concurrency, leader-drop, waiter, cooldown, filesystem-mutation, signal, teardown, and redaction fixtures; stop on global mutation or surviving work.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Cancel and join all fixture tasks, terminate child processes, clear invocation-scoped credentials, and verify no global service or durable secret survives.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-04 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/165/design.md

Digest: 334ebfae1abd19464ff33204a5421d95206a58310d34049ea821ac322b272e53

## Diagram

.csdlc/prepared/issues/165/diagram.mmd

Digest: c595f7fba6cd1e45bb8f3f9dec61266794fbda5aa9e1c95c5071e827b447ffc4

## Stop Conditions

- A service requires global mutation, credentials enter state/config output, a detached task survives command completion, or local commands initialize network clients.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
