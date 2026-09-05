# Structured Review Prompt

Template: 1.0.0

Issue: 689

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

CSMctl
adl-runtime-kernel/tests/configuration.rs
adl-runtime/tests/runtime_api_wss.rs
adl/tools/test_csmctl_linux_backend.sh
adl/tools/test_csmctl_observatory_origins.sh
adl/tools/validate_v092_observatory_restart_reconnect.sh

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

- The live restart validator was migrated but not executed because issue #689 explicitly excludes live Runtime and service-manager mutation.
- The separately observed Observatory schema-v2 contract mismatch remains outside the PR #690 remediation scope.

## Review Result

Revision: Some("git-blake3:dbcd20d9ffb648f9638d819201d86427d188ff90:a5c3402430b4d40da9b38d72cb4f51bfa6e7468f5e8e1e0f8c1d68e56b04ae2e")

Reviewer: Some("codex:/root/review_689_fixes")

Result: pass
