# Structured Task Prompt

Template: 1.0.0

Issue: 680

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement the minimum first-class Moonshot/Kimi K3 provider support and tests needed for PR publication.

## Deliverables

- A stable ADL-facing kimi:k3 profile mapped to the current Moonshot API model id when confirmed
- Moonshot/Kimi provider setup/help text including MOONSHOT_API_KEY guidance
- Provider selection support that exposes kimi/moonshot as first-class where the repo's provider kind list requires it
- Focused deterministic tests for profile lookup, setup/help text, request/auth behavior, and failure classification

## Acceptance

1. AC-1: kimi:k3 has a stable ADL-facing profile mapped to the correct Moonshot provider-native model id, with source-truth noted if the external catalog naming is ambiguous or changed.
2. AC-2: The provider setup/help surface includes Moonshot/Kimi with required environment variable and endpoint guidance.
3. AC-3: The provider selection path treats Moonshot/Kimi as first-class instead of only as incidental hosted-adapter plumbing.
4. AC-4: Unit tests cover vendor inference, profile lookup, setup/help text, request construction, auth header behavior, and missing-credential or transport-failure classification.
5. AC-5: Evidence distinguishes offline deterministic validation from optional live Moonshot calls; the PR does not claim live provider proof.

## Dependencies

- Current origin/main
- Existing Kimi/Moonshot adapter and kimi:k2.5 profile surfaces

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/680
- adl/src/provider_adapter.rs
- adl/src/provider/profiles.rs
- adl/src/provider_substrate.rs
- adl/src/provider/mod.rs
- adl/src/cli/provider_cmd.rs
- docs/tooling/PROVIDER_SETUP.md
- Official Kimi API docs: https://platform.kimi.ai/docs/models and https://platform.kimi.ai/docs/api/list-models
- Moonshot Kimi K3 model card: https://huggingface.co/moonshotai/Kimi-K3

## Non Goals

- Do not add, rotate, print, or commit Moonshot credentials.
- Do not perform live paid model calls.
- Do not remove kimi:k2.5 or OpenRouter Kimi compatibility.
- Do not redesign provider architecture beyond the bounded first-class Moonshot/Kimi K3 provider path.
