# RD02 repository boundaries and cutover contract

Issue: #1182. This is a technically reviewable contract with **operator policy
acceptance pending**. It does not authorize repository bootstrap, source export,
access changes, product activation or release. A passing technical validator is
not an accepted policy decision.

## Source and ownership denominator

RD01 #1181 closed through native `finish`, generation 35, with no unknown effects.
PR #1198 merged as `48b4cc15e8cbe29654ec6d485594149c8bb82682`; its reviewed
candidate was `682be6b8babb69f73c5e9fef8580f1b3c88966d5`.

`ownership.jsonl.gz` covers all **36,899** paths in RD01's frozen
`248f00e359e412f6bc0061ac88be9ff374e3a212` census. Every row preserves the
path, mode, object, size and type, names one accountable repository and retains
the detailed proposed disposition. `opening-ownership.jsonl.gz` separately
covers all **42** changes through opening revision
`08236f86026f2e317e7f405bb0381b782a1417b9`. Neither candidate nor opening
silently replaces the frozen denominator.

`mapping-inputs/` preserves the source-backed lane records. Explicit tool
proposals replace the mixed-map unresolved placeholders. Five overlapping
proposals have explicit adjudications, applied last. `mapping-audit.json`
records their hashes and final output identity. The earlier tool proposal's
`remaining` list is retained source-time evidence, not the final denominator;
the complete residual packet and assembled mapping supersede it.

The subsequent five-path `provider-boundary-dispositions.json` review correction
has final precedence over those proposals. It separates the public
`ProviderSpec`/`ProviderMap` schema leaf from operational provider-core, replaces
the public schema's private reexport, and assigns endpoint/key admission policy
to Runtime while keeping portable document syntax with ADL. This prevents
whole-file family labels from hiding private dependencies. Consumers share one
schema implementation; independent extraction and wire compatibility still need
proof.

The accountable repository may be a product owner, original-file integration
custodian or historical evidence custodian. It is not necessarily permission
to export the entire file. Mixed component and section responsibilities remain
explicit in each row. Original evidence custody does not retire active planning
or validators, confer public export rights, or authorize a second maintained
implementation.

## Repository identities and technical boundaries

| Repository | Visibility | Boundary |
|---|---|---|
| agent-design-language | Existing public | Portable language, compiler, schemas and conformance; original history/evidence custody |
| cognitive-sdlc | Approved new private name | Lifecycle owners, card/template/manual resources, workcell orchestration and retained v2 rollback |
| agent-logic-runtime | Approved new private name | Runtime/CSM, providers, generic memory/continuity, Observatory and baseline enforcement |
| codefriend | Approved new private name | Review acquisition, reports, server/local agent, product memory interpretation and CLI |
| codefriend.ai | Existing private | Website, sign-in boundary and site-specific application/deployment contracts |
| agent-logic-infrastructure | Approved new private name | Reusable build, assembly and deployment mechanisms |
| agent-logic-enterprise-security | Approved new private name | Future optional narrowing-policy/backend interface |

No enterprise backend implementation or enterprise-owned implementation
component was established by the census. Planning references and a future
interface are not an implemented artifact. Runtime retains baseline enforcement;
an enterprise adapter may narrow grants and must never broaden them when invalid
or unavailable.

The mixed `adl` crate cannot move intact. Source, CLI, manifests, tests and
documentation must follow their actual consumer contracts. Provider execution
and generic memory have one Runtime implementation; CodeFriend owns its review
interpretation and adapters. Runtime stewards one small resilience distribution
usable by retained C-SDLC v2 without installing the Runtime application.
Versioned directory names do not authorize retirement or a command switch.

Public ADL build, test, package and conformance must work without private
credentials, private implementation or sibling checkouts. Optional private
C-SDLC contributor tooling is not a public build dependency. Artifact edges in
`contract.json` distinguish that use explicitly.

## Delivery and access proposals

The eight artifact/interface rows in `contract.json` identify one producer,
consumers, usage class, immutable identity fields, rollback obligation and
qualification limits. They cover public ADL, C-SDLC, Runtime, resilience,
CodeFriend, website, delivery mechanisms and the future enterprise interface.

Proposed private delivery is producer-owned GitHub Releases with authenticated
consumer read access. Consumers pin a source revision, asset identity, digest,
package/dependency lock and supported interface versions; a mutable `latest`
selector or public fallback is not acceptable. Digests alone are not signatures.
Reuse a supported signer only when actual signing evidence can be verified.
Do not invent a signing service or assume repository membership proves release
asset access. Effective principals and authenticated download are downstream
bootstrap/installation proof.

These consolidated choices remain **pending**:

- Inherited organization-member read access on the five private destinations.
- Daniel as the access, original evidence and rollback approver.
- Private GitHub Releases, producer-write and consumer-read distribution.
- Preservation of current license metadata and original history/evidence in place.

Names are approved; these policy proposals are not. Current root and package
licenses differ (Apache-2.0 versus MIT OR Apache-2.0 in some manifests).
Preserve notices and source provenance; extraction must not silently relicense
or flatten that distinction. No blanket export of `.csdlc` records is permitted.
Copied recovery fixtures need a bounded reviewed allowlist and original object
references. Existing public history cannot be made confidential retroactively.

## Cutover, rollback and downstream proof

RD02 acceptance precedes RD12 bootstrap and RD13 immutable issue mapping.
Producer extraction then supplies versioned artifacts to independent consumers;
RD11 accepts the complete lockset. Do not replace that dependency order with
simultaneous exports or independently refreshed shared binaries.

At each actual cutover, record the source revision/path manifest, destination
revision, artifact identities, consumer versions, active authority and preserved
pending operations. Freeze only the affected writer during export/activation.
One writer remains authoritative; unknown remote effects require authenticated
reconciliation, never a duplicate dispatch. Keep the previous accepted lockset
and retrievable artifacts until new installed consumers and rollback pass.
Rollback restores the admitted previous versions and authority, not fabricated
receipts or hand-written lifecycle state. Approval authority remains pending as
listed above.

Required later proofs include public ADL with no private dependencies; complete
independent C-SDLC installation and uncertain-effect recovery; Runtime continuity
and Observatory behavior; CodeFriend server/local-agent and memory consumer
contracts; authentic private artifact download; and complete demo/documentation
coverage. Source mapping, fixture tests and green CI do not substitute for these
installed outcomes. No product is qualified by this packet.

## Validation and review boundary

Run `python3 docs/milestones/v0.93.1/repository-decomposition/rd02-1182/validate_contract.py --self-test`
for technical consistency and adversarial contract checks. The separate
`--require-policy-acceptance` gate must fail while policy remains pending; a
self-asserted approval flag cannot grant authority. The focused Rust adapter
integrates the same proof with native issue validation.

PVF: deterministic planning-contract proof, local Python/Git and bounded CPU/disk,
required for this issue. No provider, browser, cloud or installed-product run.
Review covers exact structural metadata and precedence plus bounded semantic
inspection. It is not an exhaustive manual audit of all 36,899 files.
