# Structured Task Prompt

Template: 1.0.0

Issue: 209

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and merge only the blocking production ACIP dispatch, replay-isolation, pressure/error, and public-contract repair required by #5834.

## Deliverables

- Production Runtime API dispatch and bounded pressure behavior
- Scoped replay-state implementation and adversarial regressions
- OpenAPI signature parity repair
- Focused production WSS integration proof
- Issue-local native workflow/validator and retained exact evidence
- Fresh independent exact-head review

## Acceptance

1. AC-1: A real admitted ACIP request dispatches one production Guardian/kernel operation and returns the declared typed success response.
2. AC-2: Invalid dispatch and bounded queue pressure return typed errors without echo-only substitution, secret leakage, or unrelated replay-state mutation.
3. AC-3: Replay sequencing is namespaced by authenticated principal plus an unambiguous runtime/source replay domain with bounded progression and credential-rotation cleanup.
4. AC-4: u64::MAX, stale, duplicate, concurrent rollback, delimiter collision, reconnect, cross-principal, cross-session, and capacity cases fail closed without denying unrelated valid traffic.
5. AC-5: The canonical public kernel OpenAPI exactly describes bearer-authenticated binary dispatch and structured completion/rejection; the removed legacy schema is not public, while its retained admission regression separately requires a non-null control signature.
6. AC-6: Focused production integration, replay adversarial tests, OpenAPI contract parity, legacy signed-admission proof, strict Clippy, formatting, and exact native proof pass.
7. AC-7: Fresh independent exact-head review has no unresolved actionable findings before publication.
8. AC-8: A qualified closing PR merges through typed finish and becomes ancestral before #5834 resumes.

## Dependencies

- Merged PR #76 / issue #5832 as defect baseline
- Retrospective review by /root/sprint4_5857/review_5832_retrospective_exact_head
- Current origin/main Runtime API, Guardian/kernel, ACIP, and auth contracts
- Blocked Birthday review packet #5834

## Inputs

- adl-runtime/src/runtime_api.rs
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/tests/runtime_api_wss.rs
- adl-runtime-kernel ACIP and Guardian/kernel operation APIs
- docs/api/runtime-v3/v1/acip.openapi.json
- .csdlc/evidence/5832/acip-native-receipts.json

## Non Goals

- Reopening or rewriting merged PR #76 or #5832 records
- Broad transport or Guardian architecture redesign
- Cloud provisioning or unrelated native platforms
- Birthday review-packet product changes
- Public launch, governance, Sprint 3, or closeout cleanup
