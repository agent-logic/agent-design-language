# The Cognitive Stack Directory Submission Runbooks

Status: prepared 2026-09-02 from current official provider instructions. This is not submission authority.

## Shared preflight for every provider

Before any provider account action, verify:

- RSS feed URL: `https://agent-logic.ai/podcast/feed.xml`
- Show page URL: `https://agent-logic.ai/podcast/`
- Show title in feed: `The Cognitive Stack`
- Artwork URL in feed: `https://agent-logic.ai/podcast/artwork.png`
- Owner email in feed: `podcast@agent-logic.ai`
- One launch episode is present with a stable HTTPS MP3 enclosure.
- `ruby .csdlc/prepared/issues/262/validate-podcast-hosting.rb` passes at the exact candidate SHA.
- The operator has access to the provider account, its 2FA device/recovery workflow, and the company mailbox.

Never commit, paste into issue text, or retain:

- passwords, API keys, cookies, recovery codes, verification codes, raw mailbox headers/bodies, private screenshots, or provider session state.

## Apple Podcasts

Official source sampled 2026-09-02: https://podcasters.apple.com/support/897-submit-a-show

Operator-controlled steps:

1. Sign in to Apple Podcasts Connect with the company-controlled Apple account.
2. Add a new show with an RSS feed.
3. Enter `https://agent-logic.ai/podcast/feed.xml`.
4. Review imported show details against the #261 identity packet and #262 feed.
5. Set Content Rights truthfully.
6. Provide company contact information.
7. Choose Availability: countries/regions, public Catalog API distribution, transcript handling, release timing, and show-claiming settings.
8. Resolve any validation warnings.
9. Stop before Publish unless #264 has explicit launch authorization.

Record only redacted status, canonical Apple show URL/ID after publication or claim, and any non-secret validation warning summaries.

## Spotify for Creators

Official sources sampled 2026-09-02:

- https://support.spotify.com/us/creators/article/getting-your-show-on-spotify/
- https://support.spotify.com/us/creators/article/multiple-shows-under-one-account/
- https://support.spotify.com/us/creators/article/finding-and-enabling-your-rss-feed/
- https://support.spotify.com/sg-en/creators/article/claiming-your-podcast-on-spotify-for-creators/
- https://support.spotify.com/mw/creators/article/adding-a-new-show/

Operator-controlled steps:

1. Sign in to Spotify for Creators with the company-controlled account.
2. Add a show / find an existing show hosted somewhere else.
3. Enter the show name, RSS feed, or feed-owner email as directed by Spotify.
4. Use the verification email/code sent to `podcast@agent-logic.ai` only inside Spotify’s UI.
5. Confirm imported metadata matches `The Cognitive Stack`, the public artwork, and the launch episode.
6. Stop before any irreversible publish, distribution, monetization, or account-wide setting unless #264 authorizes it.

Record only redacted verification outcome, canonical Spotify show URL/ID, owner, date, status, and follow-up.

## Amazon Music for Podcasters

Official sources sampled 2026-09-02:

- https://podcasters.amazon.com/submit-rss
- https://podcasters.amazon.com/frequently-asked-questions

Operator-controlled steps:

1. Sign in to Amazon Music for Podcasters with the company-controlled account.
2. Submit or claim one RSS feed URL: `https://agent-logic.ai/podcast/feed.xml`.
3. Confirm any required content-license agreement and territory choice.
4. Use the confirmation email sent to the RSS owner email only inside the provider flow.
5. Monitor provider status; Amazon documents states such as Unconfirmed, Pending, Active, Hidden, Suspended, and Invalid Email.
6. Stop before launch-announcement or public status claim unless #264 has exact evidence.

Record only redacted status, canonical Amazon/Audible show URL/ID, date, owner, and follow-up.

## YouTube RSS ingestion

Official source sampled 2026-09-02: https://support.google.com/youtube/answer/13525207?hl=en

Operator-controlled steps:

1. Sign in to YouTube Studio with the company-controlled channel.
2. Use the Podcasts flow to submit or connect the RSS feed.
3. Verify ownership using the email/code sent to `podcast@agent-logic.ai`; do not retain the code.
4. Confirm YouTube-created static-image videos use the show art and expected episode selection.
5. Start with private or otherwise explicitly selected visibility until all uploaded episodes are inspected.
6. Review monetization, copyright, paid-promotion, and visibility settings before making anything public.
7. Remember YouTube says RSS-ingested audio replacement does not update already published videos automatically.

Record only redacted status, channel/podcast URL or playlist ID, selected visibility, date, owner, and follow-up.
