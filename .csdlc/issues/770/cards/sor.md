# Structured Output Record

Template: 1.0.0

Issue: 770

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Static remediation implemented and ten mocked Terraform plan tests pass. Saved AWS plan is read-only evidence. Live SSH reachability, application isolation and disposal remain pending explicit cloud approval and selected identity-file path.

## Artifacts

- .csdlc/prepared/issues/770/local-validation.md
- infra/aws/csm-runtime-spot/tests/recovery.tftest.hcl
- infra/aws/csm-runtime-spot/tests/run_live_proof.sh

## Execution

- Require key and explicit SSH ingress for public Spot Runtime.
- Preserve application isolation, reject Runtime/SSH port overlap, retain IMDSv2 and encrypted storage.
- Provide focused mock tests, recovery documentation and an unexecuted live probe helper.

## Validation

[
  {
    "command": [
      "bash",
      "infra/aws/csm-runtime-spot/tests/run_contract.sh"
    ],
    "purpose": "Public SSH negative/positive and hardening contract proof",
    "outcome": "passed",
    "evidence_ref": ".csdlc/prepared/issues/770/local-validation.md"
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
