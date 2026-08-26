# Structured Task Prompt

Template: 1.0.0

Issue: 418

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #418 audited raw-gh break-glass policy only.

## Deliverables

- Fail-closed root break-glass policy in AGENTS.md.
- Three-event create-only receipt and typed reconciliation protocol in docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md.
- Aligned owner boundary in docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md.
- Updated exact route contract regression in csdlc-v2/tests/gate_github_route_policy.rs.
- .csdlc/prepared/issues/418/validate_gh_breakglass_policy.sh

## Acceptance

1. AC-1: Root policy preserves typed v2 as default and permits raw gh only after a reproducible, durably tracked typed-owner regression plus explicit exact-target operator authorization.
2. AC-2: The exception permits only six canonical argv shapes: issue comment/body-file, issue edit/body-file, PR create with exact repo/base/head/title/body-file and optional draft, PR edit/body-file, PR ready, and PR comment/body-file; all other commands, flags, aliases, extensions, APIs, bulk targets, shell expansion, and terminal/destructive operations are denied.
3. AC-3: Exact worktree, branch, HEAD, typed generation/digest, remote pre-state, regression issue, authorization reference, redacted argv, result, and later exact reconciliation are retained as three create-only intent/result/reconciliation events without secrets or sensitive bodies.
4. AC-4: A raw mutation never becomes lifecycle authority; later readiness, review, publication, merge-ready, terminal, and finish claims remain frozen until typed exact-state reconciliation succeeds and its immutable reconciliation event exists.
5. AC-5: Focused validation proves required policy text plus positive fixtures for every allowed argv shape and negative fixtures for missing identity, issue create, close/base/merge/release/workflow/admin/secret/API/alias/extension/bulk/unsafe-body/extra-flag variants.
6. AC-6: The exception is not invoked before this issue is reviewed and merged; #414, #268, #269, and AWS remain untouched.

## Dependencies

- GitHub issue #418.
- Existing typed C-SDLC v2 GitHub owners and lifecycle records.

## Inputs

- GitHub issue #418
- AGENTS.md
- docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md
- csdlc-v2/operator/skills/
- existing typed GitHub owner binaries

## Non Goals

- General raw-gh fallback.
- Bypassing review, CI, merge authority, finish, or cleanup.
- Implementing a new lifecycle binary.
- Using the exception on #414, #268, or #269 in this issue.
