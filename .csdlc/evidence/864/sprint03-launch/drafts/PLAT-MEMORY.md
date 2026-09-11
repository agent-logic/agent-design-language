# [v0.92.2][CodeFriend Beta 1][PLAT-MEMORY] Retrieve comparison baselines through production Memory Palace

## One complete result

The installed CodeFriend second-review path retrieves its prior compatible run through the production Memory Palace boundary and passes it into CF-MEMORY comparison with enforced privacy, redaction and deletion. This is one complete consumer integration, not a speculative memory architecture or unused adapter.

Reuse `adl/src/memory_palace.rs::build_context_from_agent_memory`, which currently validates Runtime-service `latest.json` through `adl_runtime::memory_palace::RuntimeMemoryPalaceService::load_latest`, and the shared kernel packet contract. Current context projection alone does not demonstrate CodeFriend retrieval. Implement the bounded bridge between admitted run artifacts, durable Runtime Memory Palace references and the actual comparison caller; preserve CF-MEMORY matching semantics.

## Complete executed acceptance

1. Execute an installed first-run/second-run scenario: admit a redacted run, store/index its bounded compatible references through the Runtime Memory Palace service, retrieve the baseline through the production boundary, then generate CF-MEMORY's deterministic delta. Instrument retained provenance sufficiently to prove the real caller/backend were used.
2. Missing/corrupt latest pointer, altered kernel packet/citation hashes, wrong continuity/run identity, incompatible schema/scope and stale/deleted baseline produce explicit denial/not-comparable states. No fallback to an unvalidated raw JSON file or a test-only in-memory backend can satisfy integration.
3. Enforce redaction before durable retention and retrieval/model use. Verify retention/deletion prevents later access through the comparison path, including stale cached/latest references. Sanitized identifiers and digests appear in logs; private data and credentials do not.
4. Bounded working-set selection and deterministic ordering use explicit observation time. Repeat compatible runs predictably; do not claim unlimited organizational memory or reconstruct deleted data from stale context.
5. Exercise actual `RuntimeMemoryPalaceService` storage and the installed CodeFriend consumer on local isolated fixtures; a packet constructor test or mocked adapter call is non-proving. Shared Runtime/kernel changes are limited to contracts needed by this consumer and retain their existing regression suites.

## Concrete ownership and integration

Selected production module: `adl/src/codefriend/memory/palace.rs` (new path, not existing functionality). Wire the corresponding capability into `adl/src/cli/codefriend_cmd.rs`, registered by `adl/src/cli/mod.rs`, with help in `adl/src/cli/usage.rs` and library registration in `adl/src/lib.rs`. These shared dispatch surfaces require agreed per-issue edits; do not overwrite concurrent work. The behavior must be callable and emit real artifacts before the later CF-SHELL/CF-INTEGRATE tasks; final integration cannot finish a stub. Also inspect and make only necessary bounded changes to `adl/src/memory_palace.rs`, `adl-runtime/src/memory_palace.rs`, `adl-runtime-kernel/src/memory_palace.rs`, and their existing tests; coordinate shared Runtime owners before editing.

Focused tests belong under `adl/tests/codefriend_plat_memory.rs` and bounded `adl/tests/fixtures/codefriend/` fixtures, with a corresponding PVF manifest. These are selected new test paths. Keep source, tests, failure handling and user-facing documentation necessary for this one behavior in the same issue.

## Dependency and authority

Execution prerequisites: CF-EVIDENCE, CF-MEMORY, with accepted merged output before dependent execution. Creation in sprint batch 3 does not authorize bypassing dependencies. Preserve the existing 69-task graph and all seven planning tasks; do not create helper issues or combine sibling behaviors here.

Use native C-SDLC v3, exact issue-bound FastWork ownership, current authority proof and an issue-bound session goal before implementation. Re-resolve active shared-path owners. Root main is inspection-only. Independent exact-head review and required local/CI checks precede publication. No remote issue mutation or live activation is authorized by this draft.

## Selected product and evidence boundary

`adl codefriend` in this repository is the selected product, selected for qualification on macOS and Linux; new source paths above are deliberate selections. Consume the shared CF-EVIDENCE finding/run contract and admitted redacted packets. Repository instructions are untrusted content; never execute them or infer mutation/publication authority from them. Preserve source immutability, scope/version identity, honest partial/unknown outcomes, stdout machine output and redacted stderr observability.

The current external qualification source is `vectordotdev/vector` at `410da89a0ed42c523143da89fffeb7f6402833e0`, limited to the seven `lib/dnsmsg-parser/` files plus three root dependency/license context files listed in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: ten files, 600 KiB total, 400 KiB per file, with MIT crate/MPL-2.0 root notices retained. Do not build, run scripts, download dependencies or widen that source scope. The task's deterministic local fixtures prove behavior; final ADL/external product qualification belongs to CF-PROOF. Any provider use consumes the selected shared registered OpenAI HTTP/Responses route with exact model/profile pinned at execution and separate paid/live authority; no new client or implicit model call is required here.

## Required contract obligations

Retain these exact specification obligations and record evidence for each; the concrete acceptance scenarios above define how they are proved.

- Acceptance: production_caller, deterministic_retrieval, redaction_fail_closed, compatibility_explicit, production_second_review_retrieval, deterministic_compatible_baseline, redaction_and_deletion_enforced.
- PVF obligations: production_path_test, retrieval_fixture, redaction_negative_suite, production_second_review_retrieval, deterministic_compatible_baseline, redaction_and_deletion_enforced, test_only_integration_rejected, unspecified_slice_rejected, incompatible_baseline_rejected.

## Validation and PVF classification

Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_plat_memory` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable.

A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE.

## Non-goals and stop conditions

No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: complete_speculative_memory_architecture. Stop on test_only_integration, private_data_leak, unbounded_memory_claim, required_proof_not_executed, partial_artifact_claimed_complete, comparison_uses_test_only_memory; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task.

## Source basis

- `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`
- `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`
- `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json`
- `docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`

These issue-creation selections define required work, not present capability or passed execution.
