# Structured Review Prompt

Template: 1.0.0

Issue: 514

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/src/provider/mod.rs
adl/src/provider/profiles.rs
docs/provider/inference-profiles.md
docs/milestones/v0.92.1/evidence/provider/prov-a/README.md
.csdlc/prepared/issues/514/validate-*.rb
.csdlc/evidence/514/*.log

## Prompts

- Does the implementation stay inside the declared unit boundary?
- Does every acceptance criterion have proving evidence?
- Are operator-only actions and private material kept outside Git?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Focused local validation passed; broader workspace integration remains deferred to PR CI.

## Review Result

Revision: Some("git-blake3:525b393dfc246e8458b135aeb82ef5b99d1810a8:824bab33091d06a2863176f557b1742770b2d89e0f9ff794cc7e375e2e7f2563")

Reviewer: Some("gpt-5.5-subagent:prov-a-current-head")

Result: pass
