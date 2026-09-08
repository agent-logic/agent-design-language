# Structured Task Prompt

Template: 1.0.0

Issue: 707

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair deterministic cross-binary generation identity, prove it with focused tests, install a coherent generation, and demonstrate live Beacon-to-Ember delivery.

## Deliverables

- Canonical dependency-independent receipt digest
- Cross-binary generation regression test
- Verified three-binary generation receipt
- Healthy canonical Wuji rollout
- Live distinct A2A delivery evidence

## Acceptance

1. AC-1: CSM, Guardian, and Kernel derive identical configuration generation and receipt identity for the same init and binary generation
2. AC-2: Generation validation remains fail closed for content, compatibility, receipt, and executable mismatch
3. AC-3: One release generation containing all three current-source binaries verifies and starts under launchd without restart exhaustion
4. AC-4: Wuji readiness, ownership identities, observability, roster eligibility, and model residency remain healthy
5. AC-5: Beacon emits a distinct agent-to-agent work item addressed to Ember and Ember receives it; operator reply alone is not accepted as proof
6. AC-6: Focused validation, diff hygiene, bounded subagent review, and typed publication pass

## Dependencies

- #692 merged configuration-generation guard
- #693 and PR #696 merged A2A implementation
- Healthy rollback generation main-cea5219f6-20260903

## Inputs

- agent-logic/agent-design-language#707
- agent-logic/agent-design-language#692
- agent-logic/agent-design-language#693
- adl-runtime-kernel/src/config_generation.rs
- adl/tools/install_runtime_v3_generation.sh
- docs/tooling/START_CSM_RUNBOOK.md

## Non Goals

- No Runtime lifecycle redesign
- No init or agent configuration changes
- No provider or model changes
- No Observatory visual redesign
- No bypass of configuration-generation validation
