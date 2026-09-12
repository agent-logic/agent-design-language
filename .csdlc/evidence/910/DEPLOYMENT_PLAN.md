# #910 exact deployment preflight

Status: infrastructure and assets prepared; **not deployed**. User approved intended static deployment through root; actual execution remains stopped until independent review and Runtime origin policy resolution. Existing `observatory.wuji.dev.csm.agent-logic.ai` is untouched.

## Exact action set

Saved plan SHA256 `0aa4a1d558cafe4de27eee3acc7e1c00249b2ed2219e9f109410b00037af9fec`: 18 creates, zero updates/deletes/replacements. See `terraform-plan-summary.json`. Provider AWS 5.100.0, Terraform 1.15.3. Existing `infra/aws/observatory` source is copied byte-for-byte into isolated working configuration. Inputs in `planned-inputs.json` override public hosted zone to `csm.agent-logic.ai`; target is `observatory.csm.agent-logic.ai`. The package default parent zone is unavailable in the business account.

Creates: private versioned site bucket prefix `adl-observatory-`; private logs bucket prefix `adl-observatory-logs-`, 90-day log expiry; existing package ACL grants; OAC; HTTPS CloudFront distribution with exact target alias; security headers including CSP permitting only self and configured Runtime HTTPS/WSS; exact ACM DNS-validated certificate in us-east-1; certificate validation CNAME and A/AAAA aliases. Generated bucket suffixes, certificate validation record, distribution ID/domain and IDs are apply-time values, not invented targets. No Runtime compute or existing Wuji distribution change.

Live STS matches the approved business baseline with profile `agent-logic-admin`. Read permissions were exercised for STS, Route53 zones/records, CloudFront distributions, ACM list, S3 bucket/state-object enumeration, and provider plan data sources. Successful reads and plan do **not** prove write IAM authorization. Target alias/DNS absent; existing foundation state bucket has no Observatory key. This bounded search is not proof that no external state custodian exists anywhere.

## Static asset and cache plan

`assets.json` records exact merged source; #720 accepted merge is in its ancestry. `upload-manifest.json` records four transformed deployment objects and hashes. Only index CSS/JS references change to content-hash filenames; script, CSS and endpoint JSON bytes remain unchanged. Upload hashed assets and public config before index. Hashed assets use one-year immutable cache; index and config use no-cache/no-store. Invalidate only `/`, `/index.html`, `/runtime-v3.config.json`. No telemetry/report evidence or credentials are uploaded.

## Live Runtime gate

Configured endpoint is `https://wuji.dev.csm.agent-logic.ai:20997`, WSS same host/port. Health without Origin returned 200; identical request with `Origin: https://observatory.csm.agent-logic.ai` returned 403. Runtime currently rejects the required browser origin. WSS, authenticated browser flow, and live telemetry are not yet proven. Root coordinates bounded origin remediation separately. Deployment is paused at this explicit gate.

## State custody and rollback

Package declares local backend. Stable private Git-common directory `csdlc-v3/private/910/terraform/` holds mode-0600 plan, lock and configuration under mode-0700 directory. This directory is outside issue worktree cleanup. Apply must use this stable working directory, preserve exact plan hash, capture state after each invocation, and back up resulting state and its SHA256 in adjacent private timestamped backups before any further mutation. Raw plan/state never enters tracked/public evidence. Root approved this custody choice; no remote backend or state mutation is proposed. Daniel / Agent Logic operator is custody and billing owner.

Content rollback dry-run: initial deployment has no prior target version, so it cannot truthfully claim an earlier deployed target release. Before subsequent uploads enumerate version IDs for index/config, retain hash-to-version map, and restore those prior objects as new versions followed by the three-path invalidation; immutable hashed assets remain available. Do not delete buckets or disable distribution as a content rollback. Infrastructure rollback is a separate reviewed Terraform plan against retained authoritative state, never automatic destroy. Initial failure preserves created resources/state for diagnosis; subsequent action requires exact review.

## Cost and monitoring

No new hosted zone, Runtime compute, load balancer, NAT, WAF or paid plan subscription. Existing zone cost unchanged; CloudFront alias DNS queries are free. Non-exportable ACM certificate integrated with CloudFront has no additional certificate charge. Pay-as-you-go CloudFront request/transfer charges plus S3 storage/requests/log writes apply; shared account free allowances must not be assumed unconsumed. No hard spending cap or budget alarm is created by this package. Low-volume planning assumption: four small static objects, under 1 GB combined active/versioned/log storage, under 10 GB monthly egress and 100,000 HTTPS requests; target monthly incremental spend under $5 is an estimate, not enforced or guaranteed, and traffic/geography/account allowances can exceed it. Operator checks actual AWS billing after deployment and daily during initial proof, plus CloudFront 4xx/5xx, logs and Runtime live browser failures. Stop expansion and investigate if estimate exceeded.

Sources checked 2026-09-12: [CloudFront pricing](https://aws.amazon.com/cloudfront/pricing/), [S3 pricing](https://aws.amazon.com/s3/pricing/), [Route53 pricing](https://aws.amazon.com/route53/pricing/), [ACM pricing](https://aws.amazon.com/certificate-manager/pricing/). No claim that paid usage has occurred.

## Validation and remaining acceptance

24/24 local UI tests pass. Terraform fmt and validate pass; actual plan exits 2 (changes), all 18 managed actions create. Native six-card validation passes generation 4. These prove preparation only. Independent exact packet review, origin remediation verification, saved-plan apply, sanitized AWS posture readbacks, content hash/headers readback, real HTTPS/WSS browser evidence, rollback dry-run and final review remain required before issue acceptance.

Preparation review: independent Sprint 8 #909 reviewer verified the actual saved plan, all 18 create actions, exact source/asset hashes and index-only transformation, and stable private custody. Initial named-resource collision evidence finding was resolved by `named-resource-preflight.json`: both exact OAC/header-policy names absent. Preparation review passed; this is not final implementation/deployment acceptance. Runtime origin 403 remains the execution gate.

Final independent command review passed after explicit upload failure guards and per-object version/hash checks; apply failure backup and invalidation completion checks were also verified. No execution was performed.
