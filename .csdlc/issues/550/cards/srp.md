# Structured Review Prompt

Template: 1.0.0

Issue: 550

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh
adl/src/cli/observability.rs
.csdlc/issues/550
.csdlc/evidence/550
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl-runtime/tests/runtime_api_wss.rs
demos/html-observatory
CSMctl
docs/tooling/CSMctl.conf.example
PR #552 exact head 436ba092f

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

- The local exact-host mapping is Wuji operational state used to avoid router hairpin while preserving hostname and public certificate validation; it is not portable deployment authority for other hosts.
- The unified Runtime kernel family and real four-node learner replication test remain authoritative in ordinary cargo test and are excluded only from LLVM coverage instrumentation because that mode exceeds their bounded test deadlines.
- GitHub CI remains the final integration authority for the exact published PR head.

## Review Result

Revision: Some("git-blake3:436ba092f338cd6277dc532b37185f72fda7c6d4:b31001c3ffc4a3a1e0f3a283b19c91c28ea8ec9751b5d6b7ec8a46e2093b45ce")

Reviewer: Some("fresh-session:01a01755-c400-7050-a049-b98e947a5684")

Result: pass
