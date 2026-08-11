# Structured Review Prompt

Template: 1.0.0

Issue: 66

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/deepgram.rs
adl/src/provider/mod.rs
adl/src/provider/profiles.rs
adl/src/provider_substrate.rs
adl/src/provider/http_family/tests.rs
adl/tests/provider_tests.rs
adl/tests/provider_tests/deepgram.rs
docs/tooling/PROVIDER_SETUP.md
demos/podcast/DEEPGRAM_PROVIDER_WORKFLOW.md
docs/milestones/v0.91.2/review/provider_native_tool_call_comparison_report.json
.csdlc/evidence/66

## Prompts

- Can any credential or authorization value appear in Debug, error, log, fixture, request identity, or retained evidence?
- Can a real Deepgram credential be sent to a non-Deepgram host?
- Do typed media checks reject JSON/HTML error bodies and incompatible encoding/container combinations?
- Does capability discovery truthfully distinguish speech operations from completion operations?
- Does the live receipt prove a real round trip without retaining source content or audio?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The live canary proves one bounded Pluto-to-Nova-3 round trip; ongoing provider availability and future response-schema changes remain external operational risks.

## Review Result

Revision: Some("git-blake3:eb841f84b071cb5a0c7ded29ead29fe6987a83cb:9a97fad8642d80cf73b4fe9093944749d959a51ea2d093ef6a5fff1bbdfad203")

Reviewer: Some("subagent:66-coverage-review")

Result: pass
