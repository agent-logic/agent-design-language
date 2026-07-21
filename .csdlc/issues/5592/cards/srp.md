# Structured Review Prompt

Template: 1.0.0

Issue: 5592

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/operations.rs
adl-runtime-kernel/src/parity_b.rs
adl-runtime-kernel/src/reasoning.rs
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

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Runtime v3 is 13,547 physical lines, an explicitly accepted +1,338 exception over the pinned 12,209 baseline and +1,547 over the reviewed target; it remains below the 20,000 hard ceiling and exact review found the cohesive issue scope complete and non-deferred.

## Review Result

Revision: Some("git-blake3:b6a63af868ca140e817c485dc2c3491222ab4675:65b144b8ccfe0c11c263284ab0ec0d1e8b08b35728123b0bee8b575f615c5352")

Reviewer: Some("subagent:/root/review_5592_final_exact")

Result: pass
