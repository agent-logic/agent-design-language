# Architecture decision acceptance — #945

Status: **all twelve design decisions accepted by the operator** in Planning #7.3. The eleven unchanged decisions bind the reviewed proposal hashes at `5507b83b19e21fb98b68ad082e01f1b91b72b6c4`. CF-09 binds the revised decision statement, followed by explicit installed-agent, invitation-only and GitHub sign-in selections. Its old CLI-only bytes were not accepted.

[Operator approval and revision history](operator-decisions.json) records the individual replies. It is a repository transcription of the conversation, not a cryptographically signed external approval receipt. [Decisions](decisions.json) preserve all 69 original task mappings as historical accounting and associate each candidate with its accepted successor. [Relationships](relationships.json) preserve prior statuses and reciprocal refinements; no accepted record is superseded. Historical issue-911 proposals and their validator remain unchanged.

| Candidate | Accepted record | Accountable owner |
|---|---|---|
| ADR-CF-01 | [ADR 0076](../../../../adr/0076-cf-01.md) | CF-ADAPTER |
| ADR-CF-02 | [ADR 0077](../../../../adr/0077-cf-02.md) | CF-EVIDENCE |
| ADR-CF-03 | [ADR 0078](../../../../adr/0078-cf-03.md) | CF-EVIDENCE |
| ADR-CF-04 | [ADR 0079](../../../../adr/0079-cf-04.md) | CF-COG |
| ADR-CF-05 | [ADR 0080](../../../../adr/0080-cf-05.md) | CF-GOV |
| ADR-CF-06 | [ADR 0081](../../../../adr/0081-cf-06.md) | CF-REVIEW |
| ADR-CF-07 | [ADR 0082](../../../../adr/0082-cf-07.md) | CF-MEMORY |
| ADR-CF-08 | [ADR 0083](../../../../adr/0083-cf-08.md) | CF-UX |
| ADR-CF-09 | [ADR 0084](../../../../adr/0084-cf-09.md) | CF-SHELL |
| ADR-PLAT-01 | [ADR 0085](../../../../adr/0085-plat-01.md) | PLAT-PROVIDER |
| ADR-CSDLC-01 | [ADR 0086](../../../../adr/0086-csdlc-01.md) | SIM-04 |
| ADR-CSDLC-02 | [ADR 0087](../../../../adr/0087-csdlc-02.md) | SIM-08 |

## Gate consequences

The recorded decisions satisfy the architectural decision-set acceptance obligation consumed by [#925](https://github.com/agent-logic/agent-design-language/issues/925). They do not establish implementation, merge, or release readiness. #848 and #910 remain separate obligations; deferred/proposed ADR0069/0072/0075 are not promoted by references from this set.

[Beta 1 delivery requirements](BETA1_DELIVERY.md) now require both server-hosted and installed-local-agent website journeys plus CLI support. #914 integration and #915 independent qualification must consume these requirements; their former CLI-only proof is insufficient. Missing website, server and local-agent implementation owners/dependencies are explicitly unresolved. This packet does not claim typed downstream cards or remote issues have already been updated.

The website remains `agent-logic/codefriend.ai`, separate from the private code repository whose name remains undecided. v0.93 repository extraction cannot defer Beta 1 functionality. PDF and other component proof remain with their original owners. The [external-state snapshot](external-state.json) is historical tracker observation, not current delivery evidence.

## Validation and review

Run `python3 docs/milestones/v0.92.2/adr/issue-945/validate_packet.py --self-test` and the historical issue-911 validator. PVF: docs_only; deterministic local CPU/file/Git reads, required document-integrity gate. Validate source and content hashes, twelve identities, 69 historical mappings, approval binding, unique accepted numbering, refinement links and both-mode delivery requirements. These checks validate recording integrity, not the correctness of a decision or the working product. Independent review and native publication evidence remain separate.
