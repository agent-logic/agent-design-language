# Structured Task Prompt

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Authority store boundary slice only; transport and peripheral runtime migration are split into follow-on issues #259 and #260.

## Deliverables

- Sealed raw certificate, lease, and fencing store APIs.
- Authority store adapter facade and expanded published receipt view.
- Focused authority-boundary test and test-fixture token updates.

## Acceptance

1. AC-1: No raw certificate/lease/fencing store mutation path compiles without explicit authority or test fixture access token.
2. AC-2: Published receipt view exposes lineage, action class, adapter kind/version, published generation, result digest, and receipt digest.
3. AC-3: Focused authority boundary validation passes.
4. AC-4: Reviewer finds no actionable P1/P2 findings before publication.

## Dependencies

- #203 split decision
- #142 runtime remediation sprint

## Inputs

- .csdlc/evidence/203/provider-reviews/gemini-adapter-decomposition-result.json
- .csdlc/evidence/203/ISSUE_203_DECOMPOSITION_PLAN.md

## Non Goals

- Governed transport integration
- Migration/recovery/peripheral caller migration
- Closeout/background lifecycle bookkeeping
