# Issue 112 Split Plan After Gemini Decomposition Review

Date: 2026-08-12

Issue: #112

Classification: `split_now`

Decision: Do not publish the current monolithic #112 branch as a single PR.

## Basis

Current real-code diff against `origin/main`, excluding docs, Markdown, JSON,
logs, locks, and lifecycle artifacts:

- total code: `+3131 / -54`, net `+3077`
- product/runtime source: about net `+1554`
- tests: about net `+1186`
- tooling script: net `+322`
- Observatory UI JS/CSS: net `+15`

Gemini 3.1 Pro supplemental decomposition evidence:

- `.csdlc/evidence/112/provider-reviews/gemini-3.1-pro-decomposition-result.json`
- Verdict: `split_required`
- Key recommendation: do not merge or publish the current monolithic branch.

Current issue-card scope is cohesive as an end state, but too broad as one
reviewable PR. The deliverables combine core authority domain logic, kernel
conversation control flow, public Runtime API integration, test expansion,
Observatory UI/tooling, and lifecycle evidence.

## Proposed Stacked PR Order

### Slice 1: Core Layer 8 authority domain module

Purpose:

Introduce the bounded authority domain types and verification logic without
activating production conversation delivery.

Paths:

- `adl-runtime-kernel/src/layer8_authority/audit.rs`
- `adl-runtime-kernel/src/layer8_authority/exchange.rs`
- `adl-runtime-kernel/src/layer8_authority/identity.rs`
- `adl-runtime-kernel/src/layer8_authority/mod.rs`
- `adl-runtime-kernel/src/lib.rs`

Validation:

- `cargo test --manifest-path adl-runtime/Cargo.toml layer8_authority`
- kernel compile/clippy for the exported module surface

Review focus:

- identity evidence binding
- key and credential-generation binding
- capability/policy intersection
- replay/audit locking and restart behavior
- signed request and recipient acknowledgement contracts

PR relationship:

- Part of #112, do not close #112.

### Slice 2: Kernel conversation integration

Purpose:

Wire the authority domain into production kernel assembly, runtime-kernel
startup profile loading, control flow, ingress, and conversation dispatch.

Paths:

- `adl-runtime-kernel/src/assembly.rs`
- `adl-runtime-kernel/src/bin/adl-runtime-kernel.rs`
- `adl-runtime-kernel/src/control.rs`
- `adl-runtime-kernel/src/ingress.rs`
- `adl-runtime-kernel/src/conversation_sessions_tests.rs`

Validation:

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml conversation_sessions_tests::authenticated_selected_agent_conversation_uses_canonical_wss_ingress -- --nocapture`
- kernel clippy

Review focus:

- authorization before sequence reservation/provider delivery
- recipient acknowledgement failure semantics
- externally held recipient key boundary
- no caller-fabricated authority
- no recipient substitution, scope widening, replay, or cross-Polis bypass

PR relationship:

- Stacks on Slice 1.
- Part of #112, do not close #112.

### Slice 3: Runtime API and integration tests

Purpose:

Expose the narrow Layer 8 authority delivery boundary through Runtime/API
surfaces and prove the external API cannot bypass the kernel authority model.

Paths:

- `adl-runtime/src/layer8_authority.rs`
- `adl-runtime/src/lib.rs`
- `adl-runtime/tests/layer8_authority.rs`
- `adl/src/csm_runtime_api.rs`
- `adl/tests/layer8_authority_runtime_api.rs`

Validation:

- `cargo test --manifest-path adl/Cargo.toml --test layer8_authority_runtime_api`
- `cargo test --manifest-path adl-runtime/Cargo.toml --test layer8_authority`
- API/runtime clippy

Review focus:

- public API request/identity binding
- delivery closure cannot run on refusal
- recipient identity key-id binding
- ack verification exactness
- temp/root portability for tests

PR relationship:

- Stacks on Slice 2.
- Part of #112, do not close #112 unless Slice 4 is removed from #112 scope.

### Slice 4: Observatory UI/tooling proof

Purpose:

Add disclosure-safe Observatory presentation validation for authority state,
bounded refusal, revoked visibility, and real-browser proof.

Paths:

- `adl/tools/validate_layer8_authority_observatory_ui.sh`
- `demos/html-observatory/app.js`
- `demos/html-observatory/styles.css`

Validation:

- `bash adl/tools/validate_layer8_authority_observatory_ui.sh`

Review focus:

- no private token/proof/policy/provider payload disclosure
- repository-local temporary artifacts
- browser proof classification truth

PR relationship:

- Stacks on Slice 3.
- Final #112 PR if all acceptance criteria remain under #112.
- Include `Closes #112` only on the final accepted slice.

## Extraction Strategy

1. Preserve the current monolithic branch as a recovery/reference branch.
2. Create a clean slice-1 branch from current `origin/main`.
3. Apply only Slice 1 paths from the monolithic branch.
4. Revalidate Slice 1 and obtain fresh exact-head review.
5. Publish Slice 1 as "Part of #112"; do not close #112.
6. Create Slice 2 from Slice 1 head and apply only Slice 2 paths.
7. Repeat validation/review/publication for Slice 2.
8. Continue with Slice 3 and Slice 4.

Do not use the current monolithic branch as a publication branch unless the
operator explicitly overrides the split recommendation.

## Dirty Worktree Boundary

At the time this plan was written, the worktree contained active uncommitted
changes in:

- `.csdlc/evidence/112/layer8-runtime-api-clippy.log`
- `.csdlc/evidence/112/layer8-runtime-api-integration.log`
- `adl/tests/layer8_authority_runtime_api.rs`
- `.csdlc/evidence/112/provider-reviews/`

These are not safe to overwrite. Treat them as live shared work until the owner
either commits, reverts, or hands them off.

## Follow-On Issue Guidance

Preferred issue graph:

- Keep #112 as the parent acceptance issue.
- Use stacked PRs against #112 when the tracker and lifecycle tooling support
  multiple PRs closing one issue only at the final slice.
- If issue-level lifecycle tooling requires one issue per PR, create follow-on
  child issues for Slices 2-4 with `split_from: #112` and keep #112 as either
  Slice 1 or the final aggregator, depending on C-SDLC v2 constraints.

No GitHub issues or PRs were created by this plan.
