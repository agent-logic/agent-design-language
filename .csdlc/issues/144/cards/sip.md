# Structured Intent Prompt

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Repair WP-13 cognitive profiles with trusted signed authority, full revision-chain verification, and governed authority rotation before #5831 publication.

## Required Outcome

Cognitive profiles cannot self-authorize policy or evidence, every revision through genesis is verified, authority rotation is signed and monotonic, and exact local/native proof plus independent review pass.

## Scope

- Trusted cognitive-profile policy and evidence authority
- Complete predecessor-chain validation through genesis
- Governed authority rotation
- Focused cognitive-profile positive and adversarial proof
- Issue-specific exact-head native Linux/macOS receipts and validation

## Authority

- Provisioned verifying keys and signed authority statements are trust roots; caller-supplied policy and evidence are untrusted
- Legacy #5830 evidence is immutable historical proof and cannot authorize the repaired contract
- Issue #144 gates #5831 publication but does not edit adaptive-learning or Sprint 3 paths
- Global CI and unrelated Runtime v3 governance remain out of scope

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle commands and repo-owned binaries
- Work only in the issue-bound worktree and never mutate main
- Use a correctly assigned fresh exact-head reviewer before publication
- Publish a ready qualified-closing PR and merge only through typed finish after exact green proof
- Do not use raw gh, AWS, private temporary paths, Sprint 3, or unrelated closeout work
