# V3-02 Design

Issue: #162

## Objective

Validate the Rust architecture and quantitative targets before the main build wave.

## Scope

One throwaway or explicitly promoted vertical slice containing `version`, `schema`, repository discovery, read-only `issue show`, fake GitHub observation, one card field authored before review and corrected after typed review recovery, human/JSON output, parser tests, and run-function tests.

## Dependencies

- V3-01: issue #161

## Architecture Decisions

- `V3-D02`

## Deliverables

- Spike source, dependency inventory, preliminary report from the exact `cargo-deny` release pinned by V3-01, build/startup/test measurements, implementation-size report, trait/object-safety decision, a reviewed decision to pin one maintained YAML parser or remove YAML input entirely, in-process jq-compatible engine decision and supported-subset conformance manifest, restricted-template engine decision, Octocrab capability-gap inventory for every required GitHub operation, per-platform commit-primitive prototype and Decision 11 recommendation, and promote-or-discard disposition. The disposition is a real stop/go decision and must state whether the capability-matrix approach prevented a stranded post-review correction path. The governing stop conditions are the ten exact threshold rows under `Expected Effect And Measurement`: binary size, direct and transitive dependency counts, clean and warm build time, startup p95, local `issue show` p95, local `doctor` p95, test-suite duration, and authored slice lines. The spike report reproduces each threshold beside its observation. The recommendation does not issue Decision 11: V3-08 remains blocked until a separate retained operator decision record explicitly approves the measured per-platform commit matrix.

## Owned Paths

- `csdlc-v3/spike/**`
- `csdlc-v3/evidence/construction-spike/**`
- `.csdlc/issues/162/**`
- `.csdlc/prepared/issues/162/**`
- `.csdlc/prepared/issues/162/validate-outcome.rb`
- `.csdlc/evidence/162/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. The slice uses one binary and one library with the proposed four layers.
2. Parsing initializes no repository, credentials, network, or child task.
3. Fake adapters reject unexpected operations and support deterministic tests.
4. Every required GitHub operation is classified as native typed Octocrab, reviewed raw request, or unsupported. More than three required raw-request operations trigger GitHub client dependency re-evaluation before V3-13.
5. The slice completes one end-to-end recovery journey: exact review, typed recovery, capability-derived field correction, projection regeneration, audit readback, and fresh exact review, with no direct state or Markdown edit.
6. Measurements either satisfy approved thresholds or trigger architecture revision before `V3-03`; a missing measurement or any threshold miss is a binding stop, not a discretionary finding.
7. The spike identifies the exact Decision 11 record required next and proves that its recommendation alone cannot satisfy the V3-08 dependency gate.

## PVF Lanes

- `v3-02-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/162/validate-outcome.rb`.
- `v3-02-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-02-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Clean and warm builds, binary inspection, startup timing, offline tests, retained recovered-correction transcript and negative bypass test, dependency policy scan, layer-boundary check, and review of all unsafe/default-feature use.

## Authority Boundary

- Issue V3-02 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Production mutation, live issue writes, complete lifecycle logic, language selection, or undeclared reuse of v2 entry points.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- The slice requires ADL product crates, cannot isolate the domain from async/adapters, exceeds approved thresholds without disposition, mutates real C-SDLC state, or cannot complete the recovered-correction journey from the frozen capability matrix. Any stop condition blocks V3-03 rather than being accepted as construction-spike paperwork.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-02`
