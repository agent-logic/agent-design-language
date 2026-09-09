# Structured Output Record

Template: 1.0.0

Issue: 770

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Public Spot SSH recovery implemented. Ten Terraform mock tests and five offline probe tests pass; authorized live SSH/isolation and AWS disposal readbacks passed. Independent final review and hosted CI pending.

## Artifacts

- .csdlc/prepared/issues/770/local-validation.md
- infra/aws/csm-runtime-spot/tests/recovery.tftest.hcl
- infra/aws/csm-runtime-spot/tests/run_live_proof.sh
- .csdlc/prepared/issues/770/local-validation.md
- .csdlc/prepared/issues/770/live-proof.json

## Execution

- Require key and explicit SSH ingress for public Spot Runtime.
- Preserve application isolation, reject Runtime/SSH port overlap, retain IMDSv2 and encrypted storage.
- Provide focused mock tests, recovery documentation and an unexecuted live probe helper.
- Require existing SSH key and explicit non-world ingress for public Spot nodes.
- Keep application ingress separate, reject TCP/22 overlap, preserve IMDSv2 and encrypted EBS.
- Retain mock tests, fail-closed live probe, and authorized live/disposal evidence.

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
  },
  {
    "command": [
      "bash",
      "infra/aws/csm-runtime-spot/tests/run_contract.sh"
    ],
    "purpose": "Ten plan contracts and five live-probe verdict cases",
    "outcome": "passed",
    "evidence_ref": "public-ssh-contract.log"
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
