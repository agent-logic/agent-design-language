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

- No live S3 or CloudFront mutation/probe was performed during the read-only exact-head review; retained local hosting and launch-packet proof remain the review denominator.
- ffprobe was unavailable in the reviewer environment, so ID3 behavior was verified through the repository validator's ID3 frame checks rather than an external ffprobe invocation.

## Review Result

Revision: Some("git-blake3:78caa6853c3113737701a69e6ec9d9023a8b5fcb:797cd0b6a8acf2f502252d2a9c18004eec192f679ce7bddb47b532ebe9948813")

Reviewer: Some("fresh-session:627ccf54-c965-42b7-8ce9-084791bad23b")

Result: pass
