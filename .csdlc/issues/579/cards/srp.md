# Structured Review Prompt

Template: 1.0.0

Issue: 579

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/579
.csdlc/prepared/issues/579/validate-aws-f-corrective.sh
adl/config/validation_lane_selector.v0.91.6.json
adl/tools/validate_aws_runtime_platform_static.sh
adl/tools/test_validation_manager.sh
adl/tools/test_ci_path_policy.sh
infra/aws/modules/csm-runtime-alb
infra/aws/runtime
docs/operations/cloud/aws/runtime-platform/README.md
docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md

## Prompts

- Does the corrective remove AWS-F public Route53/ACM ownership and leave public exposure to #122?
- Are local/static proof, deferred live AWS proof, deployment, cleanup, rollback, observability, artifact wiring, and cost/deadline claims truthful?
- Does the validator structurally reject forbidden world-open Runtime ingress and avoid egress false exclusions?
- Are backend, locking, account identity, workspace, and key isolation enforced or fail-closed?
- Is one-time Spot instance target behavior bounded as non-production-resilient?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live AWS proof was run or claimed; #579 remains a static/local corrective validation and path-policy repair.
- Review verified PR #583 head matched the exact reviewed commit and that runtime/workspace coverage is skipped for the AWS runtime-platform static lane.

## Review Result

Revision: Some("git-blake3:54e76de846ac75ae11d8b3f47faee486f6365ad4:307885dc2ada652a208ca354b283d40f2933b54c3aeee49831ee22761ee65780")

Reviewer: Some("fresh-session:7bdf9090-b7cd-4804-931c-188a20996230")

Result: pass
