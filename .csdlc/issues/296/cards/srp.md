# Structured Review Prompt

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/296

## Prompts

- Review every acceptance criterion with code, security, test, and evidence coverage, emphasizing lifecycle authority, stale approval invalidation, artifact TOCTOU and path safety, atomic SPP/VPP parity, append-only history, and exact fresh-review gating.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review distinguished substantive revision dcb9f68a806158899c2cf1334a4d0aede97a221e from later assignment-only metadata commit 848c48fc56be3e12161134e90536a944975c8016.
- Reviewer reran only the focused exact issue-local authored artifact test and relied on recorded r6 gate5, card_identity, lib, and clippy evidence for the broader validation set.

## Review Result

Revision: Some("git-blake3:dcb9f68a806158899c2cf1334a4d0aede97a221e:4a0b79ef159002d77edd8c4626ce1b70ab0e24cf10815c6098e09b3c749c4b55")

Reviewer: Some("fresh-session:646518a3-5255-482a-89e8-0cb9f9f37dc3")

Result: pass
