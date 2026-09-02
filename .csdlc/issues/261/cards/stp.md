# Structured Task Prompt

Template: 1.0.0

Issue: 261

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Create and validate only the #261 identity/artwork/rights/mailbox packet and issue-owned lifecycle evidence; downstream package, production feed, hosting, playback, directory, and launch surfaces remain outside scope.

## Deliverables

- demos/podcast/artwork.png
- docs/milestones/v0.92/review/podcast_identity_261/show-identity.json
- docs/milestones/v0.92/review/podcast_identity_261/name-decision.json
- docs/milestones/v0.92/review/podcast_identity_261/artwork-rights.json
- docs/milestones/v0.92/review/podcast_identity_261/name-conflict-review.md
- docs/milestones/v0.92/review/podcast_identity_261/mailbox-readiness.json
- docs/milestones/v0.92/review/podcast_identity_261/README.md
- docs/milestones/v0.92/review/podcast_identity_261/validate_identity_packet.py
- .csdlc/prepared/issues/261
- .csdlc/issues/261
- .csdlc/evidence/261

## Acceptance

1. AC-1: The operator explicitly approves the collision-reviewed title and intended launch metadata; candidate or pending states do not count.
2. AC-2: Artwork is square 3000 x 3000 RGB PNG and its exact path bytes SHA-256 source license and rights basis are retained truthfully.
3. AC-3: A company-controlled podcast mailbox receive test has a redacted digest-bearing receipt with no private message content credential token recovery code or verification code.
4. AC-4: Canonical metadata is versioned internally consistent and exactly allocates #261 identity/artwork #342 package and #262 production feed/hosting ownership.
5. AC-5: Focused packet validation secret scanning diff hygiene and exact-head review pass with nonzero truthful evidence.
6. AC-6: No episode audio feed hosting deployment directory submission account mutation or public-launch action is performed.

## Dependencies

- Part of #51 coordination graph
- Operator approval of final show identity and collision disposition
- Company-controlled podcast mailbox receive verification
- Exact #261/#342/#262 path allocation accepted before bind

## Inputs

- agent-logic/agent-design-language#261 live issue
- agent-logic/agent-design-language#51 coordination graph
- agent-logic/agent-design-language#262 production hosting child
- agent-logic/agent-design-language#342 episode-package producer
- demos/podcast/artwork.png
- demos/podcast/feed.xml as read-only candidate metadata
- demos/podcast/LAUNCH_READINESS.md as read-only historical readiness text
- docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json as read-only candidate metadata

## Non Goals

- Episode writing recording mastering or package mutation
- Production feed enclosure route hosting HTTP playback or deployment
- Directory account action submission monitoring or public launch
- Credential verification-code private mailbox content or paid-provider handling
