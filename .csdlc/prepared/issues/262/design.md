# Issue 262 Design — Podcast production hosting, RSS, enclosures, and playback

## Goal

Publish and validate the canonical production podcast feed and stable HTTPS media enclosures from approved identity and terminal episode packages.

## Required Outcome

The production feed, enclosure metadata, byte-range behavior, and representative desktop/mobile playback are source-grounded, digest-consistent, and rollback-safe.

## Ownership

- `demos/podcast/feed.xml`
- `docs/milestones/v0.92.1/evidence/podcast/51-b`
- `adl/tools/record_podcast_native_playback.sh`
- `adl/tools/record_podcast_browser_playback.mjs`
- `adl/tools/record_podcast_ios_safari_playback.sh`

## Dependencies

- Terminal #261
- Terminal #342 episode packages
- Sprint 8 umbrella #536

## Safety Boundary

- This issue owns only the listed result and paths.
- All external mutations and private material remain governed by the operator constraints.
- Validation and exact-head review precede publication.

## Non-Goals

- Show identity decisions
- Episode production
- Directory submission
- Public launch announcement
