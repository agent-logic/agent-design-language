# Architecture drift between admitted revisions

The drift reporter compares two actual structure reports and retains both graph
revisions, digests, admissions and original findings. It uses CF-MEMORY's
`AdmittedBaselines`, `BaselineRef` and comparison implementation: repository,
scope, included/excluded evidence, lane and policy versions, provider route and
complete coverage must agree. A changed boundary policy is not silently normalized
into a comparable scan. Partial or narrower scans return `NotComparable` and
never claim a removed dependency.

```sh
adl codefriend architecture drift --store store --baselines baselines --baseline before.json --current after.json --out drift.json
adl codefriend architecture drift-read --store store --baselines baselines --input drift.json
```

Produce `before.json` and `after.json` through `architecture report` at explicit
admitted revisions. Both must use the same declared file scope and boundary
policy. The drift command retains their original review records through the
shared memory adapter; `memory read` can consume the resulting graph comparison's
baseline/current references. Readback requires both live admissions and retained
baseline records. Deleted or expired evidence, missing/deleted baseline records,
modified graphs, mismatched versions and altered drift artifacts fail closed.
Deleting a baseline also prevents a later drift command from recreating it.

Two comparisons are retained. `graph_comparison` compares the original producer
findings without changing their Run/Finding identities. `structural_comparison`
compares observed nodes and directed references as shared CF-EVIDENCE Findings.
Their runs preserve every inherited compatibility guard and add a versioned
architecture-drift lane. The original producer gate must agree with this second
comparison; structural facts cannot bypass an incompatible graph pair.

Node identity uses its module name through `Finding::identity`; edge identity
uses source/destination module names, reference spelling and edge kind through
the same shared identity function. Repeated occurrences of one reference retain
all source locations under one fact. Evidence IDs and source lines stay attached
to each revision, but do not determine identity. Moving a reference to another
line within the same declared scope therefore remains unchanged. File-scope
changes are not inferred renames and remain subject to the compatibility gate.

Added and resolved mean that a structural declaration appears or disappears in
complete comparable graphs. They do not mean a defect appeared or was fixed.
The before/after fact records carry explanatory rationale, evidence and unknown
confidence. Edge rationale identifies declared layers and whether the edge
crosses them. A changed reference can therefore expose a changed layering
relationship under the same boundary model. Changing that model itself yields
NotComparable. No opaque risk score replaces these observed changes.

The report embeds both graphs and both fact records so each added, removed or
unchanged identity can be traced to its revision and source locations. Validation
recomputes all output using live graph evidence and CF-MEMORY. Repository text is
never executed as an instruction. Reports contain admitted redacted source
content; CLI stdout contains a JSON summary and stderr contains counts/status.

Bounds inherit the structure graph limits and add at most 1000 distinct
structural facts per revision and 32 MiB per serialized drift artifact. Exceeding
a limit errors rather than truncating. Files are create-only, private on Unix,
and reject traversal/symlinks. CLI output cannot be placed inside either managed
store. A supported NotComparable outcome returns success with `comparable:false`;
consumers must inspect it rather than interpret exit zero as no drift.

The focused test target and installed proof runner use inert local Git fixtures
with separate controlled revisions. Calibration checks a changed reference
against unchanged topology and line relocation, and records exact sampled
outcomes. These fixtures measure syntactic reference deltas only, not runtime
behavior, semantic rename accuracy or external-repository qualification. The
selected external repository pin is unchanged; no providers, source builds or
source scripts are involved.
