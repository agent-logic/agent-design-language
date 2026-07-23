# Structured Intent Prompt

Template: 1.0.0

Issue: 5339

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement a small, side-effect-free ADL v2 language crate defining provider, tool, agent, task, workflow, and run documents with strict parsing, schemas, semantic validation, and deterministic canonicalization.

## Required Outcome

The independent adl-language crate accepts the reviewed #5337 characterization corpus, rejects unknown or invalid structure and references with stable diagnostics, emits deterministic canonical representations and schemas, and introduces no runtime, control-plane, network, clock, filesystem-mutation, or cloud authority.

## Scope

- adl-v2/crates/adl-language typed six-primitives source model
- strict YAML and JSON parsing with fail-closed unknown-field handling
- versioned JSON Schema generation and checked schema fixtures
- semantic identity and reference validation within the language boundary
- deterministic canonicalization and stable machine-readable diagnostics
- issue-local lifecycle, design, validation, review, and evidence records

## Authority

- Issue #5339 owns only the adl-language crate and issue-local C-SDLC records
- #5336 architecture and budget authority plus #5337 reviewed corpus are read-only inputs
- The language crate is pure and owns no compiler DAG expansion, engine behavior, Runtime v3 services, C-SDLC lifecycle, provider transport, governed-tool execution, or cloud access
- Incumbent ADL implementation, schemas, tests, and fixtures are behavioral evidence only and must not be copied, adapted, imported, or linked into the clean-room crate
- Implementation begins only after #5337 is merged and its typed terminal phase is closed_out

## Assumptions

- none

## Operator Constraints

- Use installed typed C-SDLC v2 binaries and semantic card edits only
- Never edit tracked issue work on root main
- Do not use raw gh, AWS, credentials, or network providers
- Preparation may finish while #5337 publishes, but no product implementation may begin before its merged and typed closed_out signal
- Use /Volumes/FastWork for Rust build output when implementation starts
- Run bounded subagent design and exact-revision code reviews before publication
