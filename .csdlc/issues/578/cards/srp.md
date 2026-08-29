# Structured Review Prompt

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/profiles.rs
adl/src/provider/http_family.rs
adl/src/provider/http_family/config.rs
adl/src/provider/mod.rs
adl/src/provider/http_family/tests.rs
adl/src/provider_adapter.rs
adl/src/provider_communication.rs
adl/src/agent_comms.rs
adl/src/agent_comms/dispatch/coding.inc
adl/tests/provider_tests/profiles.rs
adl/tests/provider_tests/http_family.rs
docs/provider/inference-profiles.md
docs/tooling/PROVIDER_SETUP.md
docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash/README.md
.csdlc/prepared/issues/578/reviewer-selection-smoke.sh
.csdlc/prepared/issues/578/glm-5-3-flash-reviewer-viability-smoke.sh
.csdlc/prepared/issues/578/tooling-issue-bind-prepared-helper-materialization.md
.csdlc/issues/578/index.json
.csdlc/issues/578/cards/sor.md
.csdlc/issues/578/cards/sor.values.json

## Prompts

- Does `z_ai:glm-5.3-flash` use the existing #514 profile machinery rather than ad hoc model routing?
- Are GLM-5.3-Flash parameters source-grounded and validated before network dispatch?
- Do focused tests prove exact profile/request behavior and redaction without credentials?
- Can reviewer selection name the new profile deterministically, and is live proof truthfully credential-gated?
- Did the patch avoid #446/#455 scope?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live Z.ai dispatch remains credential-gated and was not claimed because ZAI_API_KEY was absent in the local shell.
- OpenRouter and Ollama cloud GLM-5.3-Flash routes are documented as distinct variants and intentionally not implemented by #578.
- The fresh reviewer ran focused adapter/profile/diff checks rather than the full provider test suite; local focused validation and prior PR CI cover the bounded surface.

## Review Result

Revision: Some("git-blake3:e90d097e19e4ec43580dbb5a60d876aba38b3924:b434597ecea6bdd9dd75335078acdc0a576efe0f0ad4d01b7abba518b878c58b")

Reviewer: Some("fresh-session:2fc936d8-c1bd-4350-90b3-cc50d89fc449")

Result: pass
