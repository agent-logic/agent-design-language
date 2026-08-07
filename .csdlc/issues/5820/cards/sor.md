# Structured Output Record

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Consolidated Runtime v3 under Guardian ownership and proved bounded restart, durable continuity, authenticated HTTPS/WSS recovery, clean logging, and clean shutdown on macOS and Linux, while retaining an explicit native Windows blocker.

## Artifacts

- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl-runtime/src/guardian.rs
- adl-runtime/tests/runtime_guardian_lifecycle.rs
- adl-runtime-kernel/src/config.rs
- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- .csdlc/evidence/5820/external-linux/summary.json
- .csdlc/evidence/5820/external-linux/wrapper-final-summary.json

## Execution

- Hardened Guardian startup, child ownership, bounded restart, and failure classification around the authoritative runtime init contract.
- Added a production lifecycle soak that forces one kernel failure and proves restart, durable generation continuity, authenticated Observatory HTTPS/WSS, and clean shutdown.
- Removed the Linux validation harness Ruby dependency by using Python 3 standard-library parsing and synchronized the external probe with fault injection using nonce-bound atomic markers.
- Made barrier failure paths reap Guardian and retain exact probe diagnostics instead of losing the operational cause.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace damage before review",
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
      "--bin",
      "adl-runtime-lifecycle-soak",
      "--no-default-features"
    ],
    "purpose": "Prove restart policy, marker safety, nonce pairing, cleanup, and lifecycle invariants",
    "outcome": "passed",
    "evidence_ref": "guardian-lifecycle-unit.log"
  },
  {
    "command": [
      "python3",
      "-c",
      "import json,pathlib; s=json.loads(pathlib.Path('.csdlc/evidence/5820/external-linux/summary.json').read_text()); w=json.loads(pathlib.Path('.csdlc/evidence/5820/external-linux/wrapper-final-summary.json').read_text()); assert s['status']=='passed' and s['remote_summary']['resolved_commit']=='2faa7c0ddda8e12e452d7be4b309aeb86c10f69d' and s['cleanup']['final_instance_state']=='terminated' and s['launch']['purchase_option']=='spot'; assert w['status']=='passed' and w['source_commit']=='2faa7c0ddda8e12e452d7be4b309aeb86c10f69d' and w['self_verification']['compute_teardown_verified'] is True; print('PASS: exact-head Linux Spot Guardian lifecycle and teardown evidence')"
    ],
    "purpose": "Validate the retained Linux production result, immutable builder provenance, and terminated compute state",
    "outcome": "passed",
    "evidence_ref": "linux-spot-runtime-proof.log"
  },
  {
    "command": [
      "python3",
      "-c",
      "import json; print(json.dumps({'schema':'adl.runtime_v3.platform_blocker.v1','platform':'windows','status':'blocked','reason':'native_windows_runner_unavailable','source_revision':'2faa7c0ddda8e12e452d7be4b309aeb86c10f69d'},sort_keys=True))"
    ],
    "purpose": "Retain the explicit no-native-Windows-runner blocker without claiming Windows lifecycle proof",
    "outcome": "passed",
    "evidence_ref": "native-windows-blocker.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh"
    ],
    "purpose": "Run the real Guardian and kernel, force one failure, and prove durable recovery plus authenticated HTTPS/WSS",
    "outcome": "passed",
    "evidence_ref": "production-guardian-api-wss-restart.log"
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
