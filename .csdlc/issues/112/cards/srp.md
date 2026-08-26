# Structured Review Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/layer8_authority/audit.rs
adl-runtime-kernel/src/layer8_authority/exchange.rs
adl-runtime-kernel/src/layer8_authority/identity.rs
adl-runtime-kernel/src/layer8_authority/mod.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/layer8_authority.rs
.csdlc/issues/112
.csdlc/prepared/issues/112
.csdlc/evidence/112

## Prompts

- Does the exact reviewed work identify canonical issue/title [v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core and avoid #265/#270/#271/#114 claims?
- Can any grant bypass authenticated identity, credential generation, capability, agent policy, Polis policy, replay freshness, recipient validity, canonical signatures, or audit availability?
- Can signed requests or acknowledgements be replayed, substituted, generated for unknown recipients, or accepted across stale identity/key generations?
- Can recipient substitution, recipient-set widening, action or conversation scope escalation, implicit broadcast, or cross-Polis attempts produce a grant?
- Can audit records or public refusals leak private keys, content, raw provider payloads, private policy, provider output, or private cognition?
- Does the exact diff avoid absorbing #265 Runtime ingress, #270 acknowledgement/API protocol, #271 Observatory UI, durable history, rooms, roster, presence, or sibling issue work?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Inspection-only exact-head source review; reviewer did not rerun tests or broad proof. Implementation-session evidence records the focused Layer 8 authority test target, rustfmt check, strict Clippy, diff hygiene, and typed validation at the reviewed current-main-inclusive head.
- PASS is limited to #112 / 112.a shared Layer 8 signed authority core and does not approve #265 kernel ingress enforcement, #270 served trusted acknowledgement Runtime API protocol, #271 Observatory UI, #114 durable history/integration, publication, merge, or closeout.
- Durable history, Observatory UI, served API/ack protocol, and kernel ingress remain nonclaims for this #112 core review scope.

## Review Result

Revision: Some("git-blake3:ff5e131caf2ab5e7f1cded1d94715df6fe6ea292:166e9e0564835c1dca8cf0950f64d3f7c90c7bae142d319e33282ac2717875fc")

Reviewer: Some("fresh-session:3daea360-e36e-462b-9de9-d58f7e38ba58")

Result: pass
