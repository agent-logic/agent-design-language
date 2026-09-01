# Structured Output Record

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a simple ordered Runtime v3 host lifecycle, removed the Guardian's separate startup continuity client, hardened correlated writer recovery and reload rollback, required a fresh Shepherd for readiness, and deployed bounded CloudWatch-to-SSM recovery for Wuji.

## Artifacts

- adl/src/cli/csm_runtime_v3_cmd.rs
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/observability.rs
- infra/aws/csm-runtime-health

## Execution

- CSM start and reload now use one ordered stop-wait-start-readiness sequence without overlapping host-service generations.
- Guardian ordinary startup supervises the kernel through its authenticated child lease and no longer creates a separate continuity-channel client.
- Writer and reload transaction cleanup is correlated, atomic, and removes partial staging files after failed copies.
- Runtime readiness requires a fresh admitted Shepherd lease and emits a bounded health heartbeat containing Polis, instance, readiness, liveness, and canonical config identity.
- Terraform provisions a missing-heartbeat CloudWatch alarm, EventBridge target, SNS notification, and bounded SSM recovery document for Wuji.

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
