# Structured Intent Prompt

Template: 1.0.0

Issue: 217

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Restore #209's exact ten-file native packet as immutable provenance, produce fresh current-head Linux/macOS proof, and make that current proof machine-validatable after merge, squash, or rebase without changing production behavior.

## Required Outcome

Merged main retains the exact c640 provenance packet and a fresh #217 Linux/macOS packet; the fresh packet is machine-validatable through complete digest/provenance checks plus ancestry or protected-tree equivalence, and any protected-source drift fails closed.

## Scope

- Exact historical ten-file #209 packet retention with a machine-consumed path/digest denominator
- Executable detached-c640 historical validation with evidence overlay and original GitHub environment
- Fresh current-head Linux/macOS native proof beneath issue #217 evidence
- Issue-owned producer, workflow, retained-proof validator, and focused regressions
- Typed #217 VPP/SOR proof truth, independent reviews, and visible unmerged PR

## Authority

- GitHub Actions run 31453636709 at source c640066f284a915b638add377cc4b0a2e221e6f9 is immutable source-run evidence
- The retained source manifests and recorded digests are proof inputs; prose claims are not authority
- Ancestry is sufficient only with current protected-source equality; digest equivalence is the squash-safe alternative
- Issue #142 remains blocked until the repair is merged and terminally reconciled
- Production runtime behavior is outside this issue

## Assumptions

- none

## Operator Constraints

- Use only typed C-SDLC v2 lifecycle and GitHub-owner routes
- Keep main clean and perform tracked work only in the issue-bound worktree
- Prepare all six cards and obtain independent design review before implementation
- Keep issue and work slices small and proof-focused
- Publish the issue to a visible PR when reviewed; do not merge before operator review
- Do not use AWS, cloud resources, raw GitHub writes, or production behavior changes
