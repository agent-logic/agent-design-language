# Issue 762 design

Repair A520-ARCH-005 and C520-CODE-005: executable location supplies provenance only; authenticated issue context owns writes.

Authenticate operational context before proof/install side effects; make historical routes dormant; prove stable-binary linked-worktree topology and negative guards.

- Authenticate invoking issue worktree, branch, exact HEAD and common Git directory before mutation.
- Confine every operational write to the registered issue worktree; reject primary and escaping paths.
- Classify proof, install, shadow and soak explicitly; historical routes cannot mutate.
- Prove one stable installed binary works from linked worktrees and rejects invalid contexts without writes.

Use existing canonical native authority verification and authenticated lifecycle store. Reject before command spawn, directory creation or install. Historical shadow/soak remain inspection-only blocked routes. Operational proof/install require exact context. Tests create isolated Git fixtures and install one binary under primary .adl/bin; invocation cwd selects the candidate context, never grants authority alone.
