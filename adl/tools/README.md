# adl/tools

> The twelve C-SDLC skill families retired from this checkout by #1223 are owned by the authenticated installed C-SDLC package: demo-operator, documentation-specialist, gap-analysis, issue-folding, issue-splitter, issue-watcher, portable-contract-normalizer, refactoring-helper, release-evidence, review-comment-triage, test-generator and use-case-writer. For the ten direct resource contract scripts, set `CSDLC_SKILL_CONTRACT_ROOT` to the verified package's absolute `resources/adl/tools/skills` directory. The documentation-specialist and use-case-writer contracts instead require `ADL_OPERATIONAL_SKILLS_SOURCE_ROOT` selecting an authentic complete original layout, because they retain full-layout installer assertions. A directory path alone does not authenticate its contents. These scripts preserve the original assertions and refuse a missing or relative root. The former source layout below is historical where explicitly marked; it is not current installation authority.

> CodeFriend review-skill sources now belong to [the CodeFriend owner](https://github.com/agent-logic/codefriend/tree/e93a457ad6325017627a66a2abe31a3376fa28fd/review-skills). Select its authenticated installed distribution for operational use. Retained original-layout tests require explicitly selected authentic historical fixtures; the partial ADL skill tree is not a complete installer input.

Utility scripts for ADL development workflows, PR hygiene, burst execution, and report maintenance.
This directory is tooling-focused: command wrappers, safety checks, and operator helpers.
Keep behavioral and milestone narrative in canonical docs, not here.

## What Is Here

- `.adl/bin/native-v3/csdlc`: native post-cutover lifecycle entrypoint.
- `.adl/bin/csdlc-v2/`: retained owner binaries only for explicitly authorized rollback or bounded transition remediation.
- Runtime owns `adl/tools/demo_v0871_operator_surface.sh`: the qualified mock operator-surface consumer is preserved there.
- Runtime owns `adl/tools/demo_v0871_runtime_state.sh`: the qualified mock paused/completed persistence consumer is preserved there. Retained ADL aggregate callers require an absolute `RUNTIME_DEMO_SOURCE_ROOT` pointing to the reviewed complete Runtime checkout and the owner-required `ADL_RUNTIME_BIN`; missing source selection refuses before effects. This routing does not qualify the aggregate, live providers, or historical recovery expectations.
- The v0.87.1 WP-13 aggregate is retired; see [retired v0.87.1 aggregate contract](../../docs/tooling/V0871_HISTORICAL_AGGREGATE.md). Individual provider demos retain separate qualification obligations.
- Historical compatibility-card link repair and bootstrap SOR materialization are retired. Use the selected native C-SDLC owner for current card operations; it does not restore the old duplicate-root/symlink contract.
- `demo_v0871_multi_agent_discussion.sh` (historical, retired in #1223): canonical `v0.87.1` bounded multi-agent discussion proof wrapper for a five-turn Claude + ChatGPT runtime demo.
- `demo_v0871_real_multi_agent_discussion.sh` (historical, retired in #1223): live-provider companion demo that calls real OpenAI and Anthropic APIs through a local ADL completion adapter using operator-managed keys; a missing-credentials skip in the paired test is a non-proving disposition, not live-provider proof.
- `real_multi_agent_provider_adapter.py`: local adapter that translates ADL's bounded HTTP completion contract to vendor-native OpenAI and Anthropic calls without recording secret material.
- `validate_multi_agent_transcript.py`: validates the bounded multi-agent transcript artifact contract for the D13 discussion demo.
- `worktree_doctor.sh`, `worktree_prune.sh`: deterministic worktree governance and safe cleanup helpers.
- `archive_run_artifacts.sh`: dry-run/apply helper that inventories local run roots, copies unique run artifacts into `.adl/trace-archive/milestones/<milestone>/runs/`, and can move archived active `.adl/runs` entries into `.adl/trace-archive/source-roots/`.
- `release_ceremony.sh`: nonmutating native v3 release candidate preflight; requires an exact typed candidate request and stable owner. See `docs/csdlc-v3/RELEASE_PREFLIGHT.md`.
- Historical `v0.89` D11 quality-gate aggregate: retired with its monolithic source route under #1223; original scripts and assertions remain in retirement custody. Independent public and owner checks remain separate.
- Native v3 `csdlc edit` and `csdlc validate` own current typed card edits and
  validation.
- Issue execution uses the selected native C-SDLC owner and bound FastWork worktrees.
- `batched_checks.sh`, `preflight_review.sh`: quality/preflight checks, including the repo-code-review skill contract guard.
- `check_repo_quality_staleness.py`: focused reviewer-facing docs and tracked-junk audit for the current milestone package and root repo status surfaces.
- Public coverage uses `tools/public_adl/coverage.py` with its declared inventory; see [coverage authority](../../docs/tooling/COVERAGE_AUTHORITY_AND_RELEASE_PROOF.md). Historical mixed-workspace gates and prerequisite installers are retired under #1223.
- `clean_coverage_artifacts.sh`: removes local generated coverage summaries and loose LLVM profile shards after coverage/report runs.
- `report_large_rust_modules.sh`: non-blocking Rust source-and-test module size report for maintainability review.
- Inspect cards and reports directly in their registered issue worktree or retained evidence directory.
- `update_reports_index.sh`, `update_latest_reports.sh`: report index maintenance.

## Canonical Workflow Commands

The retired original `test_skill_documentation_completeness.sh` checked the
historical monorepo skill layout; its source and assertions remain in retirement
custody. Its 51-family, 205-input inventory and missing Sprint
Review schema are preserved in the [historical disposition](../../docs/milestones/v0.93.1/repository-decomposition/rd07-1223/SKILL_COMPLETENESS_HISTORICAL_DISPOSITION.json);
no full completeness pass is claimed. Each skill owner must check its declared
current inventory, manifests, schemas, output contracts, guide coverage, and
installation contract in that owner's layout. Individual resource checks do not
establish complete owner coverage.

From repo root:

```bash
# Historical full-layout compatibility installer: requires explicit complete reviewed ADL_OPERATIONAL_SKILLS_SOURCE_ROOT; not the partial ADL tree
bash adl/tools/install_adl_operational_skills.sh

# Same explicit complete root, symlink mode; not the partial ADL source tree
ADL_OPERATIONAL_SKILLS_INSTALL_MODE=symlink bash adl/tools/install_adl_operational_skills.sh

# Original full-layout contract only: select authentic complete ADL_OPERATIONAL_SKILLS_SOURCE_ROOT first; not a current whole-owner acceptance claim
bash adl/tools/test_install_adl_operational_skills.sh

# verify the multi-agent repo-review specialist skill contracts
bash adl/tools/test_multi_agent_repo_review_specialist_skill_contracts.sh

# Compare a deployed skill with verified installed owner resources; no output means identical bytes
diff -qr "$CSDLC_SKILL_CONTRACT_ROOT/test-generator" "$CODEX_HOME/skills/test-generator"

# run the bounded Claude + ChatGPT discussion demo through the real runtime
# Historical retired command: bash adl/tools/demo_v0871_multi_agent_discussion.sh

# run the live-provider Claude + ChatGPT discussion demo using env vars or explicit key-file overrides
# Historical retired command: bash adl/tools/demo_v0871_real_multi_agent_discussion.sh

# validate the generated bounded multi-agent transcript artifact
python3 adl/tools/validate_multi_agent_transcript.py artifacts/v0871/multi_agent_discussion/transcript.md

# The v0.87.1 aggregate is historical; see docs/tooling/V0871_HISTORICAL_AGGREGATE.md

# initialize through the semantic issue intent, then inspect through read-only owners
.adl/bin/native-v3/csdlc prepare <issue> --plan <plan.json>
.adl/bin/native-v3/csdlc validate --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
.adl/bin/native-v3/csdlc doctor --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>

# bind execution context at the last responsible moment
.adl/bin/native-v3/csdlc bind <issue>

# inspect worktree status/fate across managed, stale, orphan, and Codex-ephemeral namespaces
./adl/tools/worktree_doctor.sh

# dry-run safe repo-local cleanup of merged clean worktrees + stale registrations
./adl/tools/worktree_prune.sh

# write a reviewable report for a bounded first batch without deleting anything
./adl/tools/worktree_prune.sh --limit 10 --report docs/tooling/worktree_cleanup/first_batch.md

# apply the same bounded batch after review
./adl/tools/worktree_prune.sh --limit 10 --report docs/tooling/worktree_cleanup/first_batch.md --apply

# inventory local run artifacts without deleting or moving the source data
./adl/tools/archive_run_artifacts.sh --include-worktrees

# copy unique run artifacts into .adl/trace-archive/milestones/
./adl/tools/archive_run_artifacts.sh --include-worktrees --apply

# copy unique run artifacts, then clear active .adl/runs by preserving source dirs under .adl/trace-archive/source-roots/
./adl/tools/archive_run_artifacts.sh --include-worktrees --apply --prune-active-runs

# install the reviewed native v3 owner, then check an exact candidate request
./adl/tools/install_owner_binaries.sh --bin csdlc
./adl/tools/release_ceremony.sh --request /absolute/path/to/release-preflight.json

# run standard checks
./adl/tools/batched_checks.sh

# Historical v0.89 D11 aggregate retired under #1223; original proof is retained.
# Current public CLI checks: bash tools/public_adl/run_cli_ci.sh

# audit root + milestone reviewer-facing docs for current-milestone staleness
python3 ./adl/tools/check_repo_quality_staleness.py --milestone v0.91.6

# inspect the public coverage contract and exact-commit proof requirements
# docs/tooling/COVERAGE_AUTHORITY_AND_RELEASE_PROOF.md

# remove local generated coverage artifacts after they are no longer needed
./adl/tools/clean_coverage_artifacts.sh --include-worktrees

# report large Rust source and test modules without failing the build
./adl/tools/report_large_rust_modules.sh

# The following v2 commands apply only to explicitly authorized rollback or
# bounded transition remediation. Default native v3 routing: docs/default_workflow.md.
# edit and validate prompt cards through typed requests
.adl/bin/csdlc-v2/csdlc-edit --repo <worktree> apply --request <edit-request.json>
.adl/bin/csdlc-v2/csdlc-validate --root <worktree> finalize --request <finalize-request.json>

# finalize, review, and publish through typed requests
.adl/bin/csdlc-v2/csdlc-validate --root <worktree> finalize --request <finalize-request.json>
.adl/bin/csdlc-v2/csdlc-review record --request <review-request.json>
.adl/bin/csdlc-v2/csdlc-publish publish --request <publication-request.json>
```

## Compatibility / Maintenance Surfaces

The repo still carries a few compatibility and maintenance entrypoints, but they
are not the preferred public workflow:

- v1 lifecycle aliases are removed and must not be used for issue work
- `adl_pr_cycle` install/resync helpers are retained only for historical or
  compatibility inspection, not for current native v3 lifecycle operation
- `codex_pr.sh` and `codexw.sh` have been removed after their fail-closed
  migration period. Use the current native C-SDLC authority and run Codex
  directly in its bound worktree; historical v2 guidance is not current routing.

## See Also / Canonical Docs

- Root project entrypoint: `../../README.md`
- Runtime/CLI usage: `../README.md`
- Operational skills guide: `skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
  - includes operational skill and helper card-editor usage patterns; native v3
    authority remains defined by `../../docs/csdlc-v3/CURRENT_AUTHORITY.md`
  - includes the full skill documentation matrix, install/resync guidance, and
    deployed-copy verification recipe for `$CODEX_HOME/skills`
  - when touching `stp.md`, `sip.md`, or `sor.md`, use the matching editor
    skill explicitly rather than doing ad hoc card edits
- Active milestone docs: `../../docs/milestones/v0.87/`
- Documentation index: `../../docs/README.md`
- ADRs: `../../docs/adr/`
- ADL spec entrypoint: `../../adl-spec/README.md`

## Retired In v0.87 Cleanup

The following low-confidence legacy residue was retired during the bounded `adl/tools`
cleanup because it no longer backed the live PR control plane, active demos, or
current regression-test surfaces:

- `BURST_PLAYBOOK.md`
- `REPORT_SCHEMA.md`
- `default.rules.profiles.example`
- `demo_v0_4.sh`
- `pr_smoke.sh`

## Historical original-layout diff recipe

The original monorepo documentation contract requires this exact recipe. It is
retained as historical evidence only; the source directory has been retired. Use
the explicit authenticated owner resource comparison above for current copies.

```bash
# Historical only; not executable against the split ADL checkout.
diff -qr adl/tools/skills/test-generator "$CODEX_HOME/skills/test-generator"
```

## Historical v0.91.5 packet validation

The three `validate_v0915_*` validators for `openrouter_matrix`,
`remote_gemma_watcher_probe` and `multi_agent_quality_comparison` validate the
retained ADL review packets and their repository-relative evidence references.
They do not repeat the original provider experiments or establish current
provider readiness. Keep the immutable packets under
`docs/milestones/v0.91.5/review/` unchanged.

The corresponding three shell tests exercise packet acceptance and mutated-copy
refusals. `test_v0915_openrouter_matrix.sh` now runs only its original two valid
packet checks and two negative mutations. Its former credential-dependent live
runner assertions and three monolithic Cargo selectors remain historical
obligations in #1223 custody; an offline packet pass does not satisfy them.
The two quality/watcher test bodies remain unchanged.

The legacy provider demo shell family was retired in #1223 with its original assertions in Git and custody. Owner source delivery does not establish new live-provider, transcript, musical-performance or archive acceptance. Retained audio rendering requires an explicit existing source episode through `ADL_HEY_JUDE_AUDIO_SOURCE_DIR`; it does not regenerate an episode through a removed ADL command.
