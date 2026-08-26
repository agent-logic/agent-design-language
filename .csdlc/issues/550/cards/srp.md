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
Runtime/CORS/WSS and Observatory paths previously reviewed for #550
PR #552 exact source head 82966d183 and green CI run 33019833958

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

## Review Result

Revision: Some("git-blake3:82966d18360a215eb9183ae74e3f7dde394c246b:86fa74b349556a519c20117875dd95b03c75429ca75756fc068b9656c4ced945")

Reviewer: Some("fresh-session:01a01755-c400-7050-a049-b98e947a5684")

Result: pass
