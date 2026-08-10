# Structured Review Prompt

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/cognitive_profile.rs
adl-runtime-kernel/tests/cognitive_profile.rs
adl-runtime-kernel/tests/fixtures/cognitive_profile
docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
.csdlc/prepared/issues/144/produce-native-receipt.rb
.csdlc/prepared/issues/144/validate-native-receipts.rb
.github/workflows/wp13-authority-repair.yml
.csdlc/evidence/144
.csdlc/issues/144

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
