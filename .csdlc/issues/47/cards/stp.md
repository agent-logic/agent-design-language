# Structured Task Prompt

Template: 1.0.0

Issue: 47

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add exact versus broad Rust selector semantics at the typed planning boundary, focused proof, diagnostics, and directly affected active guidance.

## Deliverables

- Typed exact, broad, and invalid Rust selector classification
- Planning-time diagnostics for missing, conflicting, or ambiguous target selectors
- csdlc-v2/tests/validation_selectors.rs
- csdlc-v2/src/cards.rs validator integration
- csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md

## Acceptance

1. AC-1: A named schema lane uses an exact Cargo target boundary and selects a nonzero set of schema unit tests
2. AC-2: The exact schema lane cannot select or launch the unrelated estimation_contracts integration target
3. AC-3: Exact integration target commands such as --test gate2 remain accepted
4. AC-4: Intentional broad commands with no misleading trailing substring remain accepted and retain broad semantics
5. AC-5: Free substring, missing target name, and conflicting target selector shapes fail during planning or validation with an actionable corrected-command diagnostic
6. AC-6: Active VPP/editor/planning skills and runbooks use the corrected selector shape and distinguish exact from broad intent
7. AC-7: Implementation creates csdlc-v2/tests/validation_selectors.rs, then the focused selector test, exact schema unit-test lane, typed validation, and strict lint proof pass within reviewable budgets

## Dependencies

- Current C-SDLC v2 VPP validation-lane model
- Existing schema unit tests and estimation_contracts integration target
- Observed #5881 selector defect evidence

## Inputs

- AGENTS.md
- csdlc-v2/AGENTS.md
- csdlc-v2/src/cards.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/operator/skills
- docs/tooling

## Non Goals

- Changing #5881 claim-removal behavior or records
- Weakening or skipping estimation_contracts
- Replacing Cargo's test runner
- Making every broad test command target-exact
- Unrelated validation scheduling or CI changes
