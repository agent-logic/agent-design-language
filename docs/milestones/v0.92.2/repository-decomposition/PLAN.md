# Repository decomposition plan — v0.92.2 / #848

Status: candidate for independent review under #848. No extraction, repository
creation, visibility change or v0.93 implementation is authorized.

Source: the operator-provided TBD candidate, reconciled with the tracked v0.93
security contracts. The original candidate remains unchanged. Current inventory
is [source-inventory.json](source-inventory.json), anchored to its recorded Git
revision. This is a planning input, not completed RD-01 audit proof.

## Purpose

Reduce review, ownership, build, and release complexity by separating independently operated ADL systems into a small number of repositories with explicit contracts.

Repository splitting does not itself reduce code volume. It is useful only when the resulting repositories have clear owners, narrow dependency direction, independent validation, and independent release lifecycles. Code reduction should precede or accompany extraction.

The split must also allow C-SDLC and CSM/Runtime to evolve independently. Each product should be free to change its architecture, implementation language, release cadence, compatibility policy, and roadmap without forcing unrelated migration or validation work on the other.

## Governing product-boundary hypotheses

C-SDLC and the CSM/Runtime are intended to be two different products that were historically built in the same repository. RD-01 must attempt to falsify, rather than assume, these hypotheses:

- **H1:** neither product is a component, service, control plane, or functional dependency of the other;
- **H2:** neither product has an unavoidable build, validation, release, deployment, or operational dependency on the other;
- **H3:** current cross-product references can be removed or replaced by independently versioned generic contracts without weakening either product.

- C-SDLC is a repository-agnostic software-development lifecycle product.
- CSM/Runtime is an ADL execution and multi-agent Runtime product.
- C-SDLC can be used to develop CSM/Runtime, just as it can be used to develop any other repository. That operational use does not create product coupling.
- CSM/Runtime must build, validate, release, deploy, and operate without C-SDLC.
- C-SDLC must build, validate, release, and operate without CSM/Runtime.
- A change to either product must not gate the other unless both independently choose to adopt a versioned generic contract.

If RD-01 finds contrary evidence, the map must report the edge and its proof rather than classifying it as accidental by definition. The target is to remove accidental historical coupling without inventing a new product dependency.

For each hypothesis, RD-01 must return exactly one result:

- **survived:** the complete denominator and isolated simulations found no
  unavoidable dependency;
- **falsified:** at least one evidenced dependency cannot be replaced by a
  generic published contract without changing product behavior;
- **unresolved:** the denominator, proof, or decision authority is incomplete.

Missing evidence never counts as survival. A falsified or unresolved hypothesis
stops any extraction whose safety depends on it and returns the dependency to
the operator for a boundary decision.

## Why split

The split has five product-level benefits beyond reducing repository size:

- **Independent evolution:** ADL, Runtime/CSM, C-SDLC, infrastructure, and enterprise security can change architecture, language, compatibility policy, and release cadence without forcing unrelated work on the others.
- **Faster build and validation:** each repository can run proof lanes for its own product rather than rebuilding and revalidating the historical monorepo.
- **Operational and logistical isolation:** ownership, credentials, deployment access, release permissions, issue flow, and incident response can follow the needs of each product.
- **Independent commercialization:** ADL, Runtime/CSM, C-SDLC, and the enterprise security package can be licensed, priced, supported, and sold as distinct products or composed through explicit versioned contracts.
- **Deliberate public/private control:** repository boundaries make visibility an explicit product decision. Public contracts, SDKs, schemas, and interoperability fixtures can remain reviewable, while proprietary implementations, enterprise controls, customer-specific integrations, sensitive qualification evidence, and restricted operational material can stay private without making the public products incomplete.

These are expected benefits, not completed outcomes. A repository boundary can
reduce source, CI, release, credential, and ownership blast radius; it does not
by itself isolate production failures. RD-01 must measure the current baseline
and test whether the proposed boundaries actually improve build time,
validation time, review size, release independence, infrastructure lifecycle,
and organizational flexibility.

## Current baseline

The current source inventory records every tracked Cargo manifest, top-level
tracked-path counts, and the seven selected product manifests with explicit
checkout-relative dependency declarations. These are current monorepo edges,
not the permitted post-extraction topology. RD-01 must classify and remove or
replace every relevant edge before extraction can be authorized.

The earlier TBD candidate included physical-line estimates without a retained
recomputable measurement packet. They are not used as current verified evidence
or as a claimed benefit of this plan. RD-01 must produce its own revision-bound
source, build, validation and release measurements.

Current ownership candidates are ADL for language semantics (`adl/`, `adl-v2/`),
Runtime for `adl-runtime/`, `adl-runtime-kernel/` and runtime-coupled resilience,
C-SDLC for active `csdlc-v3/` and restricted `csdlc-v2/` compatibility,
infrastructure for deployment tooling, and repository governance for immutable
historical `.csdlc/` evidence. Mixed `adl/` modules, demos, CI and evidence remain
explicit audit rows; these directory-level candidates are not a completed
path-level ownership assignment.

## Proposed repository boundaries

The accountable owner roles are provisional until the operator assigns people:

| Boundary | Accountable owner role |
|---|---|
| `agent-design-language` | ADL product owner |
| `agent-logic-runtime` | Runtime product owner |
| `cognitive-sdlc` | C-SDLC product owner |
| `agent-logic-infrastructure` | Cloud infrastructure owner |
| `agent-logic-enterprise-security` | Enterprise security product owner |

Every source path and shared artifact class must have exactly one accountable
owner. A consumer may validate a published artifact but does not thereby become
its co-owner. Unresolved ownership is an explicit RD-01 finding with a named
decision owner and deadline, not permission to duplicate the artifact.

### 1. `agent-design-language`

Own the language and execution contract:

- syntax, schema, parser and compiler;
- core execution semantics;
- provider interfaces, not provider-instance secrets or deployment configuration;
- stable public SDK and CLI;
- conformance fixtures and core compatibility tests.

It should not own cloud-account operations, C-SDLC lifecycle implementation, the production Runtime control plane, or product-specific demonstrations.

### 2. `agent-logic-runtime`

Own the production Runtime:

- `adl-runtime/`;
- `adl-runtime-kernel/`;
- `adl-resilience/` while it remains Runtime-coupled;
- CSM, agent admission, continuity, transport and Runtime operations;
- Runtime API contracts and Runtime-focused qualification;
- the Observatory while its release remains coupled to Runtime APIs.

This repository consumes versioned ADL language and execution contracts. It must not reach into ADL repository internals.

The Runtime and CSM have no functional, build, or release dependency on C-SDLC. Runtime operation must remain complete when C-SDLC is absent.

### 3. `cognitive-sdlc`

Own the development lifecycle product:

- `csdlc-v3/`;
- active lifecycle schemas and card templates;
- typed issue, review, publication, merge, finish and cleanup operations;
- C-SDLC-specific validators and operational documentation;
- only the characterization fixtures required to prove C-SDLC compatibility.

`csdlc-v2/` remains a restricted rollback path until v3 proves one complete milestone. Its later deletion is code reduction, not migration into the new repository.

The target C-SDLC v3 product has no functional, build, or release dependency on Runtime/CSM, and Runtime/CSM has no such dependency on C-SDLC. This is a target invariant to prove, not a claim about every retained monorepo artifact. The current inventory already records the restricted rollback edge `csdlc-v2 -> adl-resilience`; it is a known cross-boundary build dependency. RD-01 must assign its disposition and prove the active v3 extraction independent without deleting or stranding authorized rollback support. C-SDLC may operate on a Runtime repository in the same generic way it operates on any repository; that is tool usage, not product coupling.

### 4. `agent-logic-infrastructure`

Own deployable infrastructure:

- Terraform and HCL;
- AWS and GCP deployment modules;
- deployment inventories and account-safe operational tooling;
- infrastructure validation and rollback contracts;
- environment configuration that does not contain credentials.

Application source should not be duplicated here. Infrastructure should consume versioned build artifacts and published interface contracts.

### 5. `agent-logic-enterprise-security` (private, v0.93)

Own the proprietary policy content, enterprise backends, customer-specific
integration, qualification, and sensitive evidence scheduled by the v0.93
security plan:

- proprietary zero-trust and least-privilege policy sets;
- enterprise IAM, KMS, HSM, secrets, key-custody, audit, incident, data-governance, and compliance backends;
- customer configuration and private policy thresholds;
- sensitive qualification evidence, adversarial corpora, supply-chain checks,
  security-operations material, and proprietary Runtime-hardening packages.

This repository is the private implementation and productization boundary for
the six security work packages defined in
`docs/milestones/v0.93/features/ENTERPRISE_SECURITY_v0.93.md`. It does not
authorize external certification claims, and it must preserve the v0.93
distinction between internal control evidence and SOC 2, ISO 27001, FedRAMP,
HIPAA, or other external approval.

Security enforcement decision points remain public and Runtime-owned. The
public Runtime owns the policy decision and enforcement interfaces, a usable
default-deny baseline policy, per-message authorization hooks, audit-event
emission, isolation checks, and key/secrets provider interfaces. ADL owns only
the language-level security contracts that are genuinely part of portable ADL
semantics. No in-path enforcement decision point moves into the private
repository merely to create a product boundary.

The private package may consume published ADL and Runtime extension contracts.
It must not require public repositories to import private source, credentials,
or private build infrastructure. Integration uses this public three-state
loading contract:

1. **No enterprise provider configured:** the public Runtime's default-deny
   baseline remains authoritative and the Runtime is fully usable with a
   deliberately smaller authority surface.
2. **Enterprise provider configured but unavailable, unverified, or
   version-incompatible:** startup and affected decisions fail closed; there is
   no silent fallback to a weaker policy.
3. **Verified enterprise provider loaded:** private policy may supersede the
   public baseline only through the versioned public decision interface.

Public Runtime CI must prove states 1 and 2 with a public stub provider. Private
CI separately proves state 3 and must keep enterprise-only policy and evidence
private.

Public Runtime CI must also prove the mechanics of state 3 with a synthetic
public provider containing no proprietary policy. Private CI proves the real
enterprise provider. The private provider may narrow authority within the
public contract; it may not bypass authentication, default-deny behavior,
per-message authorization, audit emission, isolation, or fail-closed handling.
The v0.93 contract gate must define provider authentication, health transitions,
timeouts, cached-decision expiry, trust-root revocation, rollback, and recovery
after provider loss before enterprise implementation is authorized.

Repository visibility must be decided per product and per artifact rather than
inherited accidentally from the historical monorepo. Interfaces needed for
interoperability belong in the relevant public contract owner; proprietary
implementation, customer configuration, sensitive evidence, and restricted
operations belong in private repositories or private systems. Moving material
between those boundaries requires an explicit disclosure and dependency review.
The product security owner approves that review using a minimal record that
classifies policy content, customer data, credentials, qualification evidence,
adversarial corpora, public contract necessity, and intended repository.

For each v0.93 security WP, the implementation plan must separately assign
ownership and visibility for its **contract**, **implementation**, **fixtures**,
and **evidence**. Public deny reasons and audit vocabularies must remain abstract
and stable; proprietary rules, thresholds, customer identities, and sensitive
evidence must not leak into public schemas or fixtures.

Provisional ownership for that later planning gate is:

| v0.93 WP | Public contract owner | Implementation owner | Public fixtures | Sensitive evidence |
|---|---|---|---|---|
| WP-S1 identity and zero trust | Runtime | Enterprise security | Runtime conformance fixtures | Enterprise security/private evidence store |
| WP-S2 least privilege | Runtime | Runtime enforcement plus enterprise policy backend | Runtime deny-by-default fixtures | Enterprise security/private evidence store |
| WP-S3 key and secret custody | Runtime provider interface | Enterprise security backend | Runtime synthetic-provider fixtures | Enterprise security/private evidence store |
| WP-S4 audit and monitoring | Runtime audit schema | Runtime emission plus enterprise integrations | Runtime sanitized event fixtures | Enterprise security/private evidence store |
| WP-S5 data governance | Runtime data-governance contract | Enterprise security policy backend | Runtime classification/redaction fixtures | Enterprise security/private evidence store |
| WP-S6 supply chain and release | Owning public repository per artifact | Enterprise security additions remain private | Public provenance fixtures | Enterprise security/private evidence store |

This matrix is provisional. The v0.93 contract gate must resolve split
implementation rows without moving public enforcement or release truth behind
a private dependency.

## Dependency direction

The intended dependency direction is:

```text
agent-design-language <--- agent-logic-runtime <--- agent-logic-infrastructure
                              ^
                              | public, versioned extension contract
          agent-logic-enterprise-security (private policy/backends)

observatory (initially Runtime-owned) ---> published Runtime API

cognitive-sdlc ---> only the generic versioned repository/CLI contracts it uses

cognitive-sdlc      || no functional dependency ||      Runtime / CSM
```

Cross-repository dependencies must use published versions, schemas, fixtures, or artifacts. Relative filesystem dependencies and source-tree reach-through are prohibited after extraction. Operational use of C-SDLC against another repository does not create a dependency edge from that product to C-SDLC or from C-SDLC to that product.

## Preconditions

No extraction may begin until these global conditions are true:

1. RD-01 produces the complete dependency and ownership denominator.
2. Each proposed extraction has a named owner, versioning policy, validation lane and release boundary.
3. Current `main` remains reproducibly buildable while extractions proceed.

C-SDLC v3 milestone proof gates RD-04 only. C-SDLC v2 deletion eligibility
gates deletion only; it does not gate ADL, Runtime, infrastructure, or demo
analysis. The known `csdlc-v2 -> adl-resilience` build edge must receive an explicit RD-01 disposition before an extraction would remove its required source or artifact. It does not create a blanket gate for unrelated product analysis. RD-05 must publish
versioned Runtime artifacts before RD-06 can consume them. The enterprise gate
below applies only to RD-09.

RD-09 alone has an additional gate: v0.93 enterprise-security planning must
define and review the public extension contract, private implementation
boundary, per-WP contract/implementation/fixture/evidence ownership, and the
three-state loading behavior. This does not gate C-SDLC, ADL, Runtime, or
infrastructure decomposition and does not authorize v0.93 implementation.

The independently governed retirement of C-SDLC v2 is an external C-SDLC
precondition, not an RD step. It requires complete-milestone v3 proof, explicit
deletion eligibility, independent review, and separate operator authorization.
This plan neither performs nor authorizes that deletion.

## Proposed execution sequence

Each step has one primary result and should become its own issue only after the plan is approved.

### RD-01 — Dependency and ownership map

Produce one machine-readable map of package dependencies, shared files,
generated artifacts, tests, CI lanes and release owners. Classify every edge as
public contract, internal dependency, evidence dependency, build dependency or
accidental coupling. Attempt to falsify H1-H3; inventory current security
enforcement points; classify every baseline row and shared CI workflow; and
name an owner plus decision deadline for every deferred disposition.

The denominator must cover every tracked path and report orphans. It includes
source, generated source, fixtures, documentation, workflows, release
automation, deployment inputs, demos, evidence, provider adapters, credential
consumers (references only, never secret values), registries, organization bots
and approval routes. It must detect checkout-relative access, source-tree
reach-through, symlinks/submodules, CLI coupling, generated-artifact provenance,
and hidden private-access assumptions. Clean-checkout build, test, packaging and
release simulations must run with sibling repositories and private access
unavailable where applicable.

Proof: the map covers every tracked manifest, current build/test entrypoint,
shared workflow, retained-evidence owner, and observed cross-product edge. Its
conclusion records whether each hypothesis survived falsification; it does not
hide unresolved source-tree reach-through as an assumption.

### RD-02 — Boundary decision and staged stopping point

Use RD-01 evidence to compare the five-repository target with smaller and staged
alternatives, including stopping after C-SDLC extraction or combining Runtime
and infrastructure temporarily. Decide demo ownership before moving an affected
demo. Produce one operator decision that authorizes, rejects, or sequences later
RD steps; it does not itself move code.

Proof: the decision table compares ownership, release cadence, CI cost,
operational lifecycle, visibility, contract overhead and rollback cost, and
records why each proposed boundary is independent enough to justify a repository.

### RD-03 — Publish shared ADL contracts

Produce one versioned contract package containing only the schemas, SDK interfaces and fixtures required by downstream repositories.

Proof: Runtime and other genuine ADL consumers build against the published
package without relative path dependencies. C-SDLC must not consume an ADL
contract unless RD-01 falsifies the independence hypothesis and the operator
explicitly accepts that dependency.

Rollback boundary: contract publication is additive; no existing contract is
removed until every supported consumer passes against the published version.

### RD-04 — Extract C-SDLC

Move the active v3 lifecycle implementation and its owned tests/templates into `cognitive-sdlc`.

Proof: the extracted repository independently builds, validates, publishes,
merges and finishes a shadow issue using only generic repository and CLI
contracts. It must not require ADL or Runtime source, artifacts, or private
access.

Rollback boundary: retain the pre-extraction C-SDLC source and release route
until the independent shadow issue completes. Deletion and v2 retirement require
separate authorization.

### RD-05 — Extract Runtime

Move Runtime host, kernel, resilience and the Runtime-owned Observatory into
`agent-logic-runtime`, and publish the Runtime artifacts and API contracts that
deployment tooling consumes.

Proof: a local polis builds and runs from the extracted repository, the
Observatory consumes its published API contract, versioned deployment artifacts
are available, and rollback to the pre-extraction tag is documented.

Rollback boundary: preserve the pre-extraction Runtime release and deployment
inputs until local-polis, API, Observatory, and artifact-consumer proof passes.

### RD-06 — Extract infrastructure

After RD-05 publishes its inputs, move cloud modules and account-safe
operational tooling into `agent-logic-infrastructure`.

Proof: plans and validation consume explicitly pinned Runtime artifacts and API
contracts without application-source duplication, credentials in Git, or
implicit access to the original monorepo.

Rollback boundary: infrastructure state is not migrated or applied by this
step; extraction is reversed if pinned-artifact planning or validation fails.

### RD-07 — Identify remaining ADL cleanup

Produce a deletion proposal for migrated adapters, duplicate tests, obsolete
wrappers and compatibility paths remaining in `agent-design-language`. Actual
deletion is a separately authorized follow-on, not part of extraction.

Proof: every proposed deletion has an owner, behavioral replacement, proof,
rollback source and explicit retirement authority; the unchanged repository
continues to pass its conformance suite.

### RD-08 — Apply and verify demo ownership

Apply the RD-02 ownership decision for the Runtime Observatory and every other
demo: colocate each with exactly one owning product or archive it under separate
authority. RD-08 cannot move an affected demo until RD-02 has decided its owner.
This five-repository plan does not create a separate demos repository.

Proof: every demo has exactly one owner and no duplicated production source.

Rollback boundary: ownership is a decision artifact until the relevant
extraction is separately authorized; archival or deletion is not implied.

### RD-09 — Establish the private enterprise-security repository

RD-09 is future sequencing only. Its presence in this candidate does not
authorize repository creation, visibility changes, licensing decisions, or
v0.93 implementation; each requires a later explicit operator decision.

Subject to a separate operator decision on name, visibility, registry, and
licensing, create a private enterprise-security repository and move or add
only the enterprise-owned implementation for the six v0.93 security work
packages. Keep public schemas and extension interfaces in their owning public
repositories; keep proprietary policy content, enterprise backends,
qualification, hardening packages, and enterprise operations private. Public
Runtime enforcement decision points do not move.

Proof: each v0.93 security work package has explicit contract, implementation,
fixture, and evidence owners and visibility;
the private repository validates independently against published contract
versions; public ADL and Runtime builds pass without private access; public
Runtime CI proves loading states 1 and 2 with a stub provider; and private CI
proves state 3 without exposing credentials or private evidence.

Rollback boundary: public products remain releasable and useful with no private
repository access. Private repository creation, licensing, visibility changes,
and implementation each require later explicit authorization.

## Migration rules

- Extract with history where practical; do not copy-and-abandon source trees.
- Preserve public schema and protocol compatibility before changing behavior.
- Keep one authoritative source for every shared schema and fixture.
- Do not combine extraction with broad feature work.
- Keep each extraction reversible until its consumer validation passes.
- Do not delete historical lifecycle evidence merely because its executable owner was retired.
- Do not leave compatibility shims indefinitely; each shim needs an owner and removal condition.

## Contract governance

- Each cross-repository surface declares one public owning repository and owner
  role, artifact type, registry class, semantic version, schema-evolution rules,
  supported consumer matrix, conformance owner, retention window, rollback
  policy, generated-artifact provenance and release-signing policy.
- Breaking changes require approval from the contract owner and every supported
  first-party consumer owner; consumers pin an explicit supported version range.
- Each contract declares a support and deprecation window before extraction.
  RD-01 recommends the window but does not invent one without consumer data.
- Cross-repository conformance fixtures live with the public contract. Provider
  repositories run those fixtures against every supported contract range; the
  public owner runs consumer-neutral compatibility checks.
- Private packages publish only their supported public-contract ranges and
  sanitized compatibility results, never proprietary policy or customer data.
- Shared CI workflows are either versioned reusable public workflows or
  intentionally duplicated and independently owned; no repository reaches into
  another checkout for CI logic.
- Schemas, SDKs, generated bindings, CLIs, containers and deployment artifacts
  have separate compatibility promises where their breakage modes differ; one
  package version must not conceal incompatible dimensions.
- Registry products, credentials and final support windows remain explicit
  operator decisions informed by RD-01 evidence.

## Public-product sufficiency and licensing boundary

Licensing remains an operator decision for source, packages, SDKs, artifacts,
fixtures and documentation. The public Runtime must pass a capability test with
no private access: useful default-deny operation, local policy configuration,
successful synthetic-provider conformance, complete audit behavior and an
independent public release. Enterprise value may add proprietary policy,
backends, support, integrations and evidence; it must not repair a deliberately
crippled public product.

## Success measures

The decomposition succeeds when:

- each repository has one coherent product boundary and release owner;
- no repository depends on another repository's checkout layout;
- focused CI does not require building unrelated products;
- the largest repository is materially smaller and easier to review;
- C-SDLC, Runtime, infrastructure and ADL core can release independently;
- the enterprise-security package can be versioned, licensed, sold, supported,
  and released independently from the public products;
- public ADL and Runtime builds never require access to the private repository;
- every shared artifact has an explicit visibility owner, and no public release
  unintentionally includes proprietary code, customer material, credentials,
  sensitive qualification evidence, or private operational details;
- total maintained code declines rather than merely moving between repositories.

RD-01 must record reproducible pre-change baselines and measurement commands for
cold and warm build time, focused and full validation time, CI compute, review
diff size, release lead time, repository checkout/storage overhead and failure
rate. Source accounting must separately classify maintained production,
generated, test, fixture, compatibility and retained-evidence code. Later steps
must compare the same workloads before and after change.

RD-01 should report resulting repository sizes and review/build effects. No
line-count ceiling is an architectural acceptance gate until the audit
classifies generated, retained, test, compatibility, and production code and
provides evidence for an appropriate target.

## Risks and open decisions

- `adl/` currently mixes several responsibilities; its dependency map is the critical first deliverable.
- `adl-v2/` is not classified as cruft by this plan. The ADL product owner must
  make a separate product-generation decision before ADL extraction.
- The Observatory is initially Runtime-owned; RD-08 confirms or revises that
  ownership only if independent release evidence justifies a change.
- RD-01 must classify shared GitHub workflows as versioned reuse or intentional
  duplication with one owner per copy.
- Historical evidence is large but should be handled through archival policy, not silently deleted during code extraction.
- Repository names, visibility, package registries and release versions require operator approval before execution.
- The enterprise package needs an explicit public/private API and evidence
  boundary before implementation moves; repository privacy alone is not an
  architecture or data-isolation control.
- Repository separation is expected to improve source, CI, release, credential
  and ownership isolation; production failure isolation remains unproven until
  deployment evidence establishes it.
- Independent infrastructure state lifecycle, drift detection, deployment
  cadence and rollback are evaluation criteria, not assumed benefits.
- Future team, vendor, partnership and organizational restructuring flexibility
  must be weighed against cross-repository coordination cost.

## Review and validation ledger

Decision readiness requires commit-bound Claude, Gemini and OpenAI review
records, a disposition for every actionable finding, and validation that every
referenced tracked path exists. The ledger must also record scans for
machine-local paths, credential material, unsupported completion claims and
public/private dependency inversion. Reviewer approval is planning evidence,
not authorization to execute extraction.

The source candidate reported fourteen OpenAI review dispositions but did not
include the referenced review artifacts in its source directory. Those claims
are historical candidate context, not verified review approval for this version.
The preserved candidate remains available to the operator; this version requires
fresh Claude and Gemini review plus the repository's independent pre-PR review.

New reviews and finding dispositions will be recorded beside this plan, bound
to the candidate hash and source revision. Until those reviews finish, this plan
is not decision-ready.

## Decision requested

Approve, revise or reject a provisional boundary model comprising four public
product candidates plus one private enterprise policy/evidence boundary, and
authorize **RD-01 only, scoped exactly as written**. This decision does not
approve repository names, repository creation, visibility settings, registries,
licensing, extraction, v0.93 implementation, compatibility retirement, or
deletion. Every later RD step requires a separate operator decision informed by
RD-01 evidence.
