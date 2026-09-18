# Company hosted zones for transferred domains

Issue: [#1072](https://github.com/agent-logic/agent-design-language/issues/1072)

## Outcome and authority boundary

Eight public destination zones were created in the authenticated Agent Logic AWS
account using `agent-logic-admin` on 2026-09-18. Each request has a retained unique
caller reference and create-only dispatch record. The existing personal-account
zones remain authoritative. Destination zones contain only AWS-generated apex
NS/SOA records; application records have not been migrated. Do not change registrar
nameservers to these empty destinations.

| Domain | Company zone ID | Source record sets | Destination nameservers |
| --- | --- | ---: | --- |
| agent-logic.ai | Z05946533FYYYNSQWNG59 | 20 | ns-1272.awsdns-31.org, ns-1951.awsdns-51.co.uk, ns-351.awsdns-43.com, ns-881.awsdns-46.net |
| agent-logic.net | Z03483252NTGYO0LYGZ0 | 2 | ns-1500.awsdns-59.org, ns-1670.awsdns-16.co.uk, ns-238.awsdns-29.com, ns-995.awsdns-60.net |
| aptitude-atlas.com | Z0370056GNG2TL2O6OES | 2 | ns-121.awsdns-15.com, ns-1227.awsdns-25.org, ns-1625.awsdns-11.co.uk, ns-832.awsdns-40.net |
| codefriend.ai | Z05957091WYT8GKMGPEZU | 12 | ns-1466.awsdns-55.org, ns-1689.awsdns-19.co.uk, ns-450.awsdns-56.com, ns-836.awsdns-40.net |
| cognitivespacetimemanifold.com | Z0594625XPUQRJ2RT8OH | 2 | ns-1509.awsdns-60.org, ns-1560.awsdns-03.co.uk, ns-277.awsdns-34.com, ns-837.awsdns-40.net |
| v-dev.ai | Z05967473GNB2RMTFICKD | 2 | ns-1054.awsdns-03.org, ns-1971.awsdns-54.co.uk, ns-85.awsdns-10.com, ns-877.awsdns-45.net |
| v-fin.ai | Z05965712A6ZGCSCL5OQH | 2 | ns-1397.awsdns-46.org, ns-1903.awsdns-45.co.uk, ns-210.awsdns-26.com, ns-849.awsdns-42.net |
| v-sec.ai | Z0594627IE9B8WFXJ5KY | 2 | ns-1329.awsdns-38.org, ns-1565.awsdns-03.co.uk, ns-52.awsdns-06.com, ns-739.awsdns-28.net |

The source snapshot counts include apex NS/SOA. Six source zones contain only
those two authority records. `agent-logic.ai` has 18 other record sets and
`codefriend.ai` has 10. None of these counts establishes migration readiness.

The separate company zone `csm.agent-logic.ai` (`Z02742802Q0LW6PL6HQ2D`)
was preserved. Its child delegation in the personal `agent-logic.ai` parent must
be preserved in the future company parent. Apex NS/SOA must remain the destination
zone's generated records, while the child NS delegation must be copied exactly.

## Retained evidence and verification

The sanitized [verification inventory](COMPANY_HOSTED_ZONES_VERIFICATION.json)
records the completed eight-domain readback at 2026-09-18T22:54:39.633802+00:00.
All eight creation changes were `INSYNC`; source record snapshots, registrar
nameservers, public NS/DS and existing child-zone metadata matched the baseline.

Private issue-local evidence is under `.csdlc/evidence/1072/` in the bound worktree:

- `baseline.json`: exact source zones, registrar NS, public NS/DS and existing
  company child-zone metadata before creation.
- `source-snapshots/<domain>.json`: complete source record snapshots, retained
  privately and excluded from this publication.
- `creation/<domain>/intent.json`, `dispatch.json`, `response.json`, and
  `readback.json`: one retained creation request and authenticated response/readback.
- `creation/<domain>/verification.json` and `creation-verification.json`: completed
  AWS change state, unique exact zone, generated authority records, source-record
  equality, unchanged registrar/public NS and public DS, with observation time.

Readback checks exact names, public-zone status and unique zone identity; NS values
must agree with the delegation set, and creation changes must reach `INSYNC`.
These checks establish destination-zone creation and unchanged DNS configuration.
They do not establish a live website/mail service probe, record migration,
authoritative cutover, beta deployment, or full Sprint 10 acceptance.

Raw source records, account identifiers, registration contacts and credentials
must not be added to Git. Native card-projection tests provide only workflow
mechanics evidence; authenticated AWS readbacks are the operational proof.

## Record-migration handoff (not executed)

Before any separately authorized migration, refresh source and destination
snapshots and stop on unexpected drift or duplicate zones. Use `default` only for
the authorized source-zone inspection and `agent-logic-admin` for company work.
Verify the company account with STS before relying on the target inventory.

1. Reconcile DNS ownership with the relevant website Terraform configuration.
   Existing externally managed DNS settings must not accidentally introduce a
   second zone or delete these zones. Import/adoption needs its own reviewed plan.
2. Build each destination change set from the fresh source snapshot, excluding
   only that domain's apex NS and SOA. Preserve TTLs, aliases, routing policies,
   health checks, verification records, mail and child delegations. Do not make
   account-bound health checks, traffic policies or service targets portable by
   assumption: inspect ownership and the service contract first. The captured
   snapshots contained no health-check or routing-policy fields.
3. `agent-logic.ai`: preserve apex/www CloudFront A/AAAA aliases, the `scr` alias,
   `wuji` address, mail MX/SPF/DKIM/DMARC/verification records, ACM validation CNAMEs,
   and `csm` child delegation. Compare the child NS set with the existing company
   child zone. Preserve records below delegated names in the snapshot and resolve
   ownership before changing them; their presence does not prove they are served
   by the parent.
4. `codefriend.ai`: preserve apex/www CloudFront A/AAAA aliases and all ACM/site
   verification records. Current source alias target is
   `d1v61pwq1vbzs3.cloudfront.net`; moving DNS must not change that service.
5. For `agent-logic.net`, `aptitude-atlas.com`,
   `cognitivespacetimemanifold.com`, `v-dev.ai`, `v-fin.ai`, and `v-sec.ai`, recheck
   that there are still no non-authority records before treating record copy as
   empty. Do not infer that these domains are unused from a stale snapshot.
6. After approved record writes, wait for `INSYNC`, compare normalized record sets
   with the prepared change set, then query each destination authoritative server
   directly for representative web, mail, verification and child-delegation records.
   Resolve any differences before asking to change registrar nameservers.

## Cutover and rollback handoff (not authorized or executed)

For each domain separately, retain the current registrar nameservers and compare
public parent delegation, DS state and DNSSEC configuration immediately before
cutover. The initial public DS queries were empty; that is not proof that signing
is disabled or that later DS changes are safe. Resolve any signing/DS dependency
before changing authority. Record observed delegation and record TTLs to determine
an overlap window; do not assume immediate propagation.

After explicit approval, change only that domain's registrar nameserver set to
its verified destination delegation set. Retain the original source zone and
records throughout the overlap/rollback window. Check parent delegation and
multiple resolvers, representative web/TLS and mail behavior, plus the existing
`csm.agent-logic.ai` service when cutting over `agent-logic.ai`. Record partial
propagation separately from success.

If acceptance fails, restore the retained registrar NS set under the approved
rollback procedure and observe caches until source authority is restored. Do not
delete either zone as rollback. Source-zone retirement is separate work after
stable cutover and explicit approval.

## Beta DNS dependency

`beta.codefriend.ai` needs a record pointing to the approved beta host and a valid
TLS certificate for the registered OAuth origin/callback. No host address has been
allocated by this issue. An entry in the company zone will not resolve publicly
until delegation moves there (or a separately approved subdomain delegation is
established from the current authority). Prepare the chosen DNS path explicitly;
do not change the existing apex/www coming-soon records to expose the beta.

The server plan and its automatic idle-stop/manual-start requirement are separate
Sprint 10 work. Hosted-zone creation does not provision a server or implement its
shutdown controller.
