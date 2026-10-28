# Security lane — changes required; bounded boundary review

Candidate: `5c4a6149771c637f3c805985b86231077965eab4`.
Base: `c9cdb2f13ff77a06dcc79ad4e4c332f444b2e14f`.
Reviewer: dedicated security subagent; did not implement candidate changes or author their prior approval. This is independent source inspection, not a deployment/security certification. Upstream draft acceptance is separate.

## Findings

1. **SEC-001 / P2 — Credentialed mutation curl still reads implicit configuration.** `csdlc-v3/src/adapters/mod.rs:754` builds mutation curl arguments without first-position `-q`; `run_process` appends the private credential config without disabling default config. In contrast the observational route explicitly suppresses account-database/home configuration lookup. Existing operator curl configuration can redirect response output, turn on trace files, or add transfers outside typed intent. An isolated real-curl/file-protocol probe using the exact production subprocess function reproduced an unexpected output file; adding first-position `-q` prevented it. No live credential or network used. Apply the same isolation to mutations and preserve the negative control.

2. **SEC-002 / P2 — Mutation diagnostics redact after truncation.** `csdlc-v3/src/adapters/mod.rs:360` obtains already-truncated `run_process` output then replaces the complete credential value. `process_output:955–968` cuts the bytes first. A credential crossing the cutoff leaves a prefix that cannot match the complete value. The same extracted functions reproduced this with synthetic data and an 8-byte cutoff. Observational curl already redacts before bounding. Fix the shared credentialed path and test both streams with a cutoff inside the value. This is a diagnostic-boundary reproduction, not an observed leak of an operator token.

3. **SEC-003 / P2 — PDF relay verification accepts self-consistent substituted content.** `adl/src/codefriend/publication/relay.rs:240–260` accepts any nonempty returned bytes beginning `%PDF-` if caller metadata/hash fields are consistent. `pdf.rs:562–606` does not recompute semantic content or inspect the file; `common` only matches hashes of caller-selected bytes. `agent/publication.rs:958–1098` exposes this through the public stage verifier. A paired relay can keep a valid approved decision and report identity, replace the PDF, and recompute unkeyed report/export/manifest/stage digests. Markdown and HTML are instead regenerated and byte-compared. This finding is established by call-path analysis; an end-to-end substituted PDF fixture was not executed. Bind output to trusted rendering evidence or independently verify allowed PDF structure and expected report semantics, with a resealed-substitution negative regression. Do not mistake `external_resources=false` metadata for byte inspection.

All four findings (including the website finding below) route to **#921 with the named component owners**; none authorizes repairs inside #919. Full structured triggers, evidence, reproduction limits and acceptance relevance are in `security-findings.json`.

## Reviewed boundaries and positive observations

- CodeFriend local acquisition reads exact Git blobs with literal scope, rejects symlink inputs, bounds bytes, excludes unsafe credential-shaped material and writes create-only private packets. Repository content is not executed by that acquisition path.
- GitHub acquisition fixes the authenticated origin, disables redirects/proxy, validates source/head identity and blob hashes, rejects truncated/paginated incomplete trees and caps request/byte counts. No live request was made.
- Hosted service separates subject and mode, rereads credential state, uses private state and no-replay reservations, and rechecks authority after export observation. UNIX control requires owner/root peer identity in a private directory and binds commands to an instance/attempt.
- Publication admission retains verified source bytes and exact manifests. Live website/local approval capabilities are constructed by service code rather than supplied as deserializable JSON authority. Reviewed append paths refresh current approval context.
- Runtime management handlers inspected require the write credential; selected Observatory WS branches recheck token revocation and generation before result delivery. Public health/feed reads are explicitly policy-driven; complete feed privacy is not established by these section checks.
- Private-state primitives verify trusted Ed25519 signatures, lineage/equivocation, projection integrity, principal membership and sanctuary level. That does not establish integration-level principal authentication.

## Validation and limits

`security-probes/transport_probe.rs` extracts the candidate's `run_process`, `apply_minimal_child_environment`, `process_output`, `truncate`, and `redact_secret` bodies verbatim. Only invocation/output types are minimal stubs. Compiled with local `rustc`; **2 concrete defect cases reproduced, 1 negative control passed**. Real curl used `file://` only. No cloud/provider/live Runtime/GitHub calls, credential reads, product edits or full CI reruns occurred. The temporary fixture contains only synthetic values. `security-probes/results.json` records this limited proof role.

The full inventory contains **6,195 rows**. Final ledger counts: 23 complete ADL files, 17 section inspections, one targeted search/section inspection, 5,064 historical/documentary exclusions and 1,090 paths not reviewed in this security lane. The separately frozen website adds 18 fully read boundary modules. These are inventory classifications, not a claim that excluded or cross-lane source is defect-free. The final ledger and explicit residual section list supersede earlier checkpoint counts. Truncated reads are credited only by the sections actually inspected.

**This is not whole-milestone security clearance.** Four findings are actionable. Runtime/provider and C-SDLC supporting internals need the other lanes' authored observations or explicit coverage-gap disposition. No PASS or whole-source assertion is made.

## Extended boundary pass
Additional source inspection covers private runtime identity authority, ingress redaction, resident read-only tool grants, CodeFriend agent consent/transport, C-SDLC merge authorization, and hosted deployment boundaries. See per-path coverage JSON for exact sections; this does not upgrade unread sections into complete review. Historical evidence/cards/documentary rows are explicitly excluded from executable boundary coverage, not certified clean.

The separately frozen CodeFriend website a45e339c13b24716edbd3fadf29dddff36ffe02e was reviewed at its session/OAuth, HTTP, credential, host-control, store/privacy, native-verifier, paired-agent, hosted/local publication and Journey boundaries (18 complete modules).

### SEC-004 — P2: Recheck browser authority after a mutation body wait
`codefriend.ai/app/http.mjs:147-155` retains the initial authorized user while awaiting a request body, then dispatches a hosted review even after logout or session expiry. Publication mutations share this pattern. The actual frozen application plus actual Sessions reproduced 202/one dispatch after session revocation; a fresh check rejected. `security-probes/session_probe.mjs` documents the loopback-only fixture and unused throwing dependency stubs. Reauthorize after body reads and before effects; recheck before sensitive responses.

Validation is deterministic local review-diagnostic work, not a release gate. Three positive reproductions and two negative controls ran (two adapter cases and one browser case); PDF substitution remains static complete call-path evidence, not executed native verifier proof. No provider, paid model, AWS, live Runtime or GitHub writes occurred.

Residual sections remain explicitly enumerated in security-coverage.json. Provider/cloud successor coverage should be joined by the lead. This lane has actionable findings, not a security PASS, and does not certify every production function or all 6195 historical/doc rows.

Final additional static checks covered the entire canonical C-SDLC authority module and root idle-stop controller, plus approval-store locking, chain validation, external antirollback head and atomic publication sections. No additional actionable finding emerged. Remaining internal sections are listed precisely in the coverage ledger.
