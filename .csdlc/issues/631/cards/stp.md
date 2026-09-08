# Structured Task Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the V3-H.5 proof, shadow, soak, and install route behavior and issue-owned proof.

## Deliverables

- csdlc proof route behavior and tests
- csdlc shadow route behavior and tests
- csdlc soak route behavior and tests
- csdlc install route behavior and tests
- One-binary command manifest update
- Issue-owned validation script and retained evidence

## Acceptance

1. AC-1: Proof manifests validate deterministic lanes, durable evidence refs, stale evidence, and missing-manifest failures.
2. AC-2: Shadow compares bounded v2/v3 observations where parity is claimed and refuses broad unproven equivalence.
3. AC-3: Soak classifies bounded evidence without hidden state, live provider side effects, or authority claims.
4. AC-4: Install produces one stable csdlc artifact plan with source provenance, selected binary digest, selector metadata, and #505 cutover gating.
5. AC-5: Tests cover positive proof, missing manifest, stale evidence, parity mismatch, soak evidence gaps, install provenance mismatch, and fail-closed selector errors.
6. AC-6: No v2 source changes.

## Dependencies

- #625 sprint umbrella
- #627 V3-H.1 command denominator

## Inputs

- agent-logic/agent-design-language#631
- csdlc-v3/AGENTS.md
- docs/csdlc-v3/v3-command-manifest.json
- csdlc-v3/src/main.rs

## Non Goals

- Perform #505 cutover
- Retire v2
- Merge or close #505
- Run live provider soak
- Change v2 source code
