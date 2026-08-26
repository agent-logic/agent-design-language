# Structured Review Prompt

Template: 1.0.0

Issue: 194

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/issue194_private_network.cloudformation.json
adl/tools/issue194_private_wuji_aws_runner.sh
adl/tools/private_wuji_aws_recovery_qualification.py
adl/tools/issue194_model_health_command.py
adl/tools/test_issue194_private_network_template.sh
adl/tools/test_private_wuji_aws_recovery_qualification.sh
.csdlc/prepared/issues/194/design.md
.csdlc/prepared/issues/194/diagram.mmd
.csdlc/evidence/194

## Prompts

- Does any path allow public Runtime/model/Observatory exposure or hosted model fallback?
- Does cleanup prove zero resources after success, failure, and interruption?
- Are redacted receipts truthful and free of secrets/raw AWS identifiers/machine-local paths?
- Does the proof avoid overclaiming #142 or the unimplemented serial hybrid recovery behavior?
- Is the SSM maintenance plane clearly separated from direct private agent/voter data-plane traffic?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Current Agent Logic AWS quota/capacity does not permit two simultaneous model-capable AWS GPU voters; #194 records this as a quota gate and does not claim the missing two-GPU serial hybrid proof.

## Review Result

Revision: Some("git-blake3:93fa87e9657e48c85ba2433796d10c2b0595974e:3bc69e4ee874c1a34f8a70950175b10e51e1963e9de3e4bb66c7709e4b50c833")

Reviewer: Some("codex:/root/review_issue_194_exact_head_r2")

Result: pass
