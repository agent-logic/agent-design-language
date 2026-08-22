# Issue #461 — Runtime lifecycle TLS authority is config-only

## Decision

The Runtime init configuration is the sole authority for the lifecycle soak TLS certificate chain, private key, and trust roots. The lifecycle soak CLI will not accept TLS path flags and will not replace config values after parsing.

## Implementation

1. Remove the three TLS path fields from the lifecycle soak argument surface.
2. Parse the Runtime init template and resolve its existing `api.tls` paths.
3. Validate those configured files before startup: absolute paths, regular files, no unsafe symlink substitution, distinct certificate/key/root identities, and restrictive private-key permissions.
4. Keep diagnostics structural and redacted; never include key material or configured TLS paths in argv, logs, or retained receipts.
5. The bounded Guardian harness generates a temporary Runtime config whose `api.tls` fields point to its test fixtures, then invokes the same config-only lifecycle soak path.

## Security boundary

TLS authority has one source. Configuration validation fails closed before Runtime startup. Command arguments and environment variables cannot override TLS identity. The private key remains a file reference in protected Runtime configuration and is never copied into lifecycle evidence.

## Compatibility

Production configuration already contains the TLS fields. Callers that supplied the removed flags must migrate those paths into the config. HTTPS/WSS lifecycle behavior, restart behavior, and readiness semantics remain unchanged.

## Validation

- Argument parser rejects the removed flags.
- Config validation covers missing, relative, non-regular, aliased, symlinked, and permissive-key cases.
- The executable Guardian fixture proves HTTPS and WSS through config-owned TLS paths.
- Focused Rust tests, shell syntax, and diff hygiene pass.

## Non-goals

Certificate issuance, rotation, public DNS, CloudFormation, #269, and unrelated Runtime configuration changes are excluded.
