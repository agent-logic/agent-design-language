# Portable ADL Project Templates

This directory contains reusable templates for ADL-governed external
repositories.

This is the retained v1 adapter template set, not current installed-package
admission or a requirement for external public contributors. Its `ADL_HOME` and
sibling-source discovery model is historical. For current maintainer operations,
use [native authority](../../csdlc-v3/CURRENT_AUTHORITY.md) and the selected
[installed package manuals](../../csdlc-v3/man/README.md). External contributors
use the [public fork-and-PR workflow](../../../CONTRIBUTING.md#external-contributors).

The historical initial template set is:

- version: `1.0.0`
- contract issue: `#3569`
- contract doc:
  `docs/tooling/PORTABLE_ADL_PROJECT_ADAPTER_CONTRACT_v0.91.5.md`

## Templates

| Template | Purpose |
|---|---|
| `1.0.0/AGENTS.md` | Compact repo-local operating instructions for an external ADL-governed repo. |
| `1.0.0/adl_project.json` | Machine-readable project adapter contract. |

## Examples

Example `adl_project.json` files live under `1.0.0/examples/`.

They are examples only. Do not copy one into a real repository without checking
issue authority, repository name, validation profile, artifact policy, and ADL
tooling compatibility.

## Portability Rules

- Template paths must stay repo-relative.
- Templates must not embed operator-local absolute paths.
- The v1 templates describe historical `ADL_HOME`/source discovery. Current
  maintainers select an authenticated installed owner package; a path or sibling
  checkout alone does not grant authority.
- The canonical card lifecycle remains `SIP -> STP -> SPP -> VPP -> SRP -> SOR`.
