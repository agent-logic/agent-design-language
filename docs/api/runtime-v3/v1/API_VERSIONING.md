# Runtime v3 API Versioning

Runtime Core API v1 and Observatory API v1 are independently versioned contracts for the Runtime v3 Axum/Tokio/Rustls API surface.

The runtime reads ports, public base URLs, TLS material, and allowed Observatory origins from init/config. The OpenAPI `servers` entries use variables and examples; they are not runtime constants.

Compatibility rules:

- Additive fields, response headers, examples, and enum values may be added within v1 when existing clients can ignore them.
- Removing fields, changing required fields, changing authentication, changing frame direction, or changing operation semantics requires a new major API version.
- Deprecated fields must remain documented until the next major version and must include a removal note.
- Unsupported, fixture-only, degraded, simulated, or unavailable behavior must not appear as an operational API.

Current route-serving boundary:

- Runtime v3 currently exposes `POST /v1/control`, `GET /v1/observatory`, `OPTIONS /v1/observatory`, and `GET /v1/observatory/ws`.
- Runtime v3 does not currently serve `GET /v1/openapi.json` or `GET /v1/observatory/openapi.json`; this contract is retained as repository documentation until those discovery routes are implemented as real Axum routes.
