# Structured Task Prompt

Template: 1.0.0

Issue: 528

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #528 PROV-C only; implement the Vertex AI Gemini provider transport and shared Gemini codec reuse without live GCP mutation, credential exposure, #509 qualification, or unrelated provider rewrites.

## Deliverables

- Vertex AI Gemini provider transport source changes
- Shared Gemini semantic codec source changes or extraction
- Provider configuration and receipt/telemetry updates
- Focused deterministic provider tests
- Operator documentation for Vertex AI IAM/API/ADC/location/quota/non-claims
- Optional live Vertex AI smoke-test packet requiring separate authorization
- Typed C-SDLC v2 lifecycle cards, review, publication, and terminal truth

## Acceptance

1. AC-1: ADL exposes Vertex AI as a distinct provider transport without duplicating the Gemini semantic codec
2. AC-2: Vertex AI authentication uses ADC or workload identity and does not require or persist a Gemini API key
3. AC-3: Requests resolve explicit project, location, endpoint, publisher/model identity, timeout, and cancellation boundaries
4. AC-4: Streaming and non-streaming Gemini responses normalize into the existing ADL provider result contract
5. AC-5: UTS tool declarations and returned tool calls preserve both tool names and arguments; tests reject incorrect argument mappings
6. AC-6: IAM, authentication, quota, billing, safety, retryable/non-retryable errors, and redaction map into common provider failure and observability contracts
7. AC-7: Provider receipts distinguish gemini_developer_api from vertex_ai and contain no access tokens, credentials, prompt bodies, or raw sensitive responses
8. AC-8: Existing Gemini Developer API, OpenRouter, Bedrock, and Ollama focused provider tests remain green
9. AC-9: Any live Vertex call is separately authorized, cost-bounded, region-recorded, and reported as provider qualification rather than local deterministic proof

## Dependencies

- #514 shared provider inference profiles is terminal/completed
- #509 GCP portability qualification is separate and not a prerequisite for deterministic #528 implementation
- Live Vertex AI smoke testing requires aiplatform.googleapis.com, suitable IAM, ADC/workload identity, quota, region/model availability, and explicit operator authorization

## Inputs

- adl/src/provider_adapter.rs
- adl/src/provider_substrate.rs
- adl/src/provider_communication.rs
- docs/validation/pvf_lanes.json
- .adl/docs/TBD/GCP_ACCOUNT_MOVE_IN_PLAN.md
- .adl/docs/TBD/ISSUE_268_GCP_SIX_RESIDENT_QUALIFICATION_PLAN.md
- Issue #514 shared provider inference profiles
- Issue #509 GCP portability qualification

## Non Goals

- Rewriting the existing Gemini adapter from scratch
- Replacing Ollama, MLX, Bedrock, OpenRouter, or direct Gemini access
- GCP foundation, Terraform, networking, production deployment, or #509 qualification
- Enabling APIs or mutating GCP IAM
- Executing live provider calls without separate authorization
- Claiming all Gemini Developer API features are identical on Vertex AI without evidence
