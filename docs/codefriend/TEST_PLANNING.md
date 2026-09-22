# Source-bound test planning

New test plans resolve a reviewed finding's evidence identifiers through its
validated original Admission. Filenames in model prose are not authority.
Supported Rust, Java, Python and JavaScript evidence yields a bounded proposal;
unsupported evidence remains explicitly outside the proposal's coverage.
Privacy-omitted content is never restored or used to fabricate a fixture.

Use the original, still-live evidence Store:

```sh
adl codefriend plan tests --store original-store --input synthesis/synthesis.json --out tests
adl codefriend plan tests read --store original-store --input tests/test-plan.json
```

Generation requires `--store`. It verifies that the retained Admission exactly
matches the review record before and after generation. Missing, deleted, expired
or replaced original evidence fails closed. The command never refreshes the
Admission or its retention deadline.

New bundles use `codefriend.test_plan.v2` and
`codefriend.test_plan_manifest.v2`. Readback rederives the plan from the validated
review and synthesis. It rejects altered mappings, manifests and plan contents.
The original Store is required to read these bundles through the CLI.

Historical v1 bundles retain their original derivation and bytes. Reading one
without `--store` checks the retained snapshot only; it does not establish live
source authorization. The legacy library `plan` and `plan_from_file` functions
exist for this compatibility boundary. New live owners use the Store-backed
entrypoints, or wrap the deterministic record-aware transform in their existing
original-owner authorization checks. Publication readback dispatches by the
recorded plan schema and verifies canonical derivation. The existing publication
preparation API preserves historical v1 bundles; current live server, agent and
Journey owners explicitly select `prepare_publication_bundle_for_format_v2`.
Both versions preserve the corresponding component owners' artifact bytes.

A test plan is a proposal. Its test location need not already exist, and its
fixture and expected assertions still require implementation and execution.
A source-bound proposal does not certify a model finding, prove coverage or
replace independent product qualification.
