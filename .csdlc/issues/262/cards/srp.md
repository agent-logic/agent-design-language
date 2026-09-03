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
adl/tools/test_podcast_launch_packet.sh
adl/tools/validate_podcast_launch_packet.py
docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json
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
demos/podcast/studio-reference/REFERENCE_DIGESTS.txt
demos/podcast/studio-reference/podcast-studio.html
demos/podcast/studio/REFERENCE_DIGESTS.txt
demos/podcast/studio/reference.sha256
demos/podcast/studio/podcast-studio.html

## Prompts

- Does the candidate satisfy every acceptance criterion on its real owned path?
- Does it preserve sibling ownership, operator authority, privacy, and rollback?
- Are all proof claims exact-revision and non-overstated?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- PR head 2b39189ce8a6f2dcb6d848b9106f0852697e934a includes only review-assignment lifecycle metadata after the assigned substantive revision 572fe15253b5c91d36bbfd24c63a2f4692451c8e; production artifacts and live proof surfaces are unchanged.
- Directory submission remains out of scope for #262 and is reserved for #264.

## Review Result

Revision: Some("git-blake3:572fe15253b5c91d36bbfd24c63a2f4692451c8e:1e3defcb84466723601d517579a9d1584e7a642732b280ef7c65d79e03387922")

Reviewer: Some("fresh-session:43005e33-811b-442f-8235-e6bd940e39eb")

Result: pass
