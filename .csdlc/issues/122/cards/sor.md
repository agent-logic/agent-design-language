# Structured Output Record

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: sor

Status: ready

## Summary

Implemented and live-applied the Terraform-owned CSM public edge for wuji/dev. The business AWS account now owns the delegated csm.agent-logic.ai hosted zone and live CloudFront/WAF/S3/API/WSS edge resources. Observatory assets are served at https://observatory.wuji.dev.csm.agent-logic.ai with deployed config pointing at https://api.wuji.dev.csm.agent-logic.ai. API Gateway CORS allows the Observatory origin and rejects unrelated origins. Runtime API/WSS live proof remains blocked because the selected wuji Runtime origin is healthy only locally via CSMctl and is not reachable at public 47.146.81.109:443 or :20997.

## Artifacts

- infra/aws/csm-public-edge/
- adl/tools/validate_csm_public_edge_static.sh
- adl/tools/validate_csm_public_edge_live.sh
- docs/milestones/post-v0.92/features/CSM_PUBLIC_EDGE_TERRAFORM.md
- .csdlc/prepared/issues/122/terraform-execution-plan.md
- .csdlc/evidence/122/live-aws-edge-apply-2026-08-26.md

## Execution

- Added Terraform support for first-time CSM hosted zone creation or explicit hosted_zone_id reuse.
- Added optional edge_acm_certificate_arn so CloudFront can use a pre-issued per-CSM ACM wildcard certificate.
- Added optional origin_cname_target so wuji.dev.csm.agent-logic.ai can CNAME to the existing wuji DDNS hostname without hardcoding an IP address.
- Updated naming/docs from generic placeholders to <function>.<csm>.<environment>.csm.agent-logic.ai and production <function>.<csm>.csm.agent-logic.com.
- Applied the wuji/dev edge in the Agent Logic business AWS account and uploaded the HTML Observatory assets to the private Observatory S3 bucket.
- Tightened the live validator to report Observatory HTTPS, allowed-origin CORS, rejected-origin CORS, and Runtime-origin reachability separately.

## Validation

[
  {
    "command": [
      "terraform",
      "-chdir=infra/aws/csm-public-edge",
      "fmt",
      "-recursive"
    ],
    "purpose": "Terraform formatting for the CSM public edge module",
    "outcome": "passed",
    "evidence_ref": "commentary run after live edge changes"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_csm_public_edge_static.sh"
    ],
    "purpose": "Static Terraform init/validate and CSM WSS/CORS guard proof",
    "outcome": "passed",
    "evidence_ref": "commentary run after live edge changes"
  },
  {
    "command": [
      "AWS_PROFILE=agent-logic-admin",
      "terraform",
      "-chdir=infra/aws/csm-public-edge",
      "apply",
      "-auto-approve",
      "issue122-full.tfplan"
    ],
    "purpose": "Live AWS apply of CloudFront/WAF/S3/API/WSS edge resources",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/122/live-aws-edge-apply-2026-08-26.md"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_csm_public_edge_live.sh",
      "--csm",
      "wuji",
      "--environment",
      "dev",
      "--observatory-url",
      "https://observatory.wuji.dev.csm.agent-logic.ai",
      "--api-url",
      "https://api.wuji.dev.csm.agent-logic.ai",
      "--wss-url",
      "wss://wss.wuji.dev.csm.agent-logic.ai/v1/observatory/ws",
      "--wss-origin-hostname",
      "wuji.dev.csm.agent-logic.ai"
    ],
    "purpose": "Live API/WSS proof is blocked on the selected wuji Runtime origin accepting public TLS traffic",
    "outcome": "blocked",
    "evidence_ref": ".csdlc/evidence/122/live-aws-edge-apply-2026-08-26.md"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
