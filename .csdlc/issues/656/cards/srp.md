# Structured Review Prompt

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/cli/csm_runtime_v3_cmd.rs

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

- No live Runtime or service-manager mutation was exercised; issue scope intentionally excludes live restart.
- Linux systemd behavior remains covered by source and parser tests rather than a live Linux service mutation in this repair review.
- GitHub hosted CI remains the integration gate after typed republication.

## Review Result

Revision: Some("git-blake3:7414ea1edd7809644a7e566678adc996cde191c0:c91f426e8134c70cbc11a4e060269d93558c773a3e7902d6ae995478da541b0b")

Reviewer: Some("fresh-session:9caf501f-8875-4b66-b62a-4372e310e0d2")

Result: pass
