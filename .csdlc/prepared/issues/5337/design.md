# #5337 Design: Characterization-Corpus Preparation

## Decision

Prepare #5337 as the v0.91.8 WP-03 owner for a future normalized v1
characterization and determinism corpus. This preparation defines the contract,
evidence boundaries, and validation plan; it does not capture fixtures, execute
v1 behavior, implement a normalizer, or modify shared milestone documents.

## Dependency Boundary

- Sprint umbrella #5595 authorizes this opening-wave preparation slot.
- WP-02 #5336 remains the hard implementation dependency.
- Preparation may finish before #5336 integrates, but product execution may not
  start until current typed evidence confirms WP-02 acceptance and the future
  implementation paths are bound without collision.

## Future Corpus Contract

The later implementation must define a compact, versioned black-box corpus that
includes positive and negative cases, repeated outcomes, normalization rules,
coverage mapping, and explicit nondeterminism disposition. V1 is behavioral
evidence only: legacy internal tests and source movement are not clean-room
implementation authority.

Normalizer rules must distinguish stable semantic fields from volatile data
such as timestamps, host paths, generated identifiers, ordering that is not
contractual, and provider/runtime noise. Every normalization must be declared,
reviewable, and narrow enough that it cannot erase a semantic mismatch.

## Preparation Deliverables

- Six issue-specific cards rendered by the active `current.json` template set.
- This issue-local design and diagram.
- Typed PVF lanes for template/card validation, issue-local scope checks, and
  later focused corpus proof.
- A truthful readiness disposition that leaves product implementation pending
  on WP-02 and separate implementation authorization.

## Protected-Scope Boundary

This preparation claim protects only `.csdlc/issues/5337`,
`.csdlc/prepared/issues/5337`, and `.csdlc/evidence/5337`. It deliberately does
not claim shared v0.91.8 planning files or future product/corpus paths. Those
paths must be resolved and added through typed claim amendment only when
implementation is authorized.

## Non-Goals

- No corpus fixtures, normalizer code, replay execution, or product tests.
- No porting of legacy internal tests.
- No edits to `docs/milestones/v0.91.8` or other shared milestone surfaces.
- No v0.92 scope expansion, Runtime/C-SDLC implementation, AWS, or raw `gh`.

## Readiness Rule

Preparation is complete only when all six cards are current-template derived,
schema/structure-valid, issue-specific, reviewed at an exact substantive
revision, and published only if typed lifecycle gates permit a preparation-only
handoff without claiming product completion.
