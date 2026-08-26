# Structured Review Prompt

Template: 1.0.0

Issue: 300

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/tests/projection_recovery_integration.rs
.csdlc/evidence/300/bridge-fed-r12
.csdlc/requests/300/replace-sor-bridge-fed-r12.json

## Prompts

- Are both prerequisite terminal and ancestry gates exact and fail closed before bind?
- Does every production mutation and durability boundary have before/after restart proof?
- Can any mock, constant, path, or self-authored receipt become authority?
- Are symlink, repeated-inode, ancestor-swap, destination-race, recovery/cleanup, ordinary-commit, and sentinel cases explicit?
- Does scope remain one new integration test target plus issue-local records?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not mutate the target worktree and did not run lifecycle writes, publication, merge, or local test commands.
- Reviewer verified recorded local evidence by inspecting logs and recomputing SHA-256 hashes, and verified the hosted CI failure with gh run view using a repo-local ignored cache path.
- Reviewer did not independently recompute the assignment git-blake3 suffix because no b3sum/blake3 CLI was available in PATH.

## Review Result

Revision: Some("git-blake3:6a8c4c01fcd60f22f4092daf6665f8181be98076:9df605b26e6f1e9487d104687edd228223b7d595d0c144adcb18846da020f878")

Reviewer: Some("fresh-session:tesla-300-ci-portability-r12")

Result: pass
