# v0.91.8 terminal closeout exceptions

This register records issues that cannot receive a terminal receipt without
inventing or bypassing C-SDLC v2 authority. Entries remain provisional until
the final exhaustive closeout audit.

## #5007 — corrupt nonterminal projection

- GitHub issue state: closed; PR #5743 merged from exact source head
  `426d0a53fb2b7b0be571b236ca5d0a248b32e1f8` with required checks passing.
- The published generation-5 record claims digest
  `12194eb860c30b87b2e8929d2fe0726fbe7006d0c901454b581ee82fa693f6ed`.
- Commit `72b96618f7cede755d90b2c7fdda7d4fcb74de49` subsequently changed governed
  claim, review, card-value, and rendered-card content without advancing the
  generation or digest. The exact PR head retains that mismatch.
- The source issue has since completed typed closeout and retained an authentic
  generation-6 terminal receipt, but the aggregate checkout still contains the
  older corrupt generation-5 projection. `csdlc-doctor` therefore continues to
  fail closed with `corrupt_record: index digest mismatch`.
- Disposition: preserve the authentic receipt and corrupt aggregate projection
  separately until a typed cross-worktree recovery can replace the projection;
  manual redigesting or card restoration is prohibited.

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
