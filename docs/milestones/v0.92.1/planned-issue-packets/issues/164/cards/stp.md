# Structured Task Prompt

Template: 1.0.0

Issue: 164

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-03 within its exact owned paths and authority boundary.

## Deliverables

- One crate, one binary target, one library target, complete placeholder command graph, versioned output envelope and selected in-process filter/template engines, generated help/docs, completion artifacts, production configuration for the V3-01-pinned `cargo-deny`, and reproducible release metadata.

## Acceptance

1. Every approved command is discoverable from `csdlc --help`.
2. Cargo package `csdlc-v3` builds and installs exactly one binary named `csdlc`; generated docs, completions, provenance, and installer checks bind both immutable identities.
3. Constructor and parser tests invoke no repository, network, or process adapter.
4. Human and JSON output never mix machine payloads with diagnostics.
5. JSON carries the V3-01 schema discriminant; `--jq` and `--template` parse, conflict, and operate only through the V3-01/V3-02 approved in-process path.
6. `--jq` implements exactly the approved subset manifest, has golden compatibility tests for every supported form, and returns a typed usage error for unsupported jq syntax.
7. Every command that supports structured `--input` rejects combining it with any direct field flag at the Clap parser boundary; positive and conflict parser tests are required for each such command.
8. Dependency-policy CI rejects unapproved licenses, advisories, bans, and duplicate dependency families from this issue onward.
9. The release build emits one provenance-bound executable.

## Dependencies

- V3-02: issue #162

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-03
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Repository discovery, lifecycle semantics, GitHub access, state mutation, validation execution, or v2 installation changes.
