# Structured Intent Prompt

Template: 1.0.0

Issue: 678

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the stable .adl/bin/csm operator command route through the active atomic Runtime v3 generation instead of remaining an independently stale binary.

## Required Outcome

.adl/bin/csm runtime-v3 ... deterministically executes .adl/runtime-v3/current/bin/csm, so generation activation and rollback atomically switch the stable route with the Runtime CSM, Guardian, and kernel artifacts.

## Scope

- adl/tools/install_runtime_v3_generation.sh
- adl/tools/runtime_v3_generation.py
- adl/tools/test_runtime_v3_generation_install.sh
- .adl/docs/TBD/resilience/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md
- .csdlc/prepared/issues/678/validate-stable-csm-route.sh
- .csdlc/prepared/issues/678/design.md
- .csdlc/prepared/issues/678/diagram.mmd
- .csdlc/evidence/678/runtime-v3-generation-install.log
- .csdlc/evidence/678/diff-check.log

## Authority

- Issue #678 owns only the stable CSM route to the active Runtime v3 generation.
- Issue #678 does not change Runtime convergence policy from #659.
- Issue #678 does not perform a live Runtime rollout, restart, reload, or stop.
- Issue #678 does not change provider, model, agent, or Observatory behavior.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle authority.
- Use a bound FastWork issue worktree for tracked implementation edits.
- Do not restart, reload, or stop the live Runtime during local validation.
- Use isolated fixture generations for installer and rollback proof.
