# v0.92.2 Atomic Task Contracts

Status: operator-requested planning correction under #864 with the separately authorized first SIM sprint created as #866–#875; no product completion is claimed.

## One complete task

An implementation issue closes only when its one named behavior works through its production consumer, with supporting tests, failure handling and documentation. A schema, scaffold, unused library, placeholder interface, authored report or plan alone does not satisfy implementation. Qualification tasks execute the actual scenario; document tasks deliver a finished named document. Final integration connects complete features and cannot absorb unfinished upstream functionality.

## Eight task-family splits

| Original task | Complete replacement tasks |
|---|---|
| CF-ADAPTER | CF-ADAPTER, CF-ADAPTER-GITHUB, CF-ADAPTER-CI |
| CF-COG | CF-COG, CF-COG-DRIFT, CF-COG-IMPACT, CF-COG-RATIONALE |
| CF-GOV | CF-GOV, CF-GOV-CI |
| CF-REVIEW | CF-REVIEW, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN |
| CF-UX | CF-UX, CF-RENDER-MD, CF-RENDER-HTML, CF-RENDER-PDF |
| PLAT-PROVIDER | PLAT-PROVIDER, RT-PROVIDER |
| QUAL-RUNTIME | QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY, QUAL-EVIDENCE |
| CSDLC-DECOMPOSE | CSDLC-DECOMPOSE, CSDLC-REMOTE |

All nine preexisting issue identities are retained. The wave has 69 tasks: 19 assigned issues (nine preexisting plus ten newly created SIM issues #866–#875) and 50 number-free prospective tasks. The original correction had nine bindings and 60 prospective tasks before the separately authorized first sprint launch. #855 belongs to RT-PROVIDER; #852 retains the failure-event repair; #862 retains local decomposition. The first SIM sprint is created; the remaining 50 tasks require separate authorization.

## Eleven tightened completion contracts

| Task | Complete result | Rejected partial result |
|---|---|---|
| CF-SHELL | A fresh operator configures, starts, inspects, cancels and retries an actual bounded review and opens its real artifacts through the installed product. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |
| CF-EVIDENCE | The adapter invokes a real evidence admission/store boundary that enforces stable identity, immutable provenance, redaction before retention/model use, retention/deletion and the shared finding/run contract. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |
| CF-PROOF | After CF-INTEGRATE delivers the installed candidate, an independent repeatable qualification suite runs that product on ADL and the pinned external Rust repository, including fresh-operator setup, real review, second-run comparison and all three exports. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |
| CF-INTEGRATE | The installed Beta 1 product connects completed consumers into the entire operator journey and passes integration success and failure checks on one exact candidate before independent CF-PROOF qualification. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |
| PLAT-UTS | One versioned installable UTS package is consumed by the Runtime ACC/UTS tool-dispatch path with a declared compatibility contract. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |
| PLAT-RUST | One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |
| PUB-MEDIUM | One selected v0.92.2 article is fully written, source checked and ready for editorial decision without publishing. | outline_only, unspecified_revision, future_work_list |
| PUB-CSDLC | One named C-SDLC manuscript revision completes its frozen revision checklist and source/citation checks, ready for a human research decision without submission. | outline_only, unspecified_revision, future_work_list |
| PLAT-MEMORY | The CodeFriend second-review comparison path retrieves its prior compatible run from Memory Palace through the production retrieval boundary with enforced privacy and deletion behavior. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |
| SIM-03 | The installed intent CLI executes the declared local and remote command inventory against production operations, resolving context and evidence internally while preserving guards. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |
| SIM-04 | The operational application routes every local transition and verified remote/terminal outcome through one semantic issue transaction owner, proven at actual entrypoints under crash and concurrency. | plan_only, schema_only, scaffold_only, unspecified_slice, zero_executed_scenarios |

## Planning tasks retained

WP-01, ARCH-SPLIT, OPS-GCP, ARCH-ADR, SIM-08, TAIL-07 and TAIL-08 all remain required. They complete their own decision, document or rehearsal deliverables; they do not count as implemented CodeFriend features. Cloud application, external publication, writer activation and future issue creation retain their explicit authorization boundaries.

## Execution selection gates

Before child creation, WP-01 binds the concrete product command and source ownership paths, external repository/revision/license/scope, and supported provider route. PLAT-RUST additionally selects the exact production responsibility and invariant; PUB-MEDIUM selects its article; PUB-CSDLC freezes a manuscript revision checklist; SIM-03 freezes its supported command inventory. These gates prevent unspecified slices from being opened as ready tasks.

## Evidence and dependency authority

[ATOMIC_TASK_CONTRACTS_v0.92.2.json](ATOMIC_TASK_CONTRACTS_v0.92.2.json) binds all split identities, the eleven corrected contracts, existing issue routing, retained planning tasks and exact dependencies. Both YAML files and every WBS/catalog dependency must agree. Its validator rejects missing consumers, missing success/failure evidence, schema-only closure, lost planning tasks and missing split prerequisites. Human review still decides semantic task cohesion; a unique result string is not proof.

## Validation classification

PVF: deterministic local planning-contract proof, local CPU/Ruby/Python, required for #864. Fixtures mutate split ownership, dependencies, completion contracts and documentation projections. No Runtime, provider, cloud or release execution is claimed.

## Integration before independent qualification

CF-INTEGRATE consumes complete feature implementations and produces the installed candidate. CF-PROOF follows CF-INTEGRATE and independently runs the ADL/external acceptance suite. TAIL-01 consumes both; CF-INTEGRATE never depends on CF-PROOF. Required independent OBS-S3 and ARCH-ADR remain outside those dependency sets but are explicit TAIL-10 dependencies with separate deployment and ADR acceptance obligations before truthful milestone closure.

TAIL-10 wave/specification dependency and acceptance parity is validated explicitly. Negative fixtures remove each required OBS-S3/ARCH-ADR closeout dependency and acceptance obligation from both projections, and from either projection alone. Planning validation remains deterministic local proof, not deployment or ADR completion evidence.

## Resolved first-sprint selection

SIM-03/#869 carries its complete supported command inventory in the created issue body. The [launch command inventory](../../../.csdlc/evidence/864/sprint01-launch/drafts/command-inventory.md) records the resolved selection; this first-sprint gate is satisfied for creation, not proof of implementation. All product and later-task selection gates remain in force.
