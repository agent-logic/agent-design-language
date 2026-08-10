# Structured Review Prompt

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/cognitive_profile.rs
adl-runtime-kernel/tests/cognitive_profile.rs
adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs
docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
.csdlc/prepared/issues/144/produce-native-receipt.rb
.csdlc/prepared/issues/144/validate-native-receipts.rb
.github/workflows/wp13-authority-repair.yml
.csdlc/evidence/144
.csdlc/issues/144
Review the opaque runtime-owned authority establishment boundary, canonical policy/evidence pins, full genesis lineage replay, old-key-governed rotation, public API non-construction boundary, exact filtered native inventory, retained evidence bindings, and truthful lifecycle claims. Explicitly attempt replacement of both attacker root and proof; verify no caller-controlled trust root is accepted.

## Prompts

- Can a caller choose both the policy/evidence and the key that supposedly authorizes them?
- Does validation recompute every ancestor through genesis rather than trusting stored older digests?
- Can rotation be replayed, self-signed by the new key, or performed at the same or lower epoch?
- Do all rejection and retained-evidence paths avoid secret and machine-local leakage?
- Are native receipts and typed claims bound to the exact repaired revision?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
