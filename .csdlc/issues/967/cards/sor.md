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
Status: ready
Generated: 2026-09-12

Execution:
- Actor: `Codex issue967 team`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `unknown`
- End Time: `unknown`

## Summary

#967 implementation is qualified: focused Runtime 1/1, OpenAPI 11/11, zero-paid matrices 5/5 and 3/3, required CI run 34685454688 green, and authorized hosted acceptance 3/3 with 16 paid requests under configured caps. PR #968 remains open pending refreshed exact-head review/publication and operator-controlled merge.

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
- Completion state: `implemented_pending_merge`
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
- Tracked implementation artifacts: `adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; docs/runtime-v3/fixtures/issue967/EXACT_HEAD_ZERO_PAID_PROOF.json; docs/runtime-v3/fixtures/issue967/EXACT_HEAD_HOSTED_ACCEPTANCE_PROOF.json; issue-local cards and proof metadata.`
- Additional proof artifacts: `docs/runtime-v3/fixtures/issue967/EXACT_HEAD_ZERO_PAID_PROOF.json remains unchanged; docs/runtime-v3/fixtures/issue967/EXACT_HEAD_HOSTED_ACCEPTANCE_PROOF.json records the separate hosted pass. Raw hosted report .adl/issue967/hosted-live-02/report.json SHA256 77eed554c7d06a3f5d130d16079d5b7a2f494ad67a21487a6b40ad2399d70152. Historical #855 hosted-live-03 remains unchanged.`

## Actions taken
- `Prepared bounded requested_agent_action repair with pre-dispatch validation and replay identity.`
- `Reused signed A2A delivery; coalesced identical actions and refused conflicts before peer dispatch.`
- `Qualified the exact source with focused Runtime 1/1, OpenAPI 11/11, zero-paid 5/5 and 3/3 matrices, green required CI run 34685454688, and one authorized hosted acceptance passing OpenAI, Anthropic, and Vertex AI with 16 paid requests under configured caps.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; primary main inspection-only`
- Worktree-only paths remaining: `PR #968 branch paths remain unmerged; no issue files exist on primary main.`
- Integration state: `pr_open`
- Verification scope: `#967 typed action, replay, signed dispatch, OpenAPI and bounded lifecycle proof only.`
- Integration method used: `PR #968 against main; merge remains operator-controlled.`
- Verification performed:
  - `gh pr checks 968 --repo agent-logic/agent-design-language; native exact-head review and publish after this evidence commit.`
    `Required CI run 34685454688 passed at 6d7bc805e5714050c85c80d5e9351e40cd10d635; merge and terminal closeout are not claimed.`
- Result: `PR #968 is open with required CI green at pre-evidence head; hosted evidence update and exact-head publication refresh are pending.`

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
  - `Focused Runtime dispatch test 1/1; OpenAPI contract test 11/11; zero-paid five-provider and hosted-topology harnesses; gh pr checks 968 --repo agent-logic/agent-design-language; one authorized issue855_provider_lifecycle.py --hosted-approved run at source 6d7bc805e5714050c85c80d5e9351e40cd10d635, retained at .adl/issue967/hosted-live-02/report.json.`
    `Proves bounded typed request validation and replay behavior locally and successful canonical A2A initiation through OpenAI, Anthropic, and Vertex AI in one authorized hosted run.`
- Results:
  - `Pass: Runtime production dispatch 1/1; OpenAPI contracts 11/11; zero-paid matrices 5/5 and 3/3; required CI run 34685454688 green; authorized hosted acceptance 3/3 with 16 paid requests (OpenAI 6, Anthropic 5, Vertex 5) and five local Ollama requests.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: pass_pending_post_metadata_exact_head_refresh
    checks_run:
      - "Hosted report SHA256 77eed554c7d06a3f5d130d16079d5b7a2f494ad67a21487a6b40ad2399d70152: 3/3 provider rows pass; 16 paid requests and five local Ollama requests; canonical beacon.axioma replies present; stable Runtime identity; owned process cleanup recorded."
  determinism:
    status: pass_for_declared_scope
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: pass_for_tracked_packet
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: complete_for_open_pr
    required_artifacts_present: true
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
- Secret leakage scan performed: `Passed: scoped changed and newly tracked JSON/Markdown were parsed and scanned for common provider key, OAuth token, and private-key patterns; independent review also found no credential or response leakage.`
- Prompt / tool argument redaction verified: `Passed: the tracked hosted packet contains no credentials, provider response bodies, or command arguments carrying secret values.`
- Absolute path leakage check: `Passed for the scoped tracked hosted packet and changed issue artifacts; repository-relative evidence paths are used, while canonical worktree identity remains only in required lifecycle fields.`
- Sandbox / policy invariants preserved: `Bound FastWork checkout only; no primary issue writes or credential/cloud changes.`

## Replay Artifacts
- Trace bundle path(s): `Corrective evidence under .adl/issue967; inherited failed/proof attempts retained under .adl/issue855 in original bound worktree.`
- Run artifact root: `.adl/issue967`
- Replay command used for verification: `Re-run only zero-paid proof without new authorization: focused Runtime dispatch test, OpenAPI contract test, and local fixture harnesses. The paid hosted acceptance is retained evidence and is not a replay instruction.`
- Replay result: `Pass: identical typed action returns cached terminal result with no calls; changed typed action under the same turn identity returns conversation_conflict with no calls; omitted optional field preserves legacy serialized shape.`

## Artifact Verification
- Primary proof surface: `docs/runtime-v3/fixtures/issue967/EXACT_HEAD_ZERO_PAID_PROOF.json; docs/runtime-v3/fixtures/issue967/EXACT_HEAD_HOSTED_ACCEPTANCE_PROOF.json; adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; docs/api/runtime-v3/v1/observatory.openapi.json.`
- Required artifacts present: `true for implementation, local validation, hosted acceptance, independent review, and CI; merge/terminal records are lifecycle-pending`
- Artifact schema/version checks: `OpenAPI contracts pass 11/11; native six-card validation follows this typed edit. Required CI run 34685454688 is green.`
- Hash/byte-stability checks: `Raw hosted report SHA256 77eed554c7d06a3f5d130d16079d5b7a2f494ad67a21487a6b40ad2399d70152 and installed binary digests are retained in the separate hosted packet. The prior zero-paid packet and #855 failures were not overwritten.`
- Missing/optional artifacts and rationale: `No required execution or hosted proof is waived. Merge and terminal closeout remain later lifecycle states.`

## Decisions / Deviations
- `Explicit authenticated typed intent replaces reliance on provider formatting; arbitrary prose parsing is excluded.`
- `Correction belongs to #967 after #855/PR964 merged; preserve historical chronology.`

## Follow-ups / Deferred work
- `Commit the hosted proof and typed card reconciliation, then refresh exact-head independent review, native review/publication, and CI for PR #968.`
- `Merge remains operator-controlled. After merge, use native finish and clean separately, then reconcile Sprint 2 umbrella #928.`
