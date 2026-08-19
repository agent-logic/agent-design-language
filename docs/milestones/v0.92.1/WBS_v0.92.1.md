# Work Breakdown — v0.92.1

| ID | Work package | Exit condition |
|---|---|---|
| REP-01 | Repository authority / #432 | No tracked dependency on local untracked paths; guardrail proof passes |
| WP-01 | Milestone planning / #431 | Complete reviewed package, issue wave, feature plans, and handoff |
| CORP-01 | Corporate and IP | Four ordered packages (IP/provenance; custody/recovery; operational transfer; diligence close) reviewed without private-data leakage |
| V3-01 | C-SDLC v3 | All #161-#180 predecessor requirements reconciled into six typed executable packages |
| DRT-01 | Distributed multi-agent Runtime / #345 | GPU Shepherd hardening followed by governed multi-agent UTS qualification with continuity and truthful receipts |
| POD-01 | Podcast publication and Studio | Operator-ready identity, feed, episode, Studio, and publication evidence chain |
| HOT-01 | Axum configuration hot reload | Validated atomic last-known-good reload with failure and concurrency proof |
| OBS-01 | Observatory redesign / #251 / #122 / #84 | TLS 1.2, public Route53/ACM exposure, Unity readiness, accessible authentic-data redesign, and bounded implementation |
| INT-01 | Release-tail root | All six lane roots terminal before the canonical serial tail starts |
| TAIL-01 | Quality gate | Required checks pass or every exception is explicitly owned and dispositioned |
| TAIL-02 | Docs and release-truth pass | Repository, feature, release-note, and milestone truth agree |
| TAIL-03 | Publication finalization | Publication artifacts and claims are finalized against landed evidence |
| TAIL-04 | Internal review | Findings-first internal review is complete |
| TAIL-05 | External / third-party review | Independent review is complete against the stabilized package |
| TAIL-06 | Findings remediation | Accepted findings are fixed or explicitly deferred with owner and milestone |
| TAIL-07 | Next-milestone planning | v0.92.2 CodeFriend Beta 1 package is ready before closeout |
| TAIL-08 | Next-milestone closeout planning | Exact terminal issue, PR, receipt, and ceremony sequence is reviewed |
| TAIL-09 | Next-milestone planning review | v0.92.2 planning and closeout readiness receive an independent review |
| TAIL-10 | Release ceremony | Final validation, notes, tag, cleanup, and milestone closeout are complete |

REP-01 precedes WP-01. The six lane roots depend on WP-01 and otherwise run independently. INT-01 consumes their terminal outputs. The release tail is strictly serial in the preceding-milestone order TAIL-01 through TAIL-10; later tail state never authorizes an earlier missing gate. #431 owns the planning-time v0.92.2 CodeFriend Beta 1 handoff, and TAIL-07 refreshes it against delivered milestone truth.

Closed predecessor issues remain fully represented without being reopened: corporate `#153`-`#160`, C-SDLC v3 `#161`-`#180`, distributed Runtime `#181`-`#187`, and integration `#188`-`#190`. Existing issues `#251`, `#122`, `#84`, and `#345` are active v0.92.1 execution rather than deferred backlog.
