# The Cognitive Stack Launch Readiness

This packet records the complete first episode and the production hosting
surface for final directory-submission preflight.

## Routes

- Production launch route: `/podcast/`
- Hidden test route: `/_preview/podcast/`
- RSS feed: `/podcast/feed.xml`
- First episode page: `/podcast/episodes/meet-the-ai-coworkers/`

The hidden test route is intentionally unlinked from the site navigation and
declares `noindex,nofollow`.

## Current Launch Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| Hidden test URL | Ready | `demos/_preview/podcast/index.html` |
| Production route target | Ready | `demos/podcast/index.html` |
| RSS feed | Published to production hosting | `https://agent-logic.ai/podcast/feed.xml` matches `demos/podcast/feed.xml` |
| Episode 001 MP3 | Published to production hosting | `https://agent-logic.ai/podcast/audio/meet-the-ai-coworkers.mp3` matches `demos/podcast/audio/meet-the-ai-coworkers.mp3` |
| Episode 001 WAV archive | Complete | `demos/podcast/audio/meet-the-ai-coworkers.wav` |
| Show artwork | Complete, 3000 x 3000 RGB PNG | `demos/podcast/artwork.png` |
| Transcript and notes | Reconciled to final script | `demos/podcast/episodes/001-meet-the-ai-coworkers/` |
| First ten topics | Drafted | episode list in `demos/podcast/index.html` |
| Guest workflow | Page-ready | contact button uses `mailto:podcast@agent-logic.ai`; FAQ invites guest suggestions |
| Contact path | Address configured; mailbox verified for launch packet | `docs/milestones/v0.92/review/podcast_identity_261/mailbox-readiness.json` records verified receipt and publication authorization |
| HTTP playback behavior | Source-SHA-bound local and public proof retained | `.csdlc/evidence/262/http-playback-proof.json` records local HEAD, GET, and 206 byte-range behavior; `.csdlc/evidence/262/live-production/public-production-proof.json` records public HTTPS feed/page/artwork/episode-page checks plus MP3 HEAD and first/tail 206 byte-range checks across desktop and mobile user-agent profiles |
| Final launch route | Published to production hosting | `https://agent-logic.ai/podcast/` matches `demos/podcast/index.html` |

## Audio Truth

Episode 001 is a complete 18 minute, 32 second four-act conversation. The feed
encloses a 160 kbps MP3 with embedded cover art. The WAV file is the 24 kHz mono
archive master. ChatGPT, Gemini, and Claude authored their respective dialogue;
the production metadata separately identifies the synthetic voice provider for
each speaker.

## Directory Submission Prerequisites

Apple Podcasts, Spotify, YouTube Music, and similar directories still require
account-side setup after this PR lands. Before submission, verify:

- `https://agent-logic.ai/podcast/feed.xml` is publicly reachable over HTTPS.
- `https://agent-logic.ai/podcast/audio/meet-the-ai-coworkers.mp3` is publicly
  reachable and returns the complete file.
- `https://agent-logic.ai/podcast/artwork.png` is publicly reachable and
  returns the 3000 x 3000 RGB PNG.
- `podcast@agent-logic.ai` receives directory verification mail; #261 records
  the mailbox as verified for the launch packet, while directory-specific
  account-side verification remains part of the later #264 submission workflow.
- The first submitted episode uses final approved audio, title, description,
  publish date, and content-rights truth.

Directory availability is not claimed by this packet. Directory submission
requires separate operator approval in #264.

## Human Review

The completed episode and production hosting surface are published for
directory-submission preflight. This packet does not claim Apple, Spotify,
Amazon, YouTube, or any other directory submission, acceptance, availability, or
approval.
