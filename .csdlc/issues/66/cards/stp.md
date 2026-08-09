# Structured Task Prompt

Template: 1.0.0

Issue: 66

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add one typed speech-provider boundary and Deepgram batch adapter with profile discovery, focused tests, one opt-in live canary, and setup documentation.

## Deliverables

- Typed SpeechProvider synthesis and transcription contracts
- Deepgram HTTP adapter with redacted credential and error handling
- Canonical Deepgram profiles and speech capability discovery
- Deterministic loopback tests and bounded live canary
- Provider setup and Podcast Studio consumption example

## Acceptance

1. AC-1: Deepgram profiles expand through the canonical provider registry and advertise native synthesis and transcription capabilities
2. AC-2: Typed synthesis validates explicit voice/media settings and returns validated bytes with redacted provenance
3. AC-3: Typed transcription accepts declared audio and returns structured transcript, timing, confidence, language, usage, and redacted provenance
4. AC-4: Stable error kinds distinguish authentication, throttling, invalid input, unsupported media, timeout, transport, and malformed response
5. AC-5: Credentials and authorization headers never appear in Debug, errors, fixtures, logs, manifests, or retained evidence
6. AC-6: Deterministic offline tests cover requests, parsing, media validation, redaction, timeout, malformed responses, and provider errors
7. AC-7: One opt-in minimal live round trip retains redacted cost, latency, identity, media, request, and usage evidence
8. AC-8: Podcast Studio can consume the provider without embedding Deepgram HTTP or credential logic
9. AC-9: Focused documentation and independent exact-head review pass without unresolved findings

## Dependencies

- agent-logic/agent-design-language#66
- Existing provider profile registry and ProviderSpec expansion
- Deepgram Aura-2 batch synthesis and prerecorded transcription APIs

## Inputs

- adl/src/provider/mod.rs
- adl/src/provider/profiles.rs
- adl/src/provider_substrate.rs
- adl/src/provider/http_family.rs
- adl/src/provider/http_family/tests.rs
- adl/tests/provider_tests/profiles.rs
- adl/tools/demo_v0911_multiagent_podcast_audio.sh
- docs/tooling/PROVIDER_SETUP.md
- docs/milestones/v0.91.8/review/podcast_launch_5711/DEEPGRAM_AUDIO_INVESTIGATION_5711.md

## Non Goals

- Migrating or rerendering Episode 001
- Realtime conversational or Gemini Live integration
- Voice cloning or custom voice training
- General media editing or automatic price-only selection
- Broad completion-provider refactoring
