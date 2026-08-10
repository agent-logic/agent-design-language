# Runtime v3 API Versioning

Runtime Core API v1 and Observatory API v1 are independently versioned contracts for the Runtime v3 Axum/Tokio/Rustls API surface.

The runtime reads ports, public base URLs, TLS material, and allowed Observatory origins from init/config. The OpenAPI `servers` entries use variables and examples; they are not runtime constants.

All HTTP and WebSocket endpoints use the Axum/Rustls stack with ordinary
server-authenticated TLS, a real DNS name, and externally issued certificate
material. The Runtime does not issue certificates or install trust anchors.
`POST /v1/control` authorizes the Ed25519-signed request body; it is not a
listener-side mutual-TLS endpoint.

Compatibility rules:

- Additive fields, response headers, examples, and enum values may be added within v1 when existing clients can ignore them.
- Removing fields, changing required fields, changing authentication, changing frame direction, or changing operation semantics requires a new major API version.
- Deprecated fields must remain documented until the next major version and must include a removal note.
- Unsupported, fixture-only, degraded, simulated, or unavailable behavior must not appear as an operational API.

Current route-serving boundary:

- Runtime v3 currently exposes `POST /v1/control`, `GET /v1/observatory`, `OPTIONS /v1/observatory`, and `GET /v1/observatory/ws`.
- Runtime v3 exposes authenticated ACIP/A2A transport at `GET /v1/acip/ws`.
  The served Core API document is canonical: the server sends a JSON session
  frame, then accepts bounded binary Protobuf work frames. Text work frames are
  rejected. `openapi.json` is the sole checked-in and served contract for this
  endpoint; no parallel companion contract is maintained.
- Runtime v3 serves the Core API contract at `GET /v1/openapi.json`.
- Runtime v3 serves the Observatory API contract at `GET /v1/observatory/openapi.json`.
- Runtime v3 serves an embedded Swagger UI at `GET /v1/docs/` with both contracts available from its API selector; `GET /v1/docs` redirects to the slash-stable route.
- Runtime v3 serves a dedicated Observatory Swagger UI at `GET /v1/observatory/docs/`.
- The documentation assets and OpenAPI documents are embedded in the Rust binary. They do not depend on the current working directory, a sidecar server, or a runtime CDN.
- The documentation and raw spec routes are intentionally unauthenticated because they serve static schema artifacts only and contain no runtime state, credentials, or operator data. Operational routes retain their declared bearer, signed-command, or WebSocket authentication policy.
