# Structured Intent Prompt

Template: 1.0.0

Issue: 571

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Correct the post-merge V3-A proof gaps from issue #500/PR #565 without rewriting historical review truth.

## Required Outcome

A bounded corrective patch makes V3-A predecessor coverage, construction-decision evidence, retained lifecycle gates, and diff-hygiene validation exact enough to gate downstream v3 work.

## Scope

- docs/csdlc-v3/CONTRACT.md
- docs/csdlc-v3/predecessor-coverage.json
- docs/csdlc-v3/proportional-lifecycle.json
- .csdlc/prepared/issues/500/validate-implementation.rb
- .csdlc/issues/571/**
- .csdlc/prepared/issues/571/**
- .csdlc/evidence/571/**
- narrowly related V3-A proof fixtures/tests only if needed to make the validator meaningful

## Authority

- #571 is a corrective follow-up to #500/#565 and must not rewrite the historical review result as passing.
- C-SDLC v2 remains the live lifecycle authority until explicit V3-F/#505 cutover.
- #571 must not implement V3-B/C/D/E/F behavior or move authority from v2 to v3.
- Validators must fail closed instead of accepting broad prose-only mappings or working-tree-only diff hygiene.

## Assumptions

- none

## Operator Constraints

- Keep this as a bounded corrective issue.
- Preserve #500/#565 historical truth: merged but reviewed as FAIL with follow-up required.
- Do not widen into authority cutover or later v3 runtime/lifecycle implementation.
- Produce exact owner issue and proof lane data for every retained #161-#163 requirement row.
