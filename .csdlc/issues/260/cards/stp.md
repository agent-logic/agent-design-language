# Structured Task Prompt

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Pre-bind #260 preparation and later #260 caller migration only; production implementation starts only after #259 terminal and bind authority.

## Deliverables

- Execution-ready #260 initialized C-SDLC card bundle with explicit #259 bind gate.
- Design and diagram packet preserving #258/#259/#203 ownership boundaries.
- Post-#259 bind handoff packet naming exact branch, worktree, validation lanes, and stop conditions.
- adl-runtime/tests/distributed_authority_adapter_callers_260.rs

## Acceptance

1. AC-1: #260 cards state that bind/source implementation is blocked until #259 terminal, reconciled, and ancestral.
2. AC-2: #260 scope is limited to non-transport distributed Runtime caller migration to the governed adapter facade.
3. AC-3: #260 non-goals preserve #258 authority-store boundary, #259 transport binding, and #203 parent integration ownership.
4. AC-4: Validation lanes are focused on the touched distributed Runtime caller surfaces and do not claim parent #203 integration.
5. AC-5: Independent pre-bind design/card review finds no actionable readiness issue before typed design approval.

## Dependencies

- #191 terminal, reconciled, and ancestral
- #201 terminal, reconciled, and ancestral
- #202 terminal, reconciled, and ancestral
- #199 terminal, reconciled, and ancestral
- #200 terminal, reconciled, and ancestral
- #258 terminal, reconciled, and ancestral
- #259 terminal, reconciled, and ancestral before bind or production implementation

## Inputs

- GitHub issue #203 coordination body
- GitHub issue #258 terminal authority-store boundary
- GitHub issue #259 governed transport child
- GitHub issue #260 current body
- .git/csdlc-v2/derived-terminal/258.json

## Non Goals

- No authority-store boundary implementation owned by #258.
- No governed transport implementation owned by #259.
- No parent #203 integration, publication, or closeout.
- No #205 serving-eligibility authority work.
- No branch/worktree bind before #259 terminal and ancestral.
