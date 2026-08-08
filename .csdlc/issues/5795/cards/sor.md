# Structured Output Record

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded local Gemma Shepherd foundation with governed admission, exact runner and model binding, failure isolation, real local MLX proof, and a quota-gated one-instance AWS CUDA proof runner.

## Artifacts

- .csdlc/evidence/5795/foundation/shepherd-contract.log
- .csdlc/evidence/5795/foundation/kernel-clippy.log
- .csdlc/evidence/5795/foundation/runtime-clippy.log
- .csdlc/evidence/5795/foundation/mac-mlx-real-local-model-smoke.json
- .csdlc/evidence/5795/foundation/aws-gpu-preflight.json
- .csdlc/evidence/5795/foundation/aws-deadline-reaper.json
- .csdlc/evidence/5795/foundation/aws-lock-collision.json
- .csdlc/evidence/5795/foundation/portable-model-publication.json

## Execution

- Added the bounded LocalShepherdExecutor and governed operation contract.
- Added process-tree containment, cancellation, timeout, output, memory, and concurrency enforcement.
- Added exact runner-byte, nonce, backend, model, and artifact attestation.
- Added real local MLX smoke proof and a pinned portable CUDA artifact bundle contract.
- Added a one-instance, one-bootstrap AWS proof runner with owner-bound locking and three cleanup layers.

## Validation

[
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/run_wp5795_aws_gpu_proof.sh"
    ],
    "purpose": "Parse the bounded AWS proof runner without executing it.",
    "outcome": "passed",
    "evidence_ref": "aws-runner-syntax.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run exact-head diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "exact-head-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "shepherd_local_model",
      "real_local_model_smoke",
      "--",
      "--ignored",
      "--exact",
      "--nocapture"
    ],
    "purpose": "Run the ignored real-local-model smoke against the configured local Ollama endpoint.",
    "outcome": "passed",
    "evidence_ref": "mac-mlx-real-local-model-smoke.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "shepherd"
    ],
    "purpose": "Run the focused Shepherd kernel contract suite.",
    "outcome": "passed",
    "evidence_ref": "shepherd-foundation-contract.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "shepherd",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run focused kernel Clippy with warnings denied.",
    "outcome": "passed",
    "evidence_ref": "shepherd-kernel-clippy.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "shepherd_local_model",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run focused runtime Clippy with warnings denied.",
    "outcome": "passed",
    "evidence_ref": "shepherd-runtime-clippy.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
