# Sprint 2 execution acceptance — ADL #1299

Status as observed 2026-10-20: **both child outcomes delivered; final umbrella review and closeout pending**. This is the current review index. Earlier preparation decisions remain historical in README, REVIEW and ACTIVITY.

## Requirements and evidence

| Requirement | Observed result | Retained evidence |
| --- | --- | --- |
| Prepare sequential execution and obtain Planning #12 review | Preparation accepted; later execution authorized with Worker #30 | [Preparation review](REVIEW.md), [current sequencing interpretation](VALIDATION_CONTRACT.md) |
| Accepted CodeFriend #120 assessment | Operator accepted assessment; PR #131 merged; native finish delivered | [Assessment](evidence/codefriend-120-reviewed-assessment.json), [finish](evidence/codefriend-120-terminal-finish.json) |
| Supported Runtime baseline authority | Runtime #189 delivered through PR #190; qualified package selected | [Finish](evidence/runtime-189-terminal-finish.json), [prerequisite](evidence/runtime-evaluator-prerequisite.json) |
| Concrete CodeFriend security enforcement | Candidate 2734eff484d25ac76e807209e01e348804eaf9d0; native proof and all CodeFriend CI jobs pass; draft PR #132 remains separate from broader #127 closure | [Native proof](evidence/codefriend-127-273-native-proof-summary.json), [CI](evidence/codefriend-127-273-ci-success.json) |
| Authenticated installed Linux consumer qualification | 34 distinct consumer scenarios PASS with exact CodeFriend and Runtime packages; four loopback model calls, zero external provider calls | [Final-head Linux report](evidence/security-9-final-linux-qualification.json), [report provenance](evidence/security-9-final-linux-provenance.json), [input pin](evidence/codefriend-127-linux-input-pin.json) |
| Full enterprise-security #9 delivery | PR #18 final head da372408a1035ea4c71d975bf862fe3f1a8bc1f5 has successful CI run 39489599848; independent integrated review passed; user explicitly approved delivery; PR merged as 644454c87ff8fdd1597168e654fc02950f021448 and #9 closed; native finish verified delivered | [Native finish](evidence/security-9-terminal-finish.json), [Integrated review](evidence/security-9-integrated-review.json), [final-head judgment](evidence/security-9-final-review-judgment.json), [native review](evidence/security-9-final-native-review.json), [final CI](evidence/security-9-final-ci.json), [PR #18](https://github.com/agent-logic/agent-logic-enterprise-security/pull/18) |
| Rendering pipeline assessment and issues for Claude | Six new #130 examples inspected read-only; timings and issues retained; final templates remain in design | [Claude handoff](CLAUDE_TEMPLATE_REVIEW.md), [measurements](evidence/new130-sample-measurements.json) |
| Final Planning #12 sprint acceptance and umbrella closeout | Child outcomes and residual routing accepted by Planning #12; umbrella exact-head review and native closeout pending | No completion claimed |

The security matrix comprises 18 tooling, 9 adapter, 17 Runtime/adapter and 34 installed-consumer cases. Its installed report proves tenant and repository denial before and after export, revocation, replay/order handling, empty service/paired grants, configured failure, redacted audit access, and deletion of actual operation snapshots, reports, publication bundles and Markdown exports. Local unit proof supplies key rotation, chain tamper/removal/reordering, partial deletion and lease behavior; do not describe every unit invariant as independently exercised in the 34-case installed matrix.

## Scope and residual work

Only new #130 templates are selected for launch. Existing Markdown templates in security tests are diagnostic fixtures, not template acceptance. #130 was not edited. Remaining template completion, final PDF/other-format qualification and rendering improvements stay routed through #121/#130 and launch acceptance; the reviewed sequencing does not make all final templates a new Sprint 2 predecessor.

The deletion contract covers the operation-owned content tree. Client ingestion checkouts, other operations, external provider copies and backups are outside it; paired-copy production is not activated by this slice. Audit and identity metadata remain under the declared retention contract. Broader #127 integration and downstream security #10/#12/#14/#15 closure remain separate.

Runtime #189 cleanup is separate from delivered status and previously retained dirty evidence. No cleanup, deployment, paid-provider execution, release approval or final-template acceptance follows from a passing CI run.

## Final review boundary

The operator explicitly approved security PR #18 merge and #9 closure after the approval-review rejection. Native delivery and finish now passed. Planning #12 reviewed evidence and confirmed the single missing-link finding resolved. Final umbrella cards and acceptance remain required.

Native #9 finish is retained. Planning #12 should now review this requirement/evidence mapping, actual delivery receipts, residual routing and the final umbrella cards. Resolve actionable findings before the umbrella closing PR. Preparation approval and implementation qualification alone do not authorize a completed-sprint claim.

Validation for this index: current source reports and PR state inspected; local Markdown links checked. No product tests rerun for this documentation index. Pending delivery/review is intentionally recorded as a gap.

Planning #12 evidence review identified one missing direct review-evidence link. Corrected by retaining the integrated report, final-head judgment and native review receipt above. Integrated Linux proof reviewed source `efbc1e33e114b5c7b32baa78dd234a98cd812acc`; final evidence changes were reviewed at `da372408a1035ea4c71d975bf862fe3f1a8bc1f5`. Final CI and installed report separately bind that final head. This correction does not imply terminal sprint acceptance.

Current native card projections are `.csdlc/v3/issues/1299/cards`. Earlier `.csdlc/issues/1299/cards` copies are preparation history and are not current execution authority. Planning #12 accepted both child outcomes and residual routing after security finish; its card-path finding was corrected through native edits to canonical projections.
