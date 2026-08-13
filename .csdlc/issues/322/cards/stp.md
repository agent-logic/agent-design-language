# Structured Task Prompt

Template: 1.0.0

Issue: 322

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue 5913 only; repair adl-review read-only compatibility routing and focused tests. Do not change provider credential execution, CodeFriend product scope, #112, #298, or other active issue state.

## Deliverables

- Working or truthfully narrowed adl-review read-only command surface
- Focused regression coverage for verify-repo-contract and removed v1 lifecycle rejection
- Repo-native CodeFriend/CodeBuddy deterministic smoke route that avoids removed v1 multiplexer
- Truthful SOR/SRP evidence and exact-head review before publication

## Acceptance

1. AC-1: adl-review --help matches implemented behavior for repaired read-only review commands
2. AC-2: adl-review verify-repo-contract --review docs/tooling/examples/repo-review/good_repo_review.md succeeds and has focused regression coverage
3. AC-3: A smallest CodeFriend/CodeBuddy read-only smoke route runs without invoking the removed v1 tooling multiplexer
4. AC-4: Removed v1 lifecycle/tooling command surfaces that should stay sunset still fail closed with truthful diagnostics
5. AC-5: Focused tests and strict relevant Clippy pass
6. AC-6: Fresh exact-head review has no unresolved actionable findings before publication

## Dependencies

- Sprint 6 tooling closeout
- CodeFriend operational proof packet under .git/codefriend-operational-proof-20260812-smoke

## Inputs

- adl/src/bin/adl_review.rs
- adl/src/cli/mod.rs
- adl/src/cli/review command dispatch modules
- adl/tools/test_adl_review_compatibility.sh
- adl/tools/demo_v090_codebuddy_review_showcase.sh
- adl/tools/validate_codebuddy_review_showcase_demo.py
- docs/tooling/examples/repo-review/good_repo_review.md

## Non Goals

- CodeFriend v1 product completion
- Live provider proof execution
- C-SDLC lifecycle redesign
- Raw GitHub lifecycle mutation
- Publication, merge, or closeout without later authorization
