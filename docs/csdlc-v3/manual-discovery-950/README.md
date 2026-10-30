# Manual discovery and active-session notice (#950)

Follow-up to #861 / merged #947. Root AGENTS now points new sessions to the
installer, actual manual root and workflow entrypoint.

## Correct default destination

The owner installer remaps its default `.adl/bin` to `.adl/bin/native-v3` for
`--bin csdlc`. The resulting manual root is `.adl/bin/native-v3/share/man`.
The initial #950 issue example omitted that remap; this implementation corrects
it. A custom destination uses its own `share/man`; the installer prints the
resolved MANPATH command. No shell profile or shared owner binary was changed.

## Reviewed versus installed

Reviewed source: #947 at `4e816f87d61d11f48c950ededae75c6ca84f6258`, thirty pages.
At this session's observation, the primary checkout had no installed pages at
its default native-v3 manual destination. `man -w csdlc-workflow` under that
MANPATH did not resolve a page. Other shells, custom destinations and hosts are
unknown; this observation does not identify their installed manuals or binaries.

## Notification coverage

A one-time message is sent to the other active ADL Codex tasks discoverable in
the current application inventory. The sending Worker #9 already has the notice.
Not-loaded tasks are not treated as active, and external Claude/terminal
sessions are not enumerable through this channel. Their delivery is unverified;
the operator can forward the retained message. Delivery records mean accepted
by the task channel, not proof of human reading or successful installation.

See `notification.md` and `validation.json`. Local session IDs and tool delivery
receipts remain in issue-local evidence rather than the public documentation.

PVF: docs_only; isolated local manual install, man lookup/render, source-byte
comparison, and diff hygiene. #947 owns full installer/manual/platform parity;
this follow-up makes no new Linux or runtime proof claim.
