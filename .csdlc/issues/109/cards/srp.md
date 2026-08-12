# Structured Review Prompt

Template: 1.0.0

Issue: 109

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md
docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md
.csdlc/prepared/issues/109/validate-fresh-session-srp.sh
.csdlc/evidence/109/focused-srp-contract.log
.csdlc/issues/109

## Prompts

- Review only the named immutable commit SHA in the named worktree; do not inherit or rely on the implementation conversation.
- Operate read-only: do not edit files, lifecycle state, PR state, or GitHub state.
- Report findings first, ordered P0 through P3, with repository-relative file and line evidence; include explicit limitations and state PASS only when no actionable findings remain.
- Check every acceptance criterion and identify any actionable finding that the implementation session must resolve.
- Apply authority-critical precedence: changes to authentication, authorization, security boundaries, lifecycle authority, or proof production require code, security, and evidence review even when the changed files are documentation.
- Verify the standard SRP remains the sole review-result authority and that any substantive fix requires a refreshed SRP and fresh-session review at the new exact head.
- Verify no daemon, scheduler, registry, claim, parallel review record, provider abstraction, lifecycle phase, or redundant broad validation was added.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
