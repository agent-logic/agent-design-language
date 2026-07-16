# Structured Review Prompt

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5353

## Prompts

- Verify issue-local paths cannot create a false existing-record condition.
- Verify both design and diagram digests refresh atomically.
- Verify tests do not widen into ADL or Runtime code.

## Findings

[
  {
    "id": "5353-TR-P1-transient-terminal-projection",
    "severity": "p1",
    "summary": "Review recovery temporarily projects implemented and worktree-only state before the known merged terminal observation can be recorded",
    "actionable": false,
    "in_scope": true,
    "disposition": "accepted_risk",
    "fix_revision": null,
    "route": "#5423"
  },
  {
    "id": "5353-TR-P2-version-identity",
    "severity": "p2",
    "summary": "All six retained card identities say v0.91.8 while live issue and PR truth say v0.91.7",
    "actionable": true,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "#5427"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The transient implemented projection must be replaced by typed merged closeout before #5353 reconciliation is complete
- Card identity version remains stale until the typed repair in #5427

## Review Result

Revision: Some("git-blake3:afca0a519e12ebf886e6406f0e0d69f5d9a5cb25:d79ef5c72dda69ff906b242ddf5e077433fe41c21e4161fd1b6d617cb4b280fc")

Reviewer: Some("codex-subagent-terminal-review")

Result: pass
