Transferred `v-dev.ai`, `v-fin.ai`, and `v-sec.ai` from the personal AWS account into Agent Logic company-account registration management. Fresh source/destination inventories and all six successful AWS operations verify the transfers. Auto-renewal, expiry, company contacts, transfer locks, nameservers, and DNSSEC configuration are preserved.

The existing NS/SOA-only hosted zones remain in the personal account under the operator-approved exception. No DNS changes or resource deletion occurred. Coming-soon websites and the associated DNS ownership decision are deferred to v0.93.

Validation: before/after zone records and public DNS checks match; existing company contact email routing and enabled auto-renewal are verified. Standard registrar renewal notifications are retained; no custom alarm or email-delivery test is claimed. Independent evidence/privacy review and six focused native card-projection tests passed. The Rust tests prove lifecycle tooling only; sanitized live AWS/DNS evidence is in `.csdlc/evidence/1017/`.

Closes #1017
