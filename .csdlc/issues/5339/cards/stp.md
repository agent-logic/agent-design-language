# Structured Task Prompt

Template: 1.0.0

Issue: 5339

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare and, only after the dependency gate becomes terminal, implement the clean-room adl-language crate; do not absorb compiler, engine, contracts, adapters, CLI, selector, parity, deletion, or Runtime v3 work.

## Deliverables

- Typed Rust models for provider, tool, agent, task, workflow, run, and the versioned root document
- Strict YAML and JSON parsers with stable diagnostic codes and source locations where available
- Versioned JSON Schemas plus positive, negative, unknown-field, duplicate-identity, and broken-reference fixtures
- Pure semantic validator and deterministic canonicalization contract
- Characterization-parity proof against the reviewed #5337 corpus
- Dependency, source-size, test-size, and validation-latency budget report
- COTS decision record for serde, serde_json, YAML parsing, schemars, and schema validation

## Acceptance

1. AC-1: Provider, tool, agent, task, workflow, and run are explicit typed contracts under one versioned document root, with unknown fields rejected
2. AC-2: YAML and JSON inputs produce one equivalent validated model or stable typed diagnostics without network, clock, environment, provider, runtime, or filesystem-mutation authority
3. AC-3: Generated checked schemas and fixtures stay structurally aligned with the Rust types, including negative unknown-field and malformed-value cases
4. AC-4: Semantic validation rejects duplicate identities, invalid versioning, unresolved cross-primitive references, and language-level cycles where the reviewed contract declares them invalid
5. AC-5: Canonicalization is deterministic across map order, YAML versus JSON representation, and repeated runs, and does not invent compiler ExecutionPlan semantics
6. AC-6: Every applicable #5337 language characterization case is mapped and passes, while intentional differences are explicitly classified rather than normalized away
7. AC-7: The crate uses only reviewed COTS serialization/schema components and excludes ADL v1, Runtime v2/v3, C-SDLC, async runtime, HTTP, cloud, database, and provider SDK dependencies
8. AC-8: WP-04 stays within a reviewed provisional allocation of 4000 implementation LoC and 4000 test/fixture LoC, with focused warm validation under 120 seconds and complete deterministic validation under 600 seconds; any variance requires evidence-backed review without weakening proof
9. AC-9: Implementation, review, and publication begin only from a current #5337 merged plus typed closed_out dependency signal

## Dependencies

- #5336 architecture, clean-room provenance, ownership, and budget authority integrated
- #5337 PR #5607 merged
- #5337 typed lifecycle phase closed_out
- Reviewed #5337 characterization corpus and normalization contract available on current main

## Inputs

- AGENTS.md
- docs/milestones/v0.91.8/DESIGN_v0.91.8.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/features/ADL_V2_CORE_v0.91.8.md
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- adl-characterization/corpus/v1/corpus.yaml
- adl-characterization/corpus/v1/schema.json
- adl-characterization/corpus/v1/COVERAGE.md
- adl-characterization/observations/v1/verification.json

## Non Goals

- Compiler resolution, composition expansion, patterns, stable execution-node identity, or ExecutionPlan generation owned by #5338
- Engine scheduling, retry, join, resume, ports, or execution owned by #5340
- Portable records, signing, trust, and verification owned by #5342
- Runtime v3, provider/tool adapters, CLI, selector, parity, cutover, deletion, deployment, or AWS work
- Compatibility aliases or broad incumbent behavior not present in the reviewed #5337 contract
