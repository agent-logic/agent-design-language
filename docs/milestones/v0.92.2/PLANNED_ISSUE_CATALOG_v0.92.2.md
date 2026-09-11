# v0.92.2 Planned Issue Catalog

Status: planned catalog with one existing issue binding. Uncreated rows remain number-free. This file does not create or authorize new GitHub issues. v0.92.1 issues #717 and #718 are predecessor inputs, not catalog rows.

| Order | Planned ID | Title | Creation boundary |
|---:|---|---|---|
| S0 | SIM-UMBRELLA | Produce one C-SDLC simplification sprint scorecard | Dedicated first sprint; coordination opens before SIM-01; closes after SIM-09 |
| S1 | SIM-01 | Read-only diagnostics and baseline journeys | Dedicated first sprint; own readiness, parallel Runtime |
| S2 | SIM-02 | One current installed command contract | Dedicated first sprint; after SIM-01 |
| S3 | SIM-03 | Typed evidence and intent-oriented commands | Dedicated first sprint; after SIM-02 |
| S4 | SIM-04 | One semantic issue transaction owner | Dedicated first sprint; after SIM-03 |
| S5 | SIM-05 | Derived cards and precise evidence invalidation | Dedicated first sprint; after SIM-04 |
| S6 | SIM-06 | Conversion rehearsal and writer fencing | Dedicated first sprint; after SIM-05 |
| S7 | SIM-07 | Produce one independent transition qualification | Dedicated first sprint; after SIM-06 |
| S8 | SIM-08 | Produce one transition operations packet | Dedicated first sprint; after SIM-07 |
| S9 | SIM-09 | Run one authorized consecutive-issue pilot | Dedicated first sprint; after SIM-08 and separate activation authority |
| 1 | WP-01 | Publish and open the v0.92.2 CodeFriend Beta 1 execution wave | Milestone setup authority |
| 2 | CF-SHELL | Deliver one operable CodeFriend shell | After WP-01 |
| 3 | CF-ADAPTER | Portable Adapter v2 and repository ingestion | After WP-01; parallel with CF-SHELL |
| 4 | CF-EVIDENCE | Evidence identity, provenance, redaction, and retention | After CF-ADAPTER |
| 5 | CF-COG | Architecture cognition and explainable change risk | After CF-EVIDENCE |
| 6 | CF-GOV | Fitness functions and CI governance | After CF-EVIDENCE; parallel |
| 7 | CF-REVIEW | Produce one complete multi-perspective review packet | After CF-EVIDENCE; parallel |
| 8 | CF-MEMORY | Longitudinal memory and second-run comparison | After CF-EVIDENCE; parallel |
| 9 | CF-UX | Produce one governed publication bundle | After CF-SHELL and CF-EVIDENCE |
| 10 | CF-PROOF | Produce one Beta 1 qualification evidence packet | After analysis/publication tracks |
| 11 | CF-INTEGRATE | Complete Beta 1 integration and qualification | After all product tracks, PLAT-PROVIDER and PLAT-MEMORY |
| 12 | PLAT-PROVIDER | Config-driven provider definitions | After WP-01 and merged v0.92.1 issue #622 |
| 13 | PLAT-MLX | Bounded MLX and Apple Metal provider adapter | After PLAT-PROVIDER |
| 14 | PLAT-PAIR | NVIDIA PAIR multi-node local-inference experiment | After PLAT-PROVIDER |
| 15 | PLAT-UTS | UTS standardization and productization | After WP-01 |
| 16 | PLAT-RUST | Milestone Rust reduction slice | After WP-01 |
| 17 | OPS-AWS | Produce one current AWS inventory packet from completed #484 baseline | After WP-01 |
| 18 | OPS-GCP | Produce one apply-ready company GCP move-in execution packet | After WP-01 and merged v0.92.1 GCP foundations |
| 19 | PUB-MEDIUM | Prepare one Medium article packet | After WP-01 |
| 20 | PUB-CSDLC | Advance one C-SDLC paper packet | After WP-01 |
| 21 | PLAT-MEMORY | Bounded Memory Palace production integration | After CF-EVIDENCE and CF-MEMORY |
| 22 | SPEC-RETEST | Speculative-decoding requalification | After WP-01 |
| 22c | OBS-LIVE (#720) | Remove retained-mode live demo hazards | Reuse existing #720; independent of CodeFriend build |
| 22d | OBS-S3 | Deploy the existing Observatory S3 and CloudFront sidecar | After WP-01 and OBS-LIVE; consume completed #679 / merged PR #685 |
| 22e | ARCH-ADR | Generate and reconcile the ADRs required by v0.92.2 | After WP-01; operator acceptance remains explicit |
| 23 | TAIL-01 | Quality gate | After CF-INTEGRATE and the declared release-gating support set; excludes OBS-S3 and ARCH-ADR |
| 24 | TAIL-02 | Documentation review and external-review handoff | After TAIL-01 |
| 25 | TAIL-03 | Publication finalization | After TAIL-02 |
| 26 | TAIL-04 | Internal milestone review | After TAIL-03 |
| 27 | TAIL-05 | External or third-party review | After TAIL-04 |
| 28 | TAIL-06 | Findings remediation or explicit deferral capture | After TAIL-05 |
| 29 | TAIL-07 | Next-milestone planning | After TAIL-06 |
| 30 | TAIL-08 | Next-milestone closeout planning | After TAIL-07 |
| 31 | TAIL-09 | Next-milestone planning review | After TAIL-08 |
| 32 | TAIL-10 | Release ceremony and milestone close | After TAIL-09 |

WP-01 reuses existing #720 and creates only the separately authorized unassigned child rows after reconciling its own conductor identity. This includes one issue for the `OBS-S3` deployment sidecar and one for the `ARCH-ADR` work package; neither is created by #525. Closed merged #717/#718 are consumed here as v0.92.1 predecessor capabilities. Other backlog is excluded. CodeFriend new-wave creation retains its prior-milestone closure gate. The operator-selected SIM sprint can launch first through its dedicated authority and own readiness, without WP-01 or unrelated closeout. Creating its ten issue identities is a separate launch operation; #525 creates none.

Each catalog row owns one primary result. Lists of supporting artifacts or proof do not authorize additional independently valuable work; WP-01 must split any row whose execution contract cannot preserve that boundary before creating its issue.

## Deferred, Not Missing

Jira, Linear, Slack, broad Workspace integrations, autonomous mutation, public customer-scale or multi-tenant deployment, security tournaments, ATE, OCI packaging, optional OpenRewrite/modernization, and Runtime v4 are intentionally outside this catalog. The bounded static Observatory sidecar is admitted only through `OBS-S3`; NVIDIA PAIR and GCP move-in residuals are admitted only through `PLAT-PAIR` and `OPS-GCP`.
