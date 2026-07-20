# Issue 5358 preparation readiness blockers

Status: blocked for acceptance execution; preparation artifacts retained.

## B-5358-01 — prompt-template authority mismatch

The active repository registry `docs/templates/prompts/current.json` selects
prompt-template set `1.0.3`. The sole typed v2 card engine currently emits and
validates card identity `template_version: 1.0.0` for every generated card.
`csdlc-doctor` therefore proves internal typed-v2 consistency, but it does not
prove compliance with the root contract's active `1.0.3` registry.

Disposition: owner-tooling blocker. Do not hand-edit generated cards or change
shared templates/implementation in issue `#5358`.

## B-5358-02 — preparation review scope cannot be represented while bound

The generated SRP defaults to `Exact implementation revision before
publication.` This conflicts with the authorized preparation-only review and
STP AC-6. A typed `set_field(review_scope, ...)` attempt was rejected because
SRP mutation is forbidden during the `bound` phase. Advancing `#5358` to
`implemented` would falsely imply that acceptance implementation ran.

Disposition: owner-tool-supported recovery/transition or explicit operator
disposition is required. Do not patch SRP Markdown or values directly.

## Bound-phase doctor interpretation

The generation-7 doctor report passes with no integrity findings and reports
`ready: false`, `next_operation: inspect_phase`. This is expected bound-phase
lifecycle classification and must not be described as acceptance readiness.

## Completed preparation truth

- Issue-local typed initialization and binding completed.
- SPP preparation steps S1-S3 are complete; S4 remains pending because review
  found the two blockers above.
- `#5540` and `#5541` remain closed evidence inputs.
- `#5548` and `#5558` remain independent open defect/blocker inputs.
- Typed PVF doctor and file-level issue-scope inventory lanes pass.
- No acceptance, implementation, deployment, publication, merge, or closeout
  was executed.
