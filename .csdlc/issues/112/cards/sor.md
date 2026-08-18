# Structured Output Record

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Recovered after fresh-session:e05b2801-0ba6-4c49-a30b-d8e0f3f5c0b6 found a P1 empty-recipient grant path, fixed the Layer 8 authority core so every recipient-scoped action refuses empty recipient sets before scope matching, and expanded focused tests to cover all non-AddressRecipients action variants.

## Artifacts

- adl-runtime-kernel/src/layer8_authority/audit.rs
- adl-runtime-kernel/tests/layer8_authority.rs
- .csdlc/issues/112

## Execution

- Recovered the failed e05b review assignment through typed csdlc-review recover before mutating source.
- Changed Layer 8 authority scope matching to reject empty request recipient sets before exact or subset recipient checks for all actions.
- Added focused regression coverage for empty-recipient Discover, Contact, Continue, and Attach requests; Attach carries an attachment id so the refusal proves recipient validation rather than missing attachment validation.
- Preserved exact AddressRecipients matching and the existing Attach missing-attachment refusal.
- Kept #265 Runtime ingress, #270 served acknowledgement/API protocol, #271 Observatory UI, #114 durable history/integration, and sibling work outside the #112 core diff.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "layer8_authority",
      "--",
      "--nocapture"
    ],
    "purpose": "Focused Layer 8 signed authority core regression target, including empty-recipient refusal for all non-AddressRecipients variants.",
    "outcome": "passed",
    "evidence_ref": "local-command:5 tests passed; layer8_authority_core_rejects_empty_recipient_non_address_scopes included"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Verify runtime-kernel Rust formatting after the empty-recipient fix.",
    "outcome": "passed",
    "evidence_ref": "local-command:exit 0"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict warning-free proof for runtime-kernel crate and tests after the empty-recipient fix.",
    "outcome": "passed",
    "evidence_ref": "local-command:exit 0"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify whitespace and conflict-marker hygiene for the empty-recipient fix diff.",
    "outcome": "passed",
    "evidence_ref": "local-command:exit 0"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- Canonical issue identity/title for #112 core publication gate is [v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core. Supported post-recovery card repairs updated SIP, STP, SPP, SRP, and this SOR truth; VPP/card-identity title repair remains a bounded C-SDLC v2 tooling follow-on candidate because implemented-phase typed editors do not expose a direct card identity/title edit route, and publication/fresh-session review remain held until exact cross-card identity is consistent or the canonical standard explicitly permits the historical label.
