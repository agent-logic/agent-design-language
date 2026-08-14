# Structured Intent Prompt

Template: 1.0.0

Issue: 252

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Eliminate the deterministic hosted Runtime CI cause that makes valid Guardian test children intermittently surface as SpawnFailed.

## Required Outcome

The two observed Guardian regressions and the full required Runtime lane resolve child executables independently of test order and caller cwd while genuine missing programs remain fail-closed.

## Scope

- adl-runtime/src/guardian.rs
- adl-runtime-kernel/tests/parity_b_live_kernel.rs
- Focused Guardian spawn-fixture proof

## Authority

- Production Guardian spawn errors remain fail-closed
- Issue #252 owns only executable resolution/invocation at the Guardian fixture boundary
- PR #243 birth-witness semantics remain unchanged

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 only
- Use repository or Git-common paths and never /private/tmp
- Run no optional or paid CI
- Do not merge without explicit authorization
