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
origin/main merge at 5bc84a0f27a522b6d500551d64f8d12dc2357427

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
- The current-main refresh was a clean merge from origin/main 5bc84a0f27a522b6d500551d64f8d12dc2357427; no #550 path conflict was observed, and CI remains the integration authority for the refreshed PR head.

## Review Result

Revision: Some("git-blake3:d1e4f1d8264f622e31c430fe07bdb12006384164:fb079a52ca19d723bb88d95ebbcafaf8540c5d62edb72bdf36ed7118737cfbed")

Reviewer: Some("fresh-session:janitor-550-pr552-after-main-refresh")

Result: pass
