# Structured Task Prompt

Template: 1.0.0

Issue: 262

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only issue #262: Podcast production hosting, RSS, enclosures, and playback.

## Deliverables

- The production feed, enclosure metadata, byte-range behavior, and representative desktop/mobile playback are source-grounded, digest-consistent, and rollback-safe.
- Issue-specific retained validation evidence
- Exact-head review and truthful terminal record

## Acceptance

1. AC-1: The canonical feed validates with no local, preview, placeholder, smoke-test, or fixture URLs.
2. AC-2: Every enclosure is stable HTTPS media with correct MIME type, bytes, duration, GUID, date, and digest.
3. AC-3: HEAD, GET, and 206 byte-range behavior plus representative desktop/mobile playback pass.
4. AC-4: Feed, artwork, show metadata, and episode metadata match #261 and #342 exactly.
5. AC-5: Rollback preserves episode packages and prior evidence; exact-head review is clean.

## Dependencies

- Terminal #261
- Terminal #342 episode packages
- Sprint 8 umbrella #536

## Inputs

- demos/podcast/feed.xml
- docs/milestones/v0.92.1/evidence/podcast/51-b
- adl/tools/record_podcast_native_playback.sh
- adl/tools/record_podcast_browser_playback.mjs
- adl/tools/record_podcast_ios_safari_playback.sh
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- .csdlc/prepared/issues/536/sprint-execution-packet.yaml

## Non Goals

- Show identity decisions
- Episode production
- Directory submission
- Public launch announcement
