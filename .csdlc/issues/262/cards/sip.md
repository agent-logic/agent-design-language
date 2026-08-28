# Structured Intent Prompt

Template: 1.0.0

Issue: 262

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Publish and validate the canonical production podcast feed and stable HTTPS media enclosures from approved identity and terminal episode packages.

## Required Outcome

The production feed, enclosure metadata, byte-range behavior, and representative desktop/mobile playback are source-grounded, digest-consistent, and rollback-safe.

## Scope

- demos/podcast/feed.xml
- docs/milestones/v0.92.1/evidence/podcast/51-b
- adl/tools/record_podcast_native_playback.sh
- adl/tools/record_podcast_browser_playback.mjs
- adl/tools/record_podcast_ios_safari_playback.sh

## Authority

- Issue 262 owns only its declared result and paths; Sprint 8 umbrella #536 coordinates but cannot implement or approve this child.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use a dedicated FastWork issue worktree and issue-bound session goal
- Run one bounded exact-head review before publication
- Do not retain credentials, verification codes, recovery material, TLS private keys, or private account data
- Do not widen into another Sprint 8 child's ownership
