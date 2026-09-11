# Executable Governance

Status: planned. Owners: CF-GOV and CF-GOV-CI.

Beta 1 will express bounded architecture fitness functions as reviewable contracts and run them locally and in CI. Machine-checkable invariants remain separate from human architectural judgment; routing policy stays in manifests and runners rather than ordinary tests.

Acceptance requires deterministic pass/fail fixtures, actionable diagnostics, declared PVF classification, and consistent local/CI semantics. This is not a general CI orchestration product.

The listed owners deliver separate working tasks, with complete consumer and failure evidence defined in the [atomic task contracts](../ATOMIC_TASK_CONTRACTS_v0.92.2.md). A feature packet or schema alone cannot close an implementation row.
