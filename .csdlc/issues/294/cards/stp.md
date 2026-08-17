# Structured Task Prompt

Template: 1.0.0

Issue: 294

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #294 control-plane recovery and bootstrap rejection only; no #292 product behavior.

## Deliverables

- Typed initialized design-envelope recovery request and result
- Append-only recovery audit evidence
- Bootstrap unsafe authored-path rejection
- Focused recovery and linked-worktree tests

## Acceptance

1. AC-1: Typed initialized recovery atomically corrects reviewer identity and relocates artifact paths
2. AC-2: Operation is generation/digest guarded and append-only
3. AC-3: Audit records complete old/new reviewer, path, digest, reviewed-generation/digest, session, and no-inheritance evidence
4. AC-4: Insufficient proof invalidates approval and requires reapproval
5. AC-5: Noncanonical reviewer identity requires concrete projectless UUID and no-inheritance evidence
6. AC-6: Unsafe path classes fail closed
7. AC-7: Bootstrap rejects .git authored paths
8. AC-8: Linked-worktree fixture fails before recovery and succeeds after recovery
9. AC-9: Negative tests cover phase, CAS, source, drift, path, history, and reviewer failures
10. AC-10: #292 remains blocked until #294 is terminal and ancestral

## Dependencies

- #292 blocked on terminal ancestral #294

## Inputs

- csdlc-v2/
- .csdlc/issues/292/index.json read-only reproduction truth
- docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md

## Non Goals

- #292 product implementation or mutation
- #112 mutation
- manual lifecycle record edits
- generic lifecycle rewrite
- merge or closeout
