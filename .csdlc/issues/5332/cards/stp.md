# Structured Task Prompt

Template: 1.0.0

Issue: 5332

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Create only the #5332 issue-bound FastWork worktree and minimal typed preparation packet.

## Deliverables

- Issue-local design note
- Issue-local diagram
- Native C-SDLC v2 six-card/index preparation record
- Readiness status that preserves the occupied sidecar blocker

## Acceptance

1. AC-1: #5332 has one /Volumes/FastWork issue-bound worktree on its preparation branch
2. AC-2: C-SDLC v2 issue-local preparation files exist and validate mechanically
3. AC-3: no source, Unity, proof, planning, #4739, or #4741 worktree files are changed
4. AC-4: source implementation remains blocked while #4739/#4741 are occupied
5. AC-5: #5107 readiness is read-only and does not mutate the stale branch

## Dependencies

- Occupied #4739 Unity-MCP live project and port proof worktree
- Occupied #4741 Unity editor liveness and batch proof worktree
- v0.91.8 WP-14A sidecar disposition plan

## Inputs

- GitHub issue #5332 local inventory snapshot
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh
- demos/v0.91.6/unity-observatory/PROOF_PACKET.md

## Non Goals

- No Unity execution
- No wrapper or Unity asset edits
- No broad validation
- No PR publication or review
- No GitHub, AWS, credential, or provider operation
