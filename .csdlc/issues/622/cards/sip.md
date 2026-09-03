# Structured Intent Prompt

Template: 1.0.0

Issue: 622

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Production-wire validated provider-instance and inference-profile hot loading without restarting the consuming process.

## Required Outcome

A provider-only configuration sidecar is watched through the existing reload substrate, accepted candidates atomically replace the active provider snapshot, rejected candidates retain last-known-good, and subsequent production inference consumes the new snapshot.

## Scope

- adl/src/provider/reload.rs
- adl/src/provider/mod.rs
- adl/src/provider/profiles.rs
- adl/src/execute/runner.rs
- adl/tests/provider_profile_hot_reload.rs
- docs/providers/provider-profile-hot-loading.md
- .csdlc/prepared/issues/622/**
- .csdlc/issues/622/**

## Authority

- Reuse the existing Runtime-kernel watcher and existing provider candidate activation
- The watched document is provider-only and cannot grant authority or define executable work
- Each inference call retains the immutable provider snapshot selected before dispatch
- Credentials remain separately governed references and values are never reload payload
- Only later inference consumes an accepted replacement

## Assumptions

- none

## Operator Constraints

- Do not build a second watcher or provider registry
- Do not reload secrets or authority-bearing objects
- Do not implement MLX or OCI packaging
- Do not restart the process to claim hot loading
- Do not expand into provider architecture redesign
