# Repository structure reports

`adl codefriend architecture report` reads an admitted packet through the governed
evidence store, parses the selected Rust module tree, and writes a new JSON report.
It does not compile or execute the repository, run macros, download dependencies,
call a provider, or change analyzed source files. Analysis is a bounded syntactic
slice of one explicit Rust crate, not a complete architectural correctness verdict.

```sh
adl codefriend architecture report --store /local/evidence-store \
  --packet-id PACKET_ID --policy boundary-policy.json --out new-report.json
adl codefriend architecture read --store /local/evidence-store --input new-report.json
```

Use the existing `codefriend evidence admit-local` or `admit-packet` command first.
Choose a new output path outside the analyzed checkout. Output is create-only,
owner-readable on Unix, and rejects symlink paths. JSON summaries go to stdout;
content-free `adl_event` status goes to stderr. An incomplete report is a valid
result with `analysis_complete: false`, not a successful complete assessment.

The policy is explicit input. Every Rust analysis file needs a layer assignment.
Same-layer references are allowed; all cross-layer pairs are forbidden unless
listed. A policy is not obtained by following instructions in repository content.

```json
{
  "schema": "codefriend.structure.v1",
  "crate_root": "src/lib.rs",
  "manifest_path": "Cargo.toml",
  "layers": {"src/lib.rs": "api", "src/domain.rs": "domain"},
  "allowed": [["api", "domain"]],
  "coupling_threshold": 4
}
```

Set `manifest_path` to null when no admitted manifest is selected. The manifest
must be in acquisition scope. Direct dependency tables produce declared external
dependency nodes and edges. Version selection, features, target conditions,
workspace inheritance and external implementation are not resolved. These are
declaration facts, never evidence that the dependency is actually executed.

The Rust graph follows declared out-of-line modules from the root and records
explicit `crate`, `self` and `super` references from source to dependency. File
names alone do not establish module membership. `mod.rs` and module-named files
are supported; missing or ambiguous module files are explicit unknowns. Inline
modules, globs, aliases, attributes, macros, unresolved external paths, type and
dynamic dispatch, parse failures, omitted content and unsupported languages are
partial/unknown cases. This version does not resolve symbol definitions or prove
that a referenced symbol exists. `analysis_complete` only covers this documented
syntactic slice, with complete acquisition and no observed unknowns.

The report contains the exact boundary policy and digest, stable module IDs,
sorted nodes and edges with file/line/evidence IDs, unknown reasons, and a shared
`ReviewRecord`. That record binds repository, source revision, acquisition scope,
admission identity and analysis version. Findings explain forbidden edges,
directed cycles, policy-threshold fanout, and inferred name connascence where
multiple modules reference the same spelled symbol. There is no opaque risk
score or claim about value, timing, execution-order or semantic connascence.
Finding identity uses the shared repository/rule/semantic anchor contract;
revision-bound evidence remains separate so later comparison can track identity.

Readback re-fetches the live admission and deterministically recomputes the
report, rejecting modified graph, findings, policy, digest or admission. Deleted
or expired admissions cannot be read through this command. Exported report files
contain admitted source and must be handled under the same privacy/retention
policy; deleting a store admission does not erase independently exported copies.
No prior-run Memory Palace storage, impact, rationale or drift analysis is
implemented here; those are separate Sprint 3 children.

Bounds: at most 512 graph nodes, 4,096 edges, 4,096 unknowns, 128 KiB policy and
16 MiB report. Existing shared contracts further bound findings and text. Exceeding
any bound fails rather than silently truncating the graph.

Before recursive Rust parsing, each source is limited to 32 KiB and 128 raw
lexical units. Each ASCII alphanumeric/underscore run counts once; each punctuation
or non-ASCII byte counts once; ASCII whitespace is ignored. This conservative
check also counts comments and literals. It bounds keyword chains, nested
delimiters, unary expressions, generics and path chains without first invoking an untrusted-input parser. Larger sources
produce `rust_syntax_complexity_limit` and a partial report, never a clean result.
This first bounded reporter is suitable for small admitted module slices; it
does not claim full analysis of arbitrarily large Rust files.

Proof is declared in `ARCHITECTURE_PROOF_INVENTORY.json`. Cargo CLI tests and
isolated installed-binary evidence are recorded separately; tests do not imply
unrun operating-system, provider or external-repository qualification.

## Evidence-grounded 4+1 package (#1109)

The optional native journey continuation generates logical, development, process,
deployment and scenario views from the same admitted revision as the structure
report. A governed provider receives the admitted source with exact evidence IDs
and line numbers. Model output cannot choose repository identity, completeness or
digests. Citations must reproduce admitted source lines exactly. This establishes
provenance, not semantic entailment: a reviewer must still assess each inference.

For an already prepared native journey, write a continuation request outside the
analyzed checkout. `provider_request` names an operator-owned invocation template
with no preloaded `input_text`; the journey constructs its evidence-bound prompt.

```json
{"stage":"four_plus_one","provider_request":"/local/operator/provider-request.json"}
```

```sh
adl codefriend journey resume --output /local/operator/journey \
  --request /local/operator/four-plus-one-request.json
```

Use the installed, clean candidate that created the journey. A generation request
reserves its intent before dispatch and is not automatically retried following an
uncertain result. The retained `four_plus_one.json` contains the generated package;
`four-plus-one-rendered/` contains the readable Markdown package, editable Mermaid
sources and deterministic SVGs. Resume verifies response reconstruction, current
admission and rendered bytes before returning journey status.

Run generation before preparing the existing Markdown, HTML or PDF publication
bundles. The same approval binds the package, source graph, retained model response,
diagram sources and rendered artifacts. Exports include the architecture narrative
and diagram attachments; PDF draws the same entities and directed relationships
with the existing vector renderer. No additional approval or review pipeline is
introduced. Existing exported copies remain subject to the repository's retention
policy; deleting an admission does not delete independent exports.

`complete` means every view and scenario trace is populated without declared gaps,
conflicts or assumptions. It never means verified running topology. Missing runtime
or deployment evidence produces actionable missing-input descriptions. Shared
entity IDs connect views; source declarations, inferences and assumptions stay
explicit. Conflict records retain citations to conflicting evidence.

A lower-level `adl codefriend architecture four-plus-one --store STORE --graph
GRAPH --provider-request REQUEST --out NEW_DIRECTORY` command uses the same
contracts. That command alone does not establish product-journey qualification.
Hosted/paired generation and installed ADL/external-repository qualification remain
pending in this candidate. Component fixtures do not establish their completion.
