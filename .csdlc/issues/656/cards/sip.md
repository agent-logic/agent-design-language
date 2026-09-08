# Structured Intent Prompt

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Eliminate mixed Runtime v3 installations by managing CSM, Guardian, and kernel as one verified generation.

## Required Outcome

One command stages, verifies, activates, and rolls back one matched three-binary Runtime generation through a single atomic current reference.

## Scope

- adl/tools/install_runtime_v3_generation.sh
- adl/tools/runtime_v3_generation.py
- adl/tools/test_runtime_v3_generation_install.sh
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl/tests/csm_runtime_v3_generation.rs
- .csdlc/prepared/issues/656
- .csdlc/issues/656

## Authority

- Issue #656 and the TBD resilience plan define this slice
- One receipt is authority for all three installed artifacts
- Generation preflight occurs before service mutation
- Local validation does not restart the live Runtime

## Assumptions

- none

## Operator Constraints

- Never write tracked files on main
- Do not restart the live Runtime
- Do not change timeout, provider, identity, Observatory, Caddy, cloud, or Runtime v2 behavior
- Do not add another supervisor
