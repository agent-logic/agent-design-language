# Issue 96 Design: v3 source-evidence umbrella receipt topology

## Outcome And Boundary

Reconcile the Sprint #5862 implementation-wave validator with the current
C-SDLC v3 three-revision proof topology: substantive source revision `S`, one
evidence-introduction revision `E`, and terminal lifecycle/publication/PR head
`H`. The validator must accept truthful `S != E != H` histories while retaining
every product-path, evidence-immutability, ancestry, terminality, denominator,
dependency-DAG, and integrated/native proof check.

## Owned Paths

- `.csdlc/prepared/issues/5862/validate-implementation-wave.rb`
- `.csdlc/prepared/issues/5862/test-validate-implementation-wave.rb`

## Source Baseline

- `.csdlc/prepared/issues/5862/validate-implementation-wave.rb`
- `.csdlc/prepared/issues/5862/proof-receipt-contract.rb`
- `.csdlc/evidence/5863/execution-proof.json`
- `.csdlc/evidence/5866/replay-window/execution-proof.json`
- `.csdlc/evidence/5872/execution-proof.json`
- issue #53 documents the unavoidable two-revision evidence/self-reference boundary

## Contract

For every child, resolve exactly one terminal envelope and exactly one retained
execution-proof mapping. Validate that `S`, `E`, `H`, and merge revision `M` are
40-hex Git objects; `S` is an ancestor of `E`, `E` is an ancestor of `H`, `H`
is the exact merged PR head, and `M` is ancestral to the umbrella candidate.
Every child-owned product path must be byte-identical from `S` through `E` and
`H`. The selected evidence tree must be absent from `S`, introduced exactly
once at `E`, and byte-identical from `E` through `H`. Proof content must bind
`S`, exact commands, nonzero selected tests, artifacts, and negative cases; it
must not claim that its own containing commit is `S` or fake a self SHA.

Keep the exact sixteen-child/path denominator and dependency DAG. Keep live
derived terminal envelopes, exact merged PR head, merge ancestry, and unique
candidate mapping mandatory. Keep #5878 integrated Guardian proof and native
macOS/Linux/Windows receipt bindings mandatory, while allowing its substantive
proof to bind `S` rather than incorrectly requiring `S == H`.

## Validation

The focused Ruby test creates isolated Git histories and validator fixtures for
a valid `S -> E -> H` topology and rejects post-source product drift,
post-evidence drift, wrong head, wrong merge, broken ancestry, missing or
ambiguous mapping, self-referential/fake evidence, denominator/DAG weakening,
and missing #5878 integrated/native bindings. Run Ruby syntax checks and the
focused nonzero test target.

## Rollback

Revert the validator and its focused test together. Do not alter child product,
terminal, publication, or closeout records.

## Non-Goals

- No runtime or distributed product changes.
- No closeout rewrite.
- No acceptance of unmerged or non-terminal children.
- No weakening of exact paths, ancestry, evidence immutability, terminal
  envelopes, sixteen-child/DAG denominator, or native proof.
