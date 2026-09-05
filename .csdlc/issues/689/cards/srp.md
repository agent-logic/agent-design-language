# Structured Review Prompt

Template: 1.0.0

Issue: 689

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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

- The review was intentionally non-mutating and did not exercise the live Runtime, launchd or systemd, network paths, or GitHub publication.

## Review Result

Revision: Some("git-blake3:0dcf2a1a068a3379dbba655172adf40b64e6325c:173d333e19104b76364b1f767fb7cccf8d97ebba18d648853c576afcd5d4cbb2")

Reviewer: Some("codex:/root/issue_689_final_review")

Result: pass
