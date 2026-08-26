# #286 design: ADR 0069 Observatory governed Runtime consumer evidence reconciliation

## Boundary

#286 is an ADR evidence-reconciliation issue for #207.d. It consumes existing
WP-18A/WP-18C Runtime and Observatory evidence and records whether ADR 0069 has
artifact-bound, review-backed proof for the governed Runtime consumer boundary.

It does not implement Runtime, browser UI, Unity, provider, cloud, storage, or
authority behavior. It does not move ADR 0069 to Accepted and does not edit the
shared ADR index, plan, or manifest; #288 / #207.f owns final serialized ADR
edits.

## Evidence model

The issue-local reconciliation packet must classify each referenced surface as:

- terminal-proving: exact landed WP-18A and WP-18C revision identities, artifact locator and digest, retained human-review reference and
  result, retained machine-readable outcome reference, and a classification
  showing the surface proves the governed Runtime consumer boundary;
- partial/non-terminal: useful evidence exists but a dependency is open,
  blocked, credential-bound, branch-local, missing machine-readable outcome,
  missing human review, or not yet terminal;
- out-of-scope: evidence belongs to another owner and should only be referenced
  as a dependency or residual gap.

Residual gaps are allowed and should be recorded explicitly. A truthful #286
result can be ready for #207 even when it says ADR 0069 is not yet acceptance
ready.

## Inputs

- Live issue #286 contract.
- Parent #207 ADR coordination contract.
- Terminal/current WP-18A and WP-18C issue records relevant to the Observatory
  governed Runtime consumer boundary.
- ADR 0069 source/evidence surfaces, if present.
- Issue-local validator output retained under `.csdlc/prepared/issues/286`.

## Output

- `.csdlc/prepared/issues/286/validate_preparation_bundle.py`
- Issue-local C-SDLC cards for #286.
- Later, a bound evidence packet under `.csdlc/evidence/286` if implementation
  is authorized.
