# Structured Review Prompt

Template: 1.0.0

Issue: 617

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs
.csdlc/evidence/617/hosted-ci-retry.md

## Prompts

- Does every canonical name come from authoritative configuration or admitted state?
- Are operational ID, canonical name, display name, and office still distinct?
- Do roster, detail, JSON serialization, and OpenAPI agree?
- Is Shepherd naming stable without changing lifecycle semantics?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The bounded remediation review did not rerun the full Runtime suite; hosted CI remains the final integration gate.

## Review Result

Revision: Some("git-blake3:0f8fec10d388ed5c8c1c0afa401edffe8ded9269:1bed52fc6c0efa86afd3b57311a0657617a83e5feacd29e6c4e18f89bd524e41")

Reviewer: Some("fresh-session:cd23d270-c41f-4421-a05f-05a77bd00428")

Result: pass
