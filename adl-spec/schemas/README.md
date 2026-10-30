# adl-spec/schemas

Schema artifact directory for ADL spec publishing.
Place versioned schema outputs here and keep narrative documentation in tracked
spec docs.

## In This Directory

- `adl_constitution.yaml` — constitutional governance artifact for delegation/runtime planning
- `freedom_gate_event.yaml` — Freedom Gate event schema
- `uts/v1.0/universal_tool_schema.v1.schema.json` — supported UTS v1 declaration schema
- `uts/v1.1/universal_tool_schema.v1_1.schema.json` — implemented UTS v1.1 declaration schema
- `uts/v1.1/tool_invocation.v1_1.schema.json` — embedded UTS v1.1 invocation schema

The UTS files here are compatibility mirrors of the verified upstream payload
in [`adl-uts/resources/`](../../adl-uts/resources/), identified by its
[resource pin](../../adl-uts/resources-pin.json). UTS owns the normative contract.
The public package validates declarations and exposes the invocation schema for
structural validation; neither establishes execution or replay permission.
See [supported UTS contracts](../../docs/specs/uts/README.md).

ACC specification surfaces:

- `acc/v1.0/adl_capability_contract.v1.schema.json` — ACC v1.0 specification baseline
- `acc/v1.1/adl_capability_contract.v1_1.schema.json` — additive ACC v1.1 schema target

ACC enforcement belongs to the separately installed Runtime owner. The presence
of either ACC schema does not establish installed Runtime support or qualification.

## See Also / Canonical Docs

- UTS docs: `../../docs/specs/uts/`
- ACC docs: `../../docs/specs/acc/`
- Root project entrypoint: `../../README.md`
- ADRs: `../../docs/adr/`
