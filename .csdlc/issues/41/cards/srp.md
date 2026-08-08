# Structured Review Prompt

Template: 1.0.0

Issue: 41

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/41
.csdlc/prepared/issues/41
csdlc-v2/src/error.rs
csdlc-v2/src/github.rs
csdlc-v2/tests/gate_github_actions.rs

## Prompts

- Can a 401, ordinary 403, rate limit, 5xx, or connection failure be mislabeled as not-found?
- Can any token, token path, authorization header, raw response body, or Octocrab error text reach stdout or stderr?
- Does the 404 wording remain truthful for inaccessible private repositories?
- Do the tests invoke the real split CLI and assert exact JSON and exit behavior?
- Are successful issue reads byte-shape compatible?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live GitHub request was made; deterministic loopback proof is the intentional validation boundary.

## Review Result

Revision: Some("git-blake3:bda51b77837a4a8ae76e39ae39158400f36425a6:70c6fd14d59905c3000145a3d1989976ccbdec33f27adc2ec20c47134f6c1a49")

Reviewer: Some("subagent:review-41-implementation")

Result: pass
