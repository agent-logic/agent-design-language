# Editor Command Adapter Surface

This document is the current editor command adapter contract for ADL task-bundle handoff.

It defines what browser/editor surfaces may prepare, what remains human-run through repo tooling and workflow skills, and what is explicitly out of scope for direct browser invocation.

## Language Contract Assumptions

This editor stack is a direct consumer of the ADL language story even though it
does not edit full ADL documents end to end.

The browser/editor surfaces should therefore assume the same canonical language
contract as the runtime and published schema:

- six primitives: `providers`, `tools`, `agents`, `tasks`, `workflows`, `run`
- singular `run` at the top level
- `patterns` and `signature` as top-level language features, not additional
  primitives
- packet and control-plane contracts remain outside the six-primitives language
  core

## Contract

The supported adapter surface is intentionally copy-only:

- supported adapter action:
  - `adl/tools/editor_action.sh prepare --phase init|doctor-ready|run|finish --issue <number> --slug <slug> [--version <vN.N[.P]>] [--title <title>] [--paths <paths>]`
- canonical native C-SDLC v3 control-plane mapping:
  - `csdlc issue --request <request.json> --registry <registry.json> --registrations <registrations.json>`
  - `csdlc doctor --request <request.json> --registry <registry.json> --registrations <registrations.json>`
  - `csdlc bind --request <request.json> --registry <registry.json> --registrations <registrations.json>`
  - `csdlc validate --request <request.json> --registry <registry.json> --registrations <registrations.json>`
  - `csdlc review --request <review-request.json>`
  - `csdlc publish --request <publication-request.json>`
  - `csdlc finish --request <finish-request.json>`
  - `csdlc clean --request <cleanup-request.json>`
- adapter mode:
  - browser-prepared, human-run command handoff

The browser/editor may:

- prepare a lifecycle command
- copy that command for a human to run from the repo root
- validate issue, branch, slug, and version constraints before surfacing the command
- surface the canonical ADL language contract for human orientation

The browser/editor may not claim direct browser invocation of:

- `csdlc issue`
- `csdlc doctor`
- `csdlc bind`
- `csdlc validate`
- `csdlc review`
- `csdlc publish`
- `csdlc shepherd`
- `csdlc finish`
- `csdlc clean`

Those commands exist in the repo control plane and related operational skills. They are not browser-direct actions.

## Why The Surface Is Narrow

The adapter must stay thin so the browser does not duplicate workflow logic already owned by the command/control-plane layer.

That means:

- browser code should not recreate lifecycle behavior in JavaScript
- browser code should not imply hidden direct execution paths
- browser docs should distinguish implemented repo commands from browser-prepared command handoff
- editor output should remain compatible with native C-SDLC v3 and the card editor route

## Truth Table

| Lifecycle command | Exists in repo | Browser-direct adapter support | Truthful editor status |
| --- | --- | --- | --- |
| `csdlc issue` | yes | no | copy-only prepared handoff |
| `csdlc doctor` | yes | no | copy-only prepared handoff |
| `csdlc bind` | yes | no | copy-only prepared handoff |
| `csdlc validate` | yes | no | copy-only prepared handoff |
| `csdlc review` | yes | no | copy-only prepared handoff |
| `csdlc publish` | yes | no | copy-only prepared handoff |
| `csdlc shepherd` | yes | no | out of browser scope |
| `csdlc finish` | yes | no | terminal observation handoff |
| `csdlc clean` | yes | no | post-terminal cleanup handoff |

## Proof Surface

The contract is backed by:

- `adl/tools/editor_action.sh`
- `adl/tools/test_editor_action.sh`
- `docs/tooling/editor/demo.md`
- `docs/tooling/editor/current_skill_wiring_demo.md`

The former v0.85 `start` compatibility action is retired and fails closed; it
is not part of the adapter contract.

The adapter surface should only be widened in a follow-on issue with matching docs, validation, and proof updates.
