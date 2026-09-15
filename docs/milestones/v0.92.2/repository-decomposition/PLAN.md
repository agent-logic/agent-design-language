# ADL repository decomposition plan

Status: revised working plan, 2026-09-15. Repository ownership and visibility
reflect the operator decisions below. Scheduling is a recommendation, not an
approved milestone amendment or extraction authorization. No repositories,
visibility settings, releases, or infrastructure are changed by this document.

## Decision and source authority

This revision follows the current decomposition strategy reviewed under #848
and merged in PR #943. The tracked baseline is
`docs/milestones/v0.92.2/repository-decomposition/REVIEWED_PLAN_848.md` in the main ADL
repository, with its inventory, findings and review records in the same directory.
Those reviews apply to that historical version, not to this revised plan.

The operator decided on 2026-09-15:

- Use the current product-boundary split; abandon the older two-repository plan.
- Reuse the existing CodeFriend repository and keep it private.
- Only the ADL language repository will be public; every other product repository
  will be private.

Accordingly, `docs/planning/POST_V095_ADL_CSM_LOGISTIC_SPLIT_PLAN.md` and its
`docs/planning/ADL_LOGISTIC_SPLIT.md` predecessor are superseded strategy inputs.
Their historical text is not a requirement to wait until v0.95 or maintain a
stable/fast two-repository synchronization scheme. The earlier four-public plus
one-private visibility proposal is superseded too. Historical reviews and
inventories remain evidence of what was reviewed, not current execution proof.

## Product boundaries and visibility

| Repository | Visibility | Accountable owner role | Owned result |
|---|---|---|---|
| agent-design-language | Public | ADL language owner | Language syntax, parser/compiler, portable execution semantics, language SDK/CLI, public contracts and conformance fixtures |
| agent-logic-runtime | Private | Runtime owner | CSM, Runtime host/kernel, Runtime-coupled resilience, provider lifecycle, continuity, transports, Runtime qualification and Observatory |
| cognitive-sdlc | Private | C-SDLC owner | Active lifecycle engine, templates, schemas, issue/review/publication/finish/cleanup tooling, manuals and independent validation |
| agent-logic-infrastructure | Private | Infrastructure owner | Deployment modules, operational tooling, inventories and pinned-artifact deployment contracts |
| agent-logic-enterprise-security | Private | Enterprise security owner | Enterprise policies, backends, integrations and restricted qualification evidence for the six v0.93 security work packages |
| agent-logic/codefriend.ai (existing) | Private | CodeFriend owner | CodeFriend shell/CLI, acquisition, architecture analysis, review orchestration, memory consumers, evidence, action plans and report/publication experience |

The existing `agent-logic/codefriend.ai` repository was verified private on
2026-09-15. Its existing contents must be inventoried and preserved before
CodeFriend implementation is integrated; no replacement repository is proposed.
The other destination names remain proposed identifiers until creation setup.
Named people, registries, licensing, release permissions and support windows
must be recorded before the respective extraction.

Each source path and artifact has one accountable owner. Consumers of a shared
contract are not co-owners of its source. ADL provider interfaces remain with the
language only where they are portable language semantics. RD-01 must explicitly
assign `adl-provider-core`, `adl-uts`, review/evidence components, shared memory,
`adl-v2`, mixed `adl/` code, workflows, fixtures and historical `.csdlc` evidence.
CodeFriend-specific code moves to CodeFriend; shared services or contracts require
an evidenced owner and versioned consumption rather than duplicate copies.

## Independence and distribution contracts

C-SDLC and Runtime are independent products. RD-01 must attempt to falsify:
H1, neither is a functional component of the other; H2, neither requires the
other to build, validate, release, deploy or operate; H3, historical cross-product
references can be removed or replaced by generic versioned contracts without
weakening behavior. Each hypothesis is survived, falsified or unresolved against
a complete denominator. Missing proof is unresolved and blocks dependent moves.
C-SDLC being used to develop another repository creates no product dependency.

The public ADL language must build, test, package and release with no private
repository, credential, registry or workflow access. Its public documentation and
conformance fixtures must make it independently usable. Private consumers may
use published ADL artifacts. Private-to-private contracts can use private
registries with explicit scoped access. A cross-repository contract is not
necessarily a public artifact: publication visibility follows its owning product.

Runtime consumes versioned ADL interfaces where needed; infrastructure consumes
pinned Runtime artifacts and deployment interfaces. CodeFriend's actual ADL,
Runtime and memory dependencies must be measured, not invented by this plan.
No extracted product may depend on a sibling checkout or source-tree path.

Each cross-repository surface records owner, visibility, artifact type, version,
registry, consumer compatibility range, support/deprecation window, conformance
fixtures, signing/provenance, retention and rollback. Breaking changes require
contract-owner and supported first-party consumer review. CI must use versioned
workflows or independently owned copies, never another repository's checkout.

Private Runtime retains its enforcement points, default-deny policy, authentication,
per-message authorization, audit emission, isolation and key/secrets interfaces.
Enterprise security may narrow authority through a versioned Runtime interface;
it cannot bypass those invariants. No enterprise provider means a usable baseline;
a configured but unavailable/unverified/incompatible provider fails closed;
a verified provider uses the declared contract. Runtime CI proves all three
mechanics with synthetic providers; enterprise CI qualifies actual private policy.
Only portable language-level security semantics belong in public ADL. Repository
privacy is not a substitute for runtime security or a certification claim.

## Execution work and completion gates

The #977 [baseline audit](rd01-977/AUDIT.md) is executed with explicit gaps;
its complete path inventory does not settle the unresolved ownership or
independence decisions. The #848 inventory remains historical planning input. Every step below must produce its stated result and
proof. An execution issue must not close on a scaffold, schema or unexecuted packet.

| Step | Complete result and required proof |
|---|---|
| RD-01 dependency and ownership audit | Revision-bound map of every tracked path, manifest, dependency, generated artifact, test, workflow, release input, demo, evidence class, credential reference and operational entrypoint; no unreported orphans. Classify edges and resolve owner gaps. Run isolated clean-checkout build/test/package/release simulations without unrelated checkouts and, for public ADL, without private access. Record H1-H3 dispositions. |
| RD-02 boundary and execution decision | Decide exact path ownership, shared-package placement, names, licensing, visibility, contract distribution, demo placement, extraction order and rollback owners using RD-01 evidence. Evaluate a staged stopping point such as C-SDLC first, while preserving the operator's one-public/all-other-private decision. |
| RD-03 versioned shared contracts | Publish the necessary owned contracts, fixtures and artifacts to registries of the correct visibility. Actual consumers build against pinned published versions with no relative-path dependencies. C-SDLC acquires no ADL dependency unless explicitly justified and accepted. |
| RD-04 C-SDLC extraction | Independent private repository builds, validates, publishes, merges and finishes a shadow issue using generic repository interfaces. Templates, installers, manuals and authority/reconciliation behavior work without ADL or Runtime source. Retain rollback support until separately retired. |
| RD-05 Runtime extraction | Private Runtime/CSM/Observatory repository builds and runs a local polis, passes continuity and API/Observatory proof, and publishes versioned deployment artifacts. Consumers work without monorepo access; pre-extraction release rollback remains available. |
| RD-06 infrastructure extraction | Private infrastructure builds and validates plans against pinned artifacts and interfaces with no application-source duplication. State movement and cloud applies are separate operations, not side effects of extraction. |
| RD-07 remaining ADL cleanup decision | Each proposed deletion has an owner, replacement, proof, retained rollback source and retirement authority. Public ADL conformance and independent builds pass. Deletion is a separately approved operation. |
| RD-08 demo ownership application | Each demo is placed with exactly one previously decided product owner and runs from that product's supported artifacts; no copied production code. Observatory moves with Runtime. Archival requires its own disposition. |
| RD-09 enterprise-security repository | Private implementation boundary for the six v0.93 work packages, with explicit contract/implementation/fixture/evidence ownership and compatible Runtime interfaces. Real enterprise and synthetic Runtime integration pass independently. Repository setup does not imply completion of enterprise implementation. |
| RD-10 CodeFriend integration into existing repository | Preserve existing repository content/history, move owned implementation, replace monorepo imports with versioned contracts, and independently build/install/run the complete Beta 1 journey on ADL and an external repository. Verify evidence, reports, approval behavior and rollback; repository presence alone is not delivery. |

RD identifiers name results, not mandatory serial numbering. RD-01 and RD-02
precede extraction; RD-03 precedes each consumer that needs those contracts;
RD-05 precedes RD-06; RD-02 demo decisions precede RD-08. RD-10 waits for the
specific producer contracts identified by RD-01, not all unrelated extractions.
C-SDLC complete-milestone v3 proof gates RD-04, not the audit or unrelated moves.
Retirement of v2 needs its own eligibility, review and authorization; extraction
must not strand the known `csdlc-v2 -> adl-resilience` rollback dependency.

## Recommended scheduling

Recommendation as of 2026-09-15; no calendar date or new milestone is committed.

1. **During the remaining v0.92.2 work:** execute RD-01 alongside product work
   using a pinned baseline. Measure actual dependencies, owners and independent
   build feasibility; prepare RD-02 decisions. Do not move active source or
   impose new dependency gates on existing sprint issues. Refresh the audit for
   every affected merged change before extraction.
2. **At v0.92.2 release readiness:** complete RD-02 and prepare the migration
   issue wave. Preserve #914's installed integration and #915's independent
   Beta 1 qualification on the existing topology. Budget extraction explicitly
   in next-milestone planning (#922), rather than quietly adding it to this
   milestone's release tail.
3. **Immediately after accepted v0.92.2 close (#925):** use a dedicated migration
   window before substantial v0.93 feature expansion. Start RD-04 once complete-
   milestone C-SDLC proof is accepted. Publish needed RD-03 contracts and then
   execute Runtime and CodeFriend moves in dependency order. They may run in
   parallel only where source, installers, contracts and CI ownership are disjoint.
4. **After Runtime artifact publication:** extract infrastructure. Apply demo
   ownership with its respective product move. Establish enterprise-security
   interfaces/repository before its private implementation grows in v0.93;
   complete the six implementation packages in their own planned scope.
5. **Before declaring migration complete:** qualify the extracted products and
   the public ADL checkout independently, including CodeFriend's complete
   installed Beta 1 journey. Then decide remaining cleanup and rollback retirement.

#914, #915 and #925 were open when checked on 2026-09-15. This argues for
preparing now and moving after their accepted completion, not assigning an
unsupported date. No fixed duration is estimated until RD-01 measures coupling.
A decision to move sooner must explicitly rebaseline in-flight consumers and
repeat affected integration/qualification against the new topology.

## Migration safety and measurement

Extract with history where practical. Inventory pending branches, issues,
worktrees, release jobs, secrets references and deployment consumers; arrange a
bounded handoff for each moved path. Leave one active writing owner and an
explicit source-of-truth cutover per product. Do not turn retained rollback
copies into two actively maintained implementations. Do not combine extraction
with feature refactors or silently delete lifecycle evidence.

RD-01 records reproducible cold/warm build time, focused/full validation time,
CI compute, review size, release lead time, checkout/storage cost and failure
rate. Classify maintained production, generated, test, fixture, compatibility
and evidence code separately. Compare identical workloads after extraction;
moving lines does not itself reduce maintained code or prove runtime isolation.

Before public ADL publication, review tracked content and the proposed exported
history for private implementation, customer data, credentials and restricted
proof. Already-public history cannot be made undisclosed by moving files. Keep
sanitized public conformance usable without private proof. Do not change current
repository visibility as an incidental migration step.

Success means independently releasable products, one owner per artifact,
versioned dependencies, usable public ADL without private access, preserved
CodeFriend Beta 1 behavior, scoped private access and measured build/review
improvement. Stop for unresolved ownership, incompatible contracts, required
proof not run, authority collisions or unsafe disclosure. Preserve rollback
until the corresponding product's consumer qualification passes.

## Review status and next decision

The earlier #848 reviews remain historical. A fresh bounded independent
subagent review of this revised draft on 2026-09-15 found no actionable findings
in ownership, visibility, independence, completion gates or scheduling. Structural
checks passed for six owners, one public/five private repositories, all ten RD
outcomes, baseline source references, whitespace and machine-path hygiene.
This is draft review, not commit-bound publication or extraction approval.
The #977 baseline audit has now executed inventory and isolated probes with
unresolved findings. No extraction has executed. The next decision is to resolve
the audit's ownership/contract gaps for RD-02 and assign missing administrative
and independent-release proof. Refresh the audit at the actual migration head.
The recommended migration window remains after v0.92.2 acceptance and before
major v0.93 expansion.
