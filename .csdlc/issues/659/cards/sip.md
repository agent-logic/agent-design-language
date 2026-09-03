# Structured Intent Prompt

Template: 1.0.0

Issue: 659

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime-v3 service convergence configurable, generous, stage-specific, and recoverable without weakening service-manager ownership.

## Required Outcome

CSM start, stop, and reload use validated named convergence limits instead of brittle fixed waits, allow slow model-backed startup, and report exact recoverable stage failures.

## Scope

- adl/src/cli/csm_runtime_v3_cmd.rs
- directly coupled Runtime-v3 service configuration
- focused convergence tests
- issue-local lifecycle records and evidence

## Authority

- typed C-SDLC v2 remains lifecycle authority
- PR #658 is the stacked dependency base
- launchd or systemd remains sole Runtime process owner
- live Runtime restart is excluded

## Assumptions

- none

## Operator Constraints

- use generous safe defaults
- do not introduce tiny test or production timeouts
- do not change general API request timeouts
- do not restart the live Runtime
- work only in the bound FastWork worktree
