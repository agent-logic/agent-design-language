# Structured Review Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/589
adl/src/cli/csm_runtime_v3_cmd.rs
adl/Cargo.toml
adl/Cargo.lock
adl-runtime/src/guardian.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
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

- Read-only review did not deploy the repaired build to Wuji or inject a live crash.
- The Terraform-managed AWS edge currently returns 503 because its residential Wuji origin is externally unreachable; this does not invalidate source semantics but blocks production reachability proof.

## Review Result

Revision: Some("git-blake3:e6405daa5ee5008ea3eb530e2942d7a54a05a53c:625cb0199d567e2c2b5b2d1a834456abd6015d5613627e0c22b64278f10d5779")

Reviewer: Some("subagent:/root/issue_589_review")

Result: pass
