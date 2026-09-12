# issue-967-deterministic-hosted-a2a

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

Task ID: issue-0967
Run ID: issue-0967
Version: v0.92.2
Title: [v0.92.2][RT-PROVIDER][corrective] Make hosted A2A initiation deterministic across provider output formats
Branch: codex/967-deterministic-hosted-a2a
Card Status: ready
Status: in_progress
Generated: 2026-09-12

Execution:
- Actor: `Codex issue967 team`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `unknown`
- End Time: `unknown`

## Summary

Exact-source validation at 4d28de627cffc7c529de5082f0b883f9ab8f083c: Runtime production dispatch 1/1; OpenAPI contracts 11/11; zero-paid five-provider matrix 5/5 with 31 provider requests; zero-paid hosted-topology matrix 3/3 with 15 target-provider requests plus 4 local Ollama fixture requests, 19 total. Independent exact-source review passed with no actionable findings. Paid hosted acceptance and CI remain pending.

## PVF Lane Truth
- Initial PVF lane: `provider`
- Planned PVF lane: `provider`
- Final PVF lane: `provider`
- Lane change reason: `No lane change; focused Runtime behavior and separate hosted qualification remain provider lane.`

## Issue Metrics Truth
- Expected runtime class: `unknown`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `No explicit token budget supplied for this card preparation.`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `in_progress`
- Estimate error percent: `unknown`
- Completion state: `in_progress`
- Issue goal ref: `issue-967`
- Sprint goal ref: `issue-928`
- Goal metrics rollup ref: `unknown`
- Validation planning prompt: `.csdlc/issues/967/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `false`
- Variance category: `unknown`
- Variance note: `Exact session metric rollup pending; no fabricated timings or token totals.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/967/cards/sor.md`
- Tracked implementation artifacts: `adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; docs/runtime-v3/fixtures/issue967/EXACT_HEAD_ZERO_PAID_PROOF.json; issue-local cards and proof metadata.`
- Additional proof artifacts: `docs/runtime-v3/fixtures/issue967/EXACT_HEAD_ZERO_PAID_PROOF.json; raw reports .adl/issue967/installed-run-01/report.json SHA256 b2a0c430578a1936f5cd03441e95b7683ba88acf3dfbd1e85544a7ae9c654bc0 and .adl/issue967/hosted-fixture-01/report.json SHA256 75b237976ba5891f1faf628fd4a5c53a10e007ae0efe41f5b577742fe2b10740. Historical #855 hosted-live-03 report remains unchanged.`

## Actions taken
- `Prepared bounded requested_agent_action repair with pre-dispatch validation and replay identity.`
- `Reused signed A2A delivery; coalesced identical actions and refused conflicts before peer dispatch.`
- `Exact-source validation at 4d28de627cffc7c529de5082f0b883f9ab8f083c: Runtime production dispatch 1/1; OpenAPI contracts 11/11; zero-paid five-provider matrix 5/5 with 31 provider requests; zero-paid hosted-topology matrix 3/3 with 15 target-provider requests plus 4 local Ollama fixture requests, 19 total. Independent exact-source review passed with no actionable findings. Paid hosted acceptance and CI remain pending.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; primary main inspection-only`
- Worktree-only paths remaining: `adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; docs/runtime-v3/fixtures/issue967/EXACT_HEAD_ZERO_PAID_PROOF.json; issue-local cards and proof metadata.`
- Integration state: `worktree_only`
- Verification scope: `#967 typed action, replay, signed dispatch, OpenAPI and bounded lifecycle proof only.`
- Integration method used: `none; corrective PR pending`
- Verification performed:
  - `Pending exact-head native review/publication and current GitHub observation.`
    `No corrective integration acceptance yet.`
- Result: `No #967 integration performed; #855/PR964 baseline already merged.`

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
  - `CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --lib sixth_registered_provider_uses_real_canonical_a2a_dispatch; CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract; bounded zero-paid lifecycle harness using typed requested_agent_action. Exact-head installed fixture command and paid command must be recorded with their source/binary identities before execution.`
    `Proves bounded typed request dispatch, rejection, compatibility and schema behavior locally; not hosted provider success.`
- Results:
  - `Exact-source validation at 4d28de627cffc7c529de5082f0b883f9ab8f083c: Runtime production dispatch 1/1; OpenAPI contracts 11/11; zero-paid five-provider matrix 5/5 with 31 provider requests; zero-paid hosted-topology matrix 3/3 with 15 target-provider requests plus 4 local Ollama fixture requests, 19 total. Independent exact-source review passed with no actionable findings. Paid hosted acceptance and CI remain pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: in_progress
    checks_run:
      - "Exact-source validation at 4d28de627cffc7c529de5082f0b883f9ab8f083c: Runtime production dispatch 1/1; OpenAPI contracts 11/11; zero-paid five-provider matrix 5/5 with 31 provider requests; zero-paid hosted-topology matrix 3/3 with 15 target-provider requests plus 4 local Ollama fixture requests, 19 total. Independent exact-source review passed with no actionable findings. Paid hosted acceptance and CI remain pending."
  determinism:
    status: partial
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: in_progress
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: in_progress
    required_artifacts_present: false
    schema_changes:
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed: `Actual Runtime regression covers zero-call invalid actions, ordinary replies with typed action, identical-action coalescing, conflict refusal, existing model actions and replay compatibility.`
- Fixtures or scripts used: `adl/tools/issue855_provider_lifecycle.py; actual Runtime production-dispatch and OpenAPI tests.`
- Replay verification (same inputs -> same artifacts/order): `Corrective regression covers action inclusion and absent-field compatibility; byte-identical randomized fixture artifacts are not claimed.`
- Ordering guarantees (sorting / tie-break rules used): `Validate before provider execution; bind action in replay identity; reconcile action after one reply and before existing signed peer dispatch.`
- Artifact stability notes: `Preserve prior failed reports; generate distinct corrective proof, never overwrite #855 historical evidence.`

## Security / Privacy Checks
- Secret leakage scan performed: `Final corrective tracked artifact scan pending.`
- Prompt / tool argument redaction verified: `No new raw provider response retention; final exact-head redaction verification pending.`
- Absolute path leakage check: `Use repository-relative artifact references; native binding retains required checkout identity; final scan pending.`
- Sandbox / policy invariants preserved: `Bound FastWork checkout only; no primary issue writes or credential/cloud changes.`

## Replay Artifacts
- Trace bundle path(s): `Corrective evidence under .adl/issue967; inherited failed/proof attempts retained under .adl/issue855 in original bound worktree.`
- Run artifact root: `.adl/issue967`
- Replay command used for verification: `CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --lib sixth_registered_provider_uses_real_canonical_a2a_dispatch; CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract; bounded zero-paid lifecycle harness using typed requested_agent_action. Exact-head installed fixture command and paid command must be recorded with their source/binary identities before execution.`
- Replay result: `Pass: identical typed action returns cached terminal result with no calls; changed typed action under the same turn identity returns conversation_conflict with no calls; omitted optional field preserves legacy serialized shape.`

## Artifact Verification
- Primary proof surface: `docs/runtime-v3/fixtures/issue967/EXACT_HEAD_ZERO_PAID_PROOF.json; adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; docs/api/runtime-v3/v1/observatory.openapi.json.`
- Required artifacts present: `false; paid hosted acceptance and CI remain pending`
- Artifact schema/version checks: `OpenAPI contracts pass 11/11 at source revision 4d28de627cffc7c529de5082f0b883f9ab8f083c; native six-card validation follows this edit.`
- Hash/byte-stability checks: `Proof summary records exact source and binary digests. Raw report SHA256 values: b2a0c430578a1936f5cd03441e95b7683ba88acf3dfbd1e85544a7ae9c654bc0 and 75b237976ba5891f1faf628fd4a5c53a10e007ae0efe41f5b577742fe2b10740.`
- Missing/optional artifacts and rationale: `Required exact-head, CI and hosted evidence is not waived or classified as optional.`

## Decisions / Deviations
- `Explicit authenticated typed intent replaces reliance on provider formatting; arbitrary prose parsing is excluded.`
- `Correction belongs to #967 after #855/PR964 merged; preserve historical chronology.`

## Follow-ups / Deferred work
- `Publish #967 after native review and obtain current CI. Obtain fresh authorization before one bounded paid hosted acceptance run.`
- `Record the separately authorized hosted result without overwriting failed #855 historical evidence; merge remains operator-controlled.`
