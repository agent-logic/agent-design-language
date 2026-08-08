# Structured Review Prompt

Template: 1.0.0

Issue: 32

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

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

Revision: Some("git-blake3:bc19d9445d6ccfe1bb94b2a13bea6465b26cae2a:3ca66a697b4986b55a5161e0f3d9db3903a7e2c5af531eae6f6d46bccc29e95e")

Reviewer: Some("review_32_implementation")

Result: pass
