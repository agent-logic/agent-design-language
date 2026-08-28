# #551 Design — Config-owned Polis identity

## Objective

Add one validated Runtime-owned Polis identity projection and render it in the HTML Observatory. Unity remains deferred under backlog issue #84 and is neither edited nor gating.

## Authority and dependencies

- Canonical issue: `agent-logic/agent-design-language#551`.
- Merged inputs: #510 configuration reload and #550 trusted Observatory origin wiring.
- Polis identity and public endpoint presentation are configuration-owned. Every declared Polis parameter may hot-load without restarting the Runtime; no value is inferred from another field.
- DNS, TLS, routing, certificates, and public-infrastructure mutation are outside scope.

## Configuration contract

`RuntimeInitConfig` gains one required `polis` section:

- `id`: bounded durable Polis identifier using the existing safe-identifier grammar.
- `display_name`: trimmed, nonempty, bounded operator-facing text.
- `public_domain`: canonical lowercase DNS name equal to the host in `api.public_base_url` and `api.tls.server_name`.
- `observatory_public_origin`: exact HTTPS origin with no path, query, fragment, credentials, or wildcard.

Startup validation fails closed for missing, duplicated, malformed, or inconsistent values. Duplicate `polis` sections or duplicate identity keys are rejected rather than resolved by last-value-wins parsing. The configuration and diagnostics never contain private keys, tokens, certificate bytes, or machine-local paths.

## Runtime and reload behavior

The control service owns a `PolisIdentityFeed` snapshot containing the four configured fields plus the Runtime public API base. Production startup injects the validated snapshot before serving.

The #510 reload applier validates the complete next `RuntimeInitConfig`, constructs one complete next `PolisIdentityFeed`, and atomically replaces the active snapshot. Changes to `id`, `display_name`, `public_domain`, `observatory_public_origin`, and Runtime API base all take effect without restarting the Runtime. Any rejected parse or apply retains the complete last-known-good snapshot and emits one bounded, redacted `parse_invalid` or `validation_invalid` diagnostic without echoing configuration values. Readers observe either the prior complete snapshot or the next complete snapshot, never a mixed field set.

## Observatory contract

The current Observatory feed advances from schema v2 to v3 and adds one required `polis_identity` object. It exposes only:

- `polis_id`
- `display_name`
- `public_domain`
- `runtime_api_base`
- `observatory_public_origin`

The OpenAPI projection and focused Runtime tests bind the exact shape and redaction boundary. The existing legacy v1 and current v2 constants remain accepted only by their existing compatibility paths; neither is reinterpreted as v3 and neither synthesizes `polis_identity`. New production output is v3 only.

## HTML consumer

The HTML Observatory replaces the hard-coded `prod-polis` readout with a stable DOM target populated from `feed.polis_identity`. A small pure projection helper validates the feed values and fails closed to an unavailable presentation; it does not infer identity from `window.location`, query parameters, DNS lookup, localhost, or fallback deployment constants.

## Exact owned paths

- `adl-runtime-kernel/src/config.rs`
- `adl-runtime-kernel/src/control.rs`
- `adl-runtime-kernel/src/control/feeds.rs`
- `adl-runtime-kernel/src/bin/adl-runtime-kernel.rs`
- `adl-runtime-kernel/tests/configuration.rs`
- `adl-runtime-kernel/tests/control.rs`
- `adl-runtime-kernel/tests/observatory.rs`
- `adl-runtime-kernel/tests/guardian_soak.rs`
- `adl-runtime-kernel/tests/support/runtime_init.rs`
- `docs/api/runtime-v3/v1/observatory.openapi.json`
- `adl-runtime-kernel/tests/openapi_contract.rs`
- `infra/runtime-v3/runtime-init.toml`
- `demos/html-observatory/index.html`
- `demos/html-observatory/app.js`
- `demos/html-observatory/tests/polis_identity.test.mjs`
- `.csdlc/prepared/issues/551/**`
- `.csdlc/evidence/551/**`
- `.csdlc/issues/551/**`

## Validation

1. Exact nextest integration targets run with `--no-tests=fail`; configuration and control tests prove valid identity, missing/duplicate/malformed fields, endpoint mismatch, bounded reload diagnostics, full-parameter atomic hot load, and invalid last-known-good retention.
2. Exact control, Observatory, and OpenAPI integration targets run with `--no-tests=fail` and prove v3 projection plus unchanged v1/v2 compatibility and redaction.
3. The issue-owned HTML proof wrapper runs the exact Node test file and rejects a TAP result with zero tests; it proves feed-owned rendering and rejects hard-coded or inferred identity.
4. `cargo fmt --check` and branch diff hygiene pass as separate commands.
5. Independent exact-head review must pass before publication; hosted CI remains the final integration gate.

## Stop conditions

- Continuity identity would be rewritten by presentation configuration.
- A hot-loaded value is applied partially or requires a Runtime restart.
- Unity or #84 paths enter the diff.
- The UI retains or introduces a production deployment-name fallback.
- A test selects zero cases or relies only on source-string assertions.

## Rollback

Revert the issue commit. The prior feed schema and hard-coded HTML readout return together; no DNS, TLS, continuity state, or external infrastructure is mutated by this issue.
