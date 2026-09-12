# observatory-deploy

Canonical Template Source: `docs/templates/prompts/1.0.5/sor.md`

Authority notice: C-SDLC v3 is operational after V3-F/#505 and merged PR #591.
Authority requires the authenticated canonical native selector and reconciliation
receipt; missing or stale proof suspends authority. Retained typed v2 requires
explicit issue-scoped rollback or remediation approval.
Legacy `pr` editor routes are historical/retired compatibility orientation,
not current lifecycle authority.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-0910
Run ID: issue-0910
Version: 1.0.5
Title: [v0.92.2][OBS-S3] Deploy the existing Observatory S3 and CloudFront sidecar
Branch: codex/910-observatory-deploy
Card Status: ready
Status: deployed_validated_awaiting_final_review_and_pr
Generated: 2026-09-12T01:48:44.174761+00:00

Execution:
- Actor: `Sprint 8 issue #910 worker`
- Model: `not separately recorded`
- Provider: `OpenAI`
- Start Time: `not separately measured for preparation`
- End Time: `not separately measured for preparation`

## Summary

Approved no-restart origin append and exact 18-create deployment complete. Four versioned assets, HTTPS headers/hashes, exact Completed invalidation, private AWS posture and rollback reconstruction verified. Real Chrome accepted v3 frame/render/WSS and unauthenticated write gating pass with approved temporary site permission and documented Connect URL.

## PVF Lane Truth
- Initial PVF lane: `tooling and read-only cloud preflight`
- Planned PVF lane: `tooling and read-only cloud preflight`
- Final PVF lane: `tooling and read-only cloud preflight`
- Lane change reason: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`

## Issue Metrics Truth
- Expected runtime class: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Estimated elapsed seconds: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Actual elapsed seconds: `not separately measured for preparation`
- Actual active work seconds: `not separately measured for preparation`
- Estimated total tokens: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Actual total tokens: `not separately measured for preparation`
- Estimated validation seconds: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Actual validation seconds: `not separately measured for preparation`
- Actual PR wait seconds: `not separately measured for preparation`
- Actual CI wait seconds: `not separately measured for preparation`
- Budget source: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Goal metrics data source: `not separately measured for preparation`
- Goal metrics source ref: `not separately measured for preparation`
- Data-source confidence: `not separately measured for preparation`
- Estimate error percent: `not separately measured for preparation`
- Completion state: `Deployed acceptance evidence complete; final independent review and PR/checks pending`
- Issue goal ref: `Active #910 full-delivery goal: approved origin hot reload, exact static deployment, live browser proof and reviewed PR; no destructive changes or Runtime compute`
- Sprint goal ref: `Sprint 8 umbrella #934`
- Goal metrics rollup ref: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Validation planning prompt: `.csdlc/issues/910/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Variance analysis completed: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Variance category: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Variance note: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/910/cards/sor.md`
- Tracked implementation artifacts: `.csdlc/evidence/910 and native issue cards only; existing infrastructure and application source unchanged`
- Additional proof artifacts: `terraform-plan-summary.json, assets.json, upload-manifest.json, named-resource-preflight.json, EXECUTION_RUNBOOK.md`

## Actions taken
- `Atomically appended only approved browser origin; watcher hash advanced and process/instance identities unchanged; health/CORS200 and raw WSS101 verified`
- `Applied saved plan with 18 creates and preserved state backup; uploaded four assets with exact-version SHA256/cache/type checks and index last; exact invalidation Completed`
- `Deployed posture/HTTPS/rollback and real browser proof pass; initial verifier/LNA failures preserved and resolved without product/CSP changes`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `Deployment evidence/native cards in issue worktree; private Terraform state/asset/version receipts preserved in stable Git-common custody`
- Integration state: `AWS deployment applied; repository PR not opened`
- Verification scope: `Actual deployed AWS/HTTPS/rollback plus accepted live v3 frame/render and unauthenticated write gating; no mocks or privileged commands`
- Integration method used: `Approved saved Terraform plan and guarded S3 uploads/invalidation`
- Verification performed:
  - `Authenticated AWS fixed-projection reads and exact-version readbacks; public HTTPS SHA256/header checks; Chrome diagnostic`
    `Deployed infrastructure/content and operator-local browser read acceptance proven`
- Result: `18 creates, zero updates/deletes; four objects verified; CloudFront Deployed; no repository PR`

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout BRANCH -- PATH` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- Integration state describes lifecycle state of the integrated artifact set, not where verification happened.
- Verification scope describes where the verification commands were run.
- worktree_only means at least one required path still exists only outside the main repository path.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By native v3 `csdlc finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `node --test demos/html-observatory/tests/*.test.mjs; terraform fmt -check; terraform validate; terraform plan; native validate`
    `Actual deployment/browser evidence verified for operator-local Runtime; no authenticated-write or universal remote-reachability claim`
- Results:
  - `24 local UI tests pass; real deployed AWS/content/invalidation/rollback proof pass; unmocked Chrome public-read v3 frame/render/WSS and disabled unauth write controls PASS`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: preparation checked; deployed proof pending
    checks_run:
      - "preparation checked; deployed proof pending"
  determinism:
    status: preparation checked; deployed proof pending
    replay_verified: preparation checked; deployed proof pending
    ordering_guarantees_verified: preparation checked; deployed proof pending
  security_privacy:
    status: preparation checked; deployed proof pending
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: preparation checked; deployed proof pending
    required_artifacts_present: preparation checked; deployed proof pending
    schema_changes:
      present: preparation checked; deployed proof pending
      approved: preparation checked; deployed proof pending
```

## Determinism Evidence
- Determinism tests executed: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Fixtures or scripts used: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Replay verification (same inputs -> same artifacts/order): `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Ordering guarantees (sorting / tie-break rules used): `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Artifact stability notes: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`

## Security / Privacy Checks
- Secret leakage scan performed: `public packet fixed-field scan passed`
- Prompt / tool argument redaction verified: `No credentials or raw cloud response bodies included in public packet`
- Absolute path leakage check: `Public artifacts use repository-relative or Git-common-relative references`
- Sandbox / policy invariants preserved: `Business profile only; no cloud writes; private local artifacts preserved`

## Replay Artifacts
- Trace bundle path(s): `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Run artifact root: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Replay command used for verification: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Replay result: `Not claimed in preparation handoff; see .csdlc/evidence/910/DEPLOYMENT_PLAN.md`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/910/DEPLOYMENT_PLAN.md`
- Required artifacts present: `Preparation evidence present; deployed proof remains pending`
- Artifact schema/version checks: `All public JSON parses; native six-card validation passes`
- Hash/byte-stability checks: `Actual saved plan SHA256 and four upload objects verified; source code bytes unchanged`
- Missing/optional artifacts and rationale: `No authenticated command execution claimed; historical report objects intentionally absent; final review/PR/checks pending`

## Decisions / Deviations
- `Override unavailable package parent hosted zone with existing csm.agent-logic.ai zone`
- `Use operator-approved stable private Git-common local state custody; no remote backend change`

## Follow-ups / Deferred work
- `Final independent exact-candidate review and native review/publication`
- `Monitor exact-head PR checks; preserve private state on merge/worktree cleanup`
