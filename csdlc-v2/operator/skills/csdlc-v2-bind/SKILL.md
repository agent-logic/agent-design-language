---
name: csdlc-v2-bind
description: Bind an execution-ready C-SDLC v2 issue to one branch and worktree.
---
C-SDLC v2 remains the live lifecycle authority only until explicit V3-F/#505
cutover. Before that cutover, `csdlc-v3/**` is non-authoritative construction
evidence and must not mutate lifecycle state.

Invoke `csdlc-bind --root <repo> --request <json>` and report its typed result.
The request names the issue, base branch, issue branch, and worktree. For a
split-authority route, it must also name `code_repository`; bind verifies that
identity against the effective `origin` and records it in canonical issue
state. Omit `code_repository` only when issue and code use the same repository.
Derive ownership only from the bound Git topology. Do not create claims, edit
cards, or fall back to shell/Python lifecycle mutation.
