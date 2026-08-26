# Structured Intent Prompt

Template: 1.0.0

Issue: 291

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement typed initialized-phase post-decomposition card recovery in csdlc-edit without rewriting history or mutating #114.

## Required Outcome

A typed csdlc-edit recovery route can atomically repair stale initialized decomposed-parent card semantics while preserving CAS, audit history, unbound topology, and historical design/diagram evidence.

## Scope

- csdlc-edit initialized decomposition recovery request/result
- atomic CAS recovery and typed failure diagnostics
- selected semantic card fields for SIP/STP/SPP/VPP/SRP/SOR
- read-only #114 generation 35 golden fixture validation
- focused tests and operator documentation

## Authority

- csdlc-edit owns card value hydration, rendering, projection, and audit adjacency
- Recovery operates only on initialized issue records unless a later issue extends it
- The #114 fixture is read-only and must not be mutated by #291
- Historical design and diagram bytes are preserved and not reclassified as current implementation authority
- The route cannot bind, publish, merge, close out, write GitHub state, mutate #112, or implement product behavior

## Assumptions

- none

## Operator Constraints

- Use typed v2 bootstrap, doctor, bind, review, and publication owners
- Bind under /Volumes/FastWork/adl-worktrees before implementation
- Do not mutate #114 product/card state, root lock state, #112, #203, #122, #256, #84, AWS, or decomposition graphs
- Obtain #119-compliant fresh-session exact-head review before publication
- Publish a ready PR only through typed owner and stop before merge
