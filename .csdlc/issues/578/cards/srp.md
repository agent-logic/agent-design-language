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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
