# Runtime v3 Adapter

The Runtime v3 adapter connects ADL plans and engine events to Runtime v3 while
preserving Runtime v3 as the execution authority.

Required proof comes from `#5341` and `#5361`, with provider/tool adapter
support from `#5349`. Runtime v3 acceptance also consumes the WP-10A live
workcell output-contract proof from `#5501`; `#5361` must not close until that
contract is available or explicitly blocked with evidence.

Runtime v3 parity is owned under `#5361` in dependency order:

1. `#5591` proves kernel lifecycle, canonical ingress, continuity, replay, and
   graceful pressure shutdown.
2. After that ingress contract is reviewed, `#5592` proves reasoning graphs,
   bounded loops, adaptive learning, affect reasoning-control, and governed
   cognition.
3. `#5589` replaces degraded governed operations adapters.
4. `#5590` proves configuration-driven secure access, guardian supervision,
   authenticated HTML Observatory consumption, telemetry routing, soak, and
   rollback.

Parity-B/C/D may execute concurrently only after protected-path manifests prove
their writes are disjoint. Runtime v2 remains the retained source of behavior
until all owned feature rows have a reviewed Runtime v3 implementation or an
accepted boundary/defer disposition and cutover is proven.

No runtime deployment claim is valid until exact revision, install, operation,
and rollback evidence are recorded.
