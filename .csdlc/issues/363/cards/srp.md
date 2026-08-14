# Structured Review Prompt

Template: 1.0.0

Issue: 363

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/363
.csdlc/prepared/issues/363
.csdlc/evidence/363

## Prompts

- Is recovery ancestry bounded to one issue epoch?
- Does every authority-changing operation end the epoch?
- Are review publication readiness terminal guards preserved?
- Does the regression reproduce #274 sequencing?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:1765d9eaf7c6da7ef80b28d052c8d2a06d3f3bef:4563de7d1436ba72888f21863583369f28c1a5636ab36e8f7170720352ffbfa3")

Reviewer: Some("fresh-session:2e9ad15d-ff0a-4c15-8bac-ac28674348d4")

Result: pass
