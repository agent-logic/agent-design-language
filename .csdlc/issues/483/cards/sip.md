# Structured Intent Prompt

Template: 1.0.0

Issue: 483

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce a concise read-only corporate custody register and action list for the critical-service denominator without mutating external services.

## Required Outcome

A reviewed PR for #483 containing a complete custody/change register, redacted readback receipts, explicit follow-up ownership, and no live domain, DNS, account, secret, or infrastructure mutations.

## Scope

- read-only custody register derived from merged CORP-A critical-asset schedule
- five completed Route53 registration transfer receipts as factual evidence
- explicit action list for remaining custody, hosted-zone, billing, admin, MFA, recovery, vault, and break-glass gaps
- redaction and focused document validation

## Authority

- Do not transfer domains, hosted zones, accounts, secrets, infrastructure, or service control.
- Do not mutate DNS, billing, administrators, MFA, recovery, vault, break-glass, or provider settings.
- Do not schedule or gate this milestone on v-*.ai backlog domains, including v-dev.ai.
- CORP-C/#497 owns later operational-control transfer; #483 owns the custody register and action list only.
- Record credentials, PII, payment data, tax data, private instruments, and recovery materials only as excluded categories.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 and FastWork worktree.
- Docs-only; no live service mutations.
- v-dev.ai and all v-*.ai transfers are unscheduled backlog.
- Keep validation focused to JSON/Markdown/redaction/diff checks.
- Publish with Closes #483 after exact-head review.
