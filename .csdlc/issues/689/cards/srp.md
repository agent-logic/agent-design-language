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
adl-runtime-kernel/src/config_reload.rs
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

Revision: Some("git-blake3:bc4c88a54cc25b5533eb2e1adaf82574f5e3398f:abba41ca39ea1677553268bd9729d0cfd0da4e2a8ca03b725609eeb829282a77")

Reviewer: Some("codex:/root/issue_689_ci_exact_review")

Result: pass
