# Podcast Identity Packet — Issue #261

This directory is the versioned release-gated show-identity packet for v0.92.1. It records operator approval for the title, classified artwork-rights authority, and redacted company-mailbox receive proof without retaining credentials, verification codes, raw mailbox content, or private source material.

## Current truth

- Operator-approved title: `The Cognitive Stack`
- Candidate artwork: technically valid 3000 x 3000 RGB PNG; exact digest bound in `show-identity.json`
- Name-conflict sample: refreshed 2026-08-28; no exact sampled podcast-title collision, but exact phrase reuse exists in architecture/training/personal-writing contexts and at least one podcast episode title
- Artwork rights: retained source and derivative bytes proven; operator-confirmed classified Agent Logic rights basis recorded in `artwork-rights.json`
- Mailbox: public company address configured; redacted receive proof and retention approval recorded in `mailbox-readiness.json`
- Feed hosting, directory submission, and public launch: not claimed
- Operator decision authority: structured `name-decision.json`; this README and the research Markdown are non-authoritative summaries

## Ownership

- #261: `demos/podcast/artwork.png` and this directory
- #342: episode/audio/package artifacts under `demos/podcast/episode-packages/**`, including only the non-production `feed-fragment.xml`
- #262: production `demos/podcast/feed.xml` and production route/hosting/playback

## Validation

Candidate-safe validation:

```sh
python3 docs/milestones/v0.92/review/podcast_identity_261/validate_identity_packet.py
```

Release validation proves all three #261 external gates are satisfied while still making no hosting, directory-submission, or public-launch claim:

```sh
python3 docs/milestones/v0.92/review/podcast_identity_261/validate_identity_packet.py --release
```

Redaction-only validation rejects prohibited secret/private-mailbox material:

```sh
python3 docs/milestones/v0.92/review/podcast_identity_261/validate_identity_packet.py --redaction-only
```
