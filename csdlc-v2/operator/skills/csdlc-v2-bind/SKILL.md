---
name: csdlc-v2-bind
description: Bind an execution-ready C-SDLC v2 issue to one branch and worktree.
---
Invoke `csdlc-bind --root <repo> --request <json>` and report its typed result.
The request names the issue, base branch, issue branch, and worktree. For a
split-authority route, it must also name `code_repository`; bind verifies that
identity against the effective `origin` and records it in canonical issue
state. Omit `code_repository` only when issue and code use the same repository.
Derive ownership only from the bound Git topology. Do not create claims, edit
cards, or fall back to shell/Python lifecycle mutation.

## C-SDLC v3 transition boundary

C-SDLC v3 is construction evidence only until an explicit operator-reviewed
V3-F cutover changes root authority. Continue using this v2 bind route for live
worktree binding; do not treat v3 branch, adapter, transaction, or projection
models as authority to bind or rebind issue work.
