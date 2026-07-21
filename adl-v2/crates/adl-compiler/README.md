# ADL compiler

`adl-compiler` is a pure deterministic lowering layer. It accepts an
`adl_language::AdlDocument`, validates it again, resolves language references,
and returns inert `ExecutionPlan` data. It performs no I/O, scheduling,
provider calls, retries, lifecycle work, or execution.

The landed language contract represents sequential and concurrent workflows
and saved-state dependencies. Legacy top-level `patterns` are not compiler
inputs: `adl-language` deliberately rejects and cannot represent them. Pattern
syntax requires a future typed language contract before compiler support.

Plan ordering uses ordered collections and a lexical Kahn traversal. Node IDs
are SHA-256 values over a versioned, domain-separated, length-delimited semantic
tuple. Source digests use the language crate's canonical bytes. All input-size
limits are explicit through `CompilerLimits`.
