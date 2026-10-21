# UTS 1.1 consumer reconciliation

This decision matrix makes this repository's schemas authoritative for UTS
validity. It compares the standalone candidate with the legacy ADL/Runtime
`adl-uts` schema copies. It does not establish installed Runtime support.
Resource pinning belongs to UTS issue #4; Runtime adoption to Runtime #194;
public ADL contract alignment to ADL #1335.

## Decisions

| Surface | Decision | Consumer migration |
| --- | --- | --- |
| Declaration `lifecycle` | Retain optional draft/experimental/stable/deprecated/sunset vocabulary. | Add explicit typed support where unknown fields are rejected. Lifecycle conveys no permission. |
| Declaration `description` | Retain JSON Schema minimum length 12. | Match structural validation. Any additional trimmed-length requirement must be documented as consumer admission policy or proposed upstream; do not silently call it UTS invalidity. |
| Declaration and invocation `extensions` | Retain the canonical name restriction, including authority/grant/execute/permission exclusions. | Use the shared negative fixtures. Arbitrary extension values are metadata and never grant authority. |
| Declaration semantic constraints | Retain required-auth/non-none, exfiltration/high-risk equivalence and exclusive `none` side effects. | Match JSON Schema and typed validation using one corpus. |
| Declaration error models | Retain required nonempty errors and canonical error-item constraints. | Existing Rust nonempty-error enforcement agrees with the schema; include a negative corpus case. |
| Required extensions | Retain structurally permitted extension values. | Unsupported required extension semantics are consumer capability/admission refusal, separate from schema validity. Never silently execute an unsupported required behavior. |
| Invocation `caller` | Retain structured kind and optional id. Reject the old string shape. | A legacy string may supply id only after a trusted adapter independently knows kind. Do not infer identity or grants from supplied text. Reject ambiguous mappings. |
| Invocation `intent` | Retain extensible nonempty text. | Preserve known old enum strings as values; unfamiliar values remain descriptions and may require Runtime policy denial. |
| Invocation `retry_policy` | Retain optional retryable and nonnegative max_attempts. | Explicitly map old retry metadata; never increase an existing local ceiling. Preserve scheduling strategy separately in Runtime policy. Missing fields cannot imply unlimited retries. |
| Legacy invocation `retry` | Reject as an unknown UTS field. | Require explicit adapter conversion; unsupported or ambiguous strategy/count semantics must fail closed, not disappear. |
| Legacy invocation `timeout_ms` | Reject as an unknown UTS field. | Preserve deadlines in the Runtime invocation envelope/policy. A converter unable to preserve a deadline must reject conversion. |
| Invocation `tool.name` | Retain nonempty external-reference vocabulary. | Exact registry binding remains required; schema validity does not prove a registered declaration exists. |
| Invocation side-effect exclusivity | Retain canonical allOf form. | Both legacy and canonical forms reject mixed `none`; test behavior, not textual identity. |
| Schema identifiers and titles | Canonical resource identity belongs to UTS. | Do not use old ADL schema URLs as the acquisition authority. Issue #4 defines immutable resources and identity migration. |

## Compatibility and enforcement

Continue accepting v1 declarations under the retained v1 schema; do not merely
change their version label to v1.1. Consumer normalization must preserve side
effects, replay safety, idempotence, sensitivity, resource and authentication
metadata, then validate its output. `compatible_versions` is descriptive metadata,
not proof that arbitrary declarations can be downgraded without loss.

The broader invocation metadata schema is not Runtime's execution transport.
`ToolProposalV1` may remain a Runtime-owned envelope. Its adapter must preserve
attribution, timeout, retry, replay and authority controls before any execution.
Unknown or lossy mappings must stop before effects. No fixture here claims that
Runtime already performs those mappings.

The dangerous-valid retry fixture deliberately combines retry metadata with
`not_replay_safe`. It passes structural validation. A runtime must still deny
unapproved replay and enforce its own retry ceiling. Schema validation must not
be expanded into an implicit authorization engine to make this fixture fail.

## Shared corpus and validation

`conformance/manifest.json` is the shared machine-readable corpus index. Each
entry names a schema, payload and expected outcome (`valid`, `invalid`, or
`dangerous_valid`); reconciliation entries include a human-readable reason.
Consumers must treat dangerous_valid as structurally valid while proving policy
behavior separately. Existing v1 fixtures remain unchanged historical inputs.

Run `npm ci` followed by `npm test` for standalone validation, or
`npm run validate:fixtures` for a focused corpus check. The corpus is deterministic,
requires only bounded local CPU/disk, and makes no provider calls. It is contract
proof for this release candidate; it is not live Runtime qualification, provider
endorsement or public-release approval. Consumers should execute the same payloads
with their typed parser and validator and report all differing outcomes.

## CI bootstrap

The single `UTS contract validation` job runs the existing `npm test` suite with
locked dependencies on Node 22 and Python 3.12. This independent CI portion of
UTS #5 is brought forward into #3 to supply actual integration proof. The lockfile
has no packages requiring install scripts; installation disables lifecycle
scripts. Validation uses deterministic fixtures and the mock governed adapter,
not live provider calls. Dependency advisory review and public-release approval
remain in #5 and #6. Disabling npm's install-time audit avoids making advisory
service availability part of deterministic CI; it does not resolve retained audit
findings or approve dependency safety.
