# WP-07A CSM Runtime Rearchitecture Sprint Review

Issue: #5121
Review issue: #5403
Status: blocked with findings
Remediation: #5409

## Retained Boundary

### The closed umbrella does not prove sprint implementation completion

#5121 closed as a documentation/topology PR before every implementation child.
The canonical register still classifies it as setup/topology evidence only and
forbids treating it as implementation completion at
`docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md:100`. The source
architecture packet retains the full Tokio service graph as incomplete at
`docs/milestones/v0.91.7/review/runtime/csm_runtime_rearchitecture_5068.md:233`.

This is a correctly retained non-claim, not a new defect. Preserve #5121 as
setup/topology closure and add a truthful implementation-completion decision
only after the findings below pass.

## Findings

### P1: Final child #5120 closed without running its assembled-runtime soak

The retained gate at
`docs/milestones/v0.91.7/review/runtime/csm_assembled_topology_soak_5120_gate.md:7`
remains `HELD`. Lines 49 and 70 say the soak must not start while #5123 is
pending and explicitly say it has not run. PR #5265 closed #5120 with a
safe-fail repair before #5123 merged, and no later tracked soak supersedes the
held packet.

Impact: the sprint's final serial acceptance gate is unmet.

Disposition: open. Run and retain the assembled topology soak after every
required component is integrated, or record an operator-approved blocked
release disposition.

### P1: The advertised supervised topology is not the production topology

`adl-runtime/src/topology.rs:106` reports `main_task_join_set` and
`supervised_component_set`. The supervisor at
`adl-runtime/src/supervision.rs:697` is called only from unit tests beginning
around line 1124. Production directly constructs Chronosense, Vector, and the
channel fabric at `adl/src/long_lived_agent.rs:3778-3804`; reasoning health is an
unconditional static `ready` snapshot around line 4320.

Impact: status claims a supervised component set that production does not
actually execute, so restart, failure propagation, and health semantics are
not those described by the architecture.

Disposition: open. Route a WP-07A runtime issue that either wires production
through the supervisor or corrects the topology and proof claims to the actual
execution model.

### P1: `/ready` can be green without required component health

The status response at `adl/src/csm_runtime_api.rs:470` omits Vector component
health and provides only a cloud-bridge summary. Its blocker list around line
1517 omits observability, scheduler, AEE, cloud bridge, and lifelog. Typed
channel readiness explicitly exempts observability and cloud publication
channels at `adl/src/long_lived_agent.rs:4046`.

Impact: failed required components can leave `/ready` green, contradicting the
architecture and #5120 proof contract.

Disposition: open. Define the required component set once and make readiness
consume observed health for every required service and channel.

### P2: Runtime API credentials have a deterministic 24-hour availability cliff

Credentials expire after 24 hours at
`adl-runtime/src/runtime_api_auth.rs:19`. `ensure()` reuses an existing
credential without checking expiry around line 103, while authorization rejects
expired material around line 128 and the readiness client does the same at
`adl/src/cli/csm_service_cmd.rs:1172`. Rotation is manual.

Impact: a long-running CSM API becomes inaccessible after 24 hours unless an
operator intervenes. Security fails closed, but service availability does not
meet long-lived runtime expectations.

Disposition: open. Add bounded automatic renewal or proactive rotation with
overlap, audit, and expiry-boundary tests.

## Child Coverage

Reviewed source architecture #5068 and children #5110, #5111, #5116, #5117,
#5112, #5113, #5118, #5124, #5125, #5122, #5123, #5119, #5126, #5115, #5114,
and #5120. All 18 scoped issues are closed and their associated PRs merged.

## Validation And Limits

- `adl-runtime` passed 114 unit tests plus its crate-independence test.
- The integrated CSM API slice passed 40 tests.
- All four findings above are review-discovered; no test-discovered bug is
  counted among them.
- Dependency manifests and both lockfile resolutions were reviewed; Vector is
  version/checksum pinned.
- Advisory scanning was unavailable because `cargo-audit` and `cargo-deny` are
  not installed and repository Dependabot alerts are disabled.
- Child PR descriptions reference ignored local SRP/SOR records; none of the
  scoped PRs retains a formal GitHub review.

## Review Result

Blocked with findings. Substantial implementation exists, but completion is not
proven until the final soak runs, production supervision matches the claimed
topology, and readiness reflects the full required component set.
