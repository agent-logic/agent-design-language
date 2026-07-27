# Issue 5683 Live Unity Proof

Observed at `2026-07-27T01:54:09Z` from the operator-provisioned Unity 6.5 project:

- Project: `/Volumes/FastWork/adl-unity-observatory/operator-provisioned-5332/unity-observatory`
- Scene: `Assets/Scenes/FlagshipObservatoryStage.unity`
- Local MCP endpoint: `http://localhost:23011`
- Play Mode: entered successfully through Unity-MCP.
- Alignment: repository probe passed with exact project identity, loopback endpoint, permission-safe PID corroboration, and opened-scene proof.
- Stage validation: passed with 43 prefab instances, 96 game objects, 4 cameras, and 7 lights.
- Contract guardrail: `bash adl/tools/test_v0916_unity_observatory_contract.sh` passed.
- Diff hygiene: `git diff --check` passed.

## Retained Visual Proof

- `final-full-hd-game-view.png`: direct Unity-MCP Game View capture at 1920x1080.
- `final-qhd-game-view.png`: direct Unity-MCP Game View capture at 2560x1440.
- `raw-environment-full-hd-pass-3.png`: direct investor camera render at 1920x1080.
- `raw-environment-qhd-pass-3.png`: direct investor camera render at 2560x1440.

The final captures were visually inspected. The prior cyan and green square-particle corruption is absent. The command shell remains truthfully marked `DEMO DATA`, `CONTRACT ONLY`, and `FIXTURE PROJECTION`; no live runtime or cloud authority is claimed.
