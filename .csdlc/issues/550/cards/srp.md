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

- The local exact-host mapping is Wuji operational state used to avoid router hairpin while preserving the public certificate hostname; it is not portable deployment authority for other hosts.

## Review Result

Revision: Some("git-blake3:e7baf7c313d508722cd313d03d8e7b9a66228ddb:d281cd0d7d5235dc5b3aaa9d471f264ea0323ec80a3bb76e5ea76786bd853e79")

Reviewer: Some("fresh-session:01a01755-c400-7050-a049-b98e947a5684")

Result: pass
