# Structured Intent Prompt

Template: 1.0.0

Issue: 461

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime lifecycle TLS authority config-only before issue #268 resumes paid AWS qualification.

## Required Outcome

The lifecycle soak and Guardian path read validated TLS certificate, private-key, and trust-root paths only from Runtime configuration and expose no TLS path command flags.

## Scope

- adl-runtime lifecycle soak TLS argument and configuration handling
- bounded Guardian lifecycle harness
- focused config-only TLS regression tests

## Authority

- Runtime init configuration is the sole TLS path authority
- issue #268 remains blocked until this issue is merged

## Assumptions

- none

## Operator Constraints

- never expose TLS material or sensitive paths in argv, logs, or receipts
- never execute issue #269
- do not expand into certificate issuance, DNS, or CloudFormation
