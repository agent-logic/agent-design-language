# Structured Review Prompt

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/install_runtime_v3_generation.sh
adl/tools/runtime_v3_generation.py
adl/src/cli/csm_runtime_v3_cmd.rs
adl/tools/test_runtime_v3_generation_install.sh
adl/tests/csm_runtime_v3_generation.rs

## Prompts

- Can an incomplete set become current?
- Does the receipt bind exact activated files?
- Do launchd and Runtime-init agree?
- Is preflight before mutation?
- Is rollback limited to verified generations?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Linux systemd behavior was source and parser reviewed but not executed on the macOS review host.
- Stop safety was verified without mutating the live Runtime or service manager.

## Review Result

Revision: Some("git-blake3:e6d78f58cb575529e84111b489a382038a2d8d27:2f23e8e1c732dea73eb18fd282bb4543700df4b87bc055490ce7bf545f6b53f1")

Reviewer: Some("fresh-session:42d16aeb-9cb2-4418-b7bf-5be4ef111e32")

Result: pass
