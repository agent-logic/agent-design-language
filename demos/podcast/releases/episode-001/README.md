# Episode 1 launch preparation

The Cognitive Stack — Meet the AI Coworkers. The operator approved final listening on September 24 for a September 25, 2026 launch. This package is prepared locally and has not been deployed. The exact publication time remains unset.

The feed uses the approved 550.056-second recording (displayed as 9:10), its measured byte count, and the existing stable episode GUID. The descriptive MP3 and transcript are exact copies of the reviewed Episode 1 package. `release.json` records source paths, SHA-256 hashes, MIME types and intended public URLs. The original WAV remains in the reviewed source package to avoid duplicating the archive.

## Launch checklist

1. Choose the exact publication time and set the feed item's `pubDate` before publishing.
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
