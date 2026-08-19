# V3-03 Design

Issue: #164

## Objective

Establish the production crate, root parser, dispatch, schemas, completion, generated help, and release artifact.

## Scope

`main`, library `run`, Clap root/subcommands, global flags, output mode selection, typed top-level errors, version provenance, schema export, completion generation, and documentation generation.

## Dependencies

- V3-02: issue #162

## Architecture Decisions

- `V3-D03`
- `V3-D06`

## Deliverables

- One crate, one binary target, one library target, complete placeholder command graph, versioned output envelope and selected in-process filter/template engines, generated help/docs, completion artifacts, production configuration for the V3-01-pinned `cargo-deny`, and reproducible release metadata.

## Owned Paths

- `csdlc-v3/Cargo.toml`
- `csdlc-v3/Cargo.lock`
- `csdlc-v3/src/main.rs`
- `csdlc-v3/src/lib.rs`
- `csdlc-v3/src/cli/**`
- `csdlc-v3/src/output/**`
- `csdlc-v3/tests/cli/**`
- `.csdlc/issues/164/**`
- `.csdlc/prepared/issues/164/**`
- `.csdlc/prepared/issues/164/validate-outcome.rb`
- `.csdlc/evidence/164/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every approved command is discoverable from `csdlc --help`.
2. Cargo package `csdlc-v3` builds and installs exactly one binary named `csdlc`; generated docs, completions, provenance, and installer checks bind both immutable identities.
3. Constructor and parser tests invoke no repository, network, or process adapter.
4. Human and JSON output never mix machine payloads with diagnostics.
5. JSON carries the V3-01 schema discriminant; `--jq` and `--template` parse, conflict, and operate only through the V3-01/V3-02 approved in-process path.
6. `--jq` implements exactly the approved subset manifest, has golden compatibility tests for every supported form, and returns a typed usage error for unsupported jq syntax.
7. Every command that supports structured `--input` rejects combining it with any direct field flag at the Clap parser boundary; positive and conflict parser tests are required for each such command.
8. Dependency-policy CI rejects unapproved licenses, advisories, bans, and duplicate dependency families from this issue onward.
9. The release build emits one provenance-bound executable.

## PVF Lanes

- `v3-03-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/164/validate-outcome.rb`.
- `v3-03-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-03-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Parser golden tests, help/docs drift check, schema smoke tests, completion tests, stdout/stderr tests, reproducible-build check, and cross-platform compile matrix.

## Authority Boundary

- Issue V3-03 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Repository discovery, lifecycle semantics, GitHub access, state mutation, validation execution, or v2 installation changes.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A command requires hidden global state, generated docs diverge from Clap, or more than one operational binary becomes necessary.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-03`
