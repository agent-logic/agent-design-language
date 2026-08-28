# Structured Task Prompt

Template: 1.0.0

Issue: 342

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only issue #342: Podcast Studio first ten episode packages.

## Deliverables

- Ten complete episode-package directories pass package, audio, metadata, redaction, editorial, and digest checks while production feed and deployment remain outside this issue.
- Issue-specific retained validation evidence
- Exact-head review and truthful terminal record

## Acceptance

1. AC-1: All ten episode packages contain every required script, audio, transcript, note, metadata, artwork, enclosure fragment, redaction, QA, and review artifact.
2. AC-2: Audio and manifest digests, duration, sample rate, channels, loudness, peak, ID3, artwork, listen check, and archive records agree.
3. AC-3: Episode enclosure fragments reject local paths, drafts, unstable GUIDs, and metadata mismatches without mutating the production feed.
4. AC-4: Rights, consent, synthetic-voice provenance, and redaction remain truthful and privacy-safe.
5. AC-5: Source-SHA-bound playback receipts and exact-head review pass before terminal completion.

## Dependencies

- Sprint 8 umbrella #536
- Terminal #261 canonical show identity and rights inputs
- Retained Podcast Studio v2 proof and approved route/storage decision

## Inputs

- demos/podcast/episode-packages
- docs/milestones/v0.92.1/evidence/podcast/wp-24a
- adl/tools/generate_podcast_launch_packet.py
- adl/tools/validate_podcast_launch_packet.py
- adl/tools/test_podcast_launch_packet.sh
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- .csdlc/prepared/issues/536/sprint-execution-packet.yaml

## Non Goals

- Production feed ownership
- Hosting or deployment
- Directory submission
- Mailbox verification
- Public launch
