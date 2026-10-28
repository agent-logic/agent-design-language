# Structured Task Prompt

Template: 1.0.0

Issue: 162

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-02 within its exact owned paths and authority boundary.

## Deliverables

- Spike source, dependency inventory, preliminary report from the exact `cargo-deny` release pinned by V3-01, build/startup/test measurements, implementation-size report, trait/object-safety decision, a reviewed decision to pin one maintained YAML parser or remove YAML input entirely, in-process jq-compatible engine decision and supported-subset conformance manifest, restricted-template engine decision, Octocrab capability-gap inventory for every required GitHub operation, per-platform commit-primitive prototype and Decision 11 recommendation, and promote-or-discard disposition. The disposition is a real stop/go decision and must state whether the capability-matrix approach prevented a stranded post-review correction path. The governing stop conditions are the ten exact threshold rows under `Expected Effect And Measurement`: binary size, direct and transitive dependency counts, clean and warm build time, startup p95, local `issue show` p95, local `doctor` p95, test-suite duration, and authored slice lines. The spike report reproduces each threshold beside its observation. The recommendation does not issue Decision 11: V3-08 remains blocked until a separate retained operator decision record explicitly approves the measured per-platform commit matrix.

## Acceptance

1. The slice uses one binary and one library with the proposed four layers.
2. Parsing initializes no repository, credentials, network, or child task.
3. Fake adapters reject unexpected operations and support deterministic tests.
4. Every required GitHub operation is classified as native typed Octocrab, reviewed raw request, or unsupported. More than three required raw-request operations trigger GitHub client dependency re-evaluation before V3-13.
5. The slice completes one end-to-end recovery journey: exact review, typed recovery, capability-derived field correction, projection regeneration, audit readback, and fresh exact review, with no direct state or Markdown edit.
6. Measurements either satisfy approved thresholds or trigger architecture revision before `V3-03`; a missing measurement or any threshold miss is a binding stop, not a discretionary finding.
7. The spike identifies the exact Decision 11 record required next and proves that its recommendation alone cannot satisfy the V3-08 dependency gate.

## Dependencies

- V3-01: issue #161

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-02
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Production mutation, live issue writes, complete lifecycle logic, language selection, or undeclared reuse of v2 entry points.
