# Runtime v3 API resources

The Runtime owner maintains the embedded Core and Observatory API contracts and their versioning policy:

- [openapi.json](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/api/runtime-v3/v1/openapi.json)
- [observatory.openapi.json](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/api/runtime-v3/v1/observatory.openapi.json)
- [API_VERSIONING.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/api/runtime-v3/v1/API_VERSIONING.md)

These links pin the reviewed owner source. The retained 11-case OpenAPI proof checks schema and source-route contracts, including embedding and Swagger wiring; it is not an HTTP request, deployed service, or live TLS qualification.

The [distributed projection schema](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/api/runtime-v3/v1/distributed.openapi.json) is maintained by the same owner. Its two original Rust consumers passed against this exact source in an isolated offline Linux fixture. They check projection/schema parity, nested DTOs, enums, limits, security, and errors; they do not establish a live distributed deployment or a cross-platform native receipt matrix.
