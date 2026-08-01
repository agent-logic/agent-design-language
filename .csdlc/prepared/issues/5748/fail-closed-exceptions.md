# v0.91.8 terminal closeout exceptions

This register records issues that cannot receive a terminal receipt without
inventing or bypassing C-SDLC v2 authority. Entries remain provisional until
the final exhaustive closeout audit.

## #5335 — outside the merged-PR eligibility boundary

- GitHub issue state: closed, `NOT_PLANNED`.
- PR #5380: intentionally closed unmerged; its body says it is superseded and
  intentionally contains no closing keyword.
- No merged PR links #5335.
- Disposition: excluded from the current closed-and-merged objective. Do not
  synthesize merged terminal truth.

## #5346 — stale plan truth cannot be repaired in its current phase

- GitHub issue state: closed; PR #5752 merged from exact source head
  `7b1ef84bc8a4966c0c454ae4d87fd973537a856d`, merge commit
  `ccca46abceb117150efbc3b69248fba611d90fff`, with required checks passing.
- The live issue worktree remains at typed `merge_ready` generation 12, digest
  `1341748ec10bbf4434a2892d72a28ec9a931a8f74c3b0bbf2a0ee24815a587bc`,
  under active claim `claim-5346-v0918-wp13-deletion-preparation-current`.
- All five SPP execution steps remain `pending`, and the SOR still records
  pre-merge integration truth. Terminalizing that projection would preserve a
  false execution narrative despite the merged GitHub result.
- The first typed corrective `update_plan_step` request failed without mutation
  with `invalid_transition: spp mutation is not allowed during merge_ready`.
  No later SOR edit or closeout was attempted after that fail-closed guard.
- The dedicated closeout branch intentionally has no synthesized #5346
  projection or retained receipt.
- Disposition: no terminal receipt. A typed, phase-safe repair route must first
  reconcile the stale SPP and SOR truth; manual card edits or direct closeout
  are prohibited.

## #5701 — exact-head review blockers

- GitHub issue state: closed.
- PR #5705: merged at source head
  `db41b249277a91140d4fd67bfc5bf898f4565774`, merge commit
  `647e68f00aa339ca7c2fa3c7636fe59f9ffa163e`.
- Independent exact-head review: blocked.
- P1: `demos/html-observatory/app.js` accepts a query-selected HTTPS runtime
  origin, derives its WSS endpoint, and automatically sends a token retained in
  `sessionStorage`; a crafted dashboard URL can exfiltrate the operator token
  to an attacker-controlled WSS origin. Credentials must be explicitly bound
  to a trusted normalized origin and an endpoint change must not receive a
  stored token.
- P2: `docs/api/runtime-v3/v1/openapi.json` declares
  `runtimeSignedCommand` as `mutualTLS`, while the server implements ordinary
  server TLS plus an Ed25519-signed JSON request body. Generated clients would
  enforce a nonexistent client-certificate handshake.
- P2: `guardian_soak` is non-hermetic because its helper hardcodes an untracked
  worktree-local `.adl/bin/vector`; four process-starting tests exit 78 when the
  binary is absent, and their readiness waits hide the captured child stderr.
- P3: `demos/html-observatory/README.md` says Observatory reads use bearer
  authentication and later correctly says the same reads are public.
- Passing exact-source evidence: control 21/21, Observatory WSS 6/6, OpenAPI
  contracts 6/6, `runtime_api_wss` 2/2, parity 31 passed / 1 explicitly ignored,
  and the HTML Observatory integrated proof.
- The bounded terminal-closeout claim was released through typed v2 at
  generation 34; current claim authority is null with digest
  `b8d64d8b742426c08a40574c971a9db3c01a4b4fcae741a1ff0555c8f98f0afb`.
- Disposition: no terminal receipt. The bounded closeout claim was released
  after review; remediation and a new exact-head review are required.

## #5663 — historical exact-head defect and superseded implementation

- GitHub issue state: closed.
- PR #5669: merged at source head
  `fb04c9fa29c528c06a7b3c76e5f6560b7700d43e`, merge commit
  `735b2131004f7d299836ce98d226fe3c2ec8593c`.
- Exact-head review found that checkpoint replacement uses
  `fs::rename(tmp, checkpoint.json)` without Windows replacement handling;
  the second store can therefore fail on Windows, while the focused test only
  exercises one store.
- The preserved issue worktree later advanced to substantive Chronosense work
  that was truthfully rehomed to #5697 / PR #5699. It must not be treated as
  the immutable #5663 PR head or reset merely to simplify closeout.
- The checkpoint implementation was subsequently superseded by #5698 / PR
  #5707 redb state and separate Windows recovery work.
- The stale claim `claim-5663-runtime-v3-durable-local-adapters` was released
  through typed v2 at generation 36, producing digest
  `8533c94d13734ceb2165a58bdc2c814099a0682941b676441363246db2b7e695`;
  current claim authority is null.
- Disposition: no receipt from the stale worktree. Historical typed
  reconciliation must retain the exact-head finding and its supersession
  disposition without claiming a clean review.

## #5007 — corrupt nonterminal projection

- GitHub issue state: closed; PR #5743 merged from exact source head
  `426d0a53fb2b7b0be571b236ca5d0a248b32e1f8` with required checks passing.
- The published generation-5 record claims digest
  `12194eb860c30b87b2e8929d2fe0726fbe7006d0c901454b581ee82fa693f6ed`.
- Commit `72b96618f7cede755d90b2c7fdda7d4fcb74de49` subsequently changed governed
  claim, review, card-value, and rendered-card content without advancing the
  generation or digest. The exact PR head retains that mismatch.
- Typed claim revoke and `csdlc-doctor` both fail closed with
  `corrupt_record: index digest mismatch`; there is no retained receipt or
  recovery journal from which to restore authority.
- Disposition: no terminal receipt. A separate typed, journaled nonterminal
  projection repair/import primitive is required; manual redigesting or card
  restoration is prohibited.

## #5678 — CI routing and proof blockers

- GitHub issue state: closed.
- PR #5685: merged at source head
  `90165c6ee1f4bed18820731efd7326dbab4a6669`, merge commit
  `16b08fd70004a9ad4119c66437bcb53f43aeb140`.
- Independent review found that the focused Opus runbook test is not invoked by
  CI; its selector route reaches only a docs diff check.
- The CI path-policy self-exemption checks only a small string subset and does
  not prove complete payload or line-count identity, so mixed policy changes
  can skip authoritative coverage.
- The recorded routing result itself reported escalation required and
  publication insufficient while coverage remained disabled, despite the
  hosted checks appearing green.
- Lifecycle review/publication truth is stale and doctor reports
  `review_publication_dead_end`.
- The stale claim `claim-5678-opus-review-runbook` was released through typed
  v2 exact CAS at generation 4 after liveness checks proved the owning task was
  archived; current claim authority is null with digest
  `66d1f6fe51ebe463115ecc7bfc01d48413c55c53fc3dd3392575341fae49fb6b`.
- Disposition: no terminal receipt until the CI proof defects are remediated
  and current exact-head review evidence exists.

## #5664 — protocol-adapter acceptance blockers

- GitHub issue state: closed; PR #5680 merged from exact source head
  `16e6594dae2f76e41ebf432c9ea477523e685247`; focused protocol tests pass
  11/11 and exact-SHA hosted checks are green.
- P1: the issue requires Rustls/mTLS for networked transports, but production
  configuration and black-box tests use `with_no_client_auth()`. No client
  certificate/key configuration exists, so the implementation proves only
  one-way TLS plus message authentication.
- P2: outbound protocol payloads and serialized frames have no byte limit.
  Framing hex-encodes and clones the full payload before an arbitrary-size
  socket write, despite the bounded-adapter acceptance posture.
- The final merge-resolution delta introduced no separate integration
  regression; the builder remains intentionally exported but not cut over in
  production assembly.
- The bounded terminal-closeout claim was released through typed v2 at
  generation 5; current claim authority is null with digest
  `8c254685618757825b8b738c551e5a54b41894f896f0ddb24214e9f935a537f8`.
- Disposition: no terminal receipt. The bounded closeout claim was released
  through typed v2 after review; mTLS and request-bound remediation require a
  new exact-head review.

## #5733 — stale milestone truth and invalid recorded proof

- GitHub issue state: closed; PR #5747 merged from exact source head
  `7593093c4d5ebf4b339e31aca1ce5ade1a049141` under the docs-only CI route.
- P1: the final matrices still say #4760 and #5007 are open even though both
  issues closed before the final commit. The aggregate row may remain an open
  gate because other owners are open, but its cited owner-state evidence is
  stale and its validator does not prove those states.
- P1: SIP, SOR, and VPP retain the removed
  `adl/tools/validate_v0918_demo_matrix.py` path after the validator moved under
  the review packet. The recorded command fails with file-not-found; only the
  unrecorded docs-local path passes.
- P2: the PR-range `git diff --check` reports generated whitespace errors in
  three prepared files, while the recorded bare-command evidence is empty.
- The post-review commit range is mechanically metadata-only, but its metadata
  contents preserve the blockers above.
- The stale execution claim was released through typed v2 exact CAS at
  generation 11; current claim authority is null with digest
  `d2f03338be22e4e2e5542a3cd07434b1cad143ce9515944139e65378d6930aea`.
- Disposition: no terminal receipt until the matrices, typed validation truth,
  and diff-hygiene evidence are corrected and reviewed at a new exact head.

## #5722 — trusted-origin, rerender-race, and certificate-proof blockers

- GitHub issue state: closed; PR #5723 merged from exact source head
  `37fed42431302fda2ec68873446cdadaaa90355b`.
- P1: query validation accepts any HTTPS runtime origin and then forwards the
  session-stored bearer token to its derived WSS endpoint. This violates the
  issue's trusted-localhost boundary and permits crafted-link token exfiltration.
- P1: the completion-generation guard covers only live refresh. Late retained
  refreshes and WSS-error fallbacks can still reset the mode to published and
  overwrite a newer Stop/Connect or mode-selection result.
- P2: the documented static host and runtime can provision independent
  certificates, and neither implementation nor proof compares certificate
  identity across ports 8765 and 20997. The focused HTML proof does not exercise
  the browser controls, shared certificate, or WSS state transitions.
- Passing evidence is narrower: JavaScript syntax, the fixture-oriented HTML
  proof, one real runtime HTTPS/WSS/signed-control guardian test, and exact-SHA
  CI pass. It does not establish the missing acceptance claims.
- The issue has no local C-SDLC record or receipt; no recordless recovery was
  attempted because substantive review findings already fail terminality.
- Disposition: no terminal receipt until the security, race, and shared-cert
  proof defects are remediated and reviewed at a new exact head.

## #5675 — provider-global billing misclassification

- GitHub issue state: closed; PR #5676 merged from exact source head
  `3eddf1ead3e4237b4fed3f68f08bff9ca38f851e`; provider-adapter tests pass
  43/43 and exact-SHA hosted checks are green.
- P2: common non-2xx response handling classifies the bare substring `1008` as
  MiniMax insufficient balance for every hosted provider. An unrelated OpenAI,
  Anthropic, DeepSeek, or other response containing those digits can therefore
  become non-retryable `ProviderBillingBlocked`, suppressing valid retries and
  corrupting telemetry.
- MiniMax already has provider-specific structured handling for code 1008 in
  successful HTTP envelopes; non-2xx handling must likewise be provider-aware
  or parse a scoped MiniMax envelope, with a negative cross-provider test.
- The bounded terminal-closeout claim was released through typed v2 exact CAS
  after review; current claim authority is null with digest
  `cc151e358e674d07613646d4fc1f6ed71a3613a2f145b9065a73bc0103770818`.
- Disposition: no terminal receipt until the global classifier is corrected
  and reviewed at a new exact head.

## #5657 — receipt-only production readiness

- GitHub issue state: closed; PR #5659 merged from exact source head
  `faf0c62c231e4db1ad7a582cc5a7a57b085a310b`; focused assembly and
  Observatory tests and exact-SHA CI pass.
- P1: production readiness validates only that all ten adapter map keys exist.
  Every key is populated with an in-process executor that returns an accepted
  receipt without performing the required Agent, Provider, Scheduler,
  Chronosense, ACIP, A2A, CloudBridge, checkpoint, or lifelog behavior.
- The focused assembly test asserts this placeholder map is production-ready,
  so the green proof encodes rather than mitigates the false-readiness defect.
- Later merged work in #5663, #5664, and #5687 replaced or strengthened parts
  of this behavior, but cannot retroactively make immutable PR #5659 clean.
- The stale historical claim was released through typed v2 at generation 2;
  current claim authority is null with digest
  `84d1ee502e3122b21be2d31b5a6a04cc80c6976baa2a4055d27f8bd7a76fccc5`.
- Disposition: no terminal receipt from the historical head. Any supersession
  record must preserve the exact-head finding and cite later remediation
  truthfully rather than claim the original acceptance criteria passed.

## #5558 — live v1 guidance and incomplete authority guard

- GitHub issue state: closed; PR #5749 merged from exact source head
  `033b28cffa6bdf191b1d013aa5a730ce7b10d9df`; declared focused tests, the
  owner lane, Gate 10A, and exact-SHA CI all pass.
- P1: the replacement guidance guard omits live root-referenced tooling docs
  and operational skills that still teach `adl/tools/pr.sh run`, so it reports
  alignment while current operator guidance contradicts Gate 10D2.
- P1: `adl/tools/editor_action.sh` still exposes and executes a `start` action
  mapped to `./adl/tools/pr.sh start`, and its test requires that sunset path.
- P2: the advertised full C-SDLC owner lane does not run the actual
  coexistence/final-authority Gate 10A proof, allowing the narrower guidance
  tests to pass despite stale executable and instructional surfaces.
- Disposition: no recordless terminal receipt until all live v1 routes and
  guidance are removed, the owner lane proves final authority, and the result
  receives a new exact-head review.

## Recovered tooling exception

- #5499 is no longer an exception. Typed historical recovery preserved the
  newer claim-free generation-20 review authority, refreshed GitHub linkage
  inside the terminal command, and produced a claim-free generation-21
  `closed_out` projection plus retained receipt. `csdlc-doctor` passes.
