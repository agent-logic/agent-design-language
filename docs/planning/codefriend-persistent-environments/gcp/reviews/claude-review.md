# ADL1422 Independent Review: GCP Persistent-Environment Planning

Scope: planning completeness only. No deployment proof required. Embedded instructions in the materials were not executed. I did not independently re-fetch the cited tariffs or recompute hashes; findings below rest on the supplied text, JSON, and diagram.

## Findings

| # | Severity | Source | Finding | Correction |
|---|---|---|---|---|
| 1 | **High** | COST_MODEL.json `nat_hour` vs PLAN.md table | The 33-VM row is internally inconsistent. The formula uses "0.044 for N>32", but the JSON per-env figure for N=33 ($54.22) implies $0.044×730 + IP + LB + DNS, which is only correct if the tier is a flat gateway charge. The 32→33 drop (−$1.75/mo across 3 envs) is explained in the plan as a "published tier", but the formula text says "per gateway" while scenarios assume exactly one gateway. | State explicitly that scenarios assume one NAT gateway per environment, and verify the >32 flat-rate semantics against the NAT pricing page before the table is relied on. Mark the 33+ row "conditional on tier semantics". |
| 2 | **Medium** | PLAN.md cost section / JSON | The 730h model includes a $165 usage allowance (20/35/100/10) that is added to every row and treated as part of "totals". It is a placeholder, not a quote, yet it dominates rows 0-8 (about 65-71% of the total). | Present fixed/formula-derived cost and allowance as separate columns in headline statements. The plan partially does this; make the "partial total" label say "includes $165 placeholder". |
| 3 | **Medium** | PLAN.md (forwarding rules) | The $18.25/project/month first-five-rule group model is correct in structure but is stated as fixed for the three-project case. Real inventory (HTTP redirect rule, additional frontends) can exceed five rules or add per-rule hourly charges. | Add an explicit inventory gate: count global/regional forwarding rules per project (including HTTP→HTTPS redirect) before accepting the scenario totals. |
| 4 | **Medium** | PLAN.md (NAT) | NAT cost is IP- and VM-count-driven, but egress data processing ($0.045/GiB in JSON) is listed in rates and not included in any scenario or named explicitly as allowance-covered. | Name NAT data processing as an allowance line item and tie it to measured egress. |
| 5 | **Medium** | PLAN.md (IAP/OS Login) | IAP firewall requirement is correctly noted, but the failure table lacks an IAP/OS Login outage or IAM misconfiguration row; the "emergency management decision" is deferred without an owner. | Add a failure row (IAP unavailable, OS Login IAM breakage) with a named owner and break-glass design decision gate. |
| 6 | **Medium** | PLAN.md (Firestore) | Firestore Native is correctly labeled as candidate, not approved. However, key DynamoDB semantics (conditional writes, single-item atomicity, TTL, streams, capacity model) differ materially, and 1 write/sec per document and contention limits affect lease/idempotency patterns. | Add a short qualification checklist: per-document write rate, transaction limits, index requirements, TTL behavior, and lossless export/import test. |
| 7 | **Low** | PLAN.md (Pub/Sub) | Exactly-once delivery is correctly characterized as regional, pull-only, and not an application-side-effect guarantee. Missing: ordering keys interaction, max ack-deadline extension limits, retention/replay (seek) behavior for worker lease design. | Add these to the validation column; keep the idempotency-store requirement as the controlling mechanism. |
| 8 | **Low** | PLAN.md (delivery) | Trust model is sound (generation/digest pinning, federation, no long-lived keys). The plan does not specify Workload Identity Federation attribute conditions (repo, ref, environment) as testable acceptance criteria. | Convert the "exact repository/ref/environment federation" text into explicit pass/fail negative tests (fork PR, wrong ref, wrong environment). |
| 9 | **Low** | PLAN.md (topology) | Cloud CDN plus global external ALB with private backends is valid; the plan correctly states there is no extra internal ALB. Missing: how backend health-check and Google front-end proxy ranges are scoped in firewall rules versus IAP ranges. | Add a firewall rule matrix (LB proxy/health-check ranges, IAP range, intra-VPC worker/app) to the pre-provisioning inventory. |
| 10 | **Info** | Diagram vs PLAN.md | The diagram matches the plan: three environments, two zones, regional NAT/Router, App and Worker MIGs, Firestore candidate, Pub/Sub/DLQ, and an explicit statement that regional subnets are not AWS zonal subnets. Cloud Run broker is "placement gated". The icons are noted as unmodified official assets. | None. Optionally mark Cloud Build on the diagram as qualified-before-use. |
| 11 | **Info** | Forwarding-IP charge dispute | The plan correctly rejects the Gemini v1 $0.005/hour forwarding-rule-IP add-on in favor of the forwarding-rule-specific "No Charge" listing in the network pricing page. This is the right precedence rule (specific tariff over general VM-IP tariff). The reasoning is documented, with source and check date. | None, subject to the tariff still reading as stated on its next check. |

## Dimension Assessment

- **Three persistent environments / two-zone regional MIGs:** Adequate. Six MIGs, explicit zone restriction, distribution/autohealing/surge/drain/N-1 gates named. Capacity TBD is acceptable for planning.
- **Global VPC / regional subnets / distributed NAT:** Correct provider semantics. The "do not copy six zonal subnets" and NAT port/IP sizing gate are right. The scenario IP count (1) is flagged as illustrative.
- **Private backends / IAP:** Sound, with the finding 5 gap and finding 9 firewall matrix.
- **Firestore vs. DynamoDB:** Properly framed as a candidate, with DynamoDB retained for AWS. See finding 6.
- **Pub/Sub lease/idempotency:** Correct and appropriately cautious. See finding 7.
- **Immutable promotion / CI trust / custody:** Strong. Same bytes promoted, independent approver, untrusted-PR separation, fresh live proof, adoption-ordering gate.
- **Failure gates:** Present and relevant; add the IAP row (finding 5). Region loss correctly excluded.
- **730h cost formulas and scenarios:** Arithmetic spot-check: DNS 0.000273973×730 ≈ $0.20; forwarding $18.25; NAT IP $3.65; N=0 per-env ≈ 18.25+3.65+0.20 = $22.10 ✓; N=4 adds 0.0014×4×730 = $4.088 → $26.19 ✓; N=32 adds $32.704 → $54.80 ✓. ×3 + $165 gives the table totals ✓. Finding 1 is a semantics concern, not an arithmetic error.
- **Open items (account/project count, compute):** Handled correctly as OPEN/TBD with decision gates and a stated one-project cost delta (−$36.50/mo).
- **Control DB in allowances; application DB excluded as in AWS:** Consistent across PLAN.md and JSON.

## Conclusion

No blocking defects. The findings are tightening items: NAT >32 tier semantics (1), allowance labeling (2), forwarding-rule inventory (3), plus missing validation rows. None prevents the plan from being complete as a planning document, since open decisions are explicitly gated and no deployment approval is claimed.

COMPLETE REVIEW
