# Structured Review Prompt

Template: 1.0.0

Issue: 262

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/262
.csdlc/prepared/issues/262
adl/tools/record_podcast_native_playback.sh
adl/tools/record_podcast_browser_playback.mjs
adl/tools/record_podcast_ios_safari_playback.sh
demos/_preview/podcast/index.html
demos/podcast/LAUNCH_READINESS.md
demos/podcast/S3_CLOUDFRONT_RUNBOOK.md
demos/podcast/audio/meet-the-ai-coworkers.mp3
demos/podcast/episodes/001-meet-the-ai-coworkers/CREATOR_WORKFLOW.md
demos/podcast/episodes/001-meet-the-ai-coworkers/audio-manifest.json
demos/podcast/episodes/001-meet-the-ai-coworkers/episode.json
demos/podcast/episodes/001-meet-the-ai-coworkers/qa-report.md
demos/podcast/episodes/001-meet-the-ai-coworkers/rss-enclosure.json
demos/podcast/episodes/001-meet-the-ai-coworkers/script.md
demos/podcast/episodes/001-meet-the-ai-coworkers/show-notes.md
demos/podcast/episodes/001-meet-the-ai-coworkers/source-packet.md
demos/podcast/episodes/001-meet-the-ai-coworkers/storage-manifest.json
demos/podcast/episodes/001-meet-the-ai-coworkers/transcript.md
demos/podcast/episodes/meet-the-ai-coworkers/index.html
demos/podcast/feed.xml
demos/podcast/index.html
demos/podcast/studio-reference/podcast-studio.html
demos/podcast/studio/podcast-studio.html

## Prompts

- Does the candidate satisfy every acceptance criterion on its real owned path?
- Does it preserve sibling ownership, operator authority, privacy, and rollback?
- Are all proof claims exact-revision and non-overstated?

## Findings

[
  {
    "id": "262-review-r4-p2-source-packet-guid-stale",
    "severity": "p2",
    "summary": "The public review source packet still declares the old canonical GUID agent-logic-cognitive-spacetime-episode-001, contradicting feed.xml, episode.json, and rss-enclosure.json which use agent-logic-the-cognitive-stack-episode-001. The focused validator misses this because source-packet.md is not in its source-manifest denominator.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "262-review-r4-p2-stale-production-path-truth",
    "severity": "p2",
    "summary": "Stale CognitiveSpacetime/cognitive-spacetime production path truth remains in the launch/hosting package: the S3/CloudFront runbook uses Project=CognitiveSpacetime and archive/cognitive-spacetime prefixes, episode/storage manifests repeat those prefixes, and the creator workflow defaults to /Volumes/FastWork/cognitive-spacetime-production. The candidate either needs explicit legacy alias truth or updated The Cognitive Stack paths.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:54b3d66d13f3b11dada5d928ad976963e9626a88:d9cbfa67d342e3a44bd28e1e9aabd7d3ef1b396bf3329a9200dbb495ed99b946")

Reviewer: Some("fresh-session:01676b7c-d214-4993-bfda-c17dc3e696bf")

Result: changes_required
