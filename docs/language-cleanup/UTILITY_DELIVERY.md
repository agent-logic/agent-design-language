# Remaining utility migration (#1351)

Status: **in progress; not accepted or ready for publication**. Native issue
preparation and FastWork binding succeeded. The issue remains responsible for all
60 baseline Python paths, the later #1275 adapter checker, and executable inline
payloads. No exception or completion-by-omission is accepted.

## Completed source slice: adapter report consistency

`tools/public_contracts/src/adapter_evidence.rs` replaces the #1275 Python report
checker. Run:

```sh
cargo run --locked --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts -- adapter-evidence
cargo test --locked --manifest-path tools/public_contracts/Cargo.toml --test adapter_evidence
```

This is offline report consistency, not authenticated lifecycle or product
acceptance. The original checker passed before removal. Native tests cover the
current report, truthful incomplete reporting, mismatched identity/package/stage
claims, false aggregate success, missing evidence and duplicate repositories.
Missing identity fields cannot accidentally compare equal. Package-binding
ambiguity is rejected rather than silently overwritten.

`utility-dispositions.json` retains exact Git mode/blob/SHA256 custody for each
completed removal. No source payload is archived or embedded. Original #1275
reports remain unchanged. Remaining utilities, callers, final guard overlays,
independent review and CI are still required before this issue is delivered.

## Completed source slice: dependency cache warming

The Rust cache helper replaces the implementation and five Python regression
cases. The original tests passed before removal. Ten native tests preserve
artifact selection, multiline workspace manifests, dep-info/build exclusions,
explicit replacement, rollback/restoration, wrapper defaults, JSON/output-file
behavior and the optional cross-device skip. Direct CLI tests run with empty
`PATH` and verify successful JSON, runtime failure exit 2 and setup failure exit 1.
The cross-device case ran on this host using the available second filesystem.

The same-filesystem wrapper and direct dry-run interface remain available at
`adl/tools/rust_validation_warm_cache.sh` and
`adl/tools/warm_rust_dependency_cache.sh`. They invoke Rust through Cargo; the
first use may compile the helper. Warmup remains acceleration, never proof.
Current lane selectors and documentation point to the native implementation.

Safety corrections are explicit: source artifact/profile symlinks and destination
parent escapes are refused, wildcard workspace membership requires explicit
classification, and a failed rollback retains its original backup instead of
silently deleting it. Full TOML parsing replaces the old Python-version fallback.

## Completed source slice: planning draft templates

Native `planning-fill` and `planning-validate` commands replace both Python
helpers, with shell entrypoints preserving their explicit registry/template/
values/input/output options. All 11 active templates produced byte-identical
original/native drafts and passed both structural validators from outside the
repository before source removal. Six native test groups cover scalar/list/object
rendering, sorted missing keys, non-overwrite on unresolved values, explicit
partial drafts, required headings, malformed/inactive/version-mismatched
registries, missing inputs, absolute paths and symlink escapes. CLI tests run
with empty `PATH`. Draft generation and structural success retain their explicit
non-approval/non-lifecycle claim boundaries. Current documentation and registry
callers are switched; historical review assignments remain documentary evidence.

## Completed source slice: supported architecture and showcase demos

Both supported v0.90 demos retain their generators and packet contents, with
native validators behind new shell entrypoints. The architecture packet retains
its structural `proving` label; the showcase remains `non_proving`, with
publication disabled and the original staged quality-evaluator boundary. These
labels do not confer private owner or release acceptance. Current commands and
generated validation instructions now name Rust/shell tooling.

Both original demo suites passed before migration. Four native test groups run
the actual demos with failing Python/Ruby traps and verify packet existence,
manifest identities, ordered sections/evidence, publication and staged-lane
truth, leak refusal, existing-output preservation and dangling-symlink refusal.
No trap was invoked. The complete original required-file/diagram/section lists
remain native constants. Safety checks now reject ambiguous duplicate skill
lanes and single-backslash Windows host paths; leak diagnostics identify the
category without echoing potentially sensitive matched content.

## Completed source slice: prompt schema artifact smoke checks

The native `prompt-schemas` command and shell entrypoint replace the Python smoke
checker and the inline active-registry assertions in the retained validation
wrapper. Both wrappers retain their sunset-source absence guards and explicit
non-lifecycle/non-equivalence claims. The native default checks preserve schema
identity, version, card kind, template path, parser provenance, array shapes and
nonempty headings. `--active-registry` also requires active status, matching
registry versions, exactly six lifecycle cards and existing template/schema files.

The original checker passed all six versions (1.0.0–1.0.5) before removal.
Three native test groups cover those versions, malformed/missing schemas,
identity/shape mutations, invalid options, active-registry mutations and CLI
execution from outside the checkout with empty `PATH`. An independent Node JSON
parser read all 33 tracked schemas successfully; Python readability is no longer
claimed as an executed native check. Current instructions use the JSON artifact
smoke check. Schema artifacts and historical review assignments remain unchanged.
