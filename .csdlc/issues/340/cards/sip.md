# Structured Intent Prompt

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Integrate the terminal HTML/Polis Observatory with the real Runtime v3 launch, control, stop, restart, replay, and recovery path.

## Required Outcome

At one exact integrated revision, the HTML/Polis Observatory consumes the stable Runtime v3 API/WSS contract through the real operator Runtime service, survives Guardian-owned graceful stop and restart, resumes with bounded replay, applies no duplicate events, retains fresh correlation and unchanged authorization, and exposes only the appropriate redacted projection.

## Scope

- Runtime v3 launch/start/status/stop control through CSMctl
- HTML Observatory static client runtime-target configuration and reconnect observation
- Runtime API/WSS restart/reconnect proof for ready observatory and health endpoints
- Bounded replay/no-duplicate/fresh-correlation/unchanged-authorization validation
- Issue-owned restart/reconnect validator and focused runtime API/WSS test

## Authority

- #340 owns only launch/restart/reconnect integration surfaces listed in the live issue
- HTML/Polis product implementation paths remain read-only inputs owned by terminal children
- Unity, #84, #122, #251, AWS/public launch, provider execution, #341, and #343 are non-claims
- Fixture/static rendering and contract-only proof must be classified separately from live integration
- No Runtime protocol redesign or provider integration is authorized

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 owners for lifecycle and publication
- Bind implementation under /Volumes/FastWork/adl-worktrees
- Do not mutate main
- Do not mutate HTML child implementation paths
- Do not mutate Unity paths, #84, #122, #251, #341, #258, #299, or #203
- Do not claim AWS/public launch, credentials, spend, or provider execution
