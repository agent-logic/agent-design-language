# Structured Review Prompt

Template: 1.0.0

Issue: 213

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/doctor.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs
.csdlc/issues/213
.csdlc/prepared/issues/213
.csdlc/locks/213.lock

## Prompts

- Are only guarded initialized/ready STP replace_acceptance_criteria and SPP replace_plan_steps newly authorized, with ordinal AC IDs and pending-only pre-bind steps?
- Does the literal #205 sequence atomically refresh both current canonical design/diagram bindings while rejecting exact path/reference/card-identity drift?
- Does either successful repair set review pending without changing phase/topology, including the narrower ready-phase no-later-evidence and fresh-CAS guard?
- Do before/after byte snapshots prove cross-card coverage, CAS, atomic rendering, generation, audit prefix, rollback, and untouched state preservation?
- Do explicit fixtures preserve existing bound STP/SPP and implemented SPP behavior, and does the committed base-to-source diff lane cover EOF hygiene?
- Did the change leave #205 and every publication/merge/terminal boundary untouched?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:c46f231b7f0ee1f52b08a52dac0000fb6cefd6b7:7694749d5e2c7a79df548bddbb979ffeb74d8a46c86430c0ebf60067ea46d8ec")

Reviewer: Some("/root/review_213_c46f23")

Result: pass
