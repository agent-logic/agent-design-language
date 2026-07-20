# Runtime v3 Functional Parity

Runtime v3 functional parity is a first-class v0.91.8 feature and release gate.
It is owned by #5336, #5591, #5592, #5589, #5590, #5341, #5349, #5350, and
#5361.

The feature is not complete when Rust types or fixtures exist. It is complete
when representative work enters the initialized canonical runtime, executes
the intended production components, emits retained evidence, survives negative
cases and graceful recovery, and is visible through the secure runtime-owned
Observatory feed.

The canonical contract, ten proof groups, four parallel lanes, budgets,
feature-preservation dispositions, and cutover dependencies are defined in
[RUNTIME_V3_FUNCTIONAL_PARITY_PLAN_v0.91.8.md](../RUNTIME_V3_FUNCTIONAL_PARITY_PLAN_v0.91.8.md).

This feature does not authorize cutover, deletion, AWS use, remote/GPU claims,
subjective affect claims, or v0.92 activation.
