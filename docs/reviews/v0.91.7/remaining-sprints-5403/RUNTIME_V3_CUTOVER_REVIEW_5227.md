# Runtime v3 Cutover Sprint Review

Issue: #5227
Review issue: #5403
Status: changes required; no default cutover
Remediation: #5411; shared parity issue #5413; shared records issue #5406

## Findings

### P1: The entrypoint switch does not launch or select either runtime

`adl/src/cli/runtime_v3_cmd.rs:93` calculates and prints a report.
`runtime_v3_kernel_command` at line 126 is reported text, not an executed
command. `docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md:9` confirms the
command neither launches a daemon nor changes defaults.

Impact: describing #5219 as passed explicit Runtime v3 selection overstates a
diagnostic selector report as an operational entrypoint.

Disposition: open. Implement a real reviewed invocation/selection path or
rename the surface and release claim to configuration reporting only.

### P1: Cutover eligibility relies on Runtime v3-only proofs mislabeled equivalence

`docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json:19`
classifies nine capability groups as `live_equivalent_fixture` using direct
Runtime v3 tests rather than v2/v3 comparisons. Only the reasoning-loop row at
line 51 cites a live comparison. Tests around
`adl-runtime-kernel/tests/parity.rs:977` validate the JSON labels and counts,
while the shadow report still records missing v2 adapters at
`docs/architecture/runtime_v3_shadow_parity_report.v1.json:8`.

Impact: the `live black-box parity passed` claim at
`docs/architecture/RUNTIME_V3_RELEASE_PROOF_GATE_5220.md:64` is unsupported.

Disposition: same root finding as #5276. Downgrade the nine groups or retain
real cross-runtime comparisons before claiming cutover eligibility.

### P1: The fallback guardian supervises only the direct PID

Child creation at `adl-runtime/src/guardian.rs:309` establishes no process
group. Shutdown around line 438 sends `SIGTERM` only to the direct child, while
capture around line 360 waits indefinitely for EOF. A descendant retaining
stdout or stderr can survive and prevent guardian completion. Tests cover only
a direct shell child.

Impact: the guardian can leak descendants and hang while collecting output,
undermining the cross-platform keep-alive and graceful-shutdown boundary.

Disposition: open. Supervise a process tree or explicit child group and bound
capture shutdown, with descendant-retention regression tests.

### P1: Resource pressure neither serializes runtime state nor stops the runtime

`adl-runtime/src/weather.rs:306` serializes only a disposable in-memory probe.
The kernel samples weather once during startup at
`adl-runtime-kernel/src/bin/adl-runtime-kernel.rs:110`; there is no monitoring
loop, real checkpoint invocation, or shutdown action.

Impact: the release packet's graceful pressure-stop behavior is not implemented
on the executable Runtime v3 path.

Disposition: open. Wire bounded periodic weather observation to real continuity
serialization and graceful guardian/kernel shutdown.

### P2: Release completion consumes deferred and contract-only evidence

`docs/architecture/RUNTIME_V3_RELEASE_PROOF_GATE_5220.md:51` calls API health
and observability passed by contract and weather passed with GPU deferral. Its
validation account includes ignored native guardian/live-parity surfaces and no
tracked SRP/SOR bundle.

Impact: the conservative no-default-cutover conclusion is reproducible, but the
broader `release proof complete` claim is not fully supported by retained
executing evidence.

Disposition: narrow the completion claim and route lifecycle retention through
the shared records remediation.

## Child Coverage

Reviewed source gate #5218 and children #5225, #5219, #5222, and #5220. Every
issue is closed and its PR merged.

## Release Truth

Runtime v3 remains explicit opt-in. Runtime v2 remains the default and rollback
target. This review does not authorize Runtime v2 deletion or decommission.

## Validation And Limits

Default Runtime v3 and `adl-runtime` suites passed as recorded in the #5174
packet. All five findings above are review-discovered; no test-discovered
defect is counted above. No dependency defect was confirmed;
advisory-database coverage was unavailable.

## Review Result

Changes required. The sprint produced useful proof and a conservative no-go
decision, but it did not implement an operational selector, complete guardian
process-tree supervision, or real weather-triggered serialization and shutdown.
