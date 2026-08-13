# Structured Task Prompt

Template: 1.0.0

Issue: 265

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bootstrap and review the #265 design/card packet only; stop before bind, implementation, publication, merge, closeout, or parent/sibling mutation.

## Deliverables

- .csdlc/prepared/issues/265/readiness-packet.md
- .csdlc/prepared/issues/265/design.md
- .csdlc/prepared/issues/265/diagram.mmd
- .csdlc/prepared/issues/265/validate_preparation_bundle.py
- Typed #265 bootstrap record and rendered SIP/STP/SPP/VPP/SRP/SOR cards
- Fresh design/card review and typed design approval if review passes

## Acceptance

1. AC-1: #265 bootstrap records the live issue identity and #112 child role without creating a branch or worktree.
2. AC-2: Cards state that #265 depends on terminal+ancestral #112 before bind or implementation.
3. AC-3: Scope is limited to Runtime kernel conversation ingress authority enforcement before side effects.
4. AC-4: Non-goals exclude #112 authority invention, #270 served API/recipient acknowledgement protocol, durable transcript storage, Observatory/UI, #115 room/UI behavior, cloud exposure, and publication/merge/closeout.
5. AC-5: Design and validation plan remain bootstrap/design-only until #112 terminal gate clears.
6. AC-6: Preparation validator, doctor, and validate pass for the bootstrapped packet.
7. AC-7: Fresh design/card review has no unresolved actionable findings before design approval.

## Dependencies

- #112 terminal and ancestral to execution base
- #270 follows #265 and must not be absorbed here
- #114/#276/#277/#278 durable-history chain remains downstream and out of scope

## Inputs

- agent-logic/agent-design-language#112
- agent-logic/agent-design-language#265
- agent-logic/agent-design-language#270
- .git/csdlc-v2/requests/issue112-typed-read-for-265-readiness-20260813T1128Z.result.json
- .git/csdlc-v2/requests/issue265-typed-read-for-readiness-20260813T1128Z.result.json
- .git/csdlc-v2/requests/issue270-typed-read-for-265-readiness-20260813T1128Z.result.json

## Non Goals

- Binding #265 or creating a #265 worktree
- Implementing Runtime ingress enforcement in this design/bootstrap step
- Defining #112 authority primitives or identity-message contract
- Implementing #270 served API or recipient acknowledgement protocol
- Implementing durable transcript storage, Observatory/UI, #115 room/UI behavior, or cloud exposure
- Mutating #112 parent/prep, #270, #276, #277, #278, or #115
- Publishing, merging, or closing #265
