# Podcast Studio Deepgram Provider Handoff

Podcast Studio supplies the approved script text and media choices to ADL's
typed speech provider. It does not construct Deepgram URLs, authorization
headers, or JSON payloads, and it does not read credential files.

## Provider Boundary

1. Expand `deepgram:aura-2-pluto-en` through the canonical ADL provider profile
   registry.
2. Build a `SpeechProvider` from the expanded `ProviderSpec`.
3. Submit a typed `SynthesisRequest` with the approved text, Pluto voice,
   encoding, container, and sample rate.
4. Store the returned validated audio through the episode artifact workflow;
   keep only `SpeechProvenance` in provider evidence.
5. When transcription is needed, submit the approved audio artifact through a
   typed `TranscriptionRequest` using Nova-3 and an explicit language.

The episode workflow owns editorial approval, mastered artifacts, transcript
reconciliation, and publication. The Deepgram provider owns authentication,
HTTP transport, response parsing, media validation, stable error classes, and
redacted request provenance.

## Credential Boundary

Codex or an operator maps an approved credential into `DEEPGRAM_API_KEY` or
maps an approved key-file path into `ADL_DEEPGRAM_API_KEY_FILE` for the one
command that needs it. Darlicia does not manage these values. Provider errors
and retained episode manifests must never contain the key or authorization
header.

## Proof Boundary

The issue #66 canary proves a minimal Pluto-to-Nova-3 round trip. It is not an
episode render and does not modify Episode 001, the podcast feed, or public
storage. A future episode still requires the four editorial approvals in the
episode creator workflow before publication.
