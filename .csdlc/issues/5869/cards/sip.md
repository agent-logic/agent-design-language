# Structured Intent Prompt

Template: 1.0.0

Issue: 5869

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement monotonic epochs and bounded leases as prerequisites for distributed ownership decisions.

## Required Outcome

Implement OpenRaft committed authority whose certificates satisfy a stable majority or both constituent majorities during joint membership and reject insufficient union majorities. AuthorityCertificateV1 must hash its canonical body under ADL-AUTHORITY-CERTIFICATE-BODY-V1\0; each AuthorityEndorsementPayloadV1 must sign that body digest plus signer identity, certificate generation, and algorithm under ADL-AUTHORITY-ENDORSEMENT-V1\0 using Ed25519 verify_strict; enrollment and verification must reject duplicate effective control keys; and activation possession, leases, and mutation sinks must follow #5821.

## Scope

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs

## Authority

- Issue 5869 exclusively owns the declared paths
- WP-04-IMP issue 5862 coordinates only
- WP-04.16 alone owns final module registration
- No sibling, Runtime v2, or v0.93 authority

## Assumptions

- none

## Operator Constraints

- Do not start before #5821 is terminal
- Bind only the exact exclusive paths
- Use nonzero exact test selection
- Fix all actionable pre-PR findings
