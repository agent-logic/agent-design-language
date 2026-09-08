# Structured Task Prompt

Template: 1.0.0

Issue: 689

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Correct the stale operator route and disable misleading legacy Runtime lifecycle claims without reimplementing service ownership.

## Deliverables

- rewritten canonical Runtime startup runbook
- legacy CSMctl Runtime verb refusal
- routing and docs guard tests

## Acceptance

1. AC-1: The runbook names the current-generation csm binary, stable live init path, and canonical launchd label.
2. AC-2: Read-only status guidance explains service_loaded, listener_ready, Guardian PID, Runtime PID, active init hash, and observability readiness.
3. AC-3: Obsolete service root and label are not presented as permanent Runtime authority.
4. AC-4: Legacy CSMctl Runtime verbs cannot emit authoritative pass and instead return a concise canonical-command message.
5. AC-5: CSMctl Observatory-only commands remain available and separate.
6. AC-6: Existing canonical ownership tests and new deterministic routing/docs guards pass without live service mutation.
7. AC-7: Shell syntax, diff hygiene, and independent exact-head review pass.

## Dependencies

- #678 / PR #682 is merged
- #688 closed as false-premise bootstrap

## Inputs

- agent-logic/agent-design-language#689
- docs/tooling/START_CSM_RUNBOOK.md
- CSMctl
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl/src/cli/csm_cmd.rs

## Non Goals

- New ownership or recovery implementation
- Live Wuji restart or launchd changes
- Cloud edge provider agent model or Observatory UI work
- A second Runtime control plane
