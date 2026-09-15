# RD-01 executed audit — #977

Baseline: `f69019c24a9b61511e912c93f95442f96fa66d92`, inspected 2026-09-15.
Outcome: audit executed with explicit gaps; **extraction readiness is not proven**.
The user authorized the audit and one-public/five-private strategy, not extraction.

## Findings first

1. **P1 extraction gate — public ADL currently requires proposed private Runtime
   source.** `adl/Cargo.toml:78–81` has unconditional sibling path dependencies
   on provider-core, UTS, Runtime and kernel. Isolated ADL build and package
   attempts fail on missing provider-core; Runtime fails on missing resilience.
   Decide ADL's portable execution boundary before publishing a standalone
   public repository. Simply relocating folders cannot meet the public build gate.
2. **P2 — C-SDLC crate independence is not distribution independence.** A crate-
   only build fails at `csdlc-v3/src/commands/contract.rs:12` because it embeds
   `docs/csdlc-v3/v3-command-manifest.json` outside its package. Adding owned
   docs/templates permits a build, but library validation is 122 passed/2 failed:
   authority tests depend on original Git history/authority context. Packaging
   with `--no-verify` succeeds but does not prove the package builds. An offline
   publish dry-run cannot resolve required registry access. RD-04 must package
   owned resources and establish new repository authority without weakening guards.
3. **P2 — CodeFriend is still part of the generic ADL CLI.**
   `adl/src/cli/codefriend_cmd.rs:3` imports `adl::codefriend`;
   `.github/workflows/ci.yaml:473` builds the ADL binary for its CI. Metadata-only
   inspection of the existing private repository found 30 files, predominantly
   site/deployment assets and documentation, and no Rust or Node package manifest.
   The repository can be reused, but it is not a delivered Beta 1 product checkout.
4. **P2 — installation, CI and release boundaries still cross products.**
   C-SDLC standalone CI invokes `adl/tools/run_cargo_validation.sh`
   (`.github/workflows/ci.yaml:218–224`). `install_owner_binaries.sh:4–6,65–75`
   selects monorepo source and bundles products. `release_ceremony.sh:15–20`
   invokes checkout-local C-SDLC. Infrastructure builder publication reaches
   retained ADL evidence and another product's CodeBuild role
   (`publish_adl_builder_image_codebuild.sh:11,75`). These need versioned tooling,
   release and artifact contracts, not copied undocumented paths.
5. **P2 — rollback and functional adapters need explicit owners.**
   `csdlc-v2` still consumes `adl-resilience`. `adl-v2` contains a Runtime adapter
   with a kernel dependency. `memory_palace.rs` uses kernel types; shared memory
   cannot be assigned wholesale by filename. Retirement and behavior-preserving
   replacement are not implied by private repository placement.

These are observed baseline separation gaps, not regressions introduced by the
new audit tool. Severity describes the intended extraction, not today's monorepo
release. The private CodeFriend inspection retains no source, paths, credentials,
customer data or deployment identifiers in this public packet.

## Complete source denominator and limits

The [inventory summary](../../evidence/rd01-977/summary.json) accounts for every
one of **31,813 tracked paths** at the pinned baseline, including tree modes,
blob identities and sizes. Each has one candidate owner or an explicit unresolved
owner decision. **9,466** are unresolved under conservative classification and
have RD-02 accountability before extraction. There are no omitted inventory paths;
coverage is different from settled ownership.

The map contains **27 parsed Cargo manifests / 310 dependency declarations**.
A further **184 Node/Python/Terraform manifest or module-source paths** are
recognized with unparsed dependencies explicitly unresolved. The declaration
map is not a feature-resolved transitive lock graph. Symlink and submodule modes
remain visible in the denominator; the audit does not follow arbitrary links.
Lexical CI/CLI/checkout/generated/credential-reference candidates record paths
and line numbers only, never matched text or credential values.

Categories distinguish maintained, tests, generated and retained evidence by
explicit heuristics. They do not claim a reviewed production LOC denominator:
documentation can fall under maintained, inline tests remain in source files,
and generated files without recognizable names can be misclassified. Before
RD-02 accepts path moves, review uncertain rules, transitive feature resolution,
non-Cargo dependency edges and mixed documentation/implementation ownership.

The baseline is immutable; new #977 tooling, cards and this report are audit
outputs and are not silently added to its input denominator. Refresh the map
against the final pre-extraction head and explain changed rows.

## Boundary with CodeFriend #882

#882 owns the reusable, installed CodeFriend repository structure reporter,
including admitted CF-EVIDENCE input, persisted graph findings, layering,
coupling/connascence analysis, calibrated positive/negative fixtures and product
integration. It remains open at this audit snapshot. #977 does not implement or
claim any of those product capabilities. Its small static inventory tool is a
one-time audit aid with heuristic owners and lexical references, not an
architecture engine or substitute acceptance for #882. The baseline inventory
and observed split gaps can become an explicitly admitted/calibrated fixture for
#882 once its owner accepts that use; no dependency or scope change is imposed
on #882. Prefer the production reporter for later audit refreshes once available,
while retaining independent coverage and private-access checks.

## Proposed owner decisions

| Surface | Recommendation | Acceptance needed |
|---|---|---|
| adl-uts | Public ADL contract package | Retain serialization/conformance independence; isolated 20-test pass is supporting evidence, not API release acceptance |
| adl-provider-core | Runtime-owned implementation with separately distributable portable interfaces where justified | Kernel consumes registry/binding/execution; public ADL must not gain a private package requirement |
| csdlc-v3 + owned docs/templates | Private C-SDLC | Independent installation, authority bootstrapping, package and issue lifecycle proof |
| csdlc-v2 rollback | C-SDLC retained rollback distribution | Version/supply resilience until separately authorized retirement |
| CodeFriend acquisition/review/evidence/report experience | Existing private CodeFriend repository | Preserve existing contents and qualify installed Beta 1 on ADL and an external repository |
| Memory | Runtime continuity/execution; CodeFriend-specific consumers in CodeFriend | Explicit shared schemas/interfaces by actual consumers; no wholesale directory assignment |
| adl-v2 | ADL language/compiler/engine; Runtime adapter separately classified | Product-generation decision and versioned adapter contract |
| CI/install/release/deployment | Owning product or infrastructure per operation | Independent runners, scoped registry/credential access and source-free artifact consumption |
| Historical .csdlc evidence | Repository evidence/governance owner | Retention, privacy, linkage and archive policy; no silent deletion |

These are recommendations for RD-02, not approvals or completed migrations.

## Executed isolated simulations

[probe-results.json](probe-results.json) retains command arguments, elapsed times,
exit codes, test denominators, sanitized diagnostic tails and SHA-256 of local
sanitized logs. [run_probes.py](run_probes.py) reproduces the matrix from Git
archives of the baseline. It uses empty HOME, no forwarded cloud/GitHub/provider
credentials, offline Cargo, and trusted external Rust/Cargo caches. This is source
subset isolation, **not an OS network sandbox or a fully hermetic toolchain**.
Cache/toolchain configuration and binaries are environmental dependencies.

| Probe | Result | Interpretation |
|---|---|---|
| Eight Cargo metadata probes | Seven pass, adl-v2 fails | `--no-deps` metadata is not dependency resolution or build proof |
| C-SDLC crate-only build, repeat build, library tests | Fail before tests | Missing owned command manifest outside package |
| C-SDLC with owned docs/templates/worktree policy | Build passes; warm repeat passes | No ADL/Runtime source was added |
| C-SDLC owned-bundle library tests | 122 pass, 2 fail | Git-based authority context unavailable; no independent lifecycle acceptance |
| UTS isolated library tests | 20/20 pass | Actual conformance tests, no Runtime source |
| Provider-core isolated library tests | 107/107 pass | Actual component tests; not live provider qualification |
| ADL and Runtime build attempts | Fail | Sibling path dependencies |
| C-SDLC package without verification | Pass | Archive creation only; no package-build claim |
| C-SDLC offline publish dry-run | Fail | Registry request prohibited by offline mode; no publication or release claim |
| ADL, Runtime, kernel, v2 package attempts | Fail | Unresolved sibling path dependencies |

A bare source archive has no Git authority history, so the two C-SDLC authority
test failures do not establish a broken current Runtime or C-SDLC release. They
identify required inputs for the next independent-repository rehearsal. No
native shadow issue was published/merged in an extracted repository; RD-04 owns
that proof. No deployment, Terraform apply, paid inference, package upload or
repository creation was executed.

## H1–H3 dispositions

| Hypothesis | Audit disposition | Reason |
|---|---|---|
| H1: C-SDLC and Runtime are not functional components of one another | **Unresolved globally** | Active v3 has no Runtime crate dependency and the owned bundle builds, but whole-product ownership and operational edges remain unresolved; a bounded inspection cannot establish complete-denominator survival |
| H2: neither unavoidably needs the other to build/validate/release/deploy/operate | **Unresolved** | Current delivery independence fails at several observed boundaries. It is not yet proved that these dependencies are unavoidable, so this does not establish falsification of the target hypothesis |
| H3: historical references are replaceable by generic versioned contracts without weakened behavior | **Unresolved** | Candidate replacements are identified; migrated consumer, authority and release proofs have not executed |

Do not relabel these survived based on Cargo metadata, a source-only build or
absence of a direct v3 dependency. Extraction dependent on these hypotheses
remains gated until the missing proof and boundary decisions are supplied.

## Measurement, access and operational limits

The inventory supplies reproducible per-category bytes/lines. Probe timings
measure only the stated attempts: the first C-SDLC build used a fresh target and
failed; later owned-bundle builds reused dependencies. There is no successful
cold full-product comparison or before/after speedup claim. Full CI compute,
cost, release lead time, failure rate and organization-wide review size were
not measured; no historical number is substituted.

Source identifies credential references such as `AWS_CODEFRIEND_*` in
`.github/workflows/aws-codefriend-build.yaml:36–39` and
`AWS_SPOT_REMOTE_VALIDATION_*` in
`.github/workflows/aws-spot-remote-validation.yaml:124–126`. Only reference names
and owner surfaces were inspected. Organization bots, environment approvers,
actual registry ACLs, cloud permissions, signing custody and retention policies
require administrative observation by their owners before extraction. Their
absence is an explicit gap, not evidence that no such controls exist.

Decision accountability: RD-02/operator assigns product owners and path/contract
placements before each extraction; infrastructure/security owners establish
registry/access/signing/approval matrices before release rehearsal. RD-01
refresh must report manifest and rule changes before those decisions are accepted.

## Scheduling recommendation

Continue baseline mapping and boundary decisions alongside remaining v0.92.2
work. Do not move the active Beta 1 implementation while #914/#915 integration
and qualification run. Use #922 next-milestone planning for a dedicated migration
window after accepted #925 close, before major v0.93 feature expansion.

C-SDLC is a plausible first extraction because the owned bundle builds without
Runtime source, but its authority, packaging and complete-milestone proof gates
still apply. Resolve public ADL/private Runtime contracts before moving Runtime;
move CodeFriend into its existing private repository as its actual producer
contracts become available, and requalify Beta 1. Infrastructure follows published
Runtime artifacts. No fixed duration is justified by the partial timing data.

## Reproduction and review

From the bound audit checkout, run the inventory tool's `--self-test`, regenerate
at the baseline with `--baseline` and verify exact coverage. See the tool's
`--help` for the output-root option. Run `python3 docs/milestones/v0.92.2/repository-decomposition/rd01-977/run_probes.py` from the
repository root only when bounded local compilation is intended. The original retained logs stay in `.adl/runs/977`; replay defaults to the fresh
`.adl/runs/977-replay` directory and refuses existing nonempty output. The
portable probe summary records the original sanitized-log digests.

Specialist lanes independently inspected product/CodeFriend and CI/release
boundaries. Final changed-product review and audit validator outcomes are recorded
in REVIEW.md. This audit is not repository extraction, new milestone scope,
release approval or evidence that every independence gate has passed.
