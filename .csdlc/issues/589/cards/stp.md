# Structured Task Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Runtime v3 and CSM operator startup/reload reliability only.

## Deliverables

- CSM Runtime v3 lifecycle commands
- Simplified Guardian startup path
- Safe interrupted-start reconciliation
- Focused regression tests and Wuji readiness proof

## Acceptance

1. A single CSM start invocation reaches stable Runtime HTTPS readiness on 20997
2. Ordinary startup does not depend on a separate port-20998 continuity handshake
3. CSM exposes start, stop, status, and validated config reload/restart
4. Ownerless locks and interrupted startup journals reconcile automatically without discarding Polis state
5. A genuine live writer is still rejected
6. The host service manager keeps Wuji running and restarts it after failure
7. Focused local tests pass and local plus AWS-facing health is verified

## Dependencies

- Runtime v3 Guardian and kernel
- Existing CSM service operator surface

## Inputs

- adl/src/cli/csm_cmd.rs
- adl-runtime/src/guardian.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/assembly.rs
- infra/runtime-v3/runtime-init.toml

## Non Goals

- Discarding or resetting retained Wuji state
- Weakening Guardian authority or live-writer exclusion
- Changing public API or WSS route semantics
- Broad Runtime refactoring
