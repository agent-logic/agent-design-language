# Issue #689 design: canonical Runtime v3 service control

## Decision

The Rust `csm runtime-v3` implementation is the sole permanent Runtime v3
service-control authority. Operators invoke the stable current-generation
binary at `.adl/runtime-v3/current/bin/csm` with the stable init file at
`.adl/runtime-v3/live/runtime-init.toml`. On macOS its default launchd label is
`com.agentlogic.adl-runtime-v3`.

The historical root `CSMctl` shell remains only as the local static Observatory
controller. Its Runtime lifecycle verbs (`start`, `up`, `restart`, `status`,
`stop`, `logs`, `urls`, `rotate-continuity-state`, top-level `open`, and an
invocation with no arguments) fail immediately with a concise migration
message and do not probe, signal, bootstrap, or replace any service. This is
deliberately a refusal rather than delegation: it avoids
binary-resolution recursion, hides no mutation, and gives operators the exact
canonical replacement.

## Canonical operator flow

Read-only diagnosis:

```sh
.adl/runtime-v3/current/bin/csm runtime-v3 status \
  --init "$PWD/.adl/runtime-v3/live/runtime-init.toml" \
  --json
```

The result is authoritative only when `service_loaded` and `listener_ready` are
true, `guardian_process_id` matches the loaded launchd job, the Runtime PID is
present, `active_init_hash` matches the selected init, and
`observability_ready` is true.

Lifecycle mutations use the same binary and init:

```text
csm runtime-v3 start  --init <absolute-init> [--plist <absolute-plist>] [--label <label>] --json
csm runtime-v3 stop   --init <absolute-init> [--label <label>] --json
csm runtime-v3 reload --init <absolute-init> --candidate <absolute-candidate> [--label <label>] --json
```

No new recovery or ownership logic is introduced. Existing Rust validation,
launchd ownership binding, active-generation routing, and transactional reload
remain authoritative.

## Compatibility boundary

`./CSMctl observatory start|status|stop|open|urls|logs` remains available for
the separate local static UI. Runtime and Observatory control must not be mixed.
Legacy Runtime verbs exit nonzero before `ensure_current_init`, HTTP probes, or
launch-service functions execute.

## Validation

A deterministic extension to the existing
`adl/tools/test_csmctl_linux_backend.sh` runs every legacy Runtime verb,
including `open` and the empty invocation, with isolated state and stubbed
external commands, proving every Runtime verb refuses and no probe or service
mutation occurs. It also proves Observatory help/routing remains present and
asserts that the runbook contains the canonical binary, init path, and label
while no longer naming the legacy package or label as permanent authority.
Focused existing Rust tests prove the unchanged canonical service ownership
contract. Exact-range `git diff --check` proves committed diff hygiene. No
validation addresses live ports or invokes host launchd.

## Non-goals

- No live Wuji restart or launchd mutation.
- No new adoption, takeover, or orphan-recovery behavior.
- No change to Rust Runtime ownership semantics.
- No provider, agent, model, cloud, edge, or Observatory UI change.
