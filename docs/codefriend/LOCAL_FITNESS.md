# Local architecture fitness policies

Issue #887 supplies a deterministic local predicate over admitted CF-EVIDENCE.
A versioned policy names each required Rust analysis file and a forbidden literal
`use` prefix. The runner reads the admitted snapshot; it does not compile or execute
repository code, expand macros, call a provider, or infer architecture quality.

The example policy and pass/fail/error fixtures are under
`adl/tests/fixtures/codefriend/fitness/`. Policy rules are explicit input data.
`source_path` is an exact relative path required in the admission's analysis scope;
context-only or absent files cannot satisfy a rule. Unknown policy fields, unsupported
versions/rule kinds, duplicate IDs and relative forbidden prefixes are rejected.

```sh
adl codefriend fitness run --store /absolute/evidence-store \
  --packet-id PACKET_ID --policy policy.json --out new-report.json
adl codefriend fitness read --store /absolute/evidence-store --input new-report.json
```

Use an existing live evidence store populated by `adl codefriend evidence admit-local`
or `admit-packet`. Put artifacts outside the source checkout and managed store.
Outputs are create-only regular files; existing files, symlink paths, parent traversal,
and output inside the evidence store are rejected. The runner's only requested
write is its result artifact (the existing evidence store also uses its ordinary lock).

## Predicate and limitations

`forbidden_declared_use` compares identifier segments in literal Rust `use`
declarations, including grouped imports, renames, raw identifier spelling and globs
under the forbidden prefix. It reports exact admitted evidence IDs and line locations.
It does not resolve semantic aliases, infer transitive dependencies or inspect runtime
calls. Conditional imports are checked as declared regardless of active build features.
A prefix matches complete segments: `crate::bad` does not match `crate::badger`.

Relative `self`/`super` imports, globs that could contain the forbidden prefix,
macro invocations, parse failures, incomplete admissions and missing required evidence
produce error. The shared parser admits at most 400 KiB and 32,768 parsed token-tree entries
per required file, with delimiter/nested-comment depth 32 and cumulative ancestor
statement span 2,048. Comments and literals are opaque to the lexical delimiter
check. [Rust parser bounds](RUST_PARSER_BOUNDS.md) defines the complete admission,
worker and concurrency contract. Over-budget files produce error. A pass proves only the declared literal-import predicate on all
required files within these limits. Human architecture quality, runtime effects and
macro expansion remain explicitly unassessed even on pass.

## Exit and artifact contract for CF-GOV-CI

| Exit | JSON status | Meaning |
| --- | --- | --- |
| 0 | `pass` | All declared predicates executed without violations or errors. |
| 1 | `fail` | At least one located violation; all required evidence was evaluable. |
| 2 | `error` | The predicate could not be fully evaluated, or the command/artifact failed. |

A valid report retains the full explicit policy, policy digest, admitted evidence,
shared run/findings, locations, errors, unassessed judgments and deterministic digest.
Errors take precedence over violations; observed violations are still retained.
Policy identity is part of run identity. Repeating the same policy over the same
live admission yields an identical report. `read` recomputes the report from live
admission and policy; tampering, expiration and deletion fail closed.

For invalid command/policy/store or an artifact I/O failure, stdout contains an
error envelope with `report_available: false`, and exit is 2. A partially written
artifact is not acceptance evidence. Consumers must check process exit, parse JSON,
and use live `read` before accepting a saved result. The consumer must also pin
the expected policy digest, packet identity and revision from its own declared
configuration. Readback checks consistency with the embedded policy; it does not
authorize a different policy or evidence scope. Missing artifacts,
unknown schemas, error envelopes or read failures cannot become pass. This issue
exercises that contract locally; actual CI integration belongs to #888.

Stdout is JSON. Stderr carries only a content-free `adl_event` with the exit code;
raw errors, source content and host paths are not logged. Policy fields cannot grant
script execution or mutation authority. Evidence redaction and retention stay owned
by CF-EVIDENCE. Local proof and hosted CI are separate evidence surfaces.
