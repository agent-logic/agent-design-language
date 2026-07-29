# Structured Review Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5344
.csdlc/issues/5587
.csdlc/prepared/issues/5344
adl-runtime/src/guardian.rs
adl/tools/check_coverage_impact.sh
adl/tools/test_check_coverage_impact.sh

## Prompts

- Does the Runtime v3 coverage mapping select every existing guardian unit and guardian_cli integration test without naming deleted tests?
- Does the tooling-contract regression execute the mapper expression against the live nextest inventory and end with an explicit PASS marker?
- Does the healthy-window test retry only the local authenticated lease-listener connection with a strict bound while leaving production Guardian behavior unchanged?
- Does repeated exact execution prove the restart-budget test remains deterministic after the listener startup-race repair?
- Was #5587 already GitHub merged and closed before its expired claim was recovered and released through supported typed routes without running or re-enabling the paused Drive mirror?
- Do the SRP, SOR, audit, and exact review truthfully cover only this bounded CI follow-up?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:6db1d976a1b853d7f42fc15ad68870b02e2ffe0b:ae2f20c67dadfcaeb8006a5a2575309109c2964714adee5d185e2a686755603a")

Reviewer: Some("subagent:019fac6c-4d03-74e3-90a2-3c3f07ed609d")

Result: pass
