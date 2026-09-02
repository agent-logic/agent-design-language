# Polis Vertex AI configuration

Issue #592 configures the governed Polis runtime for Vertex AI without making a
live paid Vertex AI request. The runtime initialization file names the provider,
GCP project, Vertex location, model, and credential source explicitly so the
provider path cannot fall back to ambient defaults or a mock transport.

The active GCP configuration surface is `infra/runtime-v3/runtime-init.toml`:

```toml
[polis.vertex_ai]
provider = "vertex_ai"
gcp_project = "agent-logic-dev"
vertex_location = "us-central1"
model = "gemini-2.5-flash"

[polis.vertex_ai.credential_source]
kind = "application_default_credentials"
```

## Credential boundary

The tracked config records only the credential source type. Secret JSON, access
tokens, refresh tokens, and service-account private keys must not be copied into
tracked files, lifecycle cards, validation output, or PR text. Operators may use
Application Default Credentials when the host identity is already approved for
the Agent Logic GCP project.

If a service-account file is later authorized, the runtime accepts only an
absolute file path in `polis.vertex_ai.credential_source.path`. The file contents
remain outside Git and outside retained evidence.

## Failure classification

The Runtime classifies Vertex AI setup and request failures into operator-facing
buckets before any acceptance evidence is recorded:

- `missing_credentials`: ADC or the configured credential source is unavailable.
- `disabled_api`: Vertex AI / `aiplatform.googleapis.com` is disabled for the
  configured project.
- `project_location_mismatch`: the configured GCP project, Vertex location, or
  publisher model do not match.
- `quota_or_auth`: IAM, permission, quota, or rate-limit failures.
- `model_or_request`: request-shape or model errors after the route is selected.
- `transport`: network or unclassified provider transport failure.

These classifications are proof boundaries, not a live cutover. The paid Vertex
AI request remains deferred until the operator separately authorizes it.
