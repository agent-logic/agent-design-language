# Architecture decision reconciliation — #945

Status: **reviewable revised proposals; operator decision pending**. This packet reconciles all twelve issue-911 candidates with the pinned implementation. It does not accept decisions, allocate numeric ADRs, supersede records, or certify product behavior. #945 preserves the ARCH-ADR acceptance obligation consumed by [#925](https://github.com/agent-logic/agent-design-language/issues/925).

## Decision request

Recommendation: accept the twelve revised design decisions listed below. The recommendation is not an approval receipt. For each exact candidate, the operator or explicitly designated decision owner must select accepted, revised, rejected, or explicitly deferred and provide owner/rationale and remaining gate consequences. A requested revision returns to source review; a deferred required decision leaves the release gate unmet. Record actor, exact content digest and approval reference before applying status or numbering. No such decisions have been supplied yet.

| Candidate | Accountable owner | Recommendation | Current disposition |
|---|---|---|---|
| [ADR-CF-01](../../../../architecture/adr/issue-911/adr-cf-01.md) | CF-ADAPTER | Accept revised design | Pending operator decision |
| [ADR-CF-02](../../../../architecture/adr/issue-911/adr-cf-02.md) | CF-EVIDENCE | Accept revised design | Pending operator decision |
| [ADR-CF-03](../../../../architecture/adr/issue-911/adr-cf-03.md) | CF-EVIDENCE | Accept revised design | Pending operator decision |
| [ADR-CF-04](../../../../architecture/adr/issue-911/adr-cf-04.md) | CF-COG | Accept revised design | Pending operator decision |
| [ADR-CF-05](../../../../architecture/adr/issue-911/adr-cf-05.md) | CF-GOV | Accept revised design | Pending operator decision |
| [ADR-CF-06](../../../../architecture/adr/issue-911/adr-cf-06.md) | CF-REVIEW | Accept revised design | Pending operator decision |
| [ADR-CF-07](../../../../architecture/adr/issue-911/adr-cf-07.md) | CF-MEMORY | Accept revised design | Pending operator decision |
| [ADR-CF-08](../../../../architecture/adr/issue-911/adr-cf-08.md) | CF-UX | Accept revised design | Pending operator decision |
| [ADR-CF-09](../../../../architecture/adr/issue-911/adr-cf-09.md) | CF-SHELL | Accept revised design | Pending operator decision |
| [ADR-PLAT-01](../../../../architecture/adr/issue-911/adr-plat-01.md) | PLAT-PROVIDER | Accept revised design | Pending operator decision |
| [ADR-CSDLC-01](../../../../architecture/adr/issue-911/adr-csdlc-01.md) | SIM-04 | Accept revised design | Pending operator decision |
| [ADR-CSDLC-02](../../../../architecture/adr/issue-911/adr-csdlc-02.md) | SIM-08 | Accept revised design | Pending operator decision |

## Evidence and residual obligations

[Decisions and all69 task dispositions](decisions.json) preserve every original mapping exactly. Owner labels name accountable contract roles, not fabricated individual approval. [Pinned source manifest](source-manifest.json) records implementation bytes; [candidate content hashes](candidate-content.json) bind the revised text offered for decision. [Reciprocal refinement map](../issue-911/supersession-map.md) remains proposed; no accepted historical ADR is overwritten. Original issue-911 sources, review results and hashes remain historical evidence for that earlier packet, not approval of these revisions.

The concrete clarifications are recorded in each candidate's Implementation Reconciliation section. They narrow source-level claims about graph coverage, fitness rules, redaction, memory authority, approval inputs and transition evidence. They do not weaken the owning issues' required runtime acceptance.

PDF #898, complete installed integration #914 and independent qualification #915 are not proven by this packet. #848 repository decomposition and #910 Observatory deployment remain independently owned obligations; their live issue states are in [external state](external-state.json). ADR0069's dual-client human-review gate and ADR0075's historical scope are unchanged. A closed issue is not itself acceptance evidence. #925 must inspect actual per-candidate dispositions and supersession/numeric allocation after approval, alongside its separate OBS-S3 and TAIL-09 requirements; merging this draft cannot establish that release gate.

## Validation and review

Run `python3 docs/milestones/v0.92.2/adr/issue-945/validate_packet.py --self-test` and the historical issue-911 validator. PVF: docs_only, deterministic local CPU/file/Git reads, required document-integrity gate. Checks cover candidate/source digests, twelve identities, 69 mappings, pending approval truth, and links. They do not prove architecture correctness. Native Cargo projection checks are tooling proof only. Independent exact-head review is recorded separately under the issue evidence directory before PR publication. No provider/cloud or product execution is claimed.
