# Issue844 independent review remediation

Initial exact revision: `1bdb3e783289a997bba664a93c4a4981f811a073`.
Reviewers: `review_771_core` (state machine/receipts) and `review_771_routes`
(adapter/policy/routes). Both were read-only and performed no live mutation.

| ID | Severity | Finding | Repair and focused proof |
| --- | --- | --- | --- |
| 844-REV-1 | P1 | Case/whitespace variants bypassed reviewer separation | Reuse existing same_principal normalization; all three variants rejected before network |
| 844-REV-2 | P2 | Missing/null/malformed review policy booleans acted as false | Strict boolean parsing; six missing/null/type cases reject before intent/PUT |
| 844-REV-3 | P2 | Equivalent receipt path could create a fresh uncertain attempt | Create-only per-PR target guard binds original operation; relative-path replay cannot send another PUT |
| 844-REV-4 | P2 | First-use directory ancestry was not durable | Sync every directory link through Git control before intent/dispatch; ordered traversal and injected sync errors tested |

`review-remediation-tests.log` records passing targeted proof. Independent
rereview and the refreshed full native suite remain required before publication.
These fixes preserve the documented REST base/policy race boundary and the
existing typed review receipt trust model.
