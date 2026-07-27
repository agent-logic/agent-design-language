# Structured Review Prompt

Template: 1.0.0

Issue: 5683

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/5683/LIVE_UNITY_PROOF.md
.csdlc/evidence/5683/TOOLING_ANOMALIES.md
.csdlc/evidence/5683/capture_unity_mcp_image.mjs
.csdlc/evidence/5683/final-full-hd-game-view.png
.csdlc/evidence/5683/final-qhd-game-view.png
.csdlc/evidence/5683/raw-environment-full-hd-pass-3.png
.csdlc/evidence/5683/raw-environment-qhd-pass-3.png
.csdlc/evidence/5683/record-execution-request.json
.csdlc/evidence/5683/record-validation-alignment-request.json
.csdlc/evidence/5683/record-validation-contract-request.json
.csdlc/evidence/5683/record-validation-diff-request.json
.csdlc/evidence/5683/record-validation-live-unity-request.json
.csdlc/issues/5683/audit.jsonl
.csdlc/issues/5683/cards/sip.md
.csdlc/issues/5683/cards/sip.values.json
.csdlc/issues/5683/cards/sor.md
.csdlc/issues/5683/cards/sor.values.json
.csdlc/issues/5683/cards/spp.md
.csdlc/issues/5683/cards/spp.values.json
.csdlc/issues/5683/cards/srp.md
.csdlc/issues/5683/cards/srp.values.json
.csdlc/issues/5683/cards/stp.md
.csdlc/issues/5683/cards/stp.values.json
.csdlc/issues/5683/cards/vpp.md
.csdlc/issues/5683/cards/vpp.values.json
.csdlc/issues/5683/index.json
.csdlc/locks/5683.lock
.csdlc/prepared/issues/5683/advance-implemented.json
.csdlc/prepared/issues/5683/design.md
.csdlc/prepared/issues/5683/diagram.mmd
adl/tools/test_v0916_unity_observatory_contract.sh
demos/v0.91.6/unity-observatory/Assets/Editor/UnityObservatoryFlagshipStageBuilder.cs
demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs

## Prompts

- Does either target-resolution hero frame retain any corrupted-looking cyan or green geometry, particle, material, or lighting artifact?
- Are the architecture, terrain, camera, and lighting composition coherent and investor-presentable?
- Does the operator shell remain fixed, legible, non-overlapping, and truthful about runtime state?
- Does direct evidence prove the intended Unity project and scene rather than a repository-only approximation?
- Does the proof index link the actual demo evidence rather than lifecycle metadata?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The static shell guardrail remains source-presence based, so future visual regressions still require direct live Unity proof.
- Game View selection uses undocumented UnityEditor reflection APIs and may need adjustment after a Unity upgrade.
- The clean hero composition deliberately deactivates substantial imported presentation content; future investor-polish work should restore selected assets deliberately.
- Runtime remains explicitly DEMO DATA, CONTRACT ONLY, and FIXTURE PROJECTION; this review does not claim live CSM integration.

## Review Result

Revision: Some("git-blake3:757d75f6a346f8671dc075d8b32ed0f4ea0f8191:cec4e56a2f8814813913a097d0fea5a676ca5ca790133dc8e2ff3cebb3995242")

Reviewer: Some("Hooke")

Result: pass
