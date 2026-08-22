# Structured Output Record

Template: 1.0.0

Issue: 268

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and remotely qualified the #268 Runtime/ACC six-resident AWS qualification at exact head 8947231b6549f6b43f76c5d4656b333868a032ae. Run72 completed on Linux/x86_64 On-Demand r7i.2xlarge with status passed, measured exposure 21,601 seconds, 21,594 completed cycles, six resident continuity/UTS verification, replay denial, authenticated HTTPS/WSS proof, clean logs, and wrapper cleanup complete.

## Artifacts

- adl/tools/issue268_runtime_uts_task_panel.json
- adl/tools/run_issue268_six_resident_uts_cycle.py
- adl/tools/run_issue268_continuity_uts_qualification.py
- adl/tools/install_issue268_runtime_volume.py
- adl/tools/issue268_runtime_qualification.cloudformation.yaml
- adl/tools/run_issue268_remote_resident_qualification.sh
- adl/tools/run_issue268_six_hour_spot_qualification.sh
- adl/tools/test_run_issue268_six_hour_spot_qualification.sh
- tools/aws_remote_validation/src/aws_remote_validation.rs
- .csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/portable-request.json
- .csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/launch-claimed.json
- .csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/cloudformation.json
- .csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/public-summary.json
- .csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/qualification-proof.json
- .csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/successful-run-disposition.json

## Execution

- Runtime-owned six resident identities execute issue-owned UTS runtime.observe tasks through ACC and the Runtime adapter.
- Continuity qualification verifies signed dehydration with admission closed, restore with admission open, pending-only post-cycle continuation, continuation verification, and replay denial.
- AWS launch path uses the CloudFormation runtime template, retained Runtime volume, Agent Logic business profile, Linux/x86_64 package-manager bootstrap, no GPU, no On-Demand fallback from Spot, and an explicit one-attempt On-Demand run72 authorized under the USD 20 ceiling.
- Remote validation wrapper now binds both SendCommand TimeoutSeconds and AWS-RunShellScript executionTimeout to the 25,200-second provider deadline so six-hour runs are not killed by the default one-hour plugin timeout.
- Run72 passed with redaction enabled, interruption not detected, exact revision 8947231b6549f6b43f76c5d4656b333868a032ae, command_seconds 22,474, wrapper exit code 0, cleanup complete, final instance state terminated, deleted stack, and no run72-tagged residual volumes.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "tools/aws_remote_validation/Cargo.toml"
    ],
    "purpose": "Format the AWS remote validation timeout fix before installing the stable owner binary.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/268/local-operator-session/run72-timeout-fix"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue268_six_hour_spot_qualification.sh"
    ],
    "purpose": "Prove issue268 wrapper contracts, exact scope, On-Demand posture, and explicit executionTimeout marker.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/268/local-operator-session/run72-timeout-fix"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "tools/aws_remote_validation/Cargo.toml",
      "--bin",
      "adl-aws-remote-validation",
      "ssm_command_timeout_is_explicitly_bound_to_full_provider_budget"
    ],
    "purpose": "Prove the SSM command document execution timeout is explicitly bound to the full provider budget.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/268/local-operator-session/run72-timeout-fix"
  },
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "tools/aws_remote_validation/Cargo.toml",
      "--bin",
      "adl-aws-remote-validation"
    ],
    "purpose": "Compile-check the AWS remote validation binary after the timeout fix.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/268/local-operator-session/run72-timeout-fix"
  },
  {
    "command": [
      "bash",
      "adl/tools/install_aws_remote_validation_tool.sh"
    ],
    "purpose": "Install the patched stable owner binary used by run72.",
    "outcome": "passed",
    "evidence_ref": ".adl/bin/adl-aws-remote-validation-tool"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue268_six_hour_spot_qualification.sh",
      "preflight"
    ],
    "purpose": "Preflight Agent Logic AWS profile, r7i.2xlarge, 8 vCPU/64 GiB, 25,200-second deadline, USD 20 budget, one attempt, no GPU, retained Runtime volume, and no fallback before mutation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/portable-request.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue268_six_hour_spot_qualification.sh",
      "authorized-launch"
    ],
    "purpose": "Run the authorized paid AWS qualification once and verify pass, redaction, six-hour exposure, and cleanup.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/successful-run-disposition.json"
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
