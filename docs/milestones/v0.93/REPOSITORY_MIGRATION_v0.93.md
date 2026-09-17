# v0.93 Opening Repository Migration

## Status

First-pass planning under #1047. v0.93 is not open; no extraction or feature implementation is authorized by this package.

The operator selected the split as the opening phase of v0.93, after accepted v0.92.2 closure and before feature development. The older post-v0.95 split and the previous recommendation for an unversioned inter-milestone migration window are superseded. Preparation happens now; execution waits for opening authorization.

## Product owners and visibility

| Repository | Visibility | Owned result |
|---|---|---|
| agent-design-language | Public | Portable language/compiler/SDK contracts and public conformance |
| cognitive-sdlc | Private | Lifecycle engine, cards/templates, manuals, installers and its own tests |
| agent-logic-runtime | Private | Runtime/CSM, provider lifecycle, continuity, shared Runtime memory and Observatory |
| codefriend | Private | CodeFriend software, product memory consumers, acquisition/review/report experience |
| codefriend.ai | Private | Existing website, site content and website deployment |
| agent-logic-infrastructure | Private | Shared deployment modules, operational inventories and pinned-artifact delivery |
| agent-logic-enterprise-security | Private | Proprietary policies, backends, integrations and restricted evidence |

`codefriend` is a proposed software-repository name. Existing `codefriend.ai` content and history are preserved; it is not the software destination. Except for existing repositories, names must be confirmed before creation. Named owners, licenses and registry/access policy remain decisions in RD-02.

## Sequence and completion

The exact candidate graph is [EXECUTION_PLAN_v0.93.json](EXECUTION_PLAN_v0.93.json). RD-01 refreshes #977 at the final frozen revision; RD-02 settles decisions; RD-03 publishes portable public contracts; RD-04 extracts C-SDLC; RD-05 extracts Runtime; RD-06, RD-09 and RD-10 follow their producer contracts. RD-08 qualifies the demo catalog, RD-07 verifies public ADL and retires only approved superseded source, and RD-11 accepts the complete product lockset. RD IDs preserve historical outcome names, not numeric execution order.

C-SDLC is first because the migration needs reliable tooling. It must prove a complete independent shadow issue lifecycle and authentic copied-record recovery, including uncertain publication, merge, finish and cleanup. A crate build or green isolated tests are insufficient. Current complete-milestone v3 acceptance gates its cutover. Retained v2/resilience rollback support needs an explicit distribution or retirement decision; moving repositories does not retire it.

Product moves carry their own demos, manuals, test fixtures and install/release paths. RD-08 verifies coverage; it does not create a second owner for those files. RD-11 is the global feature-start gate. Parallel preparation is allowed, but shared contracts and source ownership changes are serialized.

## Boundary decisions

- Public ADL must lose unconditional private Runtime dependencies while retaining usable portable semantics. Decide `adl-v2` generation scope; do not classify it as obsolete by directory name.
- UTS portable schemas/conformance are public ADL candidates. Provider implementation and Runtime continuity belong with Runtime; only justified portable interfaces become public contracts.
- Split memory by its consumers and contract ownership. CodeFriend-specific consumers move to CodeFriend; shared Runtime services stay with Runtime. Never copy one implementation into two owners.
- Runtime owns baseline enforcement, default-deny authorization, audit hooks and key interfaces. Enterprise policy can narrow grants through a versioned interface. Test no provider, configured but invalid/unavailable provider, and verified provider. Synthetic Runtime tests do not qualify real enterprise policies.
- The website keeps site-specific deployment; shared infrastructure owns reusable deployment machinery. DNS/account movement is separate, including #1017.

## Cutover packet per repository

Record frozen source revision, exact included path/history manifest, destination revision, artifact digests, supported consumer versions, registry visibility, owner and scoped access, old/new issue and PR links, workflow references, acceptance denominator, rollback artifact and decision authority. Do not retain host paths, credentials or private payloads in public manifests. Public receipts may identify private products without copying their restricted evidence.

Freeze affected writers and active worktree/PR ownership before final export. Rehearse history filtering and clean-checkout builds in isolation. Preserve issue/PR cross-links and historical evidence; Git history export alone does not move tracker records. Choose one active writer at each cutover. A prior monorepo snapshot is rollback evidence, not a second maintained product.

Already-public history is already disclosed. Export review prevents new leakage; moving or deleting files does not undo prior publication. Repository privacy is not a security control proof.

## Qualification and rollback

Public ADL builds/tests/packages with no private registry credentials. Each private product installs and releases from its own checkout without sibling source. Runtime runs continuity/API/Observatory proof; CodeFriend repeats the accepted installed Beta 1 journey on ADL and an external repository. Infrastructure validates pinned artifacts without an implicit cloud apply. Website behavior is compared before/after and its deployment authority is preserved.

Stop on unresolved ownership, missing release proof, incompatible contracts, new private dependencies, ambiguous writer authority or damaged recovery evidence. Keep the accepted v0.92.2 source, binary and evidence lockset until all affected consumers pass and retirement is separately authorized. No calendar duration is asserted before the refreshed dependency and build measurements.

## Issue bootstrap order

WP-01 creates only the ADL coordination/audit/decision identities. RD-02 accepts destination names and boundaries, then creates/bootstrap those repositories and their execution issue identities. Bind immutable cross-repository dependency links before RD-03 and extraction execution. Never require an issue in a not-yet-created repository as the prerequisite to deciding its name.
