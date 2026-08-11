# Structured Review Prompt

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/5830/produce-native-receipt.rb
.csdlc/prepared/issues/5830/validate-native-receipts.rb
.github/workflows/wp13-native-cognitive-profile.yml
.csdlc/prepared/issues/144/produce-native-receipt.rb
.csdlc/prepared/issues/144/validate-native-receipts.rb
.github/workflows/wp13-authority-repair.yml
adl-runtime-kernel/src/cognitive_profile.rs
adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs
.csdlc/evidence/144
.csdlc/issues/144
Review the formally widened generic WP-13 compatibility repair after failed run 31422154377. Confirm both distinct producer/validator pairs execute and validate the same exact filtered fifteen-test internal authority lane while retaining their own issue, evidence namespace, workflow identity, run/job, artifact, digest, source-manifest, semantic-equivalence, and path-hygiene gates. Confirm generic #5830 cards/evidence are unchanged and opaque-authority product source remains the previously approved implementation.

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

- Replacement native Linux and macOS receipts must pass both distinct WP-13 aggregate validators at the exact republished head before merge.

## Review Result

Revision: Some("git-blake3:e749241965aad8359385020e02ad95aeb655aece:ce60d2db8561603088bfd8ed2df05c6f6642d6d8d2b50a8729a9b316eeeb13f3")

Reviewer: Some("/root/sprint4_5857/review_144_exact_head")

Result: pass
