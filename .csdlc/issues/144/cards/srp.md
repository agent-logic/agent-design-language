# Structured Review Prompt

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/144/validate-native-receipts.rb
.github/workflows/wp13-authority-repair.yml
.csdlc/prepared/issues/144/produce-native-receipt.rb
.csdlc/evidence/144
.csdlc/issues/144
Review the exact libtest-json-plus authority inventory repair against failed native run 31421144797. Confirm the validator expects the exact observed 15 structured names without weakening count, receipt digests, source-head, producer, workflow/run/job, artifact, semantic-equivalence, manifest, or path-hygiene checks. Confirm product/source remains unchanged from the approved opaque-authority revision.

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
