# Structured Intent Prompt

Template: 1.0.0

Issue: 66

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide governed typed batch synthesis and transcription through Deepgram without podcast-owned HTTP or credential logic.

## Required Outcome

ADL exposes typed Deepgram synthesis and prerecorded transcription through its canonical provider/profile boundary, with validated media, redacted provenance, deterministic offline proof, and one bounded opt-in live round trip.

## Scope

- adl/src/provider/deepgram.rs
- adl/src/provider/mod.rs
- adl/src/provider/profiles.rs
- adl/src/provider_substrate.rs
- adl/tests/provider_tests.rs
- adl/tests/provider_tests/deepgram.rs
- demos/podcast/DEEPGRAM_PROVIDER_WORKFLOW.md
- docs/tooling/PROVIDER_SETUP.md
- .csdlc/prepared/issues/66
- .csdlc/issues/66
- .csdlc/evidence/66

## Authority

- Issue and code authority are agent-logic/agent-design-language#66
- Provider profiles and ProviderSpec remain canonical construction inputs
- Deepgram credentials are request-only secrets and never durable provider state
- Live network proof is opt-in and separate from deterministic offline validation

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Use the independent Rust C-SDLC v2 route and issue-bound FastWork worktree
- Use $HOME/keys/adl-deepgram-01.key only through a process-scoped environment mapping and never print or persist its contents
- Run focused provider validation rather than broad test suites
- Do not rerender Episode 001 or add realtime speech scope
