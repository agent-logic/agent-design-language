# Structured Intent Prompt

Template: 1.0.0

Issue: 298

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement anchored classification, tagged CAS, immutable recovery receipts, verified canonical construction/install, and deterministic recovery-only restart for preserved failed projections.

## Required Outcome

Typed recovery under the issue lock classifies exact failed projection state, preserves rejected evidence, installs only a complete verified recovery-owned canonical candidate, and resumes deterministically across every recovery mutation and durability boundary.

## Scope

- typed classify and recover request/result contracts
- anchored per-node identity, projection validation, tagged CAS, and failed-operation lineage
- private immutable recovery ledger and recovery-owned canonical construction/install/resume
- recovery-only deterministic failpoints and subsequent ordinary-commit proof

## Authority

- Only typed owner operations under the existing issue lock may classify or mutate recovery namespaces
- Rejected and displaced projections are retained; destructive cleanup belongs to #299
- Issue #300 owns the exhaustive integrated recovery-plus-cleanup proof matrix
- Parent #297 and issues #291, #294, and #296 remain frozen
- Child completion does not release #296; only terminal parent #297 does

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 binaries only
- Bind under /Volumes/FastWork/adl-worktrees and quarantine root staging after bind
- Serialize exclusive csdlc-v2/src/store.rs ownership
- Use canonical fresh-session UUID design and exact-head review evidence
- Publish ready with Closes #298 and stop before merge
