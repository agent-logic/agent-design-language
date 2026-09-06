# Structured Review Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/agent_orientation.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs

## Prompts

- Can any admitted agent reach its first model turn without the active orientation snapshot?
- Does the recorded digest cover the exact delivered bytes rather than a mutable source or global resource?
- Can reload misreport the package delivered to an existing agent?
- Can invalid content replace the last valid active package?
- Does any wording or control path let orientation enlarge authority?
- Is the implementation smaller than a general prompt framework?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Focused review covered the exact-head PR #709 blocker fixes only: durable dynamic-agent orientation persistence, parts-only provider A2A multipart normalization, configured source-path loading, and deterministic cleanup-race test synchronization.
- Review did not perform live provider inference, paid Runtime execution, external agent communication, or broad non-runtime validation; local proof is limited to worktree-local Rust/Node validation already recorded in the SOR.

## Review Result

Revision: Some("git-blake3:79bcf263d7826d8e3d803f6ceb46fe1b538aaf63:ab063bfde159c636c0fa1804dca19e96809db900aabf05735a2418de85da9433")

Reviewer: Some("codex:issue-708-review-blocker-fix-review")

Result: pass
