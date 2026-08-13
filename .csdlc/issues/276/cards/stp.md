# Structured Task Prompt

Template: 1.0.0

Issue: 276

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Refresh #276 preparation truth on a clean current-main root, prove canonical #112/#265/#270 terminal-cache dependencies, obtain fresh design/readiness review, and bind only the dedicated #276 worktree under FastWork if PASS.

## Deliverables

- .csdlc/prepared/issues/276/readiness-packet.md
- .csdlc/prepared/issues/276/design.md
- .csdlc/prepared/issues/276/diagram.mmd
- .csdlc/prepared/issues/276/validate_preparation_bundle.py
- Typed #276 initialized/ready unbound record and rendered SIP/STP/SPP/VPP/SRP/SOR cards
- Fresh design/readiness review and typed design approval before bind

## Acceptance

1. AC-1: #276 records live issue identity and #114 child role without initially creating a branch or worktree.
2. AC-2: Cards state that #276 depends on terminal+ancestral #112, #265, and #270 before implementation.
3. AC-3: Dependency gates are proven through canonical derived-terminal caches for #112/#265/#270 whose merge SHAs are ancestral to current origin/main.
4. AC-4: Scope is limited to durable journal schema/storage/migrations/corruption/retention/deletion foundation.
5. AC-5: Non-goals exclude acknowledgement-watermark semantics, replay reconciliation, public history APIs, Observatory restoration, parent #114 integration proof, and any redefinition of Layer 8 authority or #270 acknowledgement trust.
6. AC-6: Preparation validator, doctor, and validate pass for the refreshed packet.
7. AC-7: Fresh design/readiness review has no unresolved actionable findings before design approval and bind.
8. AC-8: Bind creates only the dedicated #276 branch/worktree under /Volumes/FastWork/adl-worktrees and does not bind #114 parent.

## Dependencies

- #112 terminal and ancestral to execution base
- #265 terminal and ancestral to execution base
- #270 terminal and ancestral to execution base
- #114 parent remains ready/unbound coordination owner
- #277 and #278 follow #276 and must not be absorbed here

## Inputs

- agent-logic/agent-design-language#114
- agent-logic/agent-design-language#276
- agent-logic/agent-design-language#112
- agent-logic/agent-design-language#265
- agent-logic/agent-design-language#270
- .git/csdlc-v2/derived-terminal/112.json
- .git/csdlc-v2/derived-terminal/265.json
- .git/csdlc-v2/derived-terminal/270.json

## Non Goals

- Binding #114 parent
- Implementing #277 watermarks/idempotency/replay/receipts
- Implementing #278 history APIs or Observatory restoration
- Mutating #114 parent staging or any #112/#265/#270 lifecycle/code surface
- Publishing, merging, or closing #276 during preparation/bind
