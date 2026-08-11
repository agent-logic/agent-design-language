# Issue 101: GitHub lifecycle route guardrail

## Decision

Covered C-SDLC GitHub operations have exactly one Rust owner per surface:

- `csdlc-github-issue`: issue create, read, update, comment, and close.
- `csdlc-github-pr`: direct PR-state observation.
- `csdlc-publish`: PR publication.
- `csdlc-finish`: terminal delivery and closeout authority.

The ChatGPT GitHub connector and raw `gh` are prohibited for these writes. A
missing or unavailable owner binary is a fail-closed condition and never grants
fallback authority.

## Narrow Implementation

1. Align the root `AGENTS.md` owner map and prohibitions with
   `docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md`.
2. Add a dedicated focused GitHub integration target that reads both documents and requires the
   owner names, connector prohibition, raw-`gh` prohibition, and fail-closed
   language on both surfaces.
3. Add a static, credential-free incident fixture for the observed connector
   `403 Resource not accessible by integration`. The test requires the fixture
   to classify this as `integration_authorization_failure`, with
   `token_failure: false` and `fallback_authorized: false`.

## Invariants

- Shared token discovery, precedence, propagation, and redaction are unchanged.
- Connector failure does not imply the approved operator token failed.
- Failure of a forbidden route cannot authorize another forbidden route.
- The fixture contains no token, authorization header, or credential material.
- Issue #100 and its artifacts are outside the touched scope.

## Validation

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions`.
- `git diff --check`.
- Typed `csdlc-github-issue issue_read` through the approved default resolver,
  retaining only redacted structured evidence.

## Rollback

Revert the policy/test/fixture commit as one unit. No production runtime or token
resolver state is migrated, so rollback requires no data or credential action.
