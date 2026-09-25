# Episode 1 launch preparation

The Cognitive Stack — Meet the AI Coworkers. The operator approved final listening on September 24 for a September 25, 2026 launch. The operator subsequently approved the private-path playback test and authorized immediate public promotion. The package is now published; `public-deployment.json` records the verified public feed and actual publication timestamp. Directory submissions remain pending.

The feed uses the approved 550.056-second recording (displayed as 9:10), its measured byte count, and the existing stable episode GUID. The descriptive MP3 and transcript are exact copies of the reviewed Episode 1 package. `release.json` records source paths, SHA-256 hashes, MIME types and intended public URLs. The original WAV remains in the reviewed source package to avoid duplicating the archive.

## Launch checklist

1. Completed: set the feed item's `pubDate` to the authorized public promotion time.
2. Upload only the assets and URL mappings in `release.json`. Serve the approved MP3 at both its descriptive URL and the old audio URL still used by the webpage. Do not bulk-sync the podcast directory: historical files at the old source paths contain the previous episode and artwork.
3. Verify public artwork and both MP3 URLs return the expected bytes and MIME types, and verify audio playback/range requests. Verify the transcript URL.
4. Publish the feed only after those assets pass verification; fetch the public XML and confirm its enclosure and publication date.
5. Continue the remaining registration and launch checklist in issue #1169. This change does not complete the launch issue.

## Preserved webpage

The webpage HTML, layout and copy remain unchanged as requested. Its displayed duration still says 18:32; the prepared feed correctly says 9:10. The compatibility audio URL above allows the page to play the approved recording without an HTML edit. The old displayed duration remains a known launch discrepancy requiring a separate decision.

## Private-path playback test

The operator subsequently authorized publishing a separate test feed at `https://agent-logic.ai/_private/podcast/feed.xml`. `private-feed.xml` is the launch XML with the URL prefix changed consistently and a test-only comment. Its linked audio, artwork, transcript and unchanged pages are copied beneath that prefix. This does not publish the public `/podcast/feed.xml` or set the launch publication time. The test path is externally readable for subscription testing.

## Public promotion authorization

After the private-path listening test passed, the operator explicitly authorized moving the page, feed and assets to public URLs before registration. The public feed now records the actual publication time in `pubDate`; the private test snapshot remains preserved. Directory registration and social announcements are still separate actions.

## Browser delivery correction

Public HTTP availability alone did not prove browser rendering. The inherited site policy blocked the page renderer's pinned React dependencies and expression evaluation. A `podcast/*` response-header policy now permits the existing renderer and Google Fonts while preserving the main site's policy. The exact scoped policy is retained in `public-headers-policy.json`. The feed is served as `application/xml; charset=utf-8` with `Content-Disposition: inline` for browser display; RSS content and media are unchanged. Renderer CDN files were fetched and verified against their existing SHA-384 integrity pins.

Browser automation could not start because its installed runtime rejected a trusted dependency path. Header checks and asset integrity are verified; visual rendering has not been independently confirmed through that browser tool. This scoped live policy addition must be preserved in future infrastructure reconciliation.

## Date and duration correction

The operator requested a complete date/time check after promotion. Both Episode 1 listings now show September 24, 2026 and 9:10. The episode page includes the Pacific timestamp. The RSS publication instant remains September 25 at 00:40:26 UTC, equivalent to September 24 at 17:40:26 PDT. September 25 remains the planned promotional launch day, distinct from public availability. Unproduced episodes retain Proposed dates and use TBD durations. The earlier webpage-preservation and stale-duration notes describe the prior state and are superseded by this authorized correction.

## Final pre-registration reference audit

Corrected the public episode page to the approved introductory episode summary and transcript. The downloadable transcript now has publication metadata rather than a stale listening-approval hold; its spoken turns are unchanged from the approved source. Updated its release hash and byte count. Subscribe points to the outlet section, with Apple Podcasts, Spotify, Amazon Music/Audible and YouTube Music pending registration. Live assets and feed references were checked; email delivery and independent browser clicks are not claimed.
