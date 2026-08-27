# Structured Review Prompt

Template: 1.0.0

Issue: 499

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/resilience/tests.rs
.csdlc/issues/499 lifecycle metadata
focused per-file coverage proof for PR #547

## Prompts

- Does the implementation stay inside the declared unit boundary?
- Does every acceptance criterion have proving evidence?
- Are operator-only actions and private material kept outside Git?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the authoritative integration and changed-source coverage proof after the PR head is updated.

## Review Result

Revision: Some("git-blake3:3a9aa6db62eb966a3bd070e06a544f5cdc7b1626:02395d894ffd9889c2b2cbae2fe89c987f6818db8b6d9d585871f80529342a66")

Reviewer: Some("/root/review_499_final")

Result: pass
