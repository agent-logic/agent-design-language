# Scope And Evidence Index

Issue: #5403
Status: ten sprint packets, canonical register reconciliation, and refreshed
independent review complete

The complete child-to-PR closure chain is retained in
`CHILD_PR_REVISION_MATRIX.md`. Specialist lane coverage and disagreement truth
are retained in `SPECIALIST_COVERAGE.md`.
The canonical status reconciliation is retained in
`docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md` after #5383
recorded terminal merged closeout and released that protected path.

## Sprint Scope

| Sprint | Review class | Current packet status |
| --- | --- | --- |
| #4639 WP-12 | review-quality upgrade | review complete; remediation #5404/#5406 |
| #4640 WP-13 | full review | review complete; remediation #5405/#5406 |
| #4648 WP-21 | full review | review complete; records remediation #5406 |
| #5036 tools reliability tail | full review | review complete; remediation #5407/#5406 |
| #5045 WP-07 hardening follow-on | full review | review complete; remediation #5408 |
| #5121 WP-07A rearchitecture | full review | review complete; remediation #5409 |
| #5174 Runtime v3 parity | review-quality upgrade | review complete; remediation #5410/#5406 |
| #5227 Runtime v3 cutover | review-quality upgrade | review complete; remediation #5411/#5413/#5406 |
| #5247 Runtime v3 cutover readiness | full review | review complete; remediation #5412/#5406 |
| #5276 Runtime v3 parity and Observatory | full review | review complete; remediation #5413 |

## WP-12 Child And PR Inventory

| Child | Concern | Closing PR |
| ---: | --- | ---: |
| #4656 | Security and CAV requirements | #5129 |
| #4657 | SSM readiness | #5132 |
| #4658 | ACIP schema and protobuf projection | #5137 |
| #4659 | WebSocket transport | #5146 |
| #4660 | Access and activation gate | #5151 |
| #4914 | CAV red-blue runtime tactics | #5160 |
| #4917 | Tamper-evident Polis custody | #5139 |
| #4920 | Key rotation and break-glass policy | #5144 |

Primary retained evidence is under
`docs/milestones/v0.91.7/review/security/` and
`docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659/`.

## WP-13 Child And PR Inventory

| Child | Concern | Closing PR |
| ---: | --- | ---: |
| #4752 | Affect model | #5165 |
| #4753 | Godel constructability | #5171 |
| #4754 | Economics and civilization | #5185 |
| #4755 | Guild foundation | #5189 |
| #4756 | CodeFriend obligations | #5193 |
| #4757 | Publication boundary | #5197 |

Primary retained evidence is under `docs/milestones/v0.91.7/review/wp13_*`.

## Remaining Ordered Child Inventories

- #4648 WP-21: planning seed and source-capture handoff; no child issues are
  declared by the umbrella. Review must trace its actual planning outputs and
  the later #4649 review obligation.
- #5036 tools tail: #5034, #5032, #5037, #5031, #5028, #5012, #5002, #4999,
  #4995, #4987, and #4938. The operator-selected execution wave omitted #5037;
  the review must reconcile that difference explicitly.
- #5045 WP-07 hardening: #5005, #5042, #4977, #4979, #5003, #4985, #4974,
  #5040, #5039, #5041, and final gate #4906.
- #5121 WP-07A: source architecture #5068 plus #5110, #5111, #5116, #5117,
  #5112, #5113, #5118, #5124, #5125, #5122, #5123, #5119, #5126, #5115,
  #5114, and final soak #5120.
- #5174 Runtime v3 parity: #5170, #5176, #5182, #5181, #5177, #5180, #5178,
  #5183, #5179, and final guardian/soak gate #5175.
- #5227 Runtime v3 cutover: source gate #5218 plus #5225, #5219, #5222, and
  final release gate #5220.
- #5247 Runtime v3 readiness: #5248, #5249, #5250, #5251, #5252, #5253, and
  final decision #5254.
- #5276 Runtime v3 live parity: #5277, #5278, #5279, #5280, #5281, #5282,
  #5283, #5284, #5285, and Observatory gate #5286.

## Authority Notes

- GitHub state was read live on 2026-07-15.
- Child closure and PR association are inventory facts, not proof that the
  implementation or retained evidence is correct.
- Testing-discovered bugs and review-discovered findings are identified
  separately in each packet.
