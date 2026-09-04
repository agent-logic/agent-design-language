# Structured Task Prompt

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One bounded typed v2 bind-owner adoption route for verified pre-existing emergency issue branches/worktrees, plus focused regression tests and operator documentation.

## Deliverables

- Typed v2 request/result contract for adopting a verified pre-existing issue branch and worktree
- Bind-owner implementation that distinguishes safe adoption from ordinary new-worktree creation and unsafe topology
- Durable adoption evidence with pre-state, exact adopted HEAD, base relationship, issue identity, worktree path, branch, actor, generation, and digest
- Focused positive and negative regression coverage for the #660-shaped dead end and subsequent normal lifecycle eligibility
- Operator documentation for the emergency recovery command sequence and stop conditions

## Acceptance

1. AC-1: A ready-phase issue with a unique issue-named branch and registered or verifiably adoptable FastWork worktree can be adopted through one typed request without modifying commit history or tracked content.
2. AC-2: Adoption requires exact expected issue, repository, branch, worktree, HEAD SHA, base branch, generation, and digest; stale or mismatched input fails closed.
3. AC-3: Adoption rejects main, a branch/worktree owned by another issue, multiple matching worktrees, unexpected dirty state, missing base ancestry, and conflicting existing typed bindings.
4. AC-4: Successful adoption records durable machine-readable evidence and advances only from ready to bound; it does not claim implementation, review, publication, or merge readiness.
5. AC-5: The adopted issue can then use the ordinary typed PVF finalization, exact-head review, and csdlc-publish routes without manual lifecycle edits.
6. AC-6: Existing ordinary bind/create behavior remains unchanged.
7. AC-7: Focused positive and negative tests reproduce the prior dead end and prove recovery without weakening publication or exact-head review gates.
8. AC-8: Operator documentation gives a short recovery command sequence and makes clear that emergency product actions do not themselves grant lifecycle authority.

## Dependencies

- Issue #660 reproduction evidence supplied by Worker #3.1
- Current C-SDLC v2 bind, store, review, and publication gates
- Gate 10D2 typed v2 authority

## Inputs

- agent-logic/agent-design-language#665
- csdlc-v2/src/bind.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-bind.rs
- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/tests/**

## Non Goals

- Publishing directly from ready or bound
- Skipping PVF finalization, exact-head review, or publication checks
- Importing arbitrary branches, adopting main, or bypassing issue ownership
- Reimplementing Git operations outside the typed bind owner
- Changing the #660 product fix or repeating its live AWS mutation
- Broad lifecycle redesign
