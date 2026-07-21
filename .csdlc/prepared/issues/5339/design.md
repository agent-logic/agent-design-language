# Issue 5339 clean-room language-core design

## Status and gate

This design is preparation-complete but implementation-blocked. Product work
may begin only after issue #5337 is both merged and in typed `closed_out`
phase. Its reviewed corpus and normalization contract then become read-only
acceptance inputs.

## Ownership

WP-04 owns only `adl-v2/crates/adl-language` and issue-local C-SDLC records.
The crate defines a pure source-language boundary for exactly six primitives:
provider, tool, agent, task, workflow, and singular run. It does not own
compiler expansion or `ExecutionPlan`, engine scheduling, Runtime v3,
provider/tool transport, portable records/signing, CLI selection, or C-SDLC.

Incumbent ADL source, schemas, tests, and fixtures are behavioral evidence
only. The clean-room crate must not copy, adapt, import, link, or vendor them.

## Public model

The versioned root `AdlDocument` contains explicit collections of all six
primitive types. Each public type uses deny-unknown-fields deserialization and
schema generation from the same Rust definition. Identifiers and references
use narrow newtypes; stable diagnostics carry a code, document path, and
bounded message. Source locations are retained only when the selected parser
can supply them deterministically.

Parsing is a two-stage boundary:

1. YAML or JSON is decoded under strict syntax and duplicate-key policy.
2. One typed semantic validator checks version, identities, references, and
   only those cycle constraints that the reviewed language contract assigns to
   WP-04.

Canonicalization returns a deterministic source-model representation. It sorts
unordered identity collections and map keys, preserves declared ordered
sequences, normalizes representation-only differences, and never emits or
implies compiler node identity, scheduling, or execution semantics.

## COTS decisions

| Concern | Decision | Boundary |
| --- | --- | --- |
| Rust serialization | `serde` 1.0.229 with `derive` | Type conversion only; every public document type rejects unknown fields. |
| JSON | `serde_json` 1.0.151 | Strict JSON decode and canonical JSON serialization; no arbitrary JSON value escape hatch for core semantics. |
| YAML | `yaml_serde` 0.10.4 | Maintained by the YAML organization; accepted only with tests proving duplicate-key rejection and JSON/YAML equivalence. Disable or avoid include/property extensions. If those tests fail, YAML acceptance stops rather than adding a custom parser. |
| Schema generation | `schemars` 1.2.1 with derive/std only | Generate versioned JSON Schema from the same types; checked fixtures detect drift. |
| Schema fixture validation | `jsonschema` 0.48.2 with default features disabled | Dev/test only, with no HTTP/file resolver, TLS, async runtime, or remote reference resolution. |
| Diagnostics | small issue-local enums and structs | No diagnostic framework or compiler front-end dependency until measured need exists. |
| Canonical ordering | standard library ordered collections/sorts | No canonicalization framework dependency. |

These versions were selected from current crates.io metadata during preparation;
the implementation lockfile must retain the reviewed exact versions or record
an evidence-backed COTS amendment before code review. Forbidden dependencies include incumbent ADL crates, Runtime crates, C-SDLC,
async runtimes, HTTP clients, cloud/provider/database SDKs, and parser-generator
frameworks. New COTS must earn its place with a concrete contract and budget
measurement.

## Budget decision

The milestone hard ceilings remain 30,000 implementation LoC and 15,000 test
LoC for the complete ADL v2 product. WP-04 takes a provisional reviewed
allocation of at most 4,000 implementation LoC and 4,000 test/fixture LoC,
leaving capacity for compiler, engine, contracts, adapters, and CLI work. This
is an allocation, not a reason to weaken behavior or proof: an evidence-backed
exact-revision review may change it before publication.

Focused warm validation targets 120 seconds; the complete deterministic WP-04
suite must remain within 600 seconds. Cargo artifacts go under
`/Volumes/FastWork`. The default dependency graph must contain no runtime,
control-plane, network, cloud, database, or provider SDK graph.

## Validation design

- Positive fixtures cover every primitive and a complete six-primitive
  document in YAML and JSON.
- Negative fixtures cover syntax, unknown fields, duplicate keys/identities,
  invalid versioning, malformed identifiers, unresolved references, and every
  reviewed language-level cycle class.
- Schema tests compare checked generated schemas with Rust deserialization and
  fixture outcomes.
- Canonicalization tests permute unordered collections and JSON/YAML surface
  representation, then require byte-identical canonical JSON.
- A characterization map names every applicable #5337 case and records pass or
  an evidence-backed intentional difference. No broad normalization may hide a
  semantic mismatch.
- Dependency, source/test LoC, strict Clippy, diff hygiene, focused latency, and
  full deterministic latency are recorded at the exact review revision.
- The issue-local `validate-language.sh` is the single declared command adapter
  for focused, quality, parity, and budget lanes. It fails closed while the
  future crate manifest is absent, so VPP argv remains executable during
  preparation without pretending deferred proof has passed.

## Failure and rollback

Any false or stale dependency signal, corpus ambiguity, forbidden dependency,
schema/type drift, accepted unknown field, unresolved reference, unstable
canonical output, or unclassified parity mismatch stops publication. Before
WP-04 is selected as a consumer dependency, rollback is simply removal of the
new isolated crate and selector/workspace reference; no incumbent behavior is
mutated by this issue.
