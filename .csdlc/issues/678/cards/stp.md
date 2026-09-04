# Structured Task Prompt

Template: 1.0.0

Issue: 678

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only the installer-managed stable CSM command route and focused regression proof for issue #678.

## Deliverables

- adl/tools/install_runtime_v3_generation.sh
- adl/tools/test_runtime_v3_generation_install.sh
- .csdlc/prepared/issues/678/validate-stable-csm-route.sh
- .adl/docs/TBD/resilience/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md

## Acceptance

1. AC-1: The stable .adl/bin/csm runtime-v3 status --init <file> --json route executes the same generation-owned CSM artifact as .adl/runtime-v3/current/bin/csm.
2. AC-2: .adl/bin/csm cannot silently remain an independently built Runtime-control binary after an active generation is installed.
3. AC-3: Generation activation and rollback atomically switch the stable command route with the complete CSM, Guardian, and kernel generation.
4. AC-4: A missing, incomplete, or invalid active generation fails before any service mutation.
5. AC-5: A deterministic regression fixture reproduces a stale stable binary beside a valid current generation and proves the stable command follows current.
6. AC-6: Status-path validation does not restart, reload, or stop the live Runtime.

## Dependencies

- Issue #656 / PR #658 atomic Runtime generation installation baseline.
- Issue #659 convergence-policy work remains out of scope.

## Inputs

- GitHub issue #678
- adl/tools/install_runtime_v3_generation.sh
- adl/tools/test_runtime_v3_generation_install.sh
- adl/src/cli/csm_runtime_v3_cmd.rs
- .adl/docs/TBD/resilience/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md

## Non Goals

- Runtime convergence-policy changes from #659.
- Provider, model, agent, or Observatory behavior changes.
- Live Runtime rollout, restart, reload, or stop.
- Redesign of unrelated CSMctl commands.
- Replacing atomic generation installation with a second binary source of truth.
