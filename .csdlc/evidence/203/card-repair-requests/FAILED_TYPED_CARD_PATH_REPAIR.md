# Issue 203 typed card path repair blocker

## Intended bounded repair

Correct stale typed card references from:

- `adl-runtime/src/distributed/polis_runtime.rs`

to:

- `adl-runtime/src/distributed/transport/governed/polis_runtime.rs`

The source/evidence proof already binds the governed path; the stale reference
is in issue-card truth fields.

## Preserved request

- `.csdlc/evidence/203/card-repair-requests/203-path-repair-sip-required-outcome.json`

## Command

```text
cargo run --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-edit -- --repo /Volumes/FastWork/adl-worktrees/adl-issue-203-authority-serving-adapters apply --request .csdlc/evidence/203/card-repair-requests/203-path-repair-sip-required-outcome.json
```

## Output

```text
{"schema":"csdlc.error.v1","code":"invalid_transition","message":"sip mutation is not allowed during bound"}
csdlc-edit: sip mutation is not allowed during bound
```

## Disposition

Do not hand-edit or bypass the bound-phase card guard. The minimal typed recovery
route needs a v2 card-truth repair operation that permits narrow path/name
correction during bound/implemented phases without changing issue scope, status,
or acceptance semantics.
