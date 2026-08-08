# Structured Review Prompt

Template: 1.0.0

Issue: 32

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/bin/csdlc-github.rs
csdlc-v2/src/lib.rs
csdlc-v2/src/runner_preflight.rs
csdlc-v2/src/schema.rs
csdlc-v2/tests/gate_runner_preflight.rs
docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
docs/tooling/GITHUB_LARGER_RUNNER_PREFLIGHT.md

## Prompts

- Can policy-ineligible or non-dispatching routing still be misreported as capacity unavailable?
- Does eligible require explicit target-repository selection and workflow restriction off?
- Can any credential or authorization header appear in output?
- Are stale workflow refs reported without becoming false eligibility authority?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:cde7eb2fd14d97151cd99f9c57872d91fe7dd718:9070710bdffeffac6b793dceee7fe77242a26b5d38afc670936bbd0581078e91")

Reviewer: Some("review_32_implementation")

Result: pass
