# Structured Review Prompt

Template: 1.0.0

Issue: 109

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md
docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md
.csdlc/prepared/issues/109/validate-fresh-session-srp.sh

## Prompts

- Review only the named immutable commit SHA in the named worktree; do not inherit or rely on the implementation conversation.
- Operate read-only: do not edit files, lifecycle state, PR state, or GitHub state.
- Report findings first, ordered P0 through P3, with repository-relative file and line evidence; include explicit limitations and state PASS only when no actionable findings remain.
- Check every acceptance criterion and identify any actionable finding that the implementation session must resolve.
- Apply authority-critical precedence: changes to authentication, authorization, security boundaries, lifecycle authority, or proof production require code, security, and evidence review even when the changed files are documentation.
- Verify the standard SRP remains the sole review-result authority and that any substantive fix requires a refreshed SRP and fresh-session review at the new exact head.
- Verify no daemon, scheduler, registry, claim, parallel review record, provider abstraction, lifecycle phase, or redundant broad validation was added.

## Findings

[
  {
    "id": "R109-P1-SRP",
    "severity": "p1",
    "summary": "Standard SRP omitted self-contained read-only, findings-first, evidence, resolution, and scope-depth instructions.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6309cabb98605fa901ce4883ee50256f1fc72a8b:045674ae3a993d64142bb5f98f007ad8624c48eecef23a7cdca5fd4bbcec5fca",
    "route": null
  },
  {
    "id": "R109-P2-VALIDATOR",
    "severity": "p2",
    "summary": "Focused validator checked isolated substrings and did not prove AC-3 through AC-7.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6309cabb98605fa901ce4883ee50256f1fc72a8b:045674ae3a993d64142bb5f98f007ad8624c48eecef23a7cdca5fd4bbcec5fca",
    "route": null
  },
  {
    "id": "R109-P2-PRECEDENCE",
    "severity": "p2",
    "summary": "Review-depth rules did not define authority-critical precedence over documentation-only classification.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6309cabb98605fa901ce4883ee50256f1fc72a8b:045674ae3a993d64142bb5f98f007ad8624c48eecef23a7cdca5fd4bbcec5fca",
    "route": null
  },
  {
    "id": "R109-P1-COMMIT-CONTENT",
    "severity": "p1",
    "summary": "Validator read mutable working-tree policy files rather than proving their equality to exact-head blobs.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6309cabb98605fa901ce4883ee50256f1fc72a8b:045674ae3a993d64142bb5f98f007ad8624c48eecef23a7cdca5fd4bbcec5fca",
    "route": null
  },
  {
    "id": "R109-P2-COMMAND-TRUTH",
    "severity": "p2",
    "summary": "VPP and SOR omitted the validator's required immutable base and exact-head arguments.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6309cabb98605fa901ce4883ee50256f1fc72a8b:045674ae3a993d64142bb5f98f007ad8624c48eecef23a7cdca5fd4bbcec5fca",
    "route": null
  },
  {
    "id": "R109-P2-PLAN-TRUTH",
    "severity": "p2",
    "summary": "SPP plan steps remained pending after implementation completed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6309cabb98605fa901ce4883ee50256f1fc72a8b:045674ae3a993d64142bb5f98f007ad8624c48eecef23a7cdca5fd4bbcec5fca",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:6309cabb98605fa901ce4883ee50256f1fc72a8b:045674ae3a993d64142bb5f98f007ad8624c48eecef23a7cdca5fd4bbcec5fca")

Reviewer: Some("fresh-session:019fe9ad-b2d4-7fd2-b669-5abd397630b0")

Result: pass
