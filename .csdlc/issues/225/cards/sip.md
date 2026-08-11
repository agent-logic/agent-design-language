# Structured Intent Prompt

Template: 1.0.0

Issue: 225

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Permit two exact typed card corrections needed to resolve PR #224 review without direct edits or premature binding.

## Required Outcome

The typed editor atomically corrects recovered SPP plan summaries and initialized/ready SIP operator constraints under exact phase, card, topology, truth, audit, and rendering guards.

## Scope

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/prepared/issues/225
- .csdlc/issues/225

## Authority

- Only csdlc-edit apply owns both semantic corrections
- correct_plan_summary_after_recovery owns only recovered implemented SPP plan_summary
- correct_operator_constraints_before_bind owns only initialized/ready unbound SIP operator_constraints
- csdlc-review remains sole review/publication recovery authority
- Binding, execution, publication, merge, terminal, and cleanup authority are unchanged

## Assumptions

- none

## Operator Constraints

- Never hand-edit cards or canonical issue state
- Do not bind or start WP-20
- Do not add generic planning mutation authority
- Use focused FastWork Rust validation
- Publish a ready PR closing #225 and stop before merge
