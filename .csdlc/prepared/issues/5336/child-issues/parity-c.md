## Goal

Replace Runtime v3 degraded operational placeholders with small production
component adapters that execute governed work through the canonical kernel.

## Owned Capability Groups

- governance, Freedom Gate, and AEE;
- delegation and resource contracts;
- agents, Shepherd, providers, scheduler, and governed tools;
- private state, citizen identity/memory, Chronosense, checkpoint, and lifelog.

## Required Outcome

Representative admitted work passes signed gate-before-actuation, attenuating
delegation, resource and cancellation bounds, provider/scheduler execution,
private-state and identity checks, checkpoint/lifelog continuity, and
fail-closed shutdown. No component reports operational readiness while backed
only by `DegradedOperationExecutor` or a synthetic fixture.

## Deliverables

- Production or COTS-backed adapters for every degraded owned component.
- Live multi-agent/provider/scheduler proof with negative governance cases.
- Identity/private-state/checkpoint/lifelog continuity proof.
- Placeholder and duplicate deletion sufficient to preserve the runtime
  budget.

## Parent And Dependencies

- Parent acceptance umbrella: #5361.
- Architecture and budgets: #5336.
- Depends on Parity-A ingress and aligns with #5349 provider/tool contracts.

## Definition Of Done

- Production code is exercised through `adl-runtime-kernel`; fixture and
  degraded-adapter evidence is insufficient.
- Deterministic positive and negative evidence is retained at an exact
  revision, including graceful shutdown/recovery.
- Maintained third-party crates are used where practical.
- No AWS use, default switch, Runtime v2 deletion, or new product scope.
