---
name: csdlc-v2-doctor
description: Diagnose canonical v2 state without mutation or network access.
---
C-SDLC v2 remains the live lifecycle authority only until explicit V3-F/#505
cutover. Before that cutover, `csdlc-v3/**` is non-authoritative construction
evidence and must not mutate lifecycle state or replace v2 doctor authority.

Invoke `csdlc-doctor` and preserve its typed status/error category. Doctor
accepts same-repository records directly. When issue and effective `origin`
repositories differ, it accepts only the explicit `code_repository` recorded
by typed bind; an absent or mismatched split identity is
`repository_identity_drift`. Never repair silently or convert
corrupt/interrupted state into pass.
