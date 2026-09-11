# v0.92.2 existing issue reconciliation

Status: WP-01 reconciliation with the authorized first SIM sprint created as #866–#875; remaining tasks are unassigned.

The authenticated complete issue inventory originally admitted three existing issues. On 2026-09-09 the operator promoted #717 and #718 into the active v0.92.1 bugfix band. They are predecessor work consumed by v0.92.2, not part of this milestone's execution denominator.

| Existing issue | Planned ID | Scheduling | Required result |
|---|---|---|---|
| [#848](https://github.com/agent-logic/agent-design-language/issues/848) | ARCH-SPLIT | Existing architecture-decision authority | Decide whether, when, and how decomposition belongs in v0.92.2; creates no implementation authority |
| [#720](https://github.com/agent-logic/agent-design-language/issues/720) | OBS-LIVE | Existing authority; independent of product build and #512 publication | Live-only Observatory without obsolete retained polling; preserve historical evidence |
| [#849](https://github.com/agent-logic/agent-design-language/issues/849) | CSDLC-MERGE | Existing authority; post-opening correctness lane | Preserve reviewed publication linkage during native PR merge |
| [#852](https://github.com/agent-logic/agent-design-language/issues/852) | QUAL-RUNTIME | Existing authority; release-gating quality lane | Repair correlated Runtime failure events; retain five-row lineage through QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY and QUAL-EVIDENCE |
| [#854](https://github.com/agent-logic/agent-design-language/issues/854) | RT-COST | Existing authority; precedes PLAT-PROVIDER | Stop recurring metered inference health probes |
| [#855](https://github.com/agent-logic/agent-design-language/issues/855) | RT-PROVIDER | Existing authority; consumes PLAT-PROVIDER | Deliver the complete registered-provider dynamic agent lifecycle; configuration loading is a separate task |
| [#861](https://github.com/agent-logic/agent-design-language/issues/861) | CSDLC-MAN | Existing authority; post-opening documentation lane | Complete installable C-SDLC v3 operator manuals |
| [#862](https://github.com/agent-logic/agent-design-language/issues/862) | CSDLC-DECOMPOSE | Existing authority; post-opening refactor lane | Decompose local command modules; CSDLC-REMOTE separately owns remote decomposition |
| [#864](https://github.com/agent-logic/agent-design-language/issues/864) | WP-01 | Existing conductor authority | Reconcile and open the milestone without creating children absent separate authorization |

#717 and #718 still overlap welcome-package and Runtime control paths, but that collision is resolved in v0.92.1. v0.92.2 consumes their merged outcomes and must not recreate either issue.

Reuse all nine preexisting issues exactly once and preserve the ten new SIM bindings below. Do not create replacement issues for #717 or #718 at WP-01. Completed #620 is the first-pass document refresh; completed #439 is predecessor planning input. Both remain historical, not new execution rows. Other backlog is excluded from the admitted denominator.

The denominator is 69 rows: 19 assigned issues (nine preexisting plus ten newly created SIM issues) and 50 prospective creations requiring separate authorization. WP-01 is #864 and is not its own creation target. #848 is the ARCH-SPLIT decision row; it is neither backlog nor authorization to implement a split.

Snapshot: [existing-issues.json](evidence/issue-523/existing-issues.json). Captured issue text is historical evidence, not authority to override the operator correction. State/milestone metadata is a point-in-time read and must be refreshed before execution or remote reconciliation. #523 does not implement or close these issues.

The ten SIM rows are launched through their own sprint authority independently of WP-01. #717 and #718 are completed or closed out in v0.92.1 and do not wait for SIM completion.

## Operator-authorized task decomposition

The latest one-task instruction replaces the prior consolidated #852/#862 planning scope. Existing identities remain attached to their narrowed tasks; the remaining work is explicitly routed to number-free rows, with no requirement dropped. GitHub body reconciliation must be recorded before execution; planned split rows are not already-created issues. #855 retains its live dynamic-lifecycle requirements rather than being reduced to provider schemas. See [atomic task contracts](ATOMIC_TASK_CONTRACTS_v0.92.2.md).

Native v3 scope reconciliation for #852/#855/#862 and conductor #864 is recorded in the [issue-local readback evidence](../../../.csdlc/evidence/864/task-scope-revision/remote-readbacks.json). The existing issue bodies now match their narrowed tasks; prospective tasks remain uncreated.

## Created first SIM sprint

The native launch readbacks bind these ten existing issues; do not recreate them. This is issue-creation evidence, not implementation completion or writer activation.

| Planned ID | Created issue |
|---|---|
| SIM-UMBRELLA | [#866](https://github.com/agent-logic/agent-design-language/issues/866) |
| SIM-01 | [#867](https://github.com/agent-logic/agent-design-language/issues/867) |
| SIM-02 | [#868](https://github.com/agent-logic/agent-design-language/issues/868) |
| SIM-03 | [#869](https://github.com/agent-logic/agent-design-language/issues/869) |
| SIM-04 | [#870](https://github.com/agent-logic/agent-design-language/issues/870) |
| SIM-05 | [#871](https://github.com/agent-logic/agent-design-language/issues/871) |
| SIM-06 | [#872](https://github.com/agent-logic/agent-design-language/issues/872) |
| SIM-07 | [#873](https://github.com/agent-logic/agent-design-language/issues/873) |
| SIM-08 | [#874](https://github.com/agent-logic/agent-design-language/issues/874) |
| SIM-09 | [#875](https://github.com/agent-logic/agent-design-language/issues/875) |

Source: [verified issue mapping](../../../.csdlc/evidence/864/sprint01-launch/issues.json). SIM-03/#869 includes the resolved supported command inventory in its self-contained issue body; the [launch inventory](../../../.csdlc/evidence/864/sprint01-launch/drafts/command-inventory.md) preserves the selection. Later sprint creation remains separately authorized.
