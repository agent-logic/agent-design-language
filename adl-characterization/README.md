# ADL Incumbent v1 Characterization

This independent crate captures and verifies the black-box behavior that the
ADL replacement must preserve. It does not link to the incumbent `adl` crate.
The corpus pins the incumbent source revision and the SHA-256 digest of the
exact executable used to create the retained observations.

## Pinned Baseline

- source revision: `19c2b6e2ad18bddc75db9231643a54b2a446ce72`
- executable SHA-256: `f558fa2111474e2fab540f8d0244be82cdb727ebbaa15aee758d8a7d57d0969c`
- corpus: `corpus/v1/corpus.yaml`
- retained observations: `observations/v1/`
- verification report: `observations/v1/verification.json`

The harness clears the child environment and supplies only a minimal local
`PATH`, isolated `HOME` and `TMPDIR`, disabled ADL observability, and
`NO_PROXY=*`. A fail-closed command-policy validator permits only exact known
non-executing command shapes (`--help`, `--version`, planning, prompt, graph,
signing, and verification) plus one exact local-mock execution shape. A fixture
path without an approved mode is rejected because default document invocation
may execute. The sole `--run` case must declare `local-mock-run` and reference a
fixture whose providers all use `mock:*` profiles and whose run has no remote
route. The harness does not claim to be an operating-system network sandbox;
corpus validation prevents network-capable commands from reaching the child
process. No credentialed, network, cloud, or AWS provider is invoked.

## Contract

The typed YAML manifest is validated by `corpus/v1/schema.json` and then by
semantic checks. It requires at least three executions per case, exact
required-behavior coverage, unique identifiers, pinned source and binary
identity, known comparison cases, and explicit expected exits and output
fragments.

Normalization is deliberately narrow:

- object keys in declared JSON streams may be sorted; array order is retained;
- named JSON fields or an exact line may be removed only when declared;
- no-op rules fail, so the manifest cannot hide imaginary nondeterminism;
- capture replaces only the known corpus and temporary-work prefixes with
  `<ROOT>` and `<WORK>` before evidence retention; separate SHA-256 fields bind
  both exact pre-tokenization streams and recomputable portable streams;
- declared `{ROOT}` and `{WORK}` arguments become the same portable tokens in
  retained raw and normalized evidence;
- every raw observation carries the digest of the sorted corpus bundle and a
  recomputable envelope digest over its identity, command contract, stream
  digests, and portable output.

Exit codes, semantic arrays, identifiers, diagnostics, and signature verdicts
are never discarded. Verification recomputes the corpus bundle, evidence
envelope, and portable stream digests; rechecks step count/order, declared and
portable arguments, exits, and required stdout/stderr fragments; and derives
every normalized record again. It then checks repeated portable output bytes
and enforces the declared equivalence and difference groups. Every child command
has the manifest-declared timeout; timeout kills and reaps the process and
returns a deterministic case/step error.

## Run

Use external Cargo output:

```sh
CARGO_TARGET_DIR=/Volumes/FastWork/adl-characterization-target cargo test \
  --manifest-path adl-characterization/Cargo.toml

CARGO_TARGET_DIR=/Volumes/FastWork/adl-characterization-target cargo run \
  --manifest-path adl-characterization/Cargo.toml \
  --bin adl-characterize -- verify \
  --corpus adl-characterization/corpus/v1/corpus.yaml \
  --observations adl-characterization/observations/v1 \
  --report adl-characterization/observations/v1/verification.json
```

`capture` additionally requires `--binary` and refuses an executable whose
digest differs from the manifest pin.

## Validation Lanes

| Lane | Proof role | Determinism | Resource profile | Release gate |
|---|---|---|---|---|
| unit | normalizer and path-boundary invariants | hermetic | tiny | required |
| manifest | schema, pin, coverage, and repetition contract | hermetic | tiny | required |
| evidence | retained derivation, repeat stability, comparisons, tamper detection | hermetic | small | required |
| CLI | operator entrypoint and report retention | hermetic | small | required |
| capture | exact incumbent black-box observations | local process only | small after incumbent build | required when baseline changes |

The corpus is characterization evidence, not a claim that every permissive or
undesirable incumbent behavior should become future policy. Any intentional
behavior change needs an explicit reviewed disposition against this baseline.
