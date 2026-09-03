# Polis GCP Vertex AI configuration design

## Boundary

Issue #592 configures Polis to select the Vertex AI provider transport delivered
by #528. PR #603 merged that prerequisite. Its asynchronous lifecycle closeout
is bookkeeping and does not block this issue; execution must still verify that
the merge is present in the selected base revision before binding.

The implementation is limited to explicit, repository-owned configuration and
its validation/documentation. It does not redesign the provider abstraction,
change C-SDLC authority, or authorize a paid Vertex AI request.

## Configuration contract

Polis configuration must name the provider, GCP project, Vertex location, and
model explicitly. Authentication is referenced by an approved credential source
or Application Default Credentials identity; secret JSON, access tokens, and
credential contents never enter tracked configuration, logs, or evidence.

The Runtime loads the explicit fields, validates them before provider dispatch,
and rejects missing or inconsistent values. It must not silently fall back to an
ambient project, location, model, mock transport, or another provider.

## Validation and failure classification

Focused proof must cover configuration parsing, redacted diagnostics, and the
production provider-selection path. Negative cases distinguish missing
credentials, disabled Vertex APIs, project/location mismatch, quota or auth
failure, and model/request failure. A live or billable request remains deferred
until the operator separately authorizes it.

The retained tooling canary checks that the issue-specific design, diagram,
validators, and typed records remain present. Validation output must identify
the exact revision and command without including credentials.
