# Structured Review Prompt

Template: 1.0.0

Issue: 487

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/487
.csdlc/prepared/issues/487
infra/aws/account-foundation
docs/operations/cloud/aws/audit-security
docs/milestones/v0.92.1/evidence/cloud/aws-d

## Prompts

- Does the design keep #487 to AWS audit/security baseline scope without absorbing AWS-E/F/G or #486 backend ownership?
- Are CloudTrail/configuration/detection/access-analysis/alert/retention/encryption controls specified with owner and destination truth?
- Are cost and regional-scope risks explicit enough to prevent uncontrolled logging spend?
- Does the validation plan prevent credentials and unnecessary sensitive values from entering retained evidence?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review did not perform live AWS calls, read credentials, apply Terraform, publish, or merge.
- Reviewer verified the Git SHA directly but could not independently recompute the supplied BLAKE3 suffix.
- Reviewer confirmed committed proof records and source semantics; its own local proof reruns were limited by read-only sandbox restrictions.

## Review Result

Revision: Some("git-blake3:7eb5db939995ee9f191f4149b706dd50e8a89f9f:bdebfa5b11f8491bc75bab53321e71af7b045ae15cbc63515c7f71d19dad55be")

Reviewer: Some("fresh-session:487d0000-0000-4000-8000-000000000006")

Result: pass
