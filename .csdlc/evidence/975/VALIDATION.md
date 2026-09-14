# #975 merge GraphQL observation transport

## Defect and repair

The generated merge-state and merge-linkage GraphQL queries were sent using
curl GET with a URL-encoded query. The production observation returned GitHub
schema introspection instead of the requested pull request, exhausting the
bounded response and failing reconciliation before merge intent creation.

Both observations now use POST with a JSON query body. The generated body is
bounded at 64 KiB and passed through curl configuration stdin, alongside the
existing private authorization header. Neither body nor credentials enter argv
or temporary files. Curl default-config suppression, minimal child environment,
redaction-before-truncation, response bounds, and the merge eligibility,
publication-linkage, pagination, durable-intent and reconciliation guards are
unchanged. POST here transports a GraphQL query, not a GraphQL mutation.

## Validation

- Native six-card edit/validate passed before implementation.
- `cargo test --manifest-path csdlc-v3/Cargo.toml --lib`: 93 passed before adding
  the final oversized-request fixture; all existing merge partial/error and
  pagination rejection matrices passed.
- `cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_adapter_tests`:
  four passed after the final fixture, including both generated observations,
  production adapter process dispatch, private stdin JSON body, redaction,
  output truncation, target admission and request size rejection.
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands
  --test remote_publication_commands`: 20 and 13 passed respectively.
- Read-only production probes for PR #971, linkage issue #928: both corrected
  observations returned process success, `truncated=false`, empty stderr,
  `data.repository.pullRequest.number=971`, no GraphQL errors and no schema
  introspection. The probe used a 64 KiB response limit. No raw responses or
  credentials were retained and no GitHub mutation was invoked.
- Repeated the same production probes against the original #928 library: both
  returned process success but `truncated=true` and no parseable PR number at
  the same 64 KiB response limit. This reproduces the before/after failure
  through the production adapter without retaining response bytes.
- Private native owner built and installed via the owner installer into the
  issue's ignored stable directory. Shared primary owner was not replaced.
- `git diff --check`: passed.

The shared owner-lane wrapper's printed C-SDLC plan still begins with v2 Gate10A
and historical wrapper contracts. For this native v3 adapter change, the focused
native library, CLI and publication suites above are the proving owner lane.
Hosted CI and independent exact-head review remain required before publication.
No Runtime, cloud workload or release-authorization proof is claimed.

## PVF inventory

| Surface | Lane class / proof role | Determinism | Resources | Release gate |
| --- | --- | --- | --- | --- |
| `production_merge_observation_posts_query_through_private_stdin` | Local adapter contract / generated request and production process transport | Deterministic; synthetic credential and isolated subprocess PATH | Small local CPU/filesystem; Python3 fixture; no network | Required #975 |
| `merge_observation_body_is_bounded_before_transport` | Local adapter contract / request-size rejection | Deterministic | Small CPU | Required #975 |
| Existing merge target tests and merge eligibility/linkage matrices | Local lifecycle contract / identity, error, pagination and no-intent guards | Deterministic fake remote | Local CPU/filesystem | Required #975 |
| Production PR971 observation | Live read-only observation / GitHub accepts generated query through real adapter | Remote state varies; observed identity only | Two authenticated reads per candidate/control probe; no mutation | Bounded #975 transport proof, not merge authorization |
