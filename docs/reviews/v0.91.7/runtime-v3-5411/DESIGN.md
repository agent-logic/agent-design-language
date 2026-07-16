# Runtime v3 Selection, Guardian, And Pressure Stop (#5411)

## Decision

Runtime v3 remains independently executable through `adl-runtime-kernel` and
does not acquire a dependency on Runtime v2. The legacy compatibility selector
is a reporting surface, not a launcher; its documentation and release evidence
must say so without implying that a selection starts either runtime.

The existing guardian remains the child-process boundary. It must supervise a
process group on supported Unix platforms, bound output capture and shutdown,
and prove that descendant processes cannot survive guardian termination. No
new service-manager framework is introduced.

## Pressure Stop

The live kernel periodically samples the existing system-weather service. A
`StopRequired` decision closes admission, records a signed continuity
checkpoint through the live coordinator, and then requests graceful kernel
shutdown. Checkpoint failure refuses a clean-stop claim and is surfaced in the
terminal result. Warning and unavailable samples remain observable but do not
silently stop the runtime.

The monitor uses the existing `sysinfo`, cancellation, telemetry, supervisor,
and Ed25519 continuity facilities. It does not add custom polling, signing,
serialization, or retry frameworks.

## Release Evidence

Release proof entries distinguish four evidence classes:

- `executed`: a non-ignored proving path ran successfully;
- `contract_only`: schemas or static contracts exist without live execution;
- `ignored`: a test exists but was deliberately excluded from the run; and
- `deferred`: the surface is intentionally postponed with an explicit reason.

Only executed evidence may satisfy a live cutover requirement. Contract-only,
ignored, and deferred entries remain useful retained evidence but cannot be
counted as completion.

## Scope

- Runtime v3 kernel, guardian, release-proof records, and their focused tests.
- No Runtime v2 implementation changes.
- No default-runtime switch or Runtime v2 decommission.
- No overlap with #5409 protected files:
  `adl-runtime/src/runtime_api_auth.rs`, `adl-runtime/src/supervision.rs`, and
  `adl-runtime/src/topology.rs`.
- Keep Runtime v3 below the 12,000 implementation-line budget where practical;
  remove or consolidate code if a small net increase cannot be justified.

## Validation

Focused tests prove process-tree cleanup, bounded capture shutdown, periodic
pressure-triggered signed checkpoint and graceful stop, checkpoint-failure
truth, and evidence-class gating. The independent Runtime v3 fast lane, strict
Clippy, and deterministic inventory check remain integration proof.
