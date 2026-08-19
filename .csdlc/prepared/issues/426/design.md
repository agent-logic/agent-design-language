# Issue #426 — Linux service control for CSMctl

## Goal

Extend the merged `CSMctl` control surface with an explicit Linux backend so
`start_CSM.sh` can control Runtime on the Amazon Linux x86_64 host required by
#268, while preserving the existing launchd behavior on Darwin.

## Design

Keep parsing, configuration, probes, URLs, and logs shared. Select a backend
from the detected operating system. Darwin retains launchd. Linux uses a
bounded background-process backend with a private PID file and detached
streams, avoiding an assumption that systemd is PID 1 on qualification hosts.
It validates PID ownership before signaling, refuses stale or foreign PID
files, uses bounded TERM then KILL shutdown, and fails closed on unsupported
operating systems. Test overrides are accepted only under explicit test mode.

## Scope

- `CSMctl` and `start_CSM.sh`
- focused isolated launcher tests
- Linux operator documentation

## Non-goals

- Runtime, Observatory, continuity, or provider redesign
- AWS qualification execution or #269
- host-wide systemd installation

## Validation

Shell syntax, fixture-backed Linux lifecycle, Darwin selection compatibility,
unsupported-platform rejection, foreign-PID denial, Gemini review, and diff
hygiene.
