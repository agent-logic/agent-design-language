# Structured Intent Prompt

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Extend the secure #191 transport with one #201-authorized replication-only learner route and one shared durable pending-exclusion authority without weakening the exact voter cut or granting learner authority.

## Required Outcome

The unchanged authenticated voter cut may add at most one exact token-bound learner that can receive only canonical AppendEntries and InstallSnapshot traffic; pending exclusion is durably shared across #201 eligibility and ordinary sessions, while a separately governed rejoin may receive replication only.

## Scope

- Verified current-voters-plus-one-governed-learner topology
- Role-bound authenticated learner session and RPC allowlist
- Shared crash-reconciled PendingMembershipExclusionAuthority
- Narrow #201 signer/finalization eligibility consultation
- Ordinary-voter denial and exact replication-only rejoin exception
- Reconnect, rotation, replay, bounds, restart, and path-safety proof

## Authority

- The #191 voter cut remains exact and unchanged; learner admission never enters a voter configuration or quorum
- Only an opaque durably published #201 EnrollNonVoting token can construct a learner admission
- Only an opaque #201 RemoveVoter token can activate pending exclusion
- The learner may receive AppendEntries and InstallSnapshot only and cannot vote, endorse, finalize, renew, mutate, act as Shepherd, or own/serve Observatory
- A recovery learner requires a separate current token, certificate, boot generation, operation namespace, and catch-up boundary
- Joint/final membership and promotion belong to #199; concrete stores belong to #200

## Assumptions

- none

## Operator Constraints

- Do not bind or edit product source until PR #197 and #201 are externally reviewed, merged, and ancestral
- Extend the existing transport authority; do not build a parallel Quinn/OpenRaft stack
- Keep the issue bounded to learner/exclusion transport and narrow #201 eligibility consultation
- Run fresh independent exact-head review before publication
- Open a ready PR for visibility but never merge before operator review and authorization
- No AWS use and no lifecycle closeout
