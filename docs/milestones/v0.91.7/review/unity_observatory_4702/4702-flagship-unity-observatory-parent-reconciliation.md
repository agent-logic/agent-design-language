# #4702 Flagship Unity Observatory Parent Reconciliation

Date: 2026-07-11

## Result

PASS: #4702 now acts as the truthful parent issue for the flagship Unity
Observatory mini-sprint instead of a monolithic Unity implementation bucket.

The live #4702 issue body was repaired through the repo-native
`adl pr repair-issue-body` path so the parent/child issue graph is visible on
GitHub and in the local source prompt.

## Child Wave

| Order | Issue | Role | Current result |
| --- | --- | --- | --- |
| 1 | `#4652` | Unity shell integration and demo-surface child | Closed. Retained proof records flagship scene loading, runtime polis shell instantiation, investor lighting, and retained camera evidence. |
| 2 | `#4703` | Flagship environment staging from imported asset packs | Closed. Retained proof records the staged `FlagshipObservatoryStage` scene, validation pass, object counts, and 1920x1080 investor hero proof. |
| 3 | `#4704` | Reproducible Unity-MCP proof and operator walkthrough | Closed. Retained proof records endpoint binding, scene loading, runtime/polis object presence, walkthrough packet, and nonblank retained camera proof. |

## Retained Evidence

- `docs/milestones/v0.91.7/review/unity_observatory_4652/4652-unity-shell-proof-summary.md`
- `docs/milestones/v0.91.7/review/unity_observatory_4652/flagship-shell-main-camera-4652.png`
- `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-stage-proof.md`
- `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-investor-hero.png`
- `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-unity-mcp-proof-summary.md`
- `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-operator-walkthrough.md`
- `docs/milestones/v0.91.7/review/unity_observatory_4704/flagship-wide-observatory-camera-4704.png`
- `docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-policy.md`
- `docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-manifest.json`

## Issue-Graph Repair

Repo-native issue-body repair command:

```bash
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
  adl pr repair-issue-body 4702 \
  --body-file .adl/v0.91.7/bodies/issue-4702-v0-91-7-unity-sprint-build-flagship-unity-observatory-demo-mini-sprint.md \
  --version v0.91.7 \
  --force
```

Verification:

```bash
bash adl/tools/pr.sh issue view 4702 --json
rg -n "#4652|#4703|#4704" \
  .adl/v0.91.7/bodies/issue-4702-v0-91-7-unity-sprint-build-flagship-unity-observatory-demo-mini-sprint.md \
  .adl/v0.91.7/tasks/issue-4702__v0-91-7-unity-sprint-build-flagship-unity-observatory-demo-mini-sprint/stp.md
```

The repaired live issue body and local source prompt both name `#4652`,
`#4703`, and `#4704` as the child wave.

## Execution Boundary

#4702 does not re-run Unity, perform player builds, vendor third-party assets,
or absorb child implementation work. It reconciles parent truth after the child
proofs landed.

The child proofs establish:

- Unity shell and runtime-polis surface evidence through #4652
- flagship scene staging evidence through #4703
- Unity-MCP walkthrough and retained camera evidence through #4704
- asset and Unity-MCP publication boundaries through #4745

The parent keeps these non-claims explicit:

- no Unity player-build readiness
- no redistribution rights for third-party Unity Asset Store assets
- no clean-checkout replay of imported third-party asset packs
- no claim that the parent issue itself proves fresh Unity runtime behavior
- no v0.92 activation readiness claim

## Follow-On Boundary

Future Unity work such as deeper runtime binding, polis/relativity environment
work, or clean-checkout asset replay must be promoted as separate child issues
instead of being hidden under this parent.
