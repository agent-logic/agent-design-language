# Structured Review Prompt

Template: 1.0.0

Issue: 5830

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/cognitive_profile.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/cognitive_profile.rs
adl-runtime-kernel/tests/fixtures/cognitive_profile
docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
.csdlc/prepared/issues/5830/produce-native-receipt.rb
.csdlc/prepared/issues/5830/validate-native-receipts.rb
.github/workflows/wp13-native-cognitive-profile.yml
.csdlc/evidence/5830
.csdlc/issues/5830

## Prompts

- Does every profile value cite an allowed current evidence digest and preserve identity, continuity, actor, reason, and prior revision linkage?
- Can stale, forbidden, mismatched, or private evidence influence any internal or public projection?
- Can any label imply diagnosis, reputation, standing, rights, citizenship, personhood, or consciousness?
- Are #5827, #5828, #5829, bounded prerequisite evidence, and all acceptance claims current at exact HEAD?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Exact-head native Linux and macOS receipt production and semantic-equivalence validation remain mandatory at publication before merge.

## Review Result

Revision: Some("git-blake3:3f8c4eed489cd3da55086b76b842e2b5a88722d3:50a5e4c73f97093485bb7d178f9ddf7d5b82edd28334dee53351cbd4e3eccaad")

Reviewer: Some("/root/sprint4_5857/review_5830_exact_head")

Result: pass
