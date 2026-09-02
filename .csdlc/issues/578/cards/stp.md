# Structured Task Prompt

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove a first-class general GLM-5.3-Flash profile using the existing provider/profile infrastructure, with runtime overrides and reviewer smoke as a use-case proof only.

## Deliverables

- `z_ai:glm-5.3-flash` profile resolving to `hosted:adl-z-ai:glm-5.3-flash` and provider model id `glm-5.3-flash`.
- Z.ai HTTP request support for documented general inference parameters: `reasoning_effort`, `thinking.clear_thinking`, `temperature`, `top_p`, and bounded `max_tokens`.
- Focused deterministic tests for profile expansion, request shaping, invalid parameter rejection, and redaction.
- Provider docs/evidence that cite Z.ai, Hugging Face, OpenRouter, and Unsloth facts used for model id, route, context/output limits, and local-runtime notes.
- Reviewer-selection proof that selects the general provider profile and either uses configured credentials or records a credential-gated skip truthfully.

## Acceptance

1. AC-1: ADL exposes a first-class direct Z.ai profile named `z_ai:glm-5.3-flash` with stable ADL model identity `hosted:adl-z-ai:glm-5.3-flash`, provider model id `glm-5.3-flash`, and the current documented Z.ai chat-completions endpoint.
2. AC-2: Profile expansion is deterministic, redacted, and distinct from existing `z_ai:glm-5` and `z_ai:glm-5-current` profiles.
3. AC-3: The Z.ai provider request path applies good default GLM-5.3-Flash settings and supports runtime overrides for `reasoning_effort` in `low|high|max`, explicit `thinking.clear_thinking`, `temperature`, `top_p`, and `max_tokens` bounded to 131072.
4. AC-4: Invalid GLM-5.3-Flash parameter values fail closed before network dispatch and do not silently fall back to provider defaults.
5. AC-5: Focused tests prove profile expansion, request body materialization, parameter validation, and credential redaction without requiring live credentials.
6. AC-6: A reviewer-agent selection proof or smoke command can select the general `z_ai:glm-5.3-flash` profile; if `ZAI_API_KEY` is absent, the proof records a credential-gated skip rather than live model proof.
7. AC-7: Provider docs/evidence cite researched Z.ai, Hugging Face, OpenRouter, and Unsloth facts and state this is new work after #514, not retroactive #514 evidence.
8. AC-8: No #446 or #455 implementation scope is touched.

## Dependencies

- #514 / PR #549 shared provider inference-profile machinery is complete.
- Official Z.ai GLM-5.3-Flash and chat-completion docs.
- Hugging Face `zai-org/GLM-5.3-Flash` model card.
- OpenRouter model listing for provider-variant awareness.
- Unsloth GLM-5.3-Flash local-runtime notes.

## Inputs

- adl/src/provider/profiles.rs
- adl/src/provider/http_family.rs
- adl/src/provider/mod.rs
- adl/tests/provider_tests/profiles.rs
- adl/tests/provider_tests/http_family.rs
- docs/provider/inference-profiles.md
- docs/milestones/v0.92.1/evidence/provider/prov-a/README.md
- .csdlc/prepared/issues/514
- .csdlc/prepared/issues/5526/record-execution.json

## Non Goals

- Paid or live provider calls without configured credentials and explicit truthful validation scope.
- Full OpenRouter backend/provider-pinning onboarding for GLM-5.3-Flash.
- Replacing #515 local-model shadow execution.
- Benchmark, quality, or production-readiness claims beyond the proof run.
- #446 or #455 changes.
