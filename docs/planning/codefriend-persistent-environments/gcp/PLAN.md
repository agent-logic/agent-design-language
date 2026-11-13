# CodeFriend persistent GCP environments

PROPOSED, not deployment approval. 2026-11-13. AWS baseline: reviewed PR #1423, head 814817762c. Both actual GCP provider reviews returned COMPLETE REVIEW; see reviews/ for results and final refinements.

## Intent and boundaries

Three persistent logical environments: dev, staging and production. Two selected zones in one region per environment; instances, counts, purchasing and capacity remain TBD. Provision/adopt foundation once; module releases update code artifacts and manifests, not networks or service identities. No account creation, IAM/CI changes, cloud provisioning or existing campaign effects are authorized.

Proposed comparison region us-east4 (Northern Virginia), geographically close to AWS us-east-1 but a different provider location. Validate services, quotas and residency before approval. Exact zones and CIDRs require inventory. One versus three projects remains OPEN; three independent VPC networks, service identities, secrets, databases, queues/subscriptions, buckets and state boundaries either way. Projects are IAM/quota/resource boundaries, not AWS-account equivalents; organization/folder policy and shared billing add distinct scopes. A single project shares administrator/quota blast radius. Production isolation risk requires explicit operator acceptance before provisioning.

## Topology and provider semantics

Each environment has a custom-mode global VPC with regional private application and worker subnets. Both selected zones use those regional subnets: do not copy AWS's six zonal-subnet count. One regional Public Cloud NAT configuration with Cloud Router serves private VM egress in both zones; NAT is distributed, not a zonal proxy appliance. Size NAT IPs and port allocations from VM/concurrency demand; one IP per environment is only a cost scenario. Google API traffic uses Private Google Access with required routes, DNS and firewall rules. Do not assume a route grants IAM access.

Global external Application Load Balancer with managed HTTPS certificates and optional Cloud CDN provides public ingress to private Compute Engine backends. This combines functions of AWS CloudFront/private ALB and does not require an additional internal ALB. Backend VMs have no public IP. Restrict ingress to documented proxy/health-check paths and IAP management sources; no direct public SSH. Configure Host/forwarded headers, OAuth callbacks, session cache bypass and supported health checks before deployment. Cloud Armor/rate limits per environment are security decisions with extra costs. CDN cache-fill/delivery and ALB processing are volume costs.

One regional application MIG and one regional worker MIG per environment, explicitly restricted to the two chosen zones (six MIGs total). Define distribution policy, autohealing, rolling update surge/unavailable budgets, connection drain and N-1 capacity before launch. Application serves Website and private gateway on separate documented ports/credentials; no implicit public gateway exposure. Worker leases, acknowledgment renewal, idempotency and interruption recovery are mandatory. Boot disks are replaceable; durable data never lives only on them.

IAP TCP forwarding plus OS Login is the proposed SSH/identity management equivalent to SSM management plus restricted SSH. It is not SSM protocol compatibility: IAP needs explicit firewall access from its documented source ranges to target SSH ports, scoped tunnel IAM and OS Login permissions. No public VM IP or bastion required. Separate production access approval and audit metadata; do not claim shell content recording from tunnel logs. VM Manager/OS Config provides inventory/patch functions where qualified. Document dependency failures and an emergency management decision before launch.

## Service mapping

| AWS intent | GCP proposal | Required semantic validation |
| --- | --- | --- |
| CloudFront + private ALB | External Application LB + Cloud CDN, private MIG backends | Different frontend/backend trust model; cache/session/TLS tests |
| Zonal NAT gateways | Regional distributed Public NAT + Cloud Router | VM-count/port/IP pricing; no one-gateway-per-zone translation |
| EC2 ASGs | Two-zone regional MIGs | Explicit zones, distribution, autohealing and rollout policies |
| SSM + restricted SSH | IAP TCP + OS Login; VM Manager where needed | Inbound IAP firewall exception; no SSM API compatibility |
| S3 | Private Cloud Storage | Generation-pinned objects, versioning/retention, IAM and signed provenance |
| DynamoDB application/control tables | Firestore Native regional database candidate | Separate adapter, keys/documents, indexes, conditional writes, consistency, transactions and limits |
| SQS/DLQ | Pub/Sub pull subscriptions/dead-letter topics | Ack deadlines/lease extension, retries, ordering, retention and duplicate effects |
| Parameter Store SecureString | Secret Manager + versioned nonsecret config | Access/version/rotation costs and IAM differ |
| Lambda broker/control | Cloud Run service/functions candidate | Authenticated endpoint, VPC egress/ingress and invocation contract selection |
| CloudWatch | Cloud Logging/Monitoring | Retention, alert budgets, data access and sensitive-log controls |
| GitHub + CodeBuild | GitHub + Cloud Build candidate | Trigger/runner integration and producer intake must be qualified |

The operator selected DynamoDB for AWS. Firestore here is an equivalent-intent proposal, not an approved replacement or DynamoDB-compatible API. Before GCP application launch, qualify its adapter against actual Website semantics and a lossless migration/rollback test. Cross-cloud DynamoDB retention is an alternative requiring separate latency, egress and credential analysis, excluded from the native-GCP estimate. No multi-active SQLite. Single-region regional Firestore is proposed; do not inherit multi-region SLA claims. Workload/database mode and schema require approval.

Pub/Sub defaults must not be described as exactly-once execution. Optional exactly-once delivery is regional and pull-only; duplicate publishes, expired leases and application side effects still require durable idempotency. Test poison jobs, dead lettering, replay and N/N-1 consumers. SQS receipt/visibility APIs cannot be renamed into Pub/Sub acknowledgments.

## Delivery, trust and custody

Use separate state by environment/layer, pinned infrastructure modules and an exact resource inventory. Ordinary releases do not run infrastructure apply. Build immutable module artifacts once; pin object generations/digests and producer provenance, then promote identical bytes dev → staging → prod with environment configuration receipts. Container packaging may use Artifact Registry with separately modeled cost. Raw artifact/proof storage uses Cloud Storage.

Only hermetic deterministic proofs may be reused. Keys include dependency graph/generator, source/locks, runner/image/toolchain, fixtures and declared locale/time/seed inputs. Authenticate receipts through a protected trust root. Untrusted PRs use separate build projects/roles and cannot write trusted namespaces or assume deployment roles. Exact GitHub repository/ref/environment federation, short-lived service identity, dependency pinning and egress policy are required; no long-lived cloud keys. Periodic cache-bypass proof detects hidden-input drift. Live IAM/provider/migration/installed/cleanup proof is always fresh.

N/N-1 module/message/data compatibility, worker drain, idempotency and explicit expand/contract migrations gate promotion. Production approver must be independent of artifact producer. Rollback after writes requires a data strategy, not merely an older binary.

Before adoption, obtain fresh authenticated resource/state/cleanup inventory in the approved window. Prove exclusive ownership and supported cleanup exclusion without a protection gap; verify backups, deletion protections, zero-change plan and cleanup no-op against persistent inventory. Defer adoption if ordering cannot be proved. Never modify existing guards under this planning request.

## Failure and decision gates

| Failure | Required proof |
| --- | --- |
| One zone lost | Surviving MIG capacity and LB health/drain satisfy selected RTO/RPO; redeliver jobs safely |
| NAT port exhaustion/API outage | Dependency probes, queue-age alarms, bounded backoff and worker recovery; EC2-style host health alone insufficient |
| VM refresh or rollback | Persistent data survives; lease/connection drain and schema skew tested |
| Database throttle/contention | Access-pattern budgets, idempotency and transaction retries preserve correctness |
| Region lost | Outside single-region scope; explicit residual risk, no cross-region DR claim |

Infrastructure owner/operator must select project isolation, region/zones, CIDRs, NAT IP/port bounds, broker placement, TLS/ingress/management policy, WAF posture, funding owner and budgets before paid provisioning. Product/storage owner must select database schema/capacity/cost and migration design. Product/infrastructure owners select RTO/RPO, N-1 sizing and workload compute before launch. CI owner qualifies producer/image/network before trusted builds. Missing decisions stop the relevant action; no automatic defaults authorize effects.

## Cost methodology

730 hours/month, USD, no free tier/credits/discounts/tax. Same partial-footprint boundary as AWS: instances/disks, application database, backup, model/provider, CI and optional security excluded. Three environments remain persistent. GCP has no AWS LCU unit: compare fixed components separately and retain explicit traffic allowances rather than invent equivalence. See COST_MODEL.json for formulas and scenarios. Neither estimate is a complete production total or saving guarantee.

Primary sources: https://cloud.google.com/nat/pricing ; https://cloud.google.com/load-balancing/pricing ; https://cloud.google.com/dns/pricing ; https://cloud.google.com/vpc/network-pricing ; https://cloud.google.com/nat/docs/overview ; https://cloud.google.com/compute/docs/instance-groups/regional-migs ; https://cloud.google.com/iap/docs/using-tcp-forwarding ; https://docs.cloud.google.com/pubsub/docs/exactly-once-delivery ; https://cloud.google.com/firestore/native/docs/overview ; https://cloud.google.com/icons . Checked 2026-11-13. Service-specific workload prices require final selected quantities.

### Conditional monthly comparison

Three-project scenario: one HTTPS forwarding-rule group, one reserved NAT IPv4 and one DNS zone per environment. DNS is approximately $0.20/month/zone at 730 hours. Forwarding rules cost $18.25/project/month for the first group of up to five rules; these are grouped per project, not $18.25 for every rule. Forwarding-rule public addresses incur no separate public-IP charge under current pricing. NAT address costs $3.65/month per assumed address; address capacity remains a design gate.

| NAT-assigned VMs per environment (scenario only) | Three-environment fixed + VM-dependent NAT charges | Same $165 usage allowance | Partial total |
| --- | ---: | ---: | ---: |
| 0 | $66.30 | $165 | $231.30 |
| 4 | $78.56 | $165 | $243.56 |
| 8 | $90.83 | $165 | $255.83 |
| 32 | $164.41 | $165 | $329.41 |
| 33 or more | $162.66 | $165 | $327.66 |

The small 32-to-33 discontinuity follows published NAT tiers and is not a rounding error. More NAT addresses increase cost by $3.65/address/month. The zero-VM row still assumes reserved NAT IPs; it is not a guaranteed minimum. One-project placement with three global forwarding rules can share the first-five group, reducing these scenario totals by $36.50/month; exact existing project rule inventory can change this. This is a different project-boundary cost behavior from AWS, where the modeled three VPC NAT/ALB footprint is unchanged by account placement.

AWS's $452.295 includes $17.52 of assumed LCU consumption; GCP has no LCU. GCP processing/CDN/request/transfer usage remains within provisional allowances only as a placeholder, not an equal-volume quote. Equal dollars do not imply equal workload coverage. A defensible operating comparison requires measured requests, byte volumes (GB versus GiB conversion), cache ratio, logs, NAT VM/IP allocation and database/build demand. Neither cloud total includes instances, disks, database/backups, model usage, CI, security extras or taxes. No cross-cloud savings guarantee follows from these partial scenarios.

Control-database operations are within provisional usage allowances, matching AWS control DynamoDB treatment; application database remains excluded. Google network pricing currently lists forwarding-rule IP addresses as no charge; Gemini v1 recommended adding $0.005/hour based on a general VM-IP rule. That recommendation is not adopted because the forwarding-rule-specific primary tariff takes precedence. Source: https://cloud.google.com/vpc/network-pricing (checked 2026-11-13).

## Final review refinements and implementation acceptance

The cost scenarios assume exactly one Public NAT gateway/configuration per environment. The published >32 assigned-VM tier is a flat $0.044/gateway-hour, while up to 32 uses $0.0014/assigned-VM-hour (https://cloud.google.com/nat/pricing). The 33+ row is conditional on that published tier continuing to apply and one gateway; recheck price and actual gateway/VM inventory before budget acceptance. Every partial total includes a separately displayed $165 placeholder allowance. NAT processing is allowance-covered only provisionally: measure processed GiB and apply $0.045/GiB without double-counting allowance. Count all global/regional forwarding rules per project, including HTTP redirects, and existing tier inventory before accepting the group-charge scenario.

Infrastructure/security owner must decide and document management recovery before launch. Add IAP outage, OS Login IAM denial and identity-provider failure to recovery qualification: audit the denial/outage, preserve access controls, use only separately approved recovery access, or accept bounded drain/replace without interactive access. No undeclared public SSH fallback.

Before network approval, produce a firewall matrix with separately scoped LB proxy and health-check source ranges, IAP source range (currently 35.235.240.0/20 for IPv4), required destination ports, target service accounts/tags, and explicit intra-VPC app/gateway/worker flows. Recheck product-specific ranges and IPv6 rules; do not equate IAP and LB sources or allow whole-VPC ingress by default.

Firestore adapter qualification must cover conditional-write preconditions, item/document atomicity and contention, transaction limits/retries, indexes/query compatibility, observed per-document write demand, TTL asynchronous behavior, streams/change-event equivalents, database mode/cost, and lossless export/import/migration/rollback. Verify current limits from primary service docs; do not adopt a universal one-write-per-second limit from reviewer memory. Durable job idempotency is mandatory regardless of queue delivery mode. Pub/Sub qualification must cover ordering keys, maximum lease/ack extension and stalled clients, retention/seek/replay semantics and poison-job dead lettering. Acknowledgment is not proof of exactly-once provider side effects.

CI federation tests must refuse fork PR, wrong repository/ref and wrong environment claims, and prove untrusted producers cannot write trusted proof/artifact objects or assume publisher roles. These are planned pass/fail requirements, not existing test results.
