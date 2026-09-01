# Structured Review Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/589
adl/src/cli/csm_runtime_v3_cmd.rs
adl-runtime/src/guardian.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/control.rs

## Prompts

- Verify startup no longer requires the separate continuity channel while Guardian ownership remains intact.
- Verify stale-state recovery cannot remove a lock owned by a live writer.
- Verify reload preserves the last known-good running configuration on candidate failure.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
