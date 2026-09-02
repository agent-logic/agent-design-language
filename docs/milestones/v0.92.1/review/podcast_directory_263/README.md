# Podcast Directory Preflight Packet — Issue #263

This packet prepares directory submission work for `The Cognitive Stack` without submitting the show, mutating provider accounts, publishing directory listings, storing credentials, or retaining verification codes.

## Bound launch inputs

- Show title: `The Cognitive Stack`
- Show URL: `https://agent-logic.ai/podcast/`
- RSS feed: `https://agent-logic.ai/podcast/feed.xml`
- Public contact mailbox: `podcast@agent-logic.ai`
- Identity packet: `docs/milestones/v0.92/review/podcast_identity_261/show-identity.json`
- Hosting/feed validator: `.csdlc/prepared/issues/262/validate-podcast-hosting.rb`

## Packet files

- `provider-runbooks.md` records Apple Podcasts, Spotify for Creators, Amazon Music for Podcasters, and YouTube RSS-ingestion steps from official provider instructions sampled on 2026-09-02.
- `operator-preflight.md` separates repo-ready inputs from account-side operator actions.
- `submission-ledger.schema.json` defines the post-submission ledger shape for #264 without storing secrets.
- `.csdlc/prepared/issues/263/validate-directory-runbooks.rb` validates this packet and the exact #261/#262 launch inputs.

## Boundary

#263 ends at an executable preflight packet. No directory submission is performed here. #264 owns actual account-side submission, verification-code handling, provider IDs, and directory status updates.
