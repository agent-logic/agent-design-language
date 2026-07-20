# Issue 5358 preparation readiness blockers

Status: preparation repair complete; acceptance execution remains outside this preparation-only change.

## B-5358-01 — prompt-template authority mismatch — resolved

The active repository registry selects the current legacy/import template set,
while native compact C-SDLC v2 records retain their declared `1.0.0` identity.
The installed generation-aware typed v2 tooling now validates that distinction
against repository authority instead of treating every card as the registry's
legacy generation.

Disposition: resolved by the installed generation-aware typed v2 tooling.
Generation-9 `csdlc-doctor` passes with no findings and no shared template or
implementation change was made in issue `#5358`.

## B-5358-02 — preparation review scope cannot be represented while bound — resolved

The generated SRP formerly defaulted to `Exact implementation revision before
publication.` The installed preparation-safe typed v2 editor now permits a
bounded-phase SRP `review_scope` replan without advancing lifecycle phase or
claiming that acceptance implementation ran.

Disposition: resolved through `csdlc-edit apply` at generation 9. The SRP now
names the exact issue-local preparation surface, complete AC-1 through AC-6
SPP/VPP coverage, and the preparation non-claim boundary. No SRP Markdown or
values file was patched directly.

## Bound-phase doctor interpretation

The generation-9 doctor report passes with no integrity findings and reports
`ready: false`, `next_operation: inspect_phase`. This is expected bound-phase
lifecycle classification and must not be described as acceptance readiness.

## Completed preparation truth

- Issue-local typed initialization and binding completed.
- SPP preparation steps S1-S3 remain complete; the two tooling blockers found
  during preparation are resolved without changing any acceptance criterion.
- `#5540` and `#5541` remain closed evidence inputs.
- `#5548` and `#5558` remain independent open defect/blocker inputs.
- Typed PVF doctor and file-level issue-scope inventory lanes pass at
  generation 9.
- No acceptance, implementation, deployment, publication, merge, or closeout
  was executed.
