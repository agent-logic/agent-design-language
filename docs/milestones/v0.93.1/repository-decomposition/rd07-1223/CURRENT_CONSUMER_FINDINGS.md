# Bounded ownership review for #1223

Read-only review against ADL `9cfd035aac2a44637b3c8026bd40b669579a8c7f`, C-SDLC `deed45851f9e8a6847dd5adae277b78eb3531336`, and CodeFriend `273089b0ecbe20e1979654e0faeaca0604398de1`. No builds, acceptance, deletion approval or lifecycle mutations.

Scope: 429 of 432 unmapped rows (322 assigned C-SDLC, 107 assigned CodeFriend). Other three rows are outside these owners. Per-row JSON supplies 55 plausible successor mappings and bounded tracked-reference evidence for 177 rows. Reference presence is not by itself live invocation; absence is not proof of no dynamic consumer.

## 1. Some C-SDLC-assigned skill contracts plausibly belong to CodeFriend

A prefix transform identifies `adl/tools/skills/<skill>/...` -> `review-skills/<skill>/...`; skill input schemas can map to `review-skills/contracts/248f00e3/<schema>`. These are changed resources, not missing resources. CodeFriend `review-skills/adr-curator/adl-skill.yaml:10-12` explicitly points into the frozen contracts directory. CodeFriend `crates/codefriend/tests/review_skills_package.rs:27-39` executes the resource-package suite and requires exactly five passing tests. It explicitly limits its claim to resource delivery at lines 1-2.

Next: compare changed relative references/content for the 54 suffix/prefix mapping candidates in JSON, preserve their old identity, then reuse exact-artifact installed resource proof. Do not retain these indefinitely simply because the original owner/path is absent; do not call source tests current installed acceptance.

## 2. Legacy CodeFriend test coverage is not established by finding a replacement bundle

All 37 unmapped `adl/tests/codefriend*.rs` rows need a coverage correspondence decision. ADL `adl/tests/codefriend_review.rs:1-12` declares installed CLI/provider-adapter proof and imports the ADL CodeFriend/provider surfaces. CodeFriend `crates/codefriend/tests/review_regression_bundle.rs:8-16` runs a seven-resource bundle with four skill contracts, two fixture pairs, six manifest entries. That denominator does not demonstrate replacement of all 37 legacy integration tests.

Next: map named scenarios/assertions into standalone installed consumers, or explicitly retire obsolete behavior while retaining historical evidence. Keep ADL tests with their compiled modules until a coherent component removal removes/redirects both. Do not infer a gap merely from renamed tests; the coverage comparison remains pending.

## 3. CI and validation files assigned C-SDLC still serve retained ADL consumers

ADL `adl/tools/ci_path_policy.sh:5` resolves `adl/tools/validation_manager.py`; lines 1012 onward invoke that manager. `adl/tools/validation_manager.sh:5` directly execs the Python manager. `adl/tools/run_validation_manager_nessus_lane.sh:5` selects the wrapper. `tools/groupd_validation/src/lib.rs:57` calls `adl/tools/test_select_validation_lanes.sh`; `adl/tools/test_ci_path_policy.sh:192` invokes both lane-selector and validation-manager tests. `tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs:1120` retains the owner validation lane command.

Next: retain this specific ADL validation dependency closure until its retained callers are transitioned together. This is an operational ADL consumer reason, not evidence that all 169 tooling rows belong permanently in ADL. #205 need not block #1223: explicitly retain this closure as an ADL obligation rather than silently deleting it or waiting for global CI normalization.

## 4. Session ledger source has compiled ADL consumers despite retired lifecycle authority

ADL `adl/src/lib.rs:104` exports `session_ledger`; `adl/src/cli/mod.rs:51` imports `session_cmd::real_session`; `adl/src/cli/session_cmd.rs:8-10` imports ledger functions and lines 13-18 dispatch status/claim/heartbeat/release. Current policy retiring claims as authority does not automatically remove these source dependencies.

Next: keep `adl/src/session_ledger.rs` and `adl/src/cli/session_cmd.rs` with the retained CLI, or submit a coherent command/module removal with explicit compatibility disposition. This is not a reason to restore claims as authority.

## 5. AWS CodeFriend dispatch has an explicit deliberately inactive successor

CodeFriend `deployment/workflows/aws-codefriend-build.v1.json:2-5` records the old workflow path/blob and disables activation pending separate cloud authorization. ADL `.github/workflows/aws-codefriend-build.yaml:3-21` still exposes workflow_dispatch and defaults to the monolithic ADL test lane. JSON configuration is correspondence evidence, not an active workflow replacement.

Next: propose removal of the old dispatch as an explicit disabled-successor transition, preserve its exact source and document that cloud dispatch remains inactive. Do not activate the successor as part of retirement.

## Remaining documentation/demo treatment

The row breakdown is 169 tooling/CI, 19 source/other, 37 legacy CodeFriend tests, 52 skill resources, 33 historical demos, 119 documentation/template rows. Historical demos and docs are not automatically obsolete: distinguish immutable reproduction inputs from current operator entrypoints, update retained links, and preserve executable demo dependencies or record an explicit historical-only disposition. JSON leaves every row unresolved pending this concrete decision; no blanket retained category counts as issue completion.
