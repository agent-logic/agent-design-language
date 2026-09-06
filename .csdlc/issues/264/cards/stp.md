# Structured Task Prompt

Template: 1.0.0

Issue: 264

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only issue #264 non-submission gate materials and truthfully preserve the operator authorization blocker.

## Deliverables

- Operator authorization template naming provider-specific actions that remain blocked
- Submission ledger initialized with no submissions and redaction-safe evidence shape
- Monitoring and rollback runbook for future authorized execution
- Blocked disposition summary for #51 parent coordination
- Issue-specific retained validation evidence

## Acceptance

1. AC-1: Terminal #261, #262, and #263 are validated as canonical prerequisites before #264 publication.
2. AC-2: The packet identifies Apple Podcasts, Spotify for Creators, Amazon Music for Podcasters, and YouTube RSS ingestion as provider targets without performing any provider action.
3. AC-3: The authorization template requires explicit future operator approval per provider before account, mailbox, verification, submission, status-monitoring, correction, rollback, or destination-link activation work.
4. AC-4: The initialized ledger records no submission and retains no credential, verification-code, private account, or unsupported acceptance material.
5. AC-5: Monitoring, correction, rollback, and destination-link activation rules preserve evidence and fail closed on duplicate shows, provider warnings, rejected states, payment/monetization prompts, personal-account recovery, or legal/rights uncertainty.
6. AC-6: #51 parent handoff says #264 is non-submission-complete but externally blocked, not provider-submitted or publicly launched.

## Dependencies

- Terminal #261
- Terminal #262
- Terminal #263
- Sprint 8 umbrella #536
- Future explicit operator authorization before any external provider action

## Inputs

- docs/milestones/v0.92/review/podcast_identity_261/show-identity.json
- demos/podcast/feed.xml
- demos/podcast/index.html
- docs/milestones/v0.92.1/review/podcast_directory_263
- .csdlc/issues/263/index.json

## Non Goals

- Directory submission
- Provider account access or mutation
- Mailbox verification-code use
- Website destination-link activation
- Public launch announcement
- Closing #51 without a truthful parent disposition
