# V3-11B Design

Issue: #174

## Objective

Execute approved PVF plans with bounded structured concurrency, OS child control, cancellation, and tamper-evident evidence.

## Scope

`validate run/status`, bounded scheduler, process adapter integration, parallel groups, timeouts, root cancellation, child termination/drain, output caps, evidence digests, result projection, and interruption recovery.

## Dependencies

- V3-08: issue #169
- V3-09: issue #170
- V3-11A: issue #173

## Architecture Decisions

- `V3-D09`

## Deliverables

- Scheduler, process registry, cancellation wiring, evidence model, result renderer, interruption fixtures, and representative local journeys.

## Owned Paths

- `csdlc-v3/src/pvf/execute/**`
- `csdlc-v3/src/pvf/evidence/**`
- `csdlc-v3/tests/pvf/execute/**`
- `.csdlc/issues/174/**`
- `.csdlc/prepared/issues/174/**`
- `.csdlc/prepared/issues/174/validate-outcome.rb`
- `.csdlc/evidence/174/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Parallel tasks are bounded and every Tokio task is awaited after cancellation.
2. Every OS child is registered with root cancellation; Unix termination uses bounded `SIGTERM`/kill escalation and Windows uses the reviewed termination primitive, followed by handle wait and output drain.
3. Every sleep and network/process await participates in `tokio::select!` with cancellation.
4. Incomplete, cancelled, timed-out, or tampered evidence cannot appear passed.
5. Each captured stream records `truncated`, `captured_bytes`, and `original_bytes_if_known`; human and JSON output distinguish an enforced cap from naturally short process output.
6. Passing validation cannot authorize review, publication, or merge.

## PVF Lanes

- `v3-11b-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/174/validate-outcome.rb`.
- `v3-11b-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-11b-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Scheduler stress, signal/cancellation and child-process fixtures on each platform, timeout/drain tests, output/redaction tests, evidence tamper tests, interrupted-run recovery, and representative local PVF journeys.

## Authority Boundary

- Issue V3-11B owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Planning-policy invention, embedded product test logic, hidden CI routing, implicit cloud runners, background queues, review, or publication.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Detached work remains, child termination is unproven on a supported platform, live/cloud work becomes implicit, or incomplete evidence can appear passed.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-11b`
