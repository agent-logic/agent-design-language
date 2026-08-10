# Structured Intent Prompt

Template: 1.0.0

Issue: 164

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Establish the production crate, root parser, dispatch, schemas, completion, generated help, and release artifact.

## Required Outcome

parser purity help schema completion and artifact provenance is produced at an exact revision and independently reproducible.

## Scope

- `main`, library `run`, Clap root/subcommands, global flags, output mode selection, typed top-level errors, version provenance, schema export, completion generation, and documentation generation.

## Authority

- Issue V3-03 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
