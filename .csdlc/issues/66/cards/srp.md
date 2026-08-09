# Structured Review Prompt

Template: 1.0.0

Issue: 66

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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
.csdlc/issues/66
.csdlc/prepared/issues/66
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

Revision: Some("git-blake3:fe2ecc6fdac73fe66a4764bd68a9f4e5a93d5430:20a85f0be5dc3969b4e40d9a2b4563209d038fd9d83915acca3d0e1b2fc1034f")

Reviewer: Some("subagent:66-final-review")

Result: pass
