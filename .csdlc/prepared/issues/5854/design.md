# Demonstration, Handoff, and Publication Sprint Design

## Purpose

Coordinate WP-17, WP-18, WP-18B, and WP-19 while preserving the
completed WP-24 product result and preventing private production artifacts
from becoming release claims. WP-24A is an independent out-of-band stream and
cannot gate this sprint's readiness, execution, review, or closeout. WP-20 is
the first child of the final release-tail sprint #5856.

## Execution Contract

- Treat WP-24 as product/GitHub complete with asynchronous typed closeout
  pending; do not reopen its implementation path.
- Keep WP-24A outside the sprint dependency graph. Its independently owned
  episode checkpoints require no Sprint 5 dependency and cannot block Sprint 5.
- Run WP-17 only after WP-09, WP-10, and WP-16 are terminal.
- Run WP-18 only after WP-08 through WP-13, WP-14, WP-15, and WP-16 are
  terminal.
- Run provider-neutral proof only after protocol and birthday dependencies.
- Hand completed proof producers to WP-20 under release-tail sprint #5856.
- Finalize public claims only after WP-23 release truth and explicit operator
  authorization.
- Bind each unbound child in a dedicated FastWork worktree with the company
  code repository recorded by typed C-SDLC v2. The exact four requests are
  retained in `split-authority-bind-requests.json`; ordinary doctor is expected
  to report repository identity drift before bind, while typed bind performs
  the explicit code-repository diagnosis before any Git mutation.

## Non-Goals

- External publication from the umbrella.
- Coordinating, executing, reviewing, or closing WP-24A from this umbrella.
- Synthetic demos or topic-only publication placeholders.
- Treating a deferred validator, merged checkpoint, or private archive as
  acceptance proof for its parent issue.
