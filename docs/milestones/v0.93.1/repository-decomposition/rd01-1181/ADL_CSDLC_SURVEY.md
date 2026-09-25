# RD-01 / #1181 contribution: ADL language and C-SDLC ownership audit

Status: read-only planning contribution for integration by Planning #4.5. Candidate ownership is not an RD-02 decision and does not authorize extraction.

## Exact identities and denominator

- #977 audited source baseline: `f69019c24a9b61511e912c93f95442f96fa66d92`.
- Frozen final v0.92.2 predecessor: `248f00e359e412f6bc0061ac88be9ff374e3a212` (release-tail merge at the recorded v0.92.2 closure).
- v0.93.1 opening observation: `08236f86026f2e317e7f405bb0381b782a1417b9`; it is context, not an input to the frozen census.
- Repository denominator changed from 31,813 to 36,899 tracked paths.
- This contribution covers 28,797 frozen paths: ADL language surfaces, both C-SDLC generations, lifecycle evidence, the mixed `adl` monolith, directly coupled templates/workflows, and root governance files.
- `ownership-inventory.jsonl.gz` contains every in-scope frozen path exactly once with Git object, mode, candidate owner or explicit RD-02 blocking decision, and object-level change from #977. SHA-256: `3a2e8e478dc8cd6dd9310de604f27f813be70ad73de03bee1fe63b748fd64a23`.
- The remaining 8,102 repository paths are outside this contribution and must remain in the integrated RD-01 denominator. This packet is not the full-repository census.

## Findings first

### P1 — Public ADL remains blocked by the mixed `adl` crate

The original #977 P1 remains true at the frozen revision. `adl/Cargo.toml` unconditionally depends on sibling `adl-provider-core`, `adl-runtime`, and `adl-runtime-kernel`, and its dev dependency also reaches the kernel. The same package declares `adl-runtime`, `codefriend-server`, and `codefriend-agent` binaries alongside the generic `adl` executable. `adl/src/lib.rs` exports language, Runtime/CSM, provider, resident, memory, and CodeFriend modules from one crate.

The monolith also expanded after #977: this lane observes 210 added and 47 modified `adl/**` paths. The largest new ownership change is CodeFriend implementation and product proof. Moving the directory wholesale to the public repository would publish private product implementation and preserve private source requirements. RD-02 must select the portable ADL subset and contract seams before RD-03; RD-05 and RD-10 then own the Runtime and CodeFriend source respectively.

### P1 — C-SDLC product ownership now includes a much larger authority and recovery surface

`csdlc-v3/**` grew from 86 to 732 paths: 646 added, 49 modified, 37 unchanged. The additions are substantive lifecycle authority, not incidental tests: typed local/remote transaction owners, publication/merge/finish/cleanup handling, semantic journals, transition/conversion owners, copied-record recovery, and large authentic-history fixtures. The active product candidate is therefore the complete `csdlc-v3/**` tree plus its coupled contract/manual/template/policy and install paths, not only the Rust crate.

The minimum owned product surface includes:

- `csdlc-v3/**`
- `docs/csdlc-v3/**`, including `v3-command-manifest.json`, schemas, current authority, man-page sources and inventories
- `docs/templates/prompts/**` and the active registry
- `.adl/worktree-policy.json`
- the C-SDLC branches of `adl/tools/install_owner_binaries.sh`, `install_csdlc_man_pages.sh`, `run_owner_validation_lane.sh`, and `run_cargo_validation.sh`
- the C-SDLC jobs and path policy currently embedded in `.github/workflows/ci.yaml`

The command contract is still compiled from `docs/csdlc-v3/v3-command-manifest.json`, while runtime local preparation reads `docs/templates/prompts/current.json` and `.adl/worktree-policy.json`. Omitting any of those can yield a crate that compiles incompletely or an installed tool that cannot perform an authentic lifecycle.

RD-04 acceptance must therefore prove an independently installed complete shadow issue lifecycle and copied-record recovery, including ambiguous publication, merge, finish, and cleanup. Crate tests or source copying alone do not establish that result.

### P1 — Lifecycle evidence is the largest surface and needs a preservation policy, not product duplication

`.csdlc/**` contains 25,553 frozen paths, up 3,696 from #977. It is 89% of this contribution. These paths are historical lifecycle evidence and records, not C-SDLC executable source. Assigning them wholesale to the new C-SDLC repository would duplicate or relocate repository-specific issue history, may expose private payloads, and may break provenance and Git linkage.

The candidate owner is repository evidence/governance, with RD-02 required to define retention, privacy, immutable linkage, export/redaction, and archive rules. Product extraction should carry only bounded fixtures and public receipts that RD-04 explicitly admits. The original monorepo snapshot remains rollback evidence; it is not a second maintained writer.

### P2 — Retained C-SDLC v2 is still coupled to ADL resilience

All 111 `csdlc-v2/**` paths are unchanged from #977 and remain a C-SDLC rollback candidate, but `csdlc-v2/Cargo.toml` has a direct sibling path dependency on `adl-resilience`. RD-02 must choose one of two explicit outcomes before RD-04: package the required resilience contract with the retained rollback distribution, or accept a separately proven retirement. Repository movement does not retire v2.

### P2 — `adl-v2` is mostly a portable public candidate, with one explicit private boundary

Ninety of the 95 `adl-v2/**` paths form self-contained language, compiler, engine, records, adapter, CLI, and deterministic workcell crates and are public ADL candidates. The five `adl-runtime-v3-adapter/**` paths directly depend on `../../../adl-runtime-kernel`; they require an RD-02 owner/contract decision and cannot silently enter public ADL.

The generation is still operationally meaningful. Its directory name does not authorize retirement. RD-02 must confirm which generation is public and how the Runtime adapter is versioned or moved.

### P2 — CI, installer, toolchain, and root governance cannot be copied wholesale

The frozen `.github/workflows/ci.yaml` runs C-SDLC v2/v3, ADL, Runtime, provider, UTS and CodeFriend jobs through checkout-local `adl/tools/**`. The shared installer selects either the `adl` monolith or `csdlc-v3`, builds through `adl/tools/run_cargo_validation.sh`, installs manuals from another helper, and writes repo-local provenance. Root `AGENTS.md`, README, changelog, security/contribution policy, license, toolchain and Git attributes also describe the combined repository.

These 1,669 paths have explicit RD-02 blocking decisions in the inventory. Each destination needs scoped workflows, install/release paths, policy and version surfaces. Shared helpers must become versioned reusable contracts or be copied under one owner and consumed without sibling source.

## Candidate ownership map

| Frozen surface | Paths | Candidate owner | Required action |
|---|---:|---|---|
| `csdlc-v3/**` | 732 | `cognitive-sdlc` | RD-04 exports implementation, tests and authentic recovery fixtures |
| `docs/csdlc-v3/**` | 84 | `cognitive-sdlc` | Carry schemas, command contract, authority docs and manuals |
| `docs/templates/prompts/**` | 72 | `cognitive-sdlc` | Carry six-card templates, schemas, PVF policy and active registry |
| other planning/sprint templates | 50 | `cognitive-sdlc` candidate | RD-02 confirms whether any are generic public contracts |
| `.adl/worktree-policy.json` | 1 | `cognitive-sdlc` | Preserve typed bind policy with destination-specific root configuration |
| `csdlc-v2/**` | 111 | `cognitive-sdlc` rollback | Resolve `adl-resilience` distribution or retirement |
| `.csdlc/**` | 25,553 | repository evidence/governance | Retain provenance; export only admitted bounded fixtures/receipts |
| `adl-spec/**` | 39 | `agent-design-language` | RD-03 public portable specification |
| `adl-uts/**` | 11 | `agent-design-language` | RD-03 public schema/conformance package |
| `docs/templates/portable-adl/**` | 6 | `agent-design-language` | RD-03 public project contract/templates |
| portable `adl-v2/**` | 90 | `agent-design-language` candidate | RD-02 confirms generation scope; RD-03 exports |
| `adl-v2/.../adl-runtime-v3-adapter/**` | 5 | unresolved | RD-02 chooses Runtime ownership or versioned public interface |
| `adl/**` | 2,014 | mixed | Partition; do not assign wholesale |
| `.github/workflows/**`, root policy/toolchain files | 28 | unresolved | RD-02 splits by destination and assigns history owner |

Within the mixed monolith inventory, 151 paths are recognizable CodeFriend candidates and 227 are recognizable Runtime candidates; 1,636 remain intentionally blocked on RD-02 because filename-level assignment would be unsafe. These are conservative planning classifications, not permission to move source.

## Changes from #977

| Surface | #977 | Frozen | Object-level delta |
|---|---:|---:|---|
| whole repository | 31,813 | 36,899 | +5,086 paths |
| this contribution | 24,239 | 28,797 | +4,558; 117 modified; no deleted in-scope paths |
| `.csdlc/**` | 21,857 | 25,553 | +3,696 evidence/record paths |
| `csdlc-v3/**` | 86 | 732 | +646; 49 modified |
| `adl/**` | 1,804 | 2,014 | +210; 47 modified |
| `docs/csdlc-v3/**` | 80 | 84 | +4; 18 modified |
| `.github/workflows/**` | 17 | 19 | +2; one modified |
| `csdlc-v2`, `adl-v2`, `adl-spec`, `adl-uts`, `docs/templates` | unchanged counts | unchanged counts | ownership blockers remain materially relevant |

The #977 conclusions did not become stale merely because direct C-SDLC implementation improved. The frozen tree adds a far larger C-SDLC authority/recovery denominator and more product code inside the monolithic ADL crate. It also strengthens the case that C-SDLC can be extracted first, because active v3 still has no Rust dependency on Runtime, while making independent distribution and recovery proof more demanding.

## Easily missed paths and coupling

1. `docs/csdlc-v3/v3-command-manifest.json` is compiled into the C-SDLC command contract.
2. `docs/templates/prompts/current.json` selects the active card/template generation at runtime.
3. `.adl/worktree-policy.json` is read by bind, local intent, and terminal handling.
4. `docs/csdlc-v3/man/**` and `adl/tools/install_csdlc_man_pages.sh` are part of the installed operator product.
5. `adl/tools/install_owner_binaries.sh` owns C-SDLC binary provenance despite living under `adl/`.
6. `.github/workflows/ci.yaml` uses `adl/tools/run_cargo_validation.sh` for C-SDLC v2/v3.
7. `csdlc-v3/tests/fixtures/issue872-*` contains authentic copied records and current observations; treat them as admitted test fixtures with privacy review, not generic historical export.
8. `csdlc-v3/src/bin/csdlc_transition.rs` and conversion/recovery sources are separate command surfaces within the same product.
9. `adl-v2/crates/adl-runtime-v3-adapter/**` is the sole direct private Runtime dependency inside the otherwise portable v2 workspace.
10. `docs/templates/portable-adl/**` is a public ADL exception inside a mostly C-SDLC-owned template root.
11. Root policy/license/toolchain files need destination-specific decisions even when byte-identical copies are legally or operationally appropriate.
12. The tracked `.adl/docs/TBD/...Runtime...` plan belongs to Runtime, while the adjacent `.adl/worktree-policy.json` belongs with C-SDLC.

## Required RD-02 decisions

1. Define the public ADL generation and exact monolith carve-out; no unconditional private Runtime dependencies may remain.
2. Assign `adl-runtime-v3-adapter` and choose its published interface/version contract.
3. Decide retained C-SDLC v2 plus `adl-resilience` distribution or separately proven retirement.
4. Accept the precise template split, especially generic planning templates versus portable ADL templates.
5. Assign each shared CI/install/release helper and replace checkout-relative sibling access with versioned artifacts or destination-owned code.
6. Define `.csdlc` retention, privacy, export, redaction, issue-link and archive policy.
7. Decide root governance/history ownership and the destination-specific policy copies.
8. Admit the exact authentic recovery fixtures that RD-04 may carry without treating all historical records as product assets.

## Evidence and limitations

The inventory was derived with `git ls-tree -r -l --full-tree` for both exact revisions. Object identity, mode and size are retained. Coverage checks reject duplicates, omissions, and unresolved rows without an RD-02 owner. Change classification compares Git object and mode, so a same-path byte change is not reported as unchanged.

This was a read-only Git-object and source-contract audit. It did not build, install, extract, publish, mutate lifecycle state, create repositories, inspect credentials, run providers, or claim independent product qualification. The existing #977 probes remain historical evidence at their stated baseline; they were not replayed because RD-01 is a planning-contract refresh and downstream tasks own installed-consumer proof.

Integration note: downstream RD numbering is normalized to the accepted current graph. Contributor inventory/hash is retained in the coordination packet; the complete deliverable inventory is census.jsonl.gz beside this report. Its conservative classifications control this audit; narrower contributor recommendations remain RD-02 inputs.
