# Structured Review Prompt

Template: 1.0.0

Issue: 678

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/runtime_v3_generation.py
adl/tools/install_runtime_v3_generation.sh
adl/tools/test_runtime_v3_generation_install.sh
.adl/docs/TBD/resilience/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md
.csdlc/prepared/issues/678
.csdlc/evidence/678

## Prompts

- Does .adl/bin/csm now route to .adl/runtime-v3/current/bin/csm without becoming a second binary source of truth?
- Do activation and rollback switch the stable route atomically through the current symlink?
- Do missing or incomplete active generations fail before service mutation?
- Do the tests prove stale stable binary repair without touching the live Runtime?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review and validation used the isolated Runtime v3 generation fixture; no live Runtime rollout, restart, reload, stop, provider, model, agent, or Observatory path was exercised.

## Review Result

Revision: Some("git-blake3:2e6a9660d865d44027b6e9e36ea2117a4753de67:7115905ca0860569d22cb5f40b09aadeec937cd469886accaed71b2f3b1a693b")

Reviewer: Some("fresh-session:efe65191-215a-4132-bbe4-565769e1ea92")

Result: pass
