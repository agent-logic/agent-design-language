# Structured Review Prompt

Template: 1.0.0

Issue: 550

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

CSMctl
docs/tooling/CSMctl.conf.example
adl-runtime/tests/runtime_api_wss.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl/tools/test_csmctl_observatory_origins.sh
demos/html-observatory
.csdlc/issues/550
.csdlc/prepared/issues/550
.csdlc/evidence/550

## Prompts

- Does parsing accept only an exact HTTPS DNS origin with a valid optional port?
- Do executable tests cover all valid combinations and unsafe input classes?
- Can invalid input alter the generated Runtime config?
- Does the HTML Observatory trust the configured Runtime host instead of a stale hard-coded hostname?
- Is the delta isolated from newer main work and from HOT-01 dynamic reload?
- Does live proof establish trusted TLS and exact CORS for both Observatory origins?

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
