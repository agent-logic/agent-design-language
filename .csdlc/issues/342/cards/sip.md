# Structured Intent Prompt

Template: 1.0.0

Issue: 342

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Create ten complete review-ready episode packages using the approved #261 identity inputs without owning production hosting or feed publication.

## Required Outcome

Ten complete episode-package directories pass package, audio, metadata, redaction, editorial, and digest checks while production feed and deployment remain outside this issue.

## Scope

- demos/podcast/episode-packages
- docs/milestones/v0.92.1/evidence/podcast/wp-24a
- adl/tools/generate_podcast_launch_packet.py
- adl/tools/validate_podcast_launch_packet.py
- adl/tools/test_podcast_launch_packet.sh

## Authority

- Issue 342 owns only its declared result and paths; Sprint 8 umbrella #536 coordinates but cannot implement or approve this child.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use a dedicated FastWork issue worktree and issue-bound session goal
- Run one bounded exact-head review before publication
- Do not retain credentials, verification codes, recovery material, TLS private keys, or private account data
- Do not widen into another Sprint 8 child's ownership
