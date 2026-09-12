# v0.92.2 ADR set — issue #911

Status: **complete proposed candidate packet; formal decision acceptance pending**. Twelve candidate records cover the eight seed topics and four reviewed additions. All 69 core task identities are accounted for. No record is promoted or superseded by this packet.

## Candidate records

| Record | Accountable scope owner | Status |
|---|---|---|
| [ADR-CF-01: Portable repository inputs](../../../../architecture/adr/issue-911/adr-cf-01.md) | CF-ADAPTER (#878) | Proposed; acceptance pending |
| [ADR-CF-02: Evidence, finding and run identity](../../../../architecture/adr/issue-911/adr-cf-02.md) | CF-EVIDENCE (#881) | Proposed; acceptance pending |
| [ADR-CF-03: Untrusted input, redaction and retention](../../../../architecture/adr/issue-911/adr-cf-03.md) | CF-EVIDENCE (#881) | Proposed; acceptance pending |
| [ADR-CF-04: Architecture cognition and explanation](../../../../architecture/adr/issue-911/adr-cf-04.md) | CF-COG (#882) | Proposed; acceptance pending |
| [ADR-CF-05: Executable governance](../../../../architecture/adr/issue-911/adr-cf-05.md) | CF-GOV (#887) | Proposed; acceptance pending |
| [ADR-CF-06: Independent review, synthesis and action plans](../../../../architecture/adr/issue-911/adr-cf-06.md) | CF-REVIEW (#890) | Proposed; acceptance pending |
| [ADR-CF-07: Longitudinal comparison and Memory Palace](../../../../architecture/adr/issue-911/adr-cf-07.md) | CF-MEMORY (#885) | Proposed; acceptance pending |
| [ADR-CF-08: Approval and renderer parity](../../../../architecture/adr/issue-911/adr-cf-08.md) | CF-UX (#895) | Proposed; acceptance pending |
| [ADR-CF-09: Local CodeFriend product boundary](../../../../architecture/adr/issue-911/adr-cf-09.md) | CF-SHELL (#891) | Proposed; acceptance pending |
| [ADR-PLAT-01: Shared provider configuration and lifecycle](../../../../architecture/adr/issue-911/adr-plat-01.md) | PLAT-PROVIDER (#876) | Proposed; acceptance pending |
| [ADR-CSDLC-01: One authoritative semantic issue record](../../../../architecture/adr/issue-911/adr-csdlc-01.md) | SIM-04 (#870) | Proposed; acceptance pending |
| [ADR-CSDLC-02: Single-writer transition and recovery](../../../../architecture/adr/issue-911/adr-csdlc-02.md) | SIM-08 (#874) | Proposed; acceptance pending |

Scope-owner roles are derived from the declared task contracts. They identify accountable contract surfaces, not a fabricated individual assignee or acceptance receipt. The operator or an explicitly designated decision owner supplies formal acceptance; #911 curates and reviews the text.

## Decision accounting

- [69-task inventory](decision-inventory.md) and [machine-readable map](decision-inventory.json).
- [Refinement and supersession map](supersession-map.md).
- [Remaining decision authority and source conflicts](decision-dispositions.md).
- [Pinned source manifest](source-manifest.json).
- [Complete independent review and validation results](review.md); [reviewed content hashes](reviewed-content.json).

The record structure follows the existing candidate convention in ADR0072/0075: Status, Context, Decision, Alternatives, Consequences, Sources, Validation and Approval Boundary. Issue #911 additionally requires an explicit question, accountability and reversibility. There is no separately discovered universal ADR template to override that convention. Candidate labels are scoped to this issue; accepted numeric allocation remains separate.

## Validation boundary

The packet validator checks record presence/structure, source digests, owners, all69 mappings, relative links and status/refinement truth. Its negative fixtures reject changed authority status, missing candidates, missing owner and lost task identities. Structural checks are not proof of architectural correctness or implementation; independent substantive review is recorded separately.

Run `python3 docs/milestones/v0.92.2/adr/issue-911/validate_packet.py --self-test` from the repository root. PVF: docs_only; deterministic local CPU/file checks; required issue gate; no network, provider, cloud or runtime execution.

The source snapshot is historical to this packet. Before an ADR is accepted or an owner changes a contract, reconcile changed implementation/planning evidence and refresh affected review. ARCH-ADR acceptance remains a TAIL-10 obligation; this packet adds no CF-INTEGRATE or TAIL-01 gate.
