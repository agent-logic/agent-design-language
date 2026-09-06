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

- Review confirmed the duplicate in-flight test repair is deterministic and does not change source semantics beyond removing the sleep-timed race.
- The full hosted adl-runtime-v3-fast lane remains deferred to GitHub CI after republish.

## Review Result

Revision: Some("git-blake3:8f6e0daf9cc0e8e0260a8592255079bd6d6ba25e:b5fa5325ad11ead2005f21674901bb1bac3e77cc61e48300a471a6ef4d86eb63")

Reviewer: Some("codex:issue-708-ci-race-final-review")

Result: pass
