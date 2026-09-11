# v0.92.2 Feature and Proof Coverage

Status: planned ownership map.

| Exit-bar surface | Owner | Required proof |
|---|---|---|
| Shell, setup, onboarding, controls | CF-SHELL | Operator journey and failure-state tests |
| Local/GitHub/CI ingestion | CF-ADAPTER, CF-ADAPTER-GITHUB, CF-ADAPTER-CI | Portable fixture conformance |
| Stable evidence, provenance, redaction, retention | CF-EVIDENCE | Determinism, tamper, redaction, retention suites |
| Dependencies, boundaries, coupling, connascence | CF-COG | Grounded architecture fixtures |
| Drift, blast radius, quanta, ADR/rationale | CF-COG-DRIFT, CF-COG-IMPACT, CF-COG-RATIONALE | Explanation traceability and reviewer calibration |
| Fitness functions and CI | CF-GOV, CF-GOV-CI | Passing/failing deterministic policy fixtures |
| Correctness perspective | CF-REVIEW | Attributed review fixture |
| Security perspective | CF-REVIEW | Attributed security fixture |
| Adversarial perspective | CF-REVIEW | Attributed misuse/failure fixture |
| Constitutional perspective | CF-REVIEW | Attributed policy-value fixture |
| Synthesis | CF-SYNTHESIS | Executed consumer preserves attribution, deduplicates and explains severity; incomplete lanes remain incomplete |
| Remediation planning | CF-REMEDIATE | Actual synthesized findings produce bounded, evidence-linked repair steps; unsafe/incomplete input fails explicitly |
| Test planning | CF-TESTPLAN | Actual synthesized findings produce actionable behavior/fixture/assertion plans; unsupported evidence fails explicitly |
| Longitudinal second run | CF-MEMORY | Two-run and compatibility fixtures |
| Human publication controls and governance metadata | CF-UX | Approval-negative and manifest validation |
| Markdown, HTML, PDF | CF-RENDER-MD, CF-RENDER-HTML, CF-RENDER-PDF | Renderer-parity proof |
| Docs, examples, fixtures | CF-PROOF | Fresh-operator walkthrough |
| ADL self-review | CF-PROOF | Retained review packet |
| External OSS proof | CF-PROOF | Licensed, bounded retained packet |
| Complete Beta 1 | CF-INTEGRATE | Exit-bar reconciliation and end-to-end failure matrix |
| Config-driven providers | PLAT-PROVIDER | Production definition consumption/reload and invalid-definition proof after RT-COST and merged #622 |
| Dynamic provider lifecycle | RT-PROVIDER / #855 | Real Runtime provider attach/detach/continuity and failure behavior |
| MLX/Metal adapter | PLAT-MLX | Bounded platform smoke and unsupported-platform failure |
| NVIDIA PAIR experiment | PLAT-PAIR | Reproducible bounded comparison, resource/cost capture, and explicit keep/repair/retire decision |
| UTS productization | PLAT-UTS | Installable versioned package exercised through actual Runtime ACC/UTS dispatch, compatibility and rejection proof |
| Recurring Rust reduction | PLAT-RUST | Behavior parity and before/after measurement |
| AWS inventory maintenance | OPS-AWS | #484 baseline comparison, business-account readbacks, and redaction validation |
| Company GCP move-in reconciliation | OPS-GCP | Current sanitized source/destination inventory, dependency ordering, rollback notes, and explicit no-mutation proof |
| Medium article preparation | PUB-MEDIUM | Source/citation traceability and non-publication check |
| C-SDLC paper preparation | PUB-CSDLC | Source/citation traceability and non-submission check |
| Memory Palace second-review integration | PLAT-MEMORY | Actual second-review prior-run retrieval, deterministic comparison, privacy and deletion negatives |
| Speculative-decoding decision | SPEC-RETEST | Current benchmark, equivalence, and fallback proof |
| Static Observatory edge deployment | OBS-S3 | `agent-logic-admin` identity, Terraform plan/apply, authenticated AWS readback, logging, security headers, invalidation, browser HTTPS and Runtime WSS, rollback and cost record; non-gating for integration and TAIL-01; required for TAIL-10 |
| Milestone architecture decisions | ARCH-ADR | Decision inventory, source traceability, ADR structure, explicit status, and supersession checks |

No row may be marked proven from a planned demo, a zero-test invocation, or green CI that does not cover the stated behavior.

## Additional admitted proof

| Surface | Owner | Required proof |
|---|---|---|
| Shared finding/run contract | CF-EVIDENCE | Review/memory/renderer conformance fixtures and compatible-version negatives |
| Pre-synthesis independence | CF-REVIEW | Isolated lane inputs and retained disagreements |
| Canonical-name A2A predecessor | v0.92.1 / #718 | Consume the reviewed merged invalid-name/continuity and two-agent model-generated reply proof; no v0.92.2 recreation |
| Capability orientation predecessor | v0.92.1 / #717 | Consume reviewed merged first-turn, canonical-inventory, stale-entry, and version/digest proof; no v0.92.2 recreation |
| Live Observatory | OBS-LIVE / #720 | Live navigation remains functional; no retained mode, historical polling or evidence deletion |

## C-SDLC simplification sprint

SIM-01 through SIM-09 own the installed journey, strict observation, finite recovery, immutable evidence, unified semantic state, projection integrity, conversion, qualification, transition operations, and authorized-pilot proofs in the [plan](cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md). SIM-UMBRELLA aggregates the scorecard and closes after all packages; TAIL-01 consumes its result. The #523 native correctness repairs are baseline inputs, not SIM acceptance.

## Complete-task acceptance

Every implementation owner named above must exercise its real consumer, success and failure paths under [atomic task contracts](ATOMIC_TASK_CONTRACTS_v0.92.2.md). Final integration does not complete unfinished upstream features. Provider-neutral lifecycle is owned by RT-PROVIDER/#855; validated editable definitions remain PLAT-PROVIDER. QUAL-EVIDENCE joins the complete criterion-specific results without absorbing their repairs.

## Split supporting proof owners

| Surface | Owner | Required proof |
|---|---|---|
| Dispatch failure event repair | QUAL-RUNTIME / #852 | Production dispatch failure emits the correlated event with bounded redaction-safe payload |
| Resident and restore qualification | QUAL-RESIDENT | Executed resident workload and signed restore, with failure cases and resource evidence |
| Provider failure/recovery qualification | QUAL-PROVIDER | Executed real provider failure and recovery through Runtime |
| Validation inventory | QUAL-INVENTORY | Measured before/after denominator, categories and omissions |
| Criterion evidence admission | QUAL-EVIDENCE | Actual qualification results admitted/rejected against criterion, candidate and provenance; no self-declared success |
| Local command decomposition | CSDLC-DECOMPOSE / #862 | Local behavior parity and recursive source accounting |
| Remote command decomposition | CSDLC-REMOTE | Remote behavior parity, transport failure handling and recursive source accounting |

The eleven strengthened contracts also require a complete selected Rust responsibility, finished selected article/manuscript revision, installed SIM-03 command inventory and production SIM-04 transaction-owner proof. Planning outputs remain separate evidence categories.

## First sprint creation state

SIM-UMBRELLA is #866 and SIM-01 through SIM-09 are #867 through #875 respectively. The [verified mapping](../../../.csdlc/evidence/864/sprint01-launch/issues.json) records native creation. These ten issues join nine preexisting bindings: 19 assigned issues and 50 unassigned tasks in the unchanged 69-row milestone. Creation does not claim implementation or authorize live activation. Later sprint creation remains separately authorized.
