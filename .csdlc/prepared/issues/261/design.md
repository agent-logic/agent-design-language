# #261 Podcast Show Identity Design

## Purpose and decision gates

#261 produces one versioned, reviewable launch-input packet for the podcast show. The current repository contains a candidate identity, `Cognitive Spacetime: The Agent Logic Podcast`, and a 3000 x 3000 RGB PNG at `demos/podcast/artwork.png`, but neither repository history nor this preparation packet is operator approval. The title/name-conflict decision, artwork rights provenance, and proof that the company-controlled mailbox can receive ownership-verification mail remain explicit external gates. Missing proof is recorded as pending and never upgraded by inference.

The packet contains no credential, token, recovery code, mailbox message content, or directory verification code. Mailbox evidence is a redacted receipt containing only the company-controlled address identifier, test timestamp, sender class, receive outcome, and evidence digest approved for repository retention.

## Exact ownership allocation

#261 solely owns:

- `demos/podcast/artwork.png`
- `docs/milestones/v0.92/review/podcast_identity_261/**`
- issue-local `.csdlc` lifecycle and validation evidence for #261

#342 consumes the terminal #261 identity and artwork read-only. It owns episode/audio/package artifacts under `demos/podcast/episode-packages/**`; any feed-shaped package proof is explicitly non-production and named `feed-fragment.xml` or represented as a manifest. #342 must not own or mutate `demos/podcast/feed.xml`.

#262 solely owns the production `demos/podcast/feed.xml`, production route/hosting/enclosures, HTTP byte-range behavior, and desktop/mobile playback proof. #261 does not host media, publish a feed, deploy a route, or submit to directories.

This allocation is the collision boundary consumed by the #51 graph. Any requested edit outside #261's exact paths stops for graph and design reconciliation.

## Canonical packet

`docs/milestones/v0.92/review/podcast_identity_261/show-identity.json` is the single downstream metadata authority after operator approval. It records:

- schema/version and approval state;
- title, subtitle, description, author, category, language, explicit status, copyright, cadence, and intended website URL;
- the exact artwork path, byte count, dimensions, color space, format, and SHA-256 digest;
- references to the rights/provenance record, collision review, and redacted mailbox-readiness receipt.

`artwork-rights.json` binds the source and license/rights basis without embedding private contracts or credentials. `name-conflict-review.md` records searched surfaces, observed conflicts, uncertainty, candidate alternatives, and the operator decision. `mailbox-readiness.json` is fail-closed: `pending_external_verification` until a company-controlled receive test is performed and a redacted digest-bearing receipt is approved for retention.

All JSON uses deterministic UTF-8 formatting, explicit schema identifiers, and repo-relative paths. The packet validator recomputes the artwork digest and PNG properties, checks metadata parity and allowed states, rejects secrets and placeholders, and fails while any required operator/external gate is pending when run in release mode.

## Baseline evidence

At preparation base `c46b7cd8265a7e81566cdf82153c387595a6cccf`, the candidate `demos/podcast/artwork.png` is a 3000 x 3000, 8-bit RGB, non-interlaced PNG with SHA-256 `e142182ecefa06b34256d7ceeededfb3c3418c1f66e9a57750d3ed21d8d2fc8d`. Existing feed and launch-readiness text provide candidate metadata only and are read-only inputs because production feed ownership belongs to #262.

## Validation and review

Preparation proves exact allocation, current-main base, candidate artwork technical properties, and truthful pending gates. Implementation creates the canonical packet and validator without modifying production feed or episode packages. Exact-head review checks metadata consistency, rights truth, redaction, secret absence, artwork digest/properties, and graph ownership. Publication and finish remain blocked until operator identity approval and mailbox receive proof are durably but safely recorded.

## Non-goals

No episode writing, audio generation, episode package mutation, feed/enclosure publication, hosting/deployment, directory account action, submission, public launch, paid service, or credential handling.
