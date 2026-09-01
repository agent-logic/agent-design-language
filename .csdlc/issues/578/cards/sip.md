# Structured Intent Prompt

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Onboard Z.ai GLM-5.3-Flash as a deterministic first-class ADL provider profile and prove it can be selected for reviewer-style inference.

## Required Outcome

A source-grounded GLM-5.3-Flash profile, request-parameter path, focused tests, docs, and reviewer-selection proof exist without touching unrelated issue lanes.

## Scope

- adl/src/provider/profiles.rs
- adl/src/provider/http_family.rs
- adl/src/provider/mod.rs
- adl/tests/provider_tests/profiles.rs
- adl/tests/provider_tests/http_family.rs
- docs/provider/inference-profiles.md
- docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash/README.md
- .csdlc/prepared/issues/578

## Authority

- Issue #578 owns only GLM-5.3-Flash provider-profile onboarding and reviewer-selection proof.
- #514 is a completed dependency and provides profile machinery, not permission to rewrite unrelated provider architecture.
- #446 and #455 are explicitly out of scope and must not be touched.
- Live provider calls require configured credentials and truthful opt-in validation records.

## Assumptions

- none

## Operator Constraints

- Keep this to one quick provider-onboarding issue.
- Use direct Z.ai as the first deterministic target.
- Capture lifecycle/tooling regressions instead of bypassing typed v2 with raw GitHub writes.
- Do not print, copy, or persist credentials.
