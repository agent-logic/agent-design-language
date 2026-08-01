# Structured Review Prompt

Template: 1.0.0

Issue: 5558

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/check_coverage_impact.sh
adl/tools/test_check_coverage_impact.sh
.csdlc/issues/5558

## Prompts

- Does any changed active surface still expose an executable sunset v1 lifecycle route?
- Does the owner lane run the real Gate 10A final-authority test?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GitHub CI must confirm the corrected explicit coverage-impact mapping on the refreshed PR head.

## Review Result

Revision: Some("git-blake3:0c2f87c381fd9731414edd6cf959388ead3bc7aa:021f36e4f24a1e449649951cd09be5b27ae9acc6b72a4abb0a29957ef1e4858a")

Reviewer: Some("subagent:019fbf2d-07af-7ee0-a7e1-ca480b4a5d0a")

Result: pass
