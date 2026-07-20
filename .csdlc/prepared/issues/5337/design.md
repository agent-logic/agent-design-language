# #5337 Design: Independent ADL v1 Characterization Corpus

## Decision

Implement an independent `adl-characterization` crate that executes the pinned
ADL v1 binary as a black box, captures raw observations, applies only declared
normalization, and verifies a versioned positive and negative behavior corpus.
The harness does not depend on the incumbent `adl` crate and does not port
incumbent internal tests or implementation logic.

The incumbent revision is fixed at
`19c2b6e2ad18bddc75db9231643a54b2a446ce72`. Every retained observation records
that revision, the binary digest, command arguments, exit status, stdout,
stderr, normalized output, and repetition number.

## Components

- `manifest`: parses and schema-validates the versioned corpus definition.
- `runner`: invokes a caller-supplied pinned v1 binary with a denied-network,
  credential-free environment and captures byte-exact process outcomes.
- `normalize`: canonicalizes JSON object keys and explicitly declared volatile
  values only. Array order, identifiers, error classes, field values, prompt
  order, exit status, and signature verdicts remain semantic.
- `compare`: checks repeated-run stability, equivalence groups, difference
  groups, expected exits and required output fragments.
- `adl-characterize`: captures or verifies a corpus and writes deterministic
  machine-readable reports.

## Corpus Boundary

The v1 corpus covers CLI help/version, six-primitives planning, graph JSON,
prompt projection, fork/join ordering, map and branch reorder equivalence,
sequential reorder difference, invalid arguments, malformed YAML, schema and
reference errors, state-reference errors, dependency cycles, repeated byte
stability, a deterministic local mock run, and fixed Ed25519 sign, verify, and
tamper rejection.

Provider schemas may be parsed and validated, but no credentialed, network, or
AWS provider is executed. The local mock case is the only execution case.

## Normalization Contract

Normalization is opt-in per case. It may:

1. parse JSON and sort object keys recursively while preserving array order;
2. replace the declared workspace root prefix with `<ROOT>`;
3. replace fields explicitly named by the corpus as volatile timestamps,
   timing values, run identifiers, or artifact roots; and
4. remove the exact `adl_event` observability line when observability cannot be
   disabled.

It may not reorder arrays, rewrite arbitrary strings, discard exit status,
coarsen error text, hide identifiers, or normalize signature results. A rule
that matches nothing or an undeclared volatile field is a verification error.

## Evidence And Reproducibility

Each case runs at least three times. Raw observations are immutable inputs to
comparison; normalized observations are derived artifacts. The coverage map
maps every required behavior to one or more case identifiers and fails closed
for missing, duplicate, or unknown mappings. Any repeated-run divergence not
explicitly covered by a narrow normalizer fails verification.

The checked-in observations are captured from the pinned v1 binary. Unit and
integration tests also use deterministic fixture executables so ordinary test
runs never require rebuilding v1 or reaching the network.

## Scope And Dependencies

Issue #5337 owns `adl-characterization/` and issue-local C-SDLC records and
evidence. This branch is explicitly stacked on the reviewed WP-02 #5336 head
`8c9a8687d`, which includes the reviewed WP-02 work plus integrated coverage
repair #5602 and defines the broader
clean-room architecture denominator. Publication and integration remain
blocked until PR #5599 merges. After that merge, this branch must integrate
current `main` and rerun exact-revision proof before publication.

The operator-authorized implementation pins the exact incumbent revision here
so this issue can proceed without importing source authority from v1.

## Non-Goals

- No incumbent `adl` or Runtime v2 source changes.
- No network, credentialed provider, AWS, or remote execution.
- No generalized replay engine or replacement ADL implementation.
- No normalization that masks semantic differences.
- No deferred acceptance criteria.

## Completion Rule

#5337 is complete only when the full corpus, retained repeated observations,
normalizer contract, coverage map, focused and full crate tests, exact-revision
review, and typed publication truth are present and green.
