# Issue #66 Design: First-Class Deepgram Speech Provider

## Objective

Add a reusable typed speech-provider boundary to ADL and implement Deepgram
batch synthesis and prerecorded transcription behind it. Podcast Studio and
other workflows consume the typed boundary; they do not own Deepgram HTTP,
credential, response parsing, or redaction logic.

## Architecture

The existing `provider` module remains the provider-selection namespace. Its
completion-oriented `Provider` trait is not widened into an ambiguous media
interface. A sibling `SpeechProvider` trait owns two explicit operations:

- `synthesize(SpeechSynthesisRequest) -> SpeechSynthesisResult`
- `transcribe(SpeechTranscriptionRequest) -> SpeechTranscriptionResult`

`DeepgramSpeechProvider` implements that trait. Construction accepts an ADL
`ProviderSpec`, so profile expansion and provider identity remain canonical.
Deepgram profiles identify the standard Aura-2 Pluto synthesis route and
Nova-3 transcription route. Provider substrate capability discovery adds
explicit native speech-synthesis and speech-transcription capability fields;
completion capabilities remain false for Deepgram.

## Typed Contracts

The provider module defines:

- provider configuration and credential-source metadata;
- synthesis and transcription request/result structures;
- media encoding/container enums and validated audio metadata;
- structured transcript alternatives, timing, confidence, language, and usage;
- redacted request identity and provider provenance;
- a stable error kind distinguishing authentication, throttling, invalid input,
  unsupported media, timeout, transport, and malformed response.

Results retain provider/model/voice/media/request/timing/usage metadata but no
credential or authorization header. Request identity is a SHA-256 digest over
non-secret canonical request metadata, not the text or audio body.

## HTTP And Credential Boundary

Production requests target Deepgram HTTPS endpoints. Tests may use loopback
HTTP endpoints only. Credentials resolve in this order:

1. the configured environment variable, default `DEEPGRAM_API_KEY`;
2. the configured key-file environment variable, default
   `ADL_DEEPGRAM_API_KEY_FILE`, whose value names an operator-approved file.

The API key is read only for request construction and is never included in
`Debug`, errors, logs, manifests, fixtures, or evidence. Endpoint validation
prevents real Deepgram credentials from being sent to arbitrary hosts.

## Media Validation

Synthesis requests explicitly declare model, voice, encoding, sample rate,
and container. Returned bytes are checked against the declared media shape
before success. Transcription requests declare their input artifact media type,
model, and language. Unsupported combinations fail before network access.

## Validation

Deterministic loopback tests prove request construction, typed capability
discovery, synthesis bytes, transcription parsing, timeout, malformed response,
media validation, status-to-error mapping, and secret redaction. The named
ignored canary `deepgram_pluto_nova3_round_trip` uses the operator-provided
Deepgram credential to synthesize a minimal fixture, transcribe that result,
and write `.csdlc/evidence/66/deepgram-live-receipt.json` with only latency,
model/voice, media, request digests, and usage metadata.

The live canary is not an ordinary CI requirement and never retains audio or
credentials unless the operator explicitly chooses an output directory.

## Owned Paths

- `adl/src/provider/deepgram.rs`
- `adl/src/provider/mod.rs`
- `adl/src/provider/profiles.rs`
- `adl/src/provider_substrate.rs`
- `adl/tests/provider_tests.rs`
- `adl/tests/provider_tests/deepgram.rs`
- `demos/podcast/DEEPGRAM_PROVIDER_WORKFLOW.md`
- `docs/tooling/PROVIDER_SETUP.md`
- `.csdlc/prepared/issues/66`
- `.csdlc/issues/66`
- `.csdlc/evidence/66`

## Non-Goals

- Episode 001 rerendering or migration.
- Realtime streaming, Gemini Live, or conversational orchestration.
- Voice cloning, voice training, or biometric identity claims.
- A general audio editor or automatic provider selection.
- Refactoring unrelated completion-provider implementations.
