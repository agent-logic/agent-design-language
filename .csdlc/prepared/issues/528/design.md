# Issue #528 design: Vertex AI Gemini provider transport

## Purpose

Issue #528 adds a distinct Google Vertex AI transport for Gemini while preserving the existing Gemini semantic normalization path. The intended shape is one shared Gemini codec with two transport front doors:

- Gemini Developer API: existing direct Gemini credential and Generative Language endpoint route.
- Vertex AI: Google Cloud-native Application Default Credentials or workload identity, explicit project/location/model resolution, and Vertex endpoint construction.

The transport must fit the existing ADL provider substrate instead of becoming a parallel provider stack.

## Scope

Owned by #528:

- Vertex AI provider configuration for project, location, publisher, model, endpoint, timeout, retry, and cancellation boundaries.
- ADC/workload-identity bearer token acquisition boundary with no embedded API keys and no credential material in receipts/logs.
- Shared Gemini request/response/tool/safety/error normalization reused by Developer API and Vertex AI routes.
- Provider receipts and telemetry that identify `vertex_ai` separately from `gemini_developer_api` and redact project/credential-sensitive fields.
- Deterministic contract tests for endpoint construction, auth boundary behavior, request/response mapping, streaming mapping, UTS tool schema/argument preservation, retry/cancellation classification, and redaction.
- Operator documentation for IAM, `aiplatform.googleapis.com`, ADC/workload identity, regional/model selection, quota, billing, and the separately authorized live smoke test.

Not owned by #528:

- Enabling Google Cloud APIs, mutating IAM, creating service accounts, or scanning credential files.
- Replacing Gemini Developer API, Ollama, MLX, Bedrock, or OpenRouter.
- GCP infrastructure foundation, Terraform estate, networking, production deployment, or #509 GCP portability qualification.
- Claiming live Vertex AI proof without explicit paid/external authorization.

## Implementation approach

1. Inventory the provider substrate and current Gemini adapter boundaries in `adl/src/provider_adapter.rs`, `adl/src/provider_substrate.rs`, and `adl/src/provider_communication.rs`.
2. Extract or introduce the smallest shared Gemini semantic codec needed so existing Developer API behavior stays stable and Vertex AI can reuse request, response, streaming, tool, safety, and error mapping.
3. Add a Vertex AI transport selector/configuration path that resolves explicit project, location, publisher, model, endpoint, timeout, and cancellation behavior.
4. Implement an auth boundary that consumes ADC/workload identity through a narrow token provider interface. Tests should use fixtures or fake token providers; live token acquisition remains an explicitly authorized smoke lane.
5. Add telemetry/receipt fields that distinguish `vertex_ai` from `gemini_developer_api` while redacting tokens, prompts, raw responses, and credential material.
6. Add focused tests for deterministic behavior and preserve existing provider tests.
7. Record the optional live Vertex smoke command and required environment contract without executing paid/provider calls unless separately authorized.

## Validation plan

- Run focused provider tests covering Gemini codec reuse and Vertex AI route behavior.
- Run tests that reject malformed Vertex endpoint/model/project/location config and incorrect UTS tool argument mappings.
- Run redaction tests proving receipts/logs do not contain access tokens, credential file contents, prompt bodies, or raw sensitive responses.
- Run existing focused provider tests for Gemini Developer API, OpenRouter, Bedrock, and Ollama surfaces touched by shared-codec extraction.
- Run `cargo fmt`, targeted `cargo test`, and `git diff --check`.
- Treat live Vertex AI invocation as a deferred external validation lane unless explicitly authorized with cost, project, region, and identity.

## Dependency truth

- #514 is the completed shared provider-profile dependency.
- #509 remains a separate GCP portability qualification issue and is not a prerequisite for local deterministic #528 implementation proof.
- `aiplatform.googleapis.com`, quota, IAM, ADC/workload identity, and billing are required only for the separately authorized live smoke test.
