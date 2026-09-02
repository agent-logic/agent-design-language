# Structured Task Prompt

Template: 1.0.0

Issue: 51

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare #51 parent closeout readiness only; do not close #51 in this preparation step.

## Deliverables

- Parent closeout readiness packet.
- Child truth and #264 PR #649 status snapshot.
- Operator blocked-disposition acceptance gate.
- Focused validator for parent closeout readiness.

## Acceptance

1. AC-1: The packet identifies #261, #262, and #263 as closed with retained repository evidence.
2. AC-2: The packet identifies #264 PR #649 as green/mergeable but not merged at preparation time.
3. AC-3: The packet requires explicit operator acceptance of #264's blocked external-action disposition before #51 parent closeout.
4. AC-4: The packet makes no provider-submission, provider-acceptance, public-launch, or destination-link activation claim.
5. AC-5: The next worker can execute #51 without rediscovering the child graph or stale #51 mini-sprint plan.

## Dependencies

- Merged and terminal #261
- Merged and terminal #262
- Merged and terminal #263
- Merged and terminal #264 PR #649 or explicit operator decision to leave #51 open
- Explicit operator acceptance before parent closeout on blocked external-action disposition

## Inputs

- docs/milestones/v0.92/review/podcast_identity_261
- docs/milestones/v0.92.1/review/podcast_directory_263
- docs/milestones/v0.92.1/review/podcast_submission_264
- .csdlc/issues/261/index.json
- .csdlc/issues/262/index.json
- .csdlc/issues/263/index.json
- .csdlc/issues/264/index.json

## Non Goals

- Provider directory submission.
- Provider account access or mutation.
- Mailbox verification-code use.
- Website destination-link activation.
- Public launch announcement.
- Closing #51 before #264 terminal truth and operator disposition acceptance.
