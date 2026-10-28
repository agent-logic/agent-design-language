# Universal Tool Schema (UTS)

## Current authority

The [Universal Tool Schema owner](https://github.com/agent-logic/universal-tool-schema)
owns the normative contract. ADL consumes immutable resource `0.1.0-rc.1` from
source revision `09ec2659c8f82365643341d4f66bac85f1bcaf60`; the full archive hash
and packaging revision are recorded in the
[resource pin](../../../adl-uts/resources-pin.json).

Use the pinned machine-readable schemas and companion specifications:

- [UTS v1 declaration schema](../../../adl-uts/resources/schemas/uts/v1.0/universal_tool_schema.v1.schema.json)
  and [v1 specification](../../../adl-uts/resources/docs/specs/UTS_V1.0_SCHEMA.md).
- [UTS v1.1 declaration schema](../../../adl-uts/resources/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json)
  and [v1.1 specification](../../../adl-uts/resources/docs/specs/UTS_V1.1_SCHEMA.md).
- [UTS v1.1 invocation schema](../../../adl-uts/resources/schemas/uts/v1.1/tool_invocation.v1_1.schema.json)
  and [consumer reconciliation rules](../../../adl-uts/resources/docs/specs/CONSUMER_RECONCILIATION.md).

The older local [v1 design](UTS_V1.0_SCHEMA.md),
[v1.1 proposal](UTS_V1.1_SCHEMA.md), and
[narrative](ADL_UNIVERSAL_TOOL_SCHEMA.md) are retained design references. Their
historical authority statements and proposed fields do not override the pinned
upstream schemas or establish current Runtime behavior.

## Public ADL support

[`adl-uts` 0.1.0](../../../adl-uts/README.md) supports `uts.v1` and `uts.v1.1`
declarations. `load_tool_declaration` validates the original wire version,
normalizes v1 into v1.1, then validates the normalized declaration. The loader
preserves side effects, replay safety, resources and other descriptive metadata.
UTS v1.1 supports optional lifecycle, categories, side-effect tags, observability
and planning metadata. Absent optional fields remain absent when serialized;
explicit null is rejected.

The package embeds the invocation schema and exposes the canonical corpus to an
offline schema engine. This is structural validation, not an invocation executor.
Registry binding, caller identity, approval, retry ceilings, deadlines and
execution policy belong to the separately installed Runtime owner.
See [package installation and proof boundaries](PACKAGE.md).

## Examples and conformance

These examples are exact copies of pinned canonical fixtures:

- [v1.1 declaration with stable lifecycle](examples/declaration-v1_1.json)
- [v1.1 invocation metadata](examples/invocation-v1_1.json)

Both are covered by the [23-case canonical corpus](../../../adl-uts/resources/conformance/manifest.json).
The installed public consumer reports 13 declaration binding/normalization cases
and ten invocation schema-engine cases. The separate historical ADL suite has
21 cases and includes consumer admission refusals. Counts and claims must remain
separate.

A required extension can be structurally valid while unsupported by a consumer.
Consumers must refuse unsupported required behavior before execution. Lifecycle
`stable`, retry metadata and other descriptive fields grant no authority.

UTS validity grants neither execution nor replay permission. See the
[UTS and ACC explainer](../../explainers/UTS_AND_ACC.md) for that distinction.
