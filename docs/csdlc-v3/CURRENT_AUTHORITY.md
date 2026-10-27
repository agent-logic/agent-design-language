# Current C-SDLC authority

C-SDLC v3 ordinary authority comes from the authenticated installed owner package
and its repository binding. The owner verifies package integrity and retained
lifecycle evidence. Project-local selectors and historical #505 cutover receipts
are not consulted by ordinary admission. Use the exact executable returned by
installation/admission and its paired manuals; a source build or this document
cannot establish authority.

The historical ADL-side operator records have recoverable published Git identities
in [the retirement manifest](../archives/CSDLC_AUTHORITY_1373.json). Historical
relocation/recovery evidence remains historical; do not recreate it as a project
grant or bypass installed-package checks.

`csdlc prepare ISSUE --plan FILE` and `csdlc start ISSUE --plan FILE` prepare fresh
issues. `bind`, `edit`, `validate`, `doctor` and `proof` operate on bound issue
context. Independent `review` precedes `publish`. Under operator authorization,
`deliver ISSUE --approval REFERENCE --execute` checks the final reviewed head,
required checks and mergeability, then merges and reconciles delivery. Successful
`delivery.status=delivered` with `finish_required=false` is complete; optional
cleanup is separate maintenance. Remote effects with unknown outcomes require
native recovery before retrying. The installed command's help and paired manual
are the authority for supported options.

No automatic v2 fallback is permitted when installed authority is unavailable.
Explicit rollback or transition remediation remains a separately bounded action.

## Command discovery and result interpretation

The command contract built from `v3-command-manifest.json` is available through
`csdlc --contract`; `csdlc <command> --describe` returns that command's descriptor.
These discovery responses describe inputs, effects and result schema; they do not
grant authority or execute the described route. Verify discovery against the
binary selected for the invocation. A source change or isolated candidate build
does not establish that the active installed binary has been upgraded.

JSON command results retain their owner-specific fields and add `/envelope` with
schema `csdlc.v3.command_result.v1`. Successful descriptor discovery returns its
contract directly; help remains text. Read process outcome, owner status,
authority and effect outcome separately: exit success or a `ready` classification
alone is not mutation, publication or terminal authority. An unknown effect or
unreported evidence invalidation must not be interpreted as proof of no effect.
Use the underlying owner result and durable receipts for lifecycle decisions.

The observation routes `doctor`, `validate`, `eligibility`, `schedule` and
`shepherd` inspect without creating a lifecycle lock or replaying interrupted
transactions. Recovery-required state must remain visible for an explicit
mutation/recovery route. This diagnostic guarantee does not extend to every
command grouped with local preparation, such as `issue`, `edit` or `bind`.

## Issue intent interface

The [issue intent guide](INTENT_COMMANDS.md) records the ordinary command model.
Consult the installed package manuals for its exact supported version. Candidate
source and isolated validation do not upgrade the active executable.

## Historical evidence

The [original pre-cutover changeover notice](TOOLING_CHANGEOVER_NOTICE.md) is
immutable historical evidence of the warning issued before #505. Its statements
about the then-current v2 route are not current operator instructions. The
[construction readiness notice](CUTOVER_READINESS_NOTICE.md) and retained
full-replacement denominator likewise retain their source-time meaning.
Future authority changes require a new explicit operator-reviewed decision and
notification; they are not authorized by these historical notices.

## Focused fitness proof

The `command_manifest` test now belongs to the
[Cognitive-SDLC owner repository](https://github.com/agent-logic/cognitive-sdlc).
From that owner checkout, its source-level command is
`cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`.
The private manifest is no longer present in public ADL; public documentation
changes use focused link, path and contract checks rather than rebuilding an owner.
Authority checks belong to the owner package and validate installed integrity,
repository binding, retained evidence and actual command behavior. Negative cases introduce obsolete
policy/help wording and require rejection. Historical records are excluded from
the current-routing scan and checked for preservation in the issue review.
PVF: deterministic local contract proof; small CPU and local Git/subprocess;
Owner execution results must be recorded separately from public documentation
checks; this source command alone is not installed authority proof.
