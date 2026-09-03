# Structured Task Prompt

Template: 1.0.0

Issue: 660

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only issue #660: emergency removal of unintended public /podcast/ exposure and hidden-preview correction.

## Deliverables

- Public /podcast/ and related feed/media/site keys hidden by S3 delete markers and CloudFront invalidation.
- Hidden /_preview/podcast/ page updated to The Cognitive Stack with noindex and no public feed/media links.
- Retained evidence for the exact public-delete manifest, CloudFront invalidations, live URL checks, and negative authority boundaries.
- Local machine-readable validator for the emergency rollback packet.

## Acceptance

1. AC-1: https://agent-logic.ai/podcast/ returns a non-serving status for current podcast content.
2. AC-2: https://agent-logic.ai/podcast/feed.xml and the known public media/page objects return non-serving statuses for current podcast content.
3. AC-3: https://agent-logic.ai/_preview/podcast/ returns the current The Cognitive Stack page with noindex,nofollow.
4. AC-4: The hidden preview page contains no public feed, public audio, old show-name, or production provider submission link claims.
5. AC-5: The retained rollback evidence records exact S3 keys, delete marker version IDs, CloudFront invalidation IDs, and live HTTP status checks.
6. AC-6: Evidence records that no provider submission, provider directory mutation, private archive deletion, S3 version purge, or credential/private receipt retention occurred.

## Dependencies

- Operator emergency instruction to fix the unintended public exposure now.
- Existing Agent Logic public website distribution and origin bucket from demos/podcast/S3_CLOUDFRONT_RUNBOOK.md.
- The prior #264/#51 podcast launch boundary, carried forward as context only.

## Inputs

- demos/podcast/S3_CLOUDFRONT_RUNBOOK.md
- demos/_preview/podcast/index.html
- demos/podcast/studio/uploads/agent-logic-logo.svg
- demos/podcast/LAUNCH_READINESS.md
- GitHub issue #660

## Non Goals

- Podcast provider directory submission.
- Public launch announcement.
- Changing provider account credentials or mailbox state.
- Purging historical S3 versions.
- Deleting or changing the private podcast archive bucket.
- Closing #51.
