# Structured Review Prompt

Template: 1.0.0

Issue: 550

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/cli/observability.rs
adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh
.csdlc/issues/550
.csdlc/evidence/550
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl-runtime/tests/runtime_api_wss.rs
demos/html-observatory
CSMctl
docs/tooling/CSMctl.conf.example

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
- Six unified Runtime kernel tests remain authoritative in ordinary cargo test and are excluded only from LLVM coverage instrumentation because that mode exceeds the per-test timeout.

## Review Result

Revision: Some("git-blake3:e9a73ade3a5904fbf4250b867e006411697cd6cc:244b727c42c8647a8ac1ef2a78d0770532b983999abe61037b8ee4b055122b97")

Reviewer: Some("fresh-session:janitor-550-pr552-after-lock-retention")

Result: pass
