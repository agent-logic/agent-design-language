# Structured Intent Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Establish the deterministic quorum-committed authority command, opaque voter-endorsement, durable result/checkpoint, and legacy-command closure protocol that #199 and #200 consume without any leader, caller, harness, or replica-local clock self-authorizing authority.

## Required Outcome

Every published authority-operation token is deterministically derived from an exact committed intent, canonical quorum-attested time, and a strict quorum of opaque current-voter endorsements; its result and retry state are externally checkpointed, legacy direct authority commands fail closed, and no downstream membership or concrete-store side effect is claimed here.

## Scope

- Canonical prepare-and-finalize committed authority protocol
- Opaque local VoterEndorsementAuthority bound to exact current voter identity
- Canonical quorum-attested time token for deterministic replicated apply
- Durable protocol journal, exact retry cache, and external ConsensusCheckpointAuthority
- Opaque VerifiedAuthorityOperation token for downstream #199 and #200
- Legacy direct PolisCommand authority closure or versioned fail-closed replay
- Focused three-voter positive, restart, replay, rollback, and fault proof

## Authority

- Only concrete MembershipState and AuthorityMembership plus opaque distinct current-voter endorsements authorize finalization
- The leader, caller, runner, model, Shepherd, harness, local history, and replica-local clock are never quorum authority
- Raw signing keys, caller-produced endorsements, and caller-selected voter sets are rejected
- #201 emits an opaque verified token but performs no OpenRaft membership or concrete authority-store side effect
- Governed membership belongs to #199 and concrete authority-store reconciliation belongs to #200
- Kernel continuity, Guardian/API/WSS, models, AWS, live demonstrations, and #142 terminal delivery remain out of scope

## Assumptions

- none

## Operator Constraints

- Do not bind or edit product source until PR #197 is externally reviewed, merged, and ancestral
- Use typed C-SDLC v2 lifecycle commands and an issue-bound worktree
- Keep #201 bounded to the core protocol; route governed membership to #199 and concrete-store reconciliation to #200
- Run fresh independent exact-head review before publication
- Open a ready PR for visibility but never merge before operator review and authorization
- No AWS use and no lifecycle closeout
