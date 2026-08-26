# Structured Review Prompt

Template: 1.0.0

Issue: 550

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

CSMctl
docs/tooling/CSMctl.conf.example
adl-runtime/tests/runtime_api_wss.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl/tools/test_csmctl_observatory_origins.sh
demos/html-observatory
adl/src/adl_gws_context_mirror.rs
adl/tests/memory_palace_tests.rs
adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh
.csdlc/issues/550
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
- Four unified Runtime kernel tests remain authoritative in ordinary cargo test and are excluded only from LLVM coverage instrumentation because that mode exceeds the per-test timeout.

## Review Result

Revision: Some("git-blake3:567920947599c93c95e5a29cfd7e344f261b31f4:1c854f42b05979729cb7ed6be72627c026ea81a69371787f72db108c76228590")

Reviewer: Some("fresh-session:01a01755-c400-7050-a049-b98e947a5684")

Result: pass
