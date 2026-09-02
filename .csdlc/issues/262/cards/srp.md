# Structured Review Prompt

Template: 1.0.0

Issue: 262

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/262
.csdlc/prepared/issues/262
demos/_preview/podcast/index.html
demos/podcast/LAUNCH_READINESS.md
demos/podcast/S3_CLOUDFRONT_RUNBOOK.md
demos/podcast/episodes/001-meet-the-ai-coworkers/episode.json
demos/podcast/episodes/001-meet-the-ai-coworkers/rss-enclosure.json
demos/podcast/episodes/001-meet-the-ai-coworkers/show-notes.md
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
    "id": "262-review-r1-p1-public-transcript-stale-show-name",
    "severity": "p1",
    "summary": "Public episode page still exposes the old show name in publishable transcript copy at demos/podcast/episodes/meet-the-ai-coworkers/index.html lines 54, 82, and 104; the validator only checked title text and missed body transcript copy.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "262-review-r1-p1-audio-id3-stale-show-name",
    "severity": "p1",
    "summary": "Distribution MP3 embedded ID3 metadata still identifies TPE1/TALB as Cognitive Spacetime, and qa-report.md lines 16-22 confirms stale artist/album metadata.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "262-review-r1-p2-mailbox-readiness-stale",
    "severity": "p2",
    "summary": "Launch readiness still says mailbox verification is deferred even though the #261 identity dependency and validator require verified_received publication authorization.",
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

Revision: Some("git-blake3:96b32aef02826b52270f224b444583fd560b4b9d:3e96ee6c810713dd967e5c0469bf50d2d791b37d54d97690d5e5c8ad2a1e5529")

Reviewer: Some("fresh-session:ec6fb5a9-f376-4e33-97c7-8ad14b12cd20")

Result: changes_required
