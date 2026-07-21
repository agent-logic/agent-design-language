# ADL portable engine

`adl-engine` consumes the inert `adl_compiler::ExecutionPlan` contract and
implements a pure, deterministic, bounded plan-level state machine. Hosts drive
it with explicit logical ticks, typed completions, and cancellation intents;
the engine returns ordered provider, tool, and cancellation effects.

The crate performs no I/O, clock reads, sleeping, process or thread control,
networking, persistence, provider access, or Runtime integration. Checkpoints
are canonical quiescent byte snapshots that must be persisted by a separate
host-owned adapter.

The public flow is:

1. construct exact `EngineLimits` and an `EnginePolicy` for every plan node;
2. call `Engine::new` to validate and admit the plan;
3. call `Engine::turn` with monotonically increasing logical ticks;
4. deliver typed completions using the emitted request identity and attempt;
5. call `Engine::checkpoint` only when `Engine::is_quiescent` is true;
6. call `Engine::resume` with the exact same plan, policy, and limits.

Provider and governed-tool adapters are deliberately outside this crate.
