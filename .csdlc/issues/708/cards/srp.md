# Structured Review Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

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

- Review was focused on the deterministic duplicate conversation in-flight test repair and did not run live provider inference, paid Runtime execution, external communications, or broad non-runtime validation.
- The reviewer ran the formerly failing focused test with worktree-local TMPDIR and confirmed 1 passed, 0 failed, 182 filtered out; the full adl-runtime-v3-fast hosted lane remains deferred to GitHub CI after republish.

## Review Result

Revision: Some("git-blake3:400c337e1dfbf47c9be981e38accba3b34e77b1d:6d2b652e806437ce8c94306a81d3105673c2c1a1ed391bf2f8c0cc1852798af1")

Reviewer: Some("codex:issue-708-ci-race-fix-review")

Result: pass
