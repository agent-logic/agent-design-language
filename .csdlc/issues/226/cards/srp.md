# Structured Review Prompt

Template: 1.0.0

Issue: 226

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/config/validation_lane_selector.v0.91.6.json
adl/tools/test_select_validation_lanes.sh
adl/tools/test_ci_path_policy.sh
.csdlc/issues/226
.csdlc/evidence/226

## Prompts

- Are the new selectors narrowly bounded to existing proof ownership?
- Does any unknown path become silently covered?
- Can this change launch optional slow or coverage jobs?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Authoritative full coverage and slow proof remain explicitly out of band and were not run for this routing-only change.

## Review Result

Revision: Some("git-blake3:c42bafa0abab4a2b6f10b5071ef86fae6083ab2d:da9fbf4f532d6a03e5b96ec984240d4803bc7050247e839d76bec8314d80acbb")

Reviewer: Some("subagent:019ff1ef-56db-74c3-b178-26775be4021e")

Result: pass
