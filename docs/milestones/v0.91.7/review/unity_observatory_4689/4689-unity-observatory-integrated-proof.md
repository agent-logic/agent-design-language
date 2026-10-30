# #4689 Unity Observatory Integrated Proof

Date: 2026-07-11
Issue: #4689
WP: WP-09

## Result

PASS: the v0.91.7 Unity Observatory proof chain is integrated for review and
operator consumption.

This packet ties together the currently retained Unity Observatory proof
surfaces:

- #4703 flagship scene/stage proof:
  `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-stage-proof.md`
- #4704 operator walkthrough:
  `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-operator-walkthrough.md`
- #4704 Unity-MCP proof summary:
  `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-unity-mcp-proof-summary.md`
- #4652 runtime polis shell proof:
  `docs/milestones/v0.91.7/review/unity_observatory_4652/4652-unity-shell-proof-summary.md`
- #4745 asset and Unity-MCP publication policy:
  `docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-policy.md`

## Integrated Path

The integrated Unity Observatory path is:

1. The repository contains the canonical Unity Observatory scaffold under
   `demos/v0.91.6/unity-observatory/`.
2. #4745 defines the publication boundary: imported third-party asset roots are
   external/operator-provisioned inputs, while owned scripts, project metadata,
   proof summaries, and retained visual evidence are publishable repo assets.
3. #4703 proves the flagship observatory stage scene through local Unity-MCP
   editor automation and retains a 1920x1080 hero image.
4. #4652 proves the runtime polis shell can be instantiated in the flagship
   scene and retains a 1600x900 shell image.
5. #4704 proves an operator walkthrough path for the scene, endpoint binding,
   runtime/polis objects, and a retained 1920x1080 wide-camera image.
6. #4689 adds the executable rollup check:
   `bash adl/tools/test_v0917_unity_observatory_integrated_proof.sh`.

## Retained Visual Evidence

- `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-investor-hero.png`
  - Expected dimensions: 1920 x 1080
- `docs/milestones/v0.91.7/review/unity_observatory_4704/flagship-wide-observatory-camera-4704.png`
  - Expected dimensions: 1920 x 1080
- `docs/milestones/v0.91.7/review/unity_observatory_4652/flagship-shell-main-camera-4652.png`
  - Expected dimensions: 1600 x 900

## Validation

Run:

```bash
bash adl/tools/test_v0917_unity_observatory_integrated_proof.sh
```

The proof script verifies:

- all retained #4689/#4703/#4704/#4652/#4745 proof packets used by the
  rollup exist, including the #4704 walkthrough and Unity-MCP summary
- the #4745 manifest preserves the external/operator-provisioned asset route
- retained PNG proof artifacts have the expected PNG signatures and dimensions
- the Unity Observatory scaffold and proof packet still contain the required
  publication-boundary language
- the rollup does not convert local Unity-MCP editor proof into runtime or
  player-build readiness

## Consumption

WP-09 can consume this packet as the Unity Observatory integrated proof rollup.
The path is operator-usable with the retained evidence already in the repo. Full
local scene replay remains available to an operator who provisions the same
Unity Asset Store packages into the roots listed by #4745.

## Residual Risk

- Full imported-asset replay is not clean-checkout self-contained.
- Unity-MCP remains local editor/proof tooling, not runtime demo state.
- Endpoint values from prior Unity-MCP sessions are session proof facts, not
  globally stable service addresses.

## Non-Claims

- This packet does not grant redistribution rights for third-party Unity
  assets.
- This packet does not claim a clean Git checkout can replay the full imported
  flagship environment without operator-provisioned asset packs.
- This packet does not claim Unity player-build readiness.
- This packet does not claim cloud MCP connectivity is required.
- This packet does not claim final investor polish beyond the retained proof
  images and walkthrough packets listed above.
