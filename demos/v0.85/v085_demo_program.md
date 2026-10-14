# ADL v0.85 Demo Program

The Gödel/AEE shell commands below are historical ADL monolith interfaces, retired in #1223. Original scenario and artifact requirements remain historical obligations; this page does not establish a current execution or equivalent Runtime acceptance. Preserved owner source is available in the [Runtime checkout](https://github.com/agent-logic/agent-logic-runtime/tree/7d0db620ad79c50ff5d8fabf0fb543183583d05c/adl/tools) and [Infrastructure archive](https://github.com/agent-logic/agent-logic-infrastructure/tree/ed02d55985482a0dbb1469951d36efef931b63e2/adl/tools).

This is the canonical reviewer order for the `v0.85` milestone demos.

The goal is not to run every bounded script blindly. The goal is to show the
milestone's strongest claims in a legible order with clear proof surfaces.

## Primary reviewer sequence

### 1. Steering / Queueing / Checkpoint

```bash
adl/tools/demo_steering_queue_checkpoint.sh
```

Review:
- [demos/v0.85/steering_queue_checkpoint_demo.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/demos/v0.85/steering_queue_checkpoint_demo.md)

Primary proof:
- `.adl/runs/v0-85-hitl-steering-demo/pause_state.json`
- `.adl/runs/v0-85-hitl-steering-demo/run.json`

### 2. HITL Editor / Review Workflow

```bash
adl/tools/demo_hitl_editor_review.sh
```

Review:
- `demos/v0.85/hitl_editor_review_demo.md`
- `docs/tooling/editor/demo.md`

Primary proof:
- `docs/records/v0.85/tasks/task-v085-wp05-demo/`
- `.adl/reports/demo-hitl-editor-review/editor_review_demo_manifest.v1.json`

### 3. AEE Bounded Adaptation

```bash
# Historical retired command: adl/tools/demo_aee_bounded_adaptation.sh
```

Review:
- [demos/aee-recovery/README.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/demos/aee-recovery/README.md)

Primary proof:
- `.adl/runs/v0-3-aee-recovery-initial/learning/aee_decision.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/aee_decision.json`

### 4. Affect + Gödel Vertical Slice

```bash
# Historical retired command: adl/tools/demo_affect_godel_vertical_slice.sh
```

Review:
- [demos/v0.85/affect_godel_vertical_slice_demo.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/demos/v0.85/affect_godel_vertical_slice_demo.md)

Primary proof:
- `.adl/reports/demo-affect-godel-vertical-slice/runs/review-godel-affect-001/godel/godel_affect_vertical_slice.v1.json`

### 5. Gödel Hypothesis Engine

```bash
# Historical retired command: adl/tools/demo_godel_hypothesis_engine.sh
```

Review:
- `demos/v0.85/godel_hypothesis_engine_demo.md`

Primary proof:
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/godel_hypothesis.v1.json`

### 6. Five-Command Editing Lifecycle

```bash
adl/tools/demo_five_command_editing.sh
```

Review:
- `docs/tooling/editor/five_command_demo.md`
- `docs/tooling/editor/five_command_regression_suite.md`

Primary proof:
- five-command editing demo manifest emitted by the script
- `adl/tools/test_five_command_regression_suite.sh`
- `adl/tools/test_five_command_editor_truth.sh`

## Supporting demos

Use these when a reviewer wants deeper inspection of individual bounded
surfaces:

- Historical retired source: `adl/tools/demo_affect_engine.sh`
- Historical retired source: `adl/tools/demo_reasoning_graph_affect.sh`
- Historical retired source: `adl/tools/demo_adaptive_godel_loop.sh`
- Historical retired source: `adl/tools/demo_experiment_prioritization.sh`
- Historical retired source: `adl/tools/demo_cross_workflow_learning.sh`
- Historical retired source: `adl/tools/demo_promotion_eval_loop.sh`

## Success condition

The milestone demo program is successful when a reviewer can move through the
primary sequence above and understand:

- execution control surfaces
- human authoring and review surfaces
- bounded adaptation
- bounded cognitive behavior
- bounded learning artifacts

without reconstructing hidden context from the codebase.

## Closeout Read

At the current v0.85 state, no additional bounded demo pass is required before release readiness. The remaining closeout work is documentation consistency, review, remediation if needed, and release packaging rather than creation of new milestone demos.
