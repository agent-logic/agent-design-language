# Issue 295 design: mechanical compile-fallout coverage classification

## Objective

Add a fail-closed, diff-exact classification layer to the existing changed-source coverage gate. The layer may exempt only import additions and argument pass-through additions to an already governed API when every changed hunk compiles and every owning API path has mapped behavioral proof. All other changed source remains subject to the existing 80 percent whole-file threshold.

## Authority boundary

- The classifier augments `adl/tools/check_coverage_impact.sh`; it does not replace or lower the existing threshold.
- Classification is content-based and mapping-based, never path-allowlist based.
- PR-fast evidence is only an input artifact and never release-authoritative proof.
- Issue #258's `adl-runtime/src/distributed/transport/core.rs` diff is a read-only fixture source; #258 is not mutated.

## Contract

The classifier consumes an exact unified diff plus a tracked mapping that names the governed token, owner/API paths, and behavioral tests. It accepts a file only when:

1. every hunk contains exclusively an import of a mapped token or an added pass-through argument of that token;
2. no removed or context line represents semantic, predicate, branch, state, or error behavior change;
3. every hunk is covered by a successful compile proof;
4. every mapped owner/API path has at least one behavioral proof, including `EstablishedRuntimeAuthority` for the #258 case; and
5. a complete machine receipt can record file, hunk, token, owner, tests, and rationale.

Missing mappings, partial proof, malformed diffs, extra semantic edits, or receipt incompleteness reject classification. Rejected or unclassified Rust files continue through the unchanged 80 percent whole-file gate.

## Implementation shape

- Add a small deterministic parser/classifier under `adl/tools/` using repository-supported Python for coverage tooling internals.
- Add a tracked mapping/fixture contract for governed tokens and their owning behavioral proofs.
- Integrate classification into the coverage-impact checker at the narrow point where a changed Rust file would otherwise fail the per-file threshold.
- Emit a JSON receipt for accepted mechanical fallout and preserve ordinary threshold diagnostics for all rejected cases.
- Add focused parser, mapping, receipt, integration, and negative fixtures. Classify these tests as deterministic, small-resource, PR validation evidence; they do not become release coverage authority.

## Validation

- Focused classifier unit/fixture test suite covering exact positives and every required negative class.
- Existing coverage-impact contract suite proving the 80 percent gate remains intact.
- Shell syntax and Python compile checks for touched tooling.
- Diff hygiene.

## Risks and mitigations

- Parser over-acceptance: reject any token or syntax outside the two exact forms.
- Proof laundering: require every owner mapping and test to be present in the receipt and validate proof inputs fail closed.
- Threshold bypass: classification is a narrow precondition; ordinary files retain the existing threshold path unchanged.
- Fixture drift: pin the #258-shaped diff in focused tests without editing its worktree.
