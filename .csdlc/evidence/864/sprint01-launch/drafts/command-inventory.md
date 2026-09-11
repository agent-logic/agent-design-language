# SIM-03 frozen installed command inventory

Freeze baseline: `ace209ad9c701a855d165da817164ff60a755203`. Scope authority: the complete Operator commands, target architecture and journey benchmark in `docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md`, the SIM-03 specification and atomic completion contract. This freezes required behavior before issue creation, not claims of availability or execution. Insert this inventory into the published SIM-03 issue body; a local draft link is insufficient.

## Complete ordinary intent API

| Target interface | Complete behavior and production owner | Effects and proving cases |
|---|---|---|
| `status <issue> [--json]` | Issue/authority/version/worktree/evidence inspection, blockers, eligibility, scheduling and allowed next operations; `commands/local/mod.rs` diagnostic and recommendation owners | Observation only; healthy, missing, corrupt, pending and unchanged-facts recommendation coherence |
| `prepare <issue> --plan <path>` | Fetch exact issue identity, resolve registry, create reviewed typed local contract/projections through local preparation owner | Explicit local preparation; wrong repo, malformed plan, absent authority and unsupported schema rejected |
| `bind <issue>` | Resolve reviewed plan and registered FastWork target, preview target in result, bind through local owner | Guarded Git/worktree/state effect; collisions, stale version and wrong topology rejected |
| `edit <issue> --changes <path>` | Typed semantic fields and existing evidence invalidations through local owner | Guarded local effect; invalid fields/schema, stale request and evidence identity confusion rejected |
| `validate <issue>` | Inspect existing evidence/projections through local validation owner | Observation only; no validator execution, repair or silent recovery |
| `proof <issue>` | Explicitly execute declared validators with bounded outcome/evidence, through application proof owner | Explicit subprocess/evidence effects; failed/zero-test proof is not accepted |
| `review <issue> --evidence <path>` | Verify externally produced independent review identity/scope/principals/findings/exact revision via remote review owner | Guarded evidence recording; stale/altered/wrong-principal proof rejected |
| `publish <issue>` | Derive exact target and verified review inputs; governed publication and authenticated reconciliation via remote owner | Guarded remote plus local evidence; base/head/linkage mismatch and uncertain response cannot duplicate effects |
| `finish <issue>` | Authenticated terminal observation and verified completion via terminal owner | Remote observation plus guarded receipt/state write; open/unverified or mismatched issue/PR cannot close |
| `clean <issue> [--execute]` | Exact registered-worktree cleanup preview; execute rechecks fresh preview and eligibility via terminal owner | Preview observational, execute guarded removal; stale preview, dirty/unregistered/wrong checkout and missing terminal truth denied |
| `recover <issue> [--execute]` | Describe pending recovery; explicit execution against same issue version/transaction through local transaction or remote intent owner | Preview observational, execute classified recovery only; no double remote effect or fabricated approval |

Source owner paths in this table are relative to `csdlc-v3/src/`. All routes use the SIM-02 descriptor and result envelope; no schema-only implementation qualifies.

## Current route denominator and target disposition

This table accounts for every one of the 26 entries in `docs/csdlc-v3/v3-command-manifest.json`, plus the additional source-dispatched `rollback` alias. Names below are current names; the target column freezes disposition, not permanent legacy aliases. Old request schemas receive actionable rejection/conversion diagnostics after the separately authorized breaking activation. There is no second operational engine.

| Current route | Target coverage / explicit disposition | Owner path under `csdlc-v3/src/` |
|---|---|---|
| foundation | Explicit proof/administrative foundation inspection, never operational fallback | `main.rs`, `lib.rs` |
| local | Explicit proof/administrative construction report; actual issue preparation maps to prepare | `commands/local/mod.rs` |
| bind | Ordinary bind | `commands/local/mod.rs` |
| clean | Ordinary preview/execute clean | `commands/terminal.rs` |
| cutover | Explicit administrative transition operation with unchanged operator authority; no ordinary fallback | `commands/terminal.rs` |
| doctor | Complete status diagnostics, including recovery-required | `commands/local/mod.rs` |
| edit | Ordinary typed edit | `commands/local/mod.rs` |
| eligibility | Complete status eligibility facts and reasons | `commands/local/mod.rs` |
| finish | Ordinary authenticated terminal finish, including existing delivered/no-PR disposition contracts | `commands/terminal.rs` |
| github | Explicit GitHub typed operation family and exact operation routing | `commands/remote/mod.rs` |
| github-issue | GitHub issue create/comment/edit/close operations below | `commands/remote/mod.rs`, `main.rs` |
| github-pr | GitHub PR create/update/ready/merge operations below | `commands/remote/mod.rs`, `commands/remote/merge.rs` |
| install | Explicit administrative isolated candidate install/provenance contract; live installation remains separately authorized | `commands/proof.rs` |
| issue | Ordinary prepare; no conflation with remote issue creation | `commands/local/mod.rs` |
| pr-state | Explicit GitHub PR-state observation | `commands/remote/mod.rs` |
| proof | Ordinary validator execution; retained construction proof uses explicit proof/administrative namespace | `commands/proof.rs`, `application/mod.rs` |
| publish | Ordinary governed publish and reconciliation | `commands/remote/mod.rs` |
| review | Ordinary independent review evidence verification/recording | `commands/remote/mod.rs` |
| remote | Explicit GitHub-family help/overview, no implicit effect | `main.rs`, `commands/remote/mod.rs` |
| schedule | Complete status next-operation/dependency recommendation | `commands/local/mod.rs` |
| shadow | Explicit proof namespace only; retained comparison qualification | `commands/proof.rs` |
| shepherd | Complete status lifecycle recommendation and blockers | `commands/local/mod.rs` |
| soak | Explicit proof namespace only; retained qualification behavior | `commands/proof.rs` |
| sprint | Retain read-only sprint readiness/coordination helper contract | `commands/sprint.rs` |
| validate | Ordinary observational validate; never run proof implicitly | `commands/local/mod.rs` |
| release-preflight | Retain read-only candidate-consistency helper; no release authority | `commands/release.rs` |
| rollback (additional alias) | Explicit administrative typed rollback, routed only when cutover operation is rollback; reject mismatched operation | `commands/terminal.rs`, `main.rs` |

Namespace spelling for retained administrative helpers is a descriptor-level implementation choice. Every behavior above requires an explicit descriptor, documented namespace and installed positive/negative proof; classification cannot silently remove a route. Administrative operations are exercised on isolated copies with synthetic authorization evidence, never against live issue state in this task. If a current helper is deprecated in favor of an equivalent target, prove the complete equivalent behavior and explicit old-interface diagnostic.

## Frozen GitHub operation coverage

`GithubMutation` in `commands/remote/mod.rs` defines eight required mutation variants:

1. IssueCreate: title/body and admitted labels/assignees/milestone; existing direct `github-issue create` convenience parses into the same operation.
2. IssueComment: exact existing issue target and supplied comment.
3. IssueEdit: title/body, labels add/remove/replace, milestone set/clear, explicit assignees; omitted metadata preserved and invalid/null metadata rejected.
4. IssueClose: duplicate/superseded/no-op, rationale/current-body evidence, duplicate target when required and truthful state reason; no invented delivery closure. Existing direct close convenience routes here.
5. PullRequestCreate: canonical base/head, title/body and draft flag.
6. PullRequestUpdate: exact PR title/body edits.
7. PullRequestReady: draft-to-ready with verified current review and remote pre-state.
8. PullRequestMerge: reviewed base/head, supported merge method and verified receipt; preserve publication linkage. This remains guarded by the native merge owner and separate operation authorization.

PR-state readback, publication orchestration, independent-review recording, uncertain-operation reconciliation and terminal readback are also required despite not being mutation enum variants. Requests must preserve identity/topology and authenticated evidence. Tests use deterministic fake transport and retain remote intent/result evidence. Real issue creation during sprint launch does not authorize live mutation tests during SIM-03.

## Installed source discrepancy retained at launch

Read-only invocation of the primary installed `csdlc --help` omitted `release-preflight` on launch inspection, while the exact source root usage and 26-entry manifest include it. Source also dispatches `rollback` outside that manifest/root-help listing. These are SIM-02 reconciliation inputs; no active executable was rebuilt or replaced. Installed binary age/cause was not established by help output. Re-resolve exact installed provenance at implementation; retain original observations rather than editing historical evidence.

## Completion denominator and guard matrix

Coverage reports enumerate all 11 ordinary interfaces, all 26 current-route dispositions plus rollback, and all eight typed mutation variants. A many-to-one mapping such as doctor/eligibility/schedule/shepherd to status is one complete CLI capability, not four implementation tasks. No new issue is created per route.

Every route/disposition must have an installed positive case and an applicable negative or denied-effect case. Global negatives cover stale authority/version/serialized request/review, wrong repository, ambiguous checkout, tampered evidence, missing terminal proof, incorrect PR base/head/linkage, unsafe cleanup, stale preview, unknown command, unsupported platform mutation and operational-to-construction fallback. Required measurements are ordinary manually supplied fields, reconstructed request files, invocation counts, authority/Git reads and prepared-start elapsed time with disclosed environment/warmth. Failing or missing route coverage blocks SIM-03 closure.

PVF: deterministic local CPU/Rust/Git installed-command proof using isolated repositories, fake authenticated remote effects and injected clocks. Required SIM-03 acceptance and SIM-07 qualification input. Separate integration CI and any later authorized live-pilot evidence from this local proof. No activation, conversion, actual publication or release is established by this inventory.
