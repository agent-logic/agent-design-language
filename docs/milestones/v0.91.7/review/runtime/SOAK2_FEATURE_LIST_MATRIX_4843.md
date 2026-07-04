# Soak 2 Feature-List Matrix (#4843)

Issue: #4843
Consumer: #4682
Umbrella: #4634
Version: v0.91.7

## Purpose

This packet turns the v0.91.7 Runtime Soak #2 planning table into an executable
row-level matrix for the standard Soak #2 run. It is a pre-run matrix, not a
Soak #2 result packet.

The structured source is:

- `docs/milestones/v0.91.7/review/runtime/soak2_feature_list_matrix_4843.json`

The validator is:

- `bash adl/tools/validate_v0917_soak2_matrix.sh`

## Consumption Rule

#4682 should consume the JSON matrix without re-planning scope. For every row it
must either run the named command or retain an evidence-backed blocker/non-claim
that names the owner issue and v0.92 impact. A row may exit Soak #2 only as
`integrated_proven`, `blocked`, `deferred`, or `routed_to_soak_3`.

Matrix existence is not runtime proof. Rows marked `pending_upstream_pr` or
`blocked_before_soak2` are intentionally not complete.

## Required Local Setup

- Run from the repository root or the bound issue worktree.
- Use repo-native ADL commands and Cargo binaries.
- Keep generated Soak #2 artifacts under
  `docs/milestones/v0.91.7/review/runtime/soak2_4682/`.
- Use the approved Agent Logic AWS profile `agent-logic-admin` for AWS-backed
  rows, and record account-resolution proof without printing credentials.
- Treat provider credentials, AWS access, and Unity editor availability as live
  operator-bound prerequisites. Missing live access becomes a blocker or
  operator-approved non-claim, not a pass.

## Current Pre-Soak Classifications

| Row | State | Owner issues | Soak #2 handling |
| --- | --- | --- | --- |
| Tokio runtime substrate | `pending_upstream_pr` | #4681, #4842, #4682 | Consume #4681 once PR #4868 lands, then run through `adl-runtime`. |
| Agent lifecycle | `pending_upstream_pr` | #4681, #4682 | Use inherited soak runner only as prerequisite until the v0.91.7 path runs. |
| AEE path | `ready_for_soak2` | #4546, #4697, #4682 | Run ACIP/AEE/memory fixture, then attach it to assembled runtime evidence. |
| ACIP/A2A path | `blocked_before_soak2` | #4546, #4658, #4659, #4660, #4682 | Requires WP-12 protocol/access-rule disposition. |
| Provider/model substrate | `ready_for_soak2` | #4672, #4673, #4674, #4675, #4817, #4682 | Consume WP-05 provider/scheduler proof and classify live-provider access truth. |
| Scheduler | `ready_for_soak2` | #4671, #4674, #4682 | Require scheduler decisions in retained runtime packets, not only component docs. |
| Resilience | `pending_upstream_pr` | #4783, #4784, #4682 | Consume #4783/#4784 or block/reroute with operator approval. |
| Logging and observability | `ready_for_soak2` | #4718, #4682 | Reuse merged #4718 proof and rerun the focused verifier during #4682. |
| Runtime AWS and signal bridge | `blocked_before_soak2` | #4635, #4684, #4685, #4686, #4687, #4688, #4682 | Requires WP-08 proof or evidence-backed non-claim. |
| Observatory / Unity live consumption | `ready_for_soak2` | #4652, #4702, #4703, #4704, #4689, #4682 | Run Unity soak integration where editor entitlement is available. |
| ObsMem and memory handoff | `ready_for_soak2` | #4546, #4697, #4682 | Require retained handoff evidence in the assembled runtime path. |
| Identity and continuity | `pending_upstream_pr` | #4681, #4842, #4682 | Tie identity snapshots to start/wake/stop packets after #4681 lands. |
| Capability envelope | `blocked_before_soak2` | #4656, #4660, #4696, #4682 | Requires runtime authority limits or operator-approved non-claim. |
| Security / CAV boundary | `blocked_before_soak2` | #4656, #4657, #4658, #4659, #4660, #4682 | Requires integrated fail-closed behavior or blocks activation. |
| Curiosity / Constructability, if promoted before run | `optional_non_claim` | #4692, #4693, #4682 | Non-claim unless explicitly promoted before #4682. |

## Validation

For #4843, validation is matrix and fixture-discovery proof only:

```bash
bash adl/tools/validate_v0917_soak2_matrix.sh
bash -n adl/tools/validate_v0917_soak2_matrix.sh
git diff --check
```

The full Soak #2 run remains #4682.
