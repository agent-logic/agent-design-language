# Characterization parity map

The reviewed `adl-characterization/corpus/v1/fixtures` corpus from #5337 is
mapped to WP-04 as follows. The automated map lives in
`tests/characterization_parity.rs`.

| Fixtures | WP-04 outcome |
|---|---|
| `six-primitives`, `map-a`, `map-b`, `sequential-a`, `sequential-b` | Parse, validate, and preserve the characterized ordering contract. |
| `malformed`, `schema-unknown`, `unsupported-run-field` | Reject with stable syntax or unknown-field diagnostics. |
| `unknown-provider`, `unknown-agent`, `unknown-task`, `unknown-tool`, `unknown-workflow` | Reject with the corresponding stable reference diagnostic. |
| `state-missing`, `cycle` | Reject with stable state-reference or cycle diagnostics. |
| `branch-a`, `branch-b`, `fork-join` | Reject because compiler pattern expansion belongs to #5338. |
| `mock-run` | Represent and validate the language document; execution remains #5340 scope. |

Provider invocation, workflow execution, compiler lowering, placement decisions,
artifact storage, replay, and legacy migration are not parity claims of this
crate.
