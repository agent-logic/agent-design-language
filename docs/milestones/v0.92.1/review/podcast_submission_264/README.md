# Podcast Directory Submission Gate Packet — Issue #264

Status: non-submission gate complete. No provider submission has been performed.

This packet completes the repository-side work that can be done before provider-account action for `The Cognitive Stack`. Explicit future operator authorization is still required before Apple Podcasts, Spotify for Creators, Amazon Music for Podcasters, YouTube RSS ingestion, mailbox verification-code handling, correction, rollback, destination-link activation, or public-launch work begins.

## Bound inputs

- Show: `The Cognitive Stack`
- Website: `https://agent-logic.ai/podcast/`
- RSS feed: `https://agent-logic.ai/podcast/feed.xml`
- Contact mailbox: `podcast@agent-logic.ai`
- Directory runbook packet: `docs/milestones/v0.92.1/review/podcast_directory_263`
- Identity packet: `docs/milestones/v0.92/review/podcast_identity_261/show-identity.json`

## Packet files

- `operator-authorization-template.md` defines the exact future approval that must exist before any provider action.
- `submission-ledger.json` initializes all provider entries as `not_authorized`; no provider has been submitted.
- `monitoring-and-rollback.md` describes future monitoring, correction, rollback, and destination-link activation rules.
- `parent-51-handoff.md` gives #51 a truthful child-state handoff: repo-side #264 gate materials are complete, external submission remains blocked.

## Redaction and authority boundary

Do not retain credentials, verification codes, recovery codes, mailbox contents, cookies, tokens, or private screenshots. Do not activate destination links until the provider listing is live and verified.

Issue #51 remains open unless the operator explicitly accepts this blocked disposition for parent routing.
