# Issue 75 design: typed incremental publication linkage

## Decision

Add a `linkage_mode` enum to publication requests and retained publication intent/evidence. `closing` preserves today's fail-closed closing-keyword and terminal-finish behavior. `part_of` requires an exact non-closing `Part of ...#<issue>` reference, forbids closing keywords for that issue, and records a non-terminal checkpoint that `csdlc-finish` cannot treat as issue-closing authority.

The mode is explicit; omitted mode is accepted only as the backward-compatible `closing` default. Same-repository references may use `#75`; split-authority references must use the qualified issue repository. Parsing stays token-based and exact, never substring-based.

## Boundaries

- Change only C-SDLC v2 publication, GitHub observation, schema, finish, and focused tests.
- Do not weaken closing publication or terminal closeout invariants.
- Do not migrate or close the parent issue from a `part_of` checkpoint.
- Do not add shell or Python control-plane code.

## Validation

Focused Rust tests prove same-repository and split-repository `closing` and `part_of` forms, ambiguous/mixed linkage rejection, retained mode truth, remote reconciliation, and finish refusal for non-closing checkpoints.
