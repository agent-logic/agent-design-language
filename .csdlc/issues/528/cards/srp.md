# Structured Review Prompt

Template: 1.0.0

Issue: 528

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/528
.csdlc/issues/528
.csdlc/prepared/issues/528
adl/src/adl/validation.rs
adl/src/provider/http_family.rs
adl/src/provider/http_family/config.rs
adl/src/provider/mod.rs
adl/src/provider/profiles.rs
adl/src/provider_substrate.rs
adl/tests/adl_tests.rs
adl/tests/provider_tests/http_family.rs

## Prompts

- Does #528 preserve one shared Gemini semantic codec while adding a distinct Vertex AI transport?
- Does the design avoid credential disclosure and embedded API keys?
- Are project/location/model/endpoint/timeouts/cancellation boundaries explicit and testable?
- Do deterministic tests cover UTS tool names and arguments, streaming/non-streaming normalization, error classification, and redaction?
- Are live Vertex calls correctly separated as optional externally authorized proof?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live Vertex AI provider call was not run; credentialed provider proof remains separately gated by explicit operator authorization and environment configuration.
- The reviewer did not rerun Rust tests because the review was read-only and the retained proof logs already covered the exact implementation head.

## Review Result

Revision: Some("git-blake3:4867f7de80a6274230eea3cdcba8379ead3b41e7:e3c120724e0b33844d72a3a0336e5348c8aa6ae2155afc9e9f5d6d56b963217a")

Reviewer: Some("fresh-session:ad06b19c-5866-4bdc-939d-bedc2cd559d9")

Result: pass
