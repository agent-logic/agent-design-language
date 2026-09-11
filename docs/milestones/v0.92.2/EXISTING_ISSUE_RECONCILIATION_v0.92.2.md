# v0.92.2 existing issue reconciliation

Status: WP-01 reconciliation; no new issues created.

The authenticated complete issue inventory originally admitted three existing issues. On 2026-09-09 the operator promoted #717 and #718 into the active v0.92.1 bugfix band. They are predecessor work consumed by v0.92.2, not part of this milestone's execution denominator.

| Existing issue | Planned ID | Scheduling | Required result |
|---|---|---|---|
| [#848](https://github.com/agent-logic/agent-design-language/issues/848) | ARCH-SPLIT | Existing architecture-decision authority | Decide whether, when, and how decomposition belongs in v0.92.2; creates no implementation authority |
| [#720](https://github.com/agent-logic/agent-design-language/issues/720) | OBS-LIVE | Existing authority; independent of product build and #512 publication | Live-only Observatory without obsolete retained polling; preserve historical evidence |
| [#849](https://github.com/agent-logic/agent-design-language/issues/849) | CSDLC-MERGE | Existing authority; post-opening correctness lane | Preserve reviewed publication linkage during native PR merge |
| [#852](https://github.com/agent-logic/agent-design-language/issues/852) | QUAL-RUNTIME | Existing authority; release-gating quality lane | Complete executable proof and correlated Runtime failure events |
| [#854](https://github.com/agent-logic/agent-design-language/issues/854) | RT-COST | Existing authority; precedes PLAT-PROVIDER | Stop recurring metered inference health probes |
| [#855](https://github.com/agent-logic/agent-design-language/issues/855) | PLAT-PROVIDER | Existing authority; consumes RT-COST | Deliver config-driven provider definitions and dynamic consumption |
| [#861](https://github.com/agent-logic/agent-design-language/issues/861) | CSDLC-MAN | Existing authority; post-opening documentation lane | Complete installable C-SDLC v3 operator manuals |
| [#862](https://github.com/agent-logic/agent-design-language/issues/862) | CSDLC-DECOMPOSE | Existing authority; post-opening refactor lane | Decompose command modules without behavior change |
| [#864](https://github.com/agent-logic/agent-design-language/issues/864) | WP-01 | Existing conductor authority | Reconcile and open the milestone without creating children absent separate authorization |

#717 and #718 still overlap welcome-package and Runtime control paths, but that collision is resolved in v0.92.1. v0.92.2 consumes their merged outcomes and must not recreate either issue.

Reuse all nine bound issues exactly once. Do not create replacement issues for #717 or #718 at WP-01. Completed #620 is the first-pass document refresh; completed #439 is predecessor planning input. Both remain historical, not new execution rows. Other backlog is excluded from the admitted denominator.

The denominator is 51 rows: nine existing bindings and 42 prospective creations requiring separate authorization. WP-01 is #864 and is not its own creation target. #848 is the ARCH-SPLIT decision row; it is neither backlog nor authorization to implement a split.

Snapshot: [existing-issues.json](evidence/issue-523/existing-issues.json). Captured issue text is historical evidence, not authority to override the operator correction. State/milestone metadata is a point-in-time read and must be refreshed before execution or remote reconciliation. #523 does not implement or close these issues.

The ten SIM rows are launched through their own sprint authority independently of WP-01. #717 and #718 are completed or closed out in v0.92.1 and do not wait for SIM completion.
