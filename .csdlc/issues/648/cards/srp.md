# Structured Review Prompt

Template: 1.0.0

Issue: 648

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/src/provider/reload.rs
adl/src/execute/mod.rs
adl/src/execute/runner.rs
adl/src/execute/tests.rs
adl/src/long_lived_agent.rs
.csdlc/prepared/issues/648
.csdlc/evidence/648

## Prompts

- Does the corrective branch actually differ from current main by the run-scoped reload ownership repair?
- Can overlapping workflows still consume or clear each other's provider snapshot?
- Does the compatibility global guard direct test catch old-drop/new-registration clearing?
- Were all proofs offline and free of live Runtime or provider credential mutation?
- Does the corrective PR close #648 rather than pretending #622 was semantically complete?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review did not run live provider, AWS, paid/cloud, Runtime restart, cutover, or CI checks.
- Publication/PR/CI readiness remains unproven until typed publication creates a PR and GitHub checks complete.

## Review Result

Revision: Some("git-blake3:885c9e3b58637be8ee3e2b13fbfe56839b284879:7df793a6456b634fb3e995a965105715e4e519fbcb31466af561c3cf6b9cc2bd")

Reviewer: Some("codex-subagent:/root/review_648_provider_reload_corrective_exact_head")

Result: pass
