# Structured Task Prompt

Template: 1.0.0

Issue: 73

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Complete and review the Rust C-SDLC v3 architecture and issue plan; stop before implementation or child-issue creation.

## Deliverables

- .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md
- .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd
- .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_ARCHITECTURE.md as comparative source material
- A retained Claude and Gemini review record with findings and dispositions
- Fourteen complete implementation issue specifications and one deferred retirement specification

## Acceptance

1. AC-1: The plan fixes Rust as the implementation language and records the exact official cli/cli source baseline.
2. AC-2: The architecture completely defines command, application-context, domain, state, card, adapter, validation, review, publication, finish, cleanup, security, testing, observability, and dependency boundaries.
3. AC-3: The plan contains eighteen sequenced implementation issue specifications and one deferred retirement issue, each with objective, scope, non-goals, dependencies, deliverables, acceptance criteria, validation proof, and stop conditions.
4. AC-4: Quantified effects are labeled as targets or estimates until measured by the construction spike.
5. AC-5: Claude and Gemini review the same exact plan revision and every actionable finding is incorporated or explicitly dispositioned.
6. AC-6: Formatting, links, source references, repository-relative paths, and diff hygiene validate.
7. AC-7: No v3 code, implementation child issue, selector change, live migration, authority cutover, or v2 deletion is produced by this planning issue.

## Dependencies

- Official cli/cli checkout and exact analyzed revision
- Current C-SDLC v2 source and operator-skill inventory
- Operator decision that C-SDLC v3 will be implemented in Rust

## Inputs

- .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md
- .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_ARCHITECTURE.md
- csdlc-v2/Cargo.toml
- csdlc-v2/src
- csdlc-v2/operator/skills
- AGENTS.md
- official cli/cli source at the pinned revision

## Non Goals

- Implementing C-SDLC v3 code
- Creating implementation child issues
- Changing the v2 generation selector or installed binaries
- Migrating or dual-writing live records
- Publishing, merging, cutting over, or deleting v2
- Adding ADL runtime or product behavior to C-SDLC
