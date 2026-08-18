# Structured Review Prompt

Template: 1.0.0

Issue: 415

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/415
.csdlc/prepared/issues/415
.csdlc/evidence/415
adl/tools/run_aws_spot_builder_image_validation.sh
adl/tools/test_run_aws_spot_builder_image_validation.sh
tools/aws_remote_validation/scripts/remote_validation_runner.sh

## Prompts

- Does every required tool/check produce an individually attributable retained diagnostic?
- Can early exit 127 preserve exact redacted output and identify the missing executable?
- Are success compatibility and exact cleanup semantics unchanged?
- Are AWS, #268, and #269 strictly outside execution scope?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live AWS validation ran because issue #415 explicitly prohibits paid launch; the deterministic local runner and fake-provider fixtures are the proving boundary.

## Review Result

Revision: Some("git-blake3:78112644e520b52e8f221645d570d0c30bbf84ef:32d971bd2da7f7af4a5e77b3e37c9038d9300fd8172eb2631a35864db2c2421c")

Reviewer: Some("fresh-session:3c69e04a-000a-402d-b62a-7b9d865553f1")

Result: pass
