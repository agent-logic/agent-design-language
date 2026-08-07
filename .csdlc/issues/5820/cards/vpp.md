# Validation Planning Prompt

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5820/design.md

Diagram: .csdlc/prepared/issues/5820/diagram.mmd

## Selected Lanes

[
  {
    "lane": "guardian-lifecycle-unit",
    "proof_role": "Prove Guardian restart policy, lifecycle aggregation, and nonce-bound pre-restart probe synchronization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "--no-default-features"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "production-guardian-api-wss-restart",
    "proof_role": "Launch the production Guardian/kernel on macOS and prove authenticated HTTPS/WSS, forced child failure, bounded restart, durable continuity, clean logs, and shutdown.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh"
    ],
    "parallel_group": "runtime-production",
    "defer_reason": null
  },
  {
    "lane": "linux-spot-runtime-proof",
    "proof_role": "Validate the retained exact-head Linux Spot lifecycle result, immutable builder provenance, and verified instance teardown.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      "-c",
      "import json,pathlib; s=json.loads(pathlib.Path('.csdlc/evidence/5820/external-linux/summary.json').read_text()); w=json.loads(pathlib.Path('.csdlc/evidence/5820/external-linux/wrapper-final-summary.json').read_text()); assert s['status']=='passed' and s['remote_summary']['resolved_commit']=='2faa7c0ddda8e12e452d7be4b309aeb86c10f69d' and s['cleanup']['final_instance_state']=='terminated' and s['launch']['purchase_option']=='spot'; assert w['status']=='passed' and w['source_commit']=='2faa7c0ddda8e12e452d7be4b309aeb86c10f69d' and w['self_verification']['compute_teardown_verified'] is True; print('PASS: exact-head Linux Spot Guardian lifecycle and teardown evidence')"
    ],
    "parallel_group": "platform-evidence",
    "defer_reason": null
  },
  {
    "lane": "native-windows-blocker",
    "proof_role": "Retain the acceptance-authorized named blocker that no native Windows execution authority is available in this bounded issue session; do not claim Windows lifecycle proof.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
    "argv": [
      "python3",
      "-c",
      "import json; print(json.dumps({'schema':'adl.runtime_v3.platform_blocker.v1','platform':'windows','status':'blocked','reason':'native_windows_runner_unavailable','source_revision':'2faa7c0ddda8e12e452d7be4b309aeb86c10f69d'},sort_keys=True))"
    ],
    "parallel_group": "platform-evidence",
    "defer_reason": null
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject whitespace damage before exact-head review and issue-closing publication.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --bin adl-runtime-lifecycle-soak --no-default-features`
- `bash adl/tools/validate_v092_runtime_guardian_lifecycle.sh`
- `python3 -c import json,pathlib; s=json.loads(pathlib.Path('.csdlc/evidence/5820/external-linux/summary.json').read_text()); w=json.loads(pathlib.Path('.csdlc/evidence/5820/external-linux/wrapper-final-summary.json').read_text()); assert s['status']=='passed' and s['remote_summary']['resolved_commit']=='2faa7c0ddda8e12e452d7be4b309aeb86c10f69d' and s['cleanup']['final_instance_state']=='terminated' and s['launch']['purchase_option']=='spot'; assert w['status']=='passed' and w['source_commit']=='2faa7c0ddda8e12e452d7be4b309aeb86c10f69d' and w['self_verification']['compute_teardown_verified'] is True; print('PASS: exact-head Linux Spot Guardian lifecycle and teardown evidence')`
- `python3 -c import json; print(json.dumps({'schema':'adl.runtime_v3.platform_blocker.v1','platform':'windows','status':'blocked','reason':'native_windows_runner_unavailable','source_revision':'2faa7c0ddda8e12e452d7be4b309aeb86c10f69d'},sort_keys=True))`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
