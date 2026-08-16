# Structured Task Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair the #115 preparation packet on a clean current-main root, prove canonical #111/#112/#113/#270 terminal-cache dependencies, obtain fresh readiness/design review, and stop before bind, implementation, publication, merge, closeout, or parent mutation.

## Deliverables

- .csdlc/prepared/issues/115/readiness-packet.md
- .csdlc/prepared/issues/115/design.md
- .csdlc/prepared/issues/115/diagram.mmd
- .csdlc/prepared/issues/115/validate_preparation_bundle.py
- Typed #115 initialized/ready unbound record and rendered cards
- Fresh design/readiness review and typed design approval only if review passes

## Acceptance

1. AC-1: #115 records live issue identity and #110 child role without branch/worktree.
2. AC-2: Cards preserve dependencies on #111, #112, #113, and #270, including the #270 reconciliation marker.
3. AC-3: Dependency gates are proven through canonical derived-terminal caches for #111/#112/#113/#270 whose merge SHAs are ancestral to current origin/main.
4. AC-4: Scope is limited to governed rooms, explicit participants, routing, delivery states, and proof.
5. AC-5: Non-goals exclude hidden broadcast, browser-selected recipients, cross-Polis federation, and lifecycle publication/closeout.
6. AC-6: #115 remains unbound until fresh readiness/design review PASS and later explicit bind authority.
7. AC-7: Preparation validator, doctor, and validate pass for the recovered packet.

## Dependencies

- #111 terminal and ancestral to execution base
- #112 terminal and ancestral to execution base
- #113 terminal and ancestral to execution base
- #270 terminal and ancestral to execution base

## Inputs

- .git/csdlc-v2/requests/issue115-typed-read-canonical-recovery-20260813T1705Z.result.json
- .csdlc/prepared/issues/110/graph.json
- .git/csdlc-v2/derived-terminal/111.json
- .git/csdlc-v2/derived-terminal/112.json
- .git/csdlc-v2/derived-terminal/113.json
- .git/csdlc-v2/derived-terminal/270.json

## Non Goals

- Unbounded broadcast
- Implicit recipient selection by browser
- Cross-Polis federation policy
- Redefining #112 authority or #270 acknowledgement trust
- Branch/worktree bind, implementation, publication, merge, or closeout
- Mutating #110 parent staging or #114/#276/#277/#278 lifecycle state
