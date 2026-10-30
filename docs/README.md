# ADL documentation

ADL's public repository provides language and contract packages, portable planning
and a CLI that dispatches product execution to separately installed owners.

## Start here

- [Project overview and first build](../README.md)
- [Provider-free example and expected output](../demos/README.md#runnable-from-a-public-clone)
- [Public contributor onboarding](onboarding.md)
- [Fork, test and submit a PR](../CONTRIBUTING.md#external-contributors)
- [Version compatibility and support boundaries](COMPATIBILITY.md)
- [Build entrypoints and measurement](PUBLIC_BUILD.md)
- [Roadmap and current milestones](ROADMAP.md)
- [Code of Conduct](../CODE_OF_CONDUCT.md) and private conduct reporting
- [Security reporting](../SECURITY.md)

## Language, CLI and libraries

- [Language specification](../adl-spec/README.md): ADL 1.0 draft.
- [Public CLI](../adl/README.md): planning, process/session tools and owner dispatch.
- [Schemas](../adl-schema/README.md) and [legacy document contracts](../adl-legacy-contracts/README.md).
- [UTS tool declarations](specs/uts/README.md) and [ACC contract specifications](specs/acc/README.md).
- [Repository ownership map](REPOSITORIES.md): implementation and qualification boundaries.
- [Glossary](GLOSSARY.md) and [design goals](design_goals.md).

## Maintainer operations

Internal sessions use [agent guidance](../AGENTS.md), the [native issue workflow](default_workflow.md),
[review expectations](../REVIEW.md) and the selected owner's [package manuals](csdlc-v3/man/README.md).
External contributors do not need these private lifecycle tools to submit a PR.

## Historical engineering records

[Milestone records](milestones/) and [architecture decisions](adr/) retain their
original dates, implementation scope and proof limits. They are not current
public command recipes. The [historical demo catalog](../demos/HISTORICAL_INDEX.md)
retains migrated and retired scenarios; use the current demo index for runnable guidance.
[Source-retirement evidence](../tools/source_retirement/README.md) preserves custody
and restoration contracts. Current work and remaining acceptance belong in the
[roadmap](ROADMAP.md), not an inferred conclusion from old milestone names.
