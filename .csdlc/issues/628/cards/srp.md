# Structured Review Prompt

Template: 1.0.0

Issue: 628

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v3/src/commands/local/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/local_commands.rs
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/628

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

- csdlc-validate finalize cannot refresh implementation truth after a post-review fix; defect recorded for #632.

## Review Result

Revision: Some("git-blake3:63e65a58b70b12615f963137aab7499ba0cdfa1a:1cc588a593f5ec7acf1353a866020ac847d05cd7ad1659d01f80f0ffd22f533e")

Reviewer: Some("codex-reviewer:review_628_exact_head")

Result: pass
