# Structured Review Prompt

Template: 1.0.0

Issue: 628

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/local/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/local_commands.rs
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/628
.csdlc/evidence/628

## Prompts

- Verify the eight local routes are implemented or explicitly fail closed according to #628 scope.
- Verify no v3 local route delegates operational authority to v2 before #505.
- Verify missing local lifecycle state has clear typed recovery semantics.
- Verify no csdlc-v2 source changes are present.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Duplicate PR #639 remains open because no typed v2 route in scope exposes duplicate PR close; retain stacked #638 as canonical until #635 settles.

## Review Result

Revision: Some("git-blake3:453ec5973d048ed18df374b3a944e51fad42e783:a89b5cbbb4945b5e461a2139acd122c2cf68ce4b89681f64dd77a586fff72625")

Reviewer: Some("subagent:/root/review_628_local_route_state")

Result: pass
