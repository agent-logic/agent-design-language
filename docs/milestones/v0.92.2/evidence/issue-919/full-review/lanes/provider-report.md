# Provider and Runtime review — changes required; allowed static scope complete

**PRV-001 / P2 — Resident health joins different model identities.** At `adl-runtime-kernel/src/control.rs:6712`, a live-admitted resident's native provider model ID is matched against accounting's declared model reference. With an alias these differ. Successful inference is missed, yielding `inference_unverified` and preventing verified incident recovery. The small health endpoint already uses `provider_binding.model_ref`. Use that stable identity consistently and add a differing-alias regression covering live admission, roster refresh and incident resolution. Complete static data flow is in `provider-findings.json`; no executable reproduction is claimed. Owner: #921 Runtime/provider owner.

Candidate `5c4a6149771c637f3c805985b86231077965eab4`; base `c9cdb2f13ff77a06dcc79ad4e4c332f444b2e14f`. This reviewer did not implement the candidate. This is the same reviewer as the security lane, providing separate specialty observations, not a second independent reviewer.

## Coverage and observations

The exact ledger records full sources, complete changed hunks, partial ranges and unread paths. New provider modules were compared against their pre-extraction equivalents where stated, rather than treating a move as an entirely new implementation. Supporting unchanged functions were inspected for findings. No sampled subsection is credited as a complete file.

- Immutable provider definitions, strict candidate admission, model identity and generation/digest projection were read. Runtime transport limits constrain attempts to one, enforce timeout bounds and bound DNS workers. Definition budgets survive fresh bindings and stop after failure.
- Registry dispatch holds a shared worker permit until underlying blocking completion, even after async cancellation. Cancelling the caller does not establish server-side cancellation.
- Usage accounting uses binding epochs to fence retired completions. Health metadata cannot overwrite newer inference observations. Resident incident state persists before responses and bounded retries; acknowledgement alone does not establish recovery. PRV-001 exposes the missing alias join case.
- Shepherd recovery has idle ready behavior, explicit invalidation generations and replacement scopes; tests assert no recurring inference, bounded backoff, cancellation, freshness and retired-binding protection.
- Incremental master-log polling bounds scan and partial-record memory. Strict memory-palace reads compare journal/latest without silently repairing pointers. Supporting changes preserve lineage until successful validation.
- Static test review includes all changed hunks of the requested execute/provider definitions, long-lived agent, birthday, Kimi and provider construction/HTTP contracts; full MLX and PAIR gate sources; Runtime reload/TLS and kernel lifecycle/roster/observability/OpenAPI tests. MLX/PAIR live tests remain ignored and explicitly operator-attended; they were not run. PAIR serving-node identity and server-side cancellation are not proven.

## Historical findings crosswalk

D520-RUNTIME-001: `control.rs:2685` guards exhausted claims, with retained tests at 10444 onward preserving interrupted final attempts as terminal. D520-RUNTIME-002: the retry regression through 10656 preserves logical work/conversation identity, distinct internal execution attempts, canonical ingress keys and event correlation. This is renewed static inspection, not a fresh test PASS.

D520-SEC-001/002: historical cloud remediation documents were located, but authenticated exact plan-byte AWS/GCP history crosswalk was not revalidated in this lane. No cloud clearance can be inferred.

## Limitations and remaining work

The lead reported a tool-side restriction and narrowed this assignment to unaffected read-only reliability/configuration work. No restricted probe was retried or bypassed. No new provider-lane exploit/auth-bypass probe, executable test, build, credential read, live endpoint or cloud operation occurred. Earlier completed security-lane isolated diagnostics remain separately documented.

The allowed Runtime/provider reliability and configuration scope is complete at the changed-hunk boundary recorded in `provider-coverage.json`. Full move-aware deltas close HTTP-family, Deepgram, substrate, facade and test extraction coverage; complete adapter, control, conversation and startup changes were reviewed. Original cloud assurance remains **unreviewed**, and manifest/lockfile assurance belongs to the lead dependency lane. This is not an unrestricted provider/cloud PASS.

**PRV-002 / P2 — Successful HTTP response aggregation has no byte bound.** Shared JSON/text helpers at `adl-provider-core/src/http_family.rs:153/:165`, and generic JSON at `:2594`, read the whole success body. The canonical registry constructs these executors directly; its optional demo wrapper checks only extracted output after parsing. A faulty provider returning oversized JSON, including unused fields, can consume memory far beyond the nominal 4 MiB limit and disrupt all residents. Apply predecode byte bounds as the legacy Runtime adapter does. This is a source-confirmed call path, not an executed oversized-response reproduction.
