# The Cognitive Stack Launch Readiness

This packet is intentionally not a public launch packet.

Issue #660 records that the previously reachable public `/podcast/` website,
RSS, artwork, and media objects were emergency-hidden because public launch had
not been approved. The current show page belongs under the hidden preview route:

- Hidden preview route: `https://agent-logic.ai/_preview/podcast/`
- Hidden preview source: `demos/_preview/podcast/index.html`
- Required preview robots metadata: `noindex,nofollow`

## Current launch gates

| Gate | Status | Evidence |
| --- | --- | --- |
| Hidden preview URL | Ready | `https://agent-logic.ai/_preview/podcast/` returns The Cognitive Stack page with `noindex,nofollow`; the preview player uses `_preview/podcast/audio/meet-the-ai-coworkers.mp3`, not the public `/podcast/audio/` route |
| Public production route | Withheld | `https://agent-logic.ai/podcast/` returns a non-serving status after #660 rollback |
| RSS feed launch candidate | Ready but withheld | `demos/podcast/feed.xml` is retained as candidate source and must not be deployed before approval |
| Public episode media launch candidate | Ready but withheld | `demos/podcast/audio/meet-the-ai-coworkers.mp3` is retained as candidate source and must not be deployed before approval |
| Public artwork launch candidate | Ready but withheld | `demos/podcast/artwork.png` is retained as candidate source and must not be deployed before approval |
| Provider directory submission | Not authorized | Requires separate future operator approval |

## Directory submission prerequisites

Before Apple Podcasts, Spotify, Amazon Music, YouTube RSS, or another provider
is contacted, a future authorized issue must:

1. Re-stage the public website, RSS feed, artwork, and media artifacts from the
   retained episode/source packages.
2. Verify `https://agent-logic.ai/podcast/`, the public RSS feed, artwork, and
   media enclosure URLs over HTTPS.
3. Confirm `podcast@agent-logic.ai` still receives provider verification mail.
4. Retain redacted provider-specific proof without credentials, recovery codes,
   cookies, private screenshots, or unsupported availability claims.
5. Obtain explicit operator approval for the public launch/provider submission
   action.

## Human review

The completed episode/source packages remain available for future launch work,
but this packet no longer claims that the production route, RSS feed, artwork,
or media objects are publicly published.

Issue #51 remains open until the emergency #660 disposition and the separate
provider/public-launch gate are accepted by the operator.
