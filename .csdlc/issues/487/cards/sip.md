# Structured Intent Prompt

Template: 1.0.0

Issue: 487

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one operational AWS audit and security baseline.

## Required Outcome

One operational audit and security baseline with owned retention and findings destinations.

## Scope

- AWS account-foundation audit/security controls
- CloudTrail and account activity visibility
- Configuration-recording posture
- Detection findings owner and destination routing
- Access-analysis readback
- Retention, encryption, alerting, cost, and redaction guardrails

## Authority

- Use the Agent Logic business AWS profile agent-logic-admin for any AWS readback or approved apply
- Do not use personal/default AWS account state
- Do not mutate AWS until #486 is terminal and the exact #487 plan is reviewed
- Do not retain credentials, token material, or unnecessary sensitive identifiers

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Use standard runners only for hosted CI
- Preserve #486 bootstrap ownership and do not absorb later AWS-E/F/G scope
