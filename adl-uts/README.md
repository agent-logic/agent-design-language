# adl-uts 0.1.0

Canonical Rust types and semantic validators for UTS `uts.v1` and the implemented
`uts.v1.1` declaration surface. The crate version is independent of schema versions.
Schema compatibility conveys no execution, replay, or data disclosure authority.
ACC, the registry, and the Runtime freedom gate retain those decisions.

See `docs/specs/uts/PACKAGE.md` in the ADL repository for installation, compatibility,
migration and production-consumer proof. This package does not imply external
registry publication or complete implementation of proposed UTS v1.1 semantics.

The immutable upstream resource identity is recorded in `resources-pin.json`.
`resources/` preserves the verified upstream payload, including its license,
NOTICE, schemas and 23-case conformance manifest. The schema accessors return
these canonical schema bytes parsed as JSON; `JsonSchema` derives describe Rust
shapes and are not the normative contract.

`load_tool_declaration` validates the original v1 declaration before normalization,
then validates the resulting v1.1 declaration. Optional v1.1 metadata round-trips
without synthesizing null values. Required-extension metadata is structurally
valid; consumers must separately refuse unsupported required behavior before
execution. The historical 21-case ADL suite retains that admission refusal and
must not be reported as the canonical 23-case upstream suite.
