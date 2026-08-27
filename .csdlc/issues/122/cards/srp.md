# Structured Review Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.

## Prompts

- Can any public read, browser state, origin, or unsigned request gain Runtime write authority?
- Do the exact deployed Observatory and Runtime gateway revisions match through DNS, cache, HTTPS, and WSS paths?
- Are CORS, CSP, WSS origins, authentication, rate limits, redaction, health, and error responses fail-closed and public-safe?
- Does every resource belong to the verified Agent Logic business account with bounded ownership, rollback, and cleanup?
- Can any plan or tool create or operate EC2, Spot, or CodeBuild, or begin without separate operator authorization?
- Does #122 remain deferred beyond v0.92 and non-gating for #83 and #111-#117?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not wait for CI, publish, merge, mutate lifecycle state, or perform live AWS actions.
- Reviewer treated a65de5b32c81c8c5247e82aa4a4451b1ea8efe4e as review-assignment metadata over substantive revision d30dd3d11503a2a89cf035a040606f90e991f15a.
- Live WSS handshake remains explicitly gated on a separate approved Runtime probe; #122 proof does not claim that handshake.

## Review Result

Revision: Some("git-blake3:d30dd3d11503a2a89cf035a040606f90e991f15a:86e2de81bcc4d4984e499069ee2678d2f0cd7f284ff2d555330efcb1f2c0bc41")

Reviewer: Some("fresh-session:ab99c632-e4c0-4e40-9f6d-76557dc80ef7")

Result: pass
