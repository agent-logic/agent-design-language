# Work Breakdown — v0.92.1

| ID | Work package | Exit condition |
|---|---|---|
| REP-01 | Repository authority / #432 | No tracked dependency on local untracked paths; guardrail proof passes |
| WP-01 | Number-free milestone-opening conductor | Created by the milestone operator only when v0.92.1 is declared ready after the planning package merges; create the execution wave from the reviewed catalog |
| CORP-01 | Corporate and IP | Four ordered packages (IP/provenance; custody/recovery; operational transfer; diligence close) reviewed without private-data leakage |
| V3-01 | C-SDLC v3 | All #161-#180 predecessor requirements reconciled into six typed executable packages |
| DRT-01 | Distributed multi-agent Runtime / #345 | GPU Shepherd hardening followed by governed multi-agent UTS qualification with continuity and truthful receipts |
| POD-01 | Podcast publication and Studio | Operator-ready identity, feed, episode, Studio, and publication evidence chain |
| HOT-01 | Axum configuration hot reload | Validated atomic last-known-good reload with failure and concurrency proof |
| OBS-01 | Observatory redesign / #251 / #122 / #84 | TLS 1.2, public Route53/ACM exposure, Unity readiness, accessible authentic-data redesign, and bounded implementation |
| DEC-01 | Runtime v2/v3 decoupling | Complete source and reverse-reference census, single ownership, compatibility, migration, and rollback proof |
| PROV-01 | Provider inference profiles | PROV-A common profile/Ollama materialization, then PROV-B isolated shadow comparison |
| DRT-D | GCP six-resident qualification sidecar | Exact workload replay with provider identity, cost, and zero-resource cleanup evidence |
| INT-01 | Release-tail root | Every root in the issue wave reviewed and merged before the canonical serial tail starts; closeout remains asynchronous |
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

REP-01 and the reviewed planning-package merge make WP-01 eligible for creation; they do not trigger it. When the operator declares v0.92.1 ready, the operator creates WP-01; WP-01 then creates the remaining execution wave. Execution roots depend on WP-01 and otherwise run independently except that PROV-B follows PROV-A and DRT-D follows DRT-C. INT-01 consumes every root named in the issue wave. The release tail is strictly serial in the preceding-milestone order TAIL-01 through TAIL-10; later tail state never authorizes an earlier missing gate. #431 is closed planning provenance only; TAIL-07 refreshes the v0.92.2 handoff against delivered milestone truth.

Closed predecessor issues remain fully represented without being reopened: corporate `#153`-`#160`, C-SDLC v3 `#161`-`#180`, and distributed Runtime `#181`-`#187`. Integration provenance is exact: `#188` informs INT-01/TAIL-01, `#190` informs TAIL-07, and `#189` informs TAIL-10. Existing issues `#251`, `#122`, `#84`, and `#345` are active v0.92.1 execution rather than deferred backlog; `#457` is historical provider-profile provenance only.
