# Feature Proof Coverage — v0.92.1

Current accounting: merged #517 follow-up PR #752 reconciles all 245 historical non-proving rows and five exception groups. Accounting is complete; historical gate results are preserved and release authorization remains separate.
Review status: these requirements and planned outcomes are not an acceptance ledger. See [the TAIL-02 review packet](evidence/release/tail-02/README.md) for source revisions, observed proof gaps and deferred scope. The documentation handoff includes merged #517 and its blocked release decision; issue closure alone is not semantic proof.

| Lane | Required proof |
|---|---|
| Corporate and IP | Reviewed redacted transfer and rights authority |
| C-SDLC v3 | Typed schema, migration, rollback, and behavioral proof |
| Distributed multi-agent Runtime | #345 GPU Shepherd hardening plus authentic multi-agent UTS work, continuity, and resources |
| Podcast | Identity, rights, feed, episode, Studio, playback, and release authority |
| Axum configuration hot reload | Parse/validate/swap, last-known-good, debounce, failure, concurrency |
| Observatory redesign | OBS-A/#511 and OBS-B/#512 authentic Runtime projections, accessibility, redaction, and empty/degraded/recovery behavior; #84 Unity and #251 TLS remain deferred; #122 retains separate public-exposure ownership |
| Runtime v2/v3 decoupling | Complete source/reverse-reference census, exclusive ownership, compatibility, migration, and rollback |
| Provider inference profiles | Profile schema, deterministic Ollama materialization, invalid-profile/last-known-good behavior, redaction, shadow isolation, and comparison fallback |
| GCP qualification sidecar | Exact six-resident identity/workload replay, continuity, cost, and zero-resource cleanup; no #269 execution |
| AWS account move-in | Seven exact phase-result receipts: inventory, access/billing with governed Agent Toolkit setup and attributable activity, Terraform bootstrap, audit/security, adoption, Runtime modules, and retirement decision |
| GCP account move-in | Five exact phase-result receipts: hierarchy/cost, Terraform bootstrap, organization/billing, private platform, and GPU readiness |
| Cross-cloud Terraform conversion | Exact #194/#268 template census, portable contract, provider-specific plans/deployments, parity, rollback, and cleanup-zero |
| Rust resilience refactoring | API and behavior parity, fault/trace/retry/timeout/cancellation proof, module ownership, and exact validation-impact comparison; no LoC quota |

Repository authority (#432), exact scope, review identity, and immutable revision binding apply to every row.
