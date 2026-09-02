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
    "id": "262-review-r6-p1-archive-aggregate-bytes-mismatch",
    "severity": "p1",
    "summary": "Archive aggregate byte truth is internally inconsistent: storage-manifest.json records archive_total_bytes 191911392 while s3-object-inventory.json records total_bytes 191911388, and summing all 87 inventory object sizes also yields 191911388. The validator checks local critical-object byte/hash truth but does not compare aggregate totals.",
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

Revision: Some("git-blake3:e9f603706ec75f018605863e409e4431699d9f95:53b86d1aefa20b9d2dfcb2b51e731bcb63299897a01288034a2bcc6afb509891")

Reviewer: Some("fresh-session:6915b91a-64a0-40b2-83ce-12e8ab77314f")

Result: changes_required
