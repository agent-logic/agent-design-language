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
demos/podcast/episodes/001-meet-the-ai-coworkers/s3-object-inventory.json
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
    "id": "262-review-r5-p1-storage-manifest-checksums-stale",
    "severity": "p1",
    "summary": "Archive inventory/checksum truth is stale after launch-identity remediation. storage-manifest.json still records old byte counts and SHA-256 values for changed package files including CREATOR_WORKFLOW.md, episode.json, source-packet.md, script.md, transcript.md, show-notes.md, audio-manifest.json, qa-report.md, and rss-enclosure.json. The validator binds the storage manifest file but does not recompute archive object byte/hash entries against current local package files.",
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

Revision: Some("git-blake3:3ee7d25e9aed1ebbc07eee59ba64e7d891b62a65:4f6051ba627f827e1863b3d222111c41954dbf5ca82458ae7bc1946e5316c094")

Reviewer: Some("fresh-session:39c407bb-11b7-4fb0-93e9-2b35c059f03c")

Result: changes_required
