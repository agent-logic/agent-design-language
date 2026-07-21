# Structured Review Prompt

Template: 1.0.0

Issue: 5592

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/operations.rs
adl-runtime-kernel/src/parity_b.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/parity_b_live_kernel.rs
.csdlc/issues/5592/cards/sor.values.json
.csdlc/issues/5592/cards/srp.values.json
.csdlc/issues/5592/cards/vpp.values.json

## Prompts

- Does every live-credit claim require a real initialized adl-runtime-kernel process through the reviewed #5591 ingress rather than fixture, library, metadata, or fixed-bootstrap evidence?
- Can loop checkpoint/resume ever reset a bound, duplicate an effect, evade cancellation, or continue after shutdown?
- Can any untrusted task/tool/retrieval/model content create or steer affect, curiosity, review, policy, budget, mutation, or actuation authority?
- Are affect and theory-of-mind claims explicitly bounded to typed control/task-model surfaces with no subjective-state overclaim?
- Can cognition, accepted-risk review, adaptation, replay, or restart widen capability or bypass Freedom Gate, shutdown, resource limits, or human review?
- Does signed mutation bind exact before-state, policy, sequence, delta, expiry, and one-shot consumption with atomic recovery and rollback?
- Does every owned feature row have one truthful proof-bearing disposition, with metadata/context/schema-only rows denied live credit?
- Are Runtime v2, AWS, publication, cutover, deletion, other-lane ownership, claim collisions, and budget/proof weakening excluded?

## Findings

[
  {
    "id": "mutation-recovery-omits-gate-state",
    "severity": "p1",
    "summary": "Parity-B checkpoint omits authenticated MutationGate graph, consumed grants, evidence, and adaptation state, permitting grant reuse and adaptive rollback after restart.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "live-cancellation-and-inflight-shutdown-unreachable",
    "severity": "p1",
    "summary": "Loop execution uses an unreachable fresh cancellation token and does not recheck shutdown before receipt commit, so actual cancellation and in-flight monotonic shutdown are unproved.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The accepted 13,504-line exception must remain explicit in final review and SOR truth.

## Review Result

Revision: Some("git-blake3:e0276c61a3d409a87170da96ee359cb4e8b424c5:01d41ac8cc0ccb6970f62cdb7fb980d32927b1506a824c123e91678d961c398c")

Reviewer: Some("subagent:/root/review_5592_repaired_exact")

Result: changes_required
