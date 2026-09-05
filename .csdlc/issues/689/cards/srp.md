# Structured Review Prompt

Template: 1.0.0

Issue: 689

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

CSMctl
adl/tools/test_csmctl_linux_backend.sh
docs/tooling/START_CSM_RUNBOOK.md
.csdlc/evidence/689

## Prompts

- Does any documentation still present the legacy service root or label as permanent authority?
- Can any legacy Runtime verb still report pass?
- Are Observatory-only commands preserved?
- Do tests avoid launchctl and live ports?
- Is the solution a simple routing correction rather than a second controller?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live Runtime or service-manager mutation was performed; cloud reachability, providers, models, and Observatory UI behavior are outside issue #689.

## Review Result

Revision: Some("git-blake3:c73dd9d5116ef5c9b75422260466fed17bbba640:1dbdfa1e0b0ad3072a9fcfc085002e5ada714153cedbb86e87beee083903a1aa")

Reviewer: Some("codex:/root/issue_689_publication_review")

Result: pass
