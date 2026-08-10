# Structured Intent Prompt

Template: 1.0.0

Issue: 137

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Register the bounded GitHub Actions workflow that produces and aggregates WP-04 native proof receipts on Linux, macOS, and Windows.

## Required Outcome

One read-only, pinned, fail-closed workflow checks out an exact commit, runs the existing #5878 producer on three operating systems, retains distinct fragments, and validates their aggregate on Ubuntu.

## Scope

- .github/workflows/wp04-native-distributed.yml
- Issue-local C-SDLC lifecycle and proof records for #137

## Authority

- The workflow only orchestrates existing #5878 producer and validator paths; it does not alter their logic or evidence claims
- Manual dispatch authority is limited to an explicit 40-character lowercase hexadecimal commit SHA
- Hosted jobs retain read-only repository permissions and pinned action revisions
- A successful workflow requires all three native producers, live hosted-run and job attestation, and the aggregate validator

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Work only in a bound FastWork worktree
- Modify no #5878-owned file other than copying its proposed workflow into this issue-owned workflow path
- Merge the exact reviewed green head as soon as CI is green
- Do not delay for asynchronous closeout bookkeeping
