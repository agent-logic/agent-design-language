# Structured Task Prompt

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and merge only the blocking WP-13 trusted-authority, full-lineage, and rotation repair with focused evidence; closeout bookkeeping remains asynchronous.

## Deliverables

- adl-runtime-kernel/src/cognitive_profile.rs trusted authority, full-chain, and rotation contract
- adl-runtime-kernel/tests/cognitive_profile.rs focused positive and negative proof
- adl-runtime-kernel/tests/fixtures/cognitive_profile issue-owned fixtures
- docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md corrected feature truth
- .csdlc/prepared/issues/144 native producer and validator
- .github/workflows/wp13-authority-repair.yml narrow native workflow
- .csdlc/evidence/144 exact local and native proof

## Acceptance

1. AC-1: Profile creation and update require a signed statement from a provisioned cognitive authority binding exact canonical policy and evidence digests.
2. AC-2: Caller-invented keys, policy, evidence, authority identifiers, epochs, or signatures fail closed without leaking untrusted values.
3. AC-3: Every predecessor through genesis is recomputed and verified for canonical input, profile, public projection, authority context, and exact predecessor linkage.
4. AC-4: Truncated, substituted, syntactically valid rehashed, or deep forged revision chains fail closed.
5. AC-5: Authority rotation requires a signed transition from the trusted current context to a new key/context with a strictly monotonic epoch.
6. AC-6: Stale, wrong-key, discontinuous, replayed, or self-signed rotation fails closed and does not rewrite older history.
7. AC-7: Existing privacy, nonclaim, evidence-category, projection, and deterministic replay boundaries remain intact.
8. AC-8: Exact nonzero focused tests, strict Clippy, formatting, native Linux/macOS receipts, semantic equivalence, typed validation, and independent exact-head review pass.
9. AC-9: A ready PR with Closes #144 merges before legacy issue #5831 is rebased or published.

## Dependencies

- Merged PR #139 / legacy issue #5830 as defect baseline
- Current origin/main Runtime v3 cognitive_profile, governance, identity, and continuity authority
- Issue #144 live GitHub contract
- Legacy issue #5831 remains blocked until this repair is merged

## Inputs

- adl-runtime-kernel/src/cognitive_profile.rs
- adl-runtime-kernel/src/governance.rs
- adl-runtime-kernel/src/identity.rs
- adl-runtime-kernel/src/continuity.rs
- adl-runtime-kernel/tests/cognitive_profile.rs
- docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
- .csdlc/issues/5830 and .csdlc/evidence/5830 as read-only historical evidence

## Non Goals

- Adaptive-learning implementation or edits to #5831
- Broad Runtime v3 governance or identity redesign
- Global CI changes or reuse of stale #5830 receipts as current proof
- Sprint 3 work or post-merge closeout cleanup
- Diagnosis, reputation, standing, rights, citizenship, personhood, consciousness, or final Birthday completion
