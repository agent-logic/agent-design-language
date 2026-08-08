# Structured Intent Prompt

Template: 1.0.0

Issue: 47

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prevent named Rust VPP lanes from silently fanning out through Cargo substring matching while preserving truthful intentional broad commands.

## Required Outcome

Typed planning accepts exact target-bounded and intentional broad Rust test commands, rejects ambiguous named selectors before execution, and proves the schema lane runs nonzero intended unit tests without selecting estimation_contracts.

## Scope

- Typed VPP Rust test-selector classification and validation
- Actionable invalid-selector diagnostics
- Focused exact-schema, exact-integration, broad-command, and invalid-selector regression proof
- Active VPP/editor/planning skill and runbook guidance affected by selector syntax

## Authority

- Issue #47 owns validation-selector semantics only
- Broad commands remain broad when explicitly declared without a misleading named substring
- The unrelated estimation_contracts behavior remains unchanged
- Issue #5881 claim-removal behavior and records remain out of scope

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Use only typed C-SDLC v2 Rust lifecycle tools
- Do not weaken, skip, or modify unrelated tests
- Keep selector policy in manifests/planning rather than ordinary test logic
- Treat estimates as reviewable rather than hard implementation limits
