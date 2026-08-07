# Structured Task Prompt

Template: 1.0.0

Issue: 19

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Upload the existing preview page and only its required static dependencies, perform a bounded CloudFront invalidation if needed, and retain redacted digest-bound live proof.

## Deliverables

- Live HTTPS preview page at https://agent-logic.ai/_preview/podcast/
- Required preview artwork, scripts, feed target, and smoke-audio asset in S3
- Redacted local-to-deployed SHA-256 manifest and object metadata receipt
- Live route, noindex, asset, audio, and production-non-mutation verification

## Acceptance

1. AC-1: The preview URL returns HTTP 200 and the expected Synthetic Minds page
2. AC-2: The page retains noindex,nofollow and remains absent from public navigation
3. AC-3: Every required local asset resolves over HTTPS with the intended content type and matching digest
4. AC-4: The production /podcast/ route is not modified or promoted
5. AC-5: Retained evidence proves the approved Agent Logic AWS profile, bounded S3/CloudFront mutations, and no EC2 operation without exposing sensitive infrastructure identifiers
6. AC-6: One bounded exact-head review has no unresolved actionable findings

## Dependencies

- Existing Agent Logic S3 and CloudFront website infrastructure
- Operator authorization to use AWS storage and delivery but not EC2

## Inputs

- demos/_preview/podcast/index.html
- demos/podcast/LAUNCH_READINESS.md
- demos/podcast/feed.xml
- demos/podcast/audio/meet-the-ai-coworkers.wav
- demos/podcast/studio

## Non Goals

- Production /podcast/ deployment or promotion
- Final episode production or podcast-directory submission
- Mailbox verification or public launch claims
- Page redesign
- Any EC2 or remote-build operation
