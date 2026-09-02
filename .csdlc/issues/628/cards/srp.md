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
.csdlc/issues/627
.csdlc/evidence/627

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

- Cargo and typed validators were not rerun by the read-only review subagent; retained local evidence records the passing validation runs.
- Publication still depends on the typed publication route and preserving #505 pre-cutover authority boundaries.

## Review Result

Revision: Some("git-blake3:089a8d3f3729f7e85fbe935c7c991c57cfa9ebe6:a8af1cda3e8b3f64b1a0db094849b894746714cd49c5c637e1e9304fa9ede134")

Reviewer: Some("subagent:/root/review_628_post627_restack_r6")

Result: pass
