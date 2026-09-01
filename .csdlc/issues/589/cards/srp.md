# Structured Review Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/589
adl/src/cli/csm_runtime_v3_cmd.rs
adl-runtime/src/guardian.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/control.rs

## Prompts

- Verify startup no longer requires the separate continuity channel while Guardian ownership remains intact.
- Verify stale-state recovery cannot remove a lock owned by a live writer.
- Verify reload preserves the last known-good running configuration on candidate failure.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The read-only reviewer did not inject a live process crash; crash ordering is covered by focused lifecycle tests.
- The Terraform-managed AWS edge currently returns 503 because its residential Wuji origin is externally unreachable; that separate #122 edge condition does not invalidate local Guardian, Runtime, CloudWatch, or SSM recovery proof.

## Review Result

Revision: Some("git-blake3:15ff9fda869ba94bd18c90e1f076577f192969e7:0897ae90617c525bdc17e173aa39a6400cc158561081782b8bbb0c467ffbd126")

Reviewer: Some("subagent:/root/issue_589_review")

Result: pass
