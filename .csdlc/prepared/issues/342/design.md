# Issue 342 Design — Podcast Studio first ten episode packages

## Goal

Create ten complete review-ready episode packages using the approved #261 identity inputs without owning production hosting or feed publication.

## Required Outcome

Ten complete episode-package directories pass package, audio, metadata, redaction, editorial, and digest checks while production feed and deployment remain outside this issue.

## Ownership

- `demos/podcast/episode-packages`
- `docs/milestones/v0.92.1/evidence/podcast/wp-24a`
- `adl/tools/generate_podcast_launch_packet.py`
- `adl/tools/validate_podcast_launch_packet.py`
- `adl/tools/test_podcast_launch_packet.sh`

## Dependencies

- Sprint 8 umbrella #536
- Terminal #261 canonical show identity and rights inputs
- Retained Podcast Studio v2 proof and approved route/storage decision

## Safety Boundary

- This issue owns only the listed result and paths.
- All external mutations and private material remain governed by the operator constraints.
- Validation and exact-head review precede publication.

## Non-Goals

- Production feed ownership
- Hosting or deployment
- Directory submission
- Mailbox verification
- Public launch
