---
name: csdlc-v2-doctor
description: Diagnose canonical v2 state without mutation or network access.
---
Invoke `csdlc-doctor` and preserve its typed status/error category. Doctor
accepts same-repository records directly. When issue and effective `origin`
repositories differ, it accepts only the explicit `code_repository` recorded
by typed bind; an absent or mismatched split identity is
`repository_identity_drift`. Never repair silently or convert
corrupt/interrupted state into pass.

## C-SDLC v3 transition boundary

C-SDLC v3 is construction evidence only until an explicit operator-reviewed
V3-F cutover changes root authority. Continue using this v2 doctor route for
live lifecycle diagnosis; v3 projections may inform later cutover review, but
they are not pass/fail authority for current issue execution.
