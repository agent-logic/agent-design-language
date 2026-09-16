# Sprint 6 integrated review packet

Issue: #932
Roster: #903, #904, #905
Disposition: PASS WITH RETAINED LIMITS

## Delivery ledger

| Child | Reviewed head | PR | Merge | Terminal state |
| --- | --- | --- | --- | --- |
| #903 PLAT-MLX | `dd5040549b960c83921d96713223af15b62c333a` | #965, merged, green | `6aaf316996d68726d38a20ae5f3cef4f81ce1412` | closed; native receipt `e4592cd1306fee7d8eeec3ecac426e7ea291ade1b8998d3d6bf1878ac9989c70`; worktree absent |
| #904 PLAT-PAIR | `85b6d24eb14129451851c12df53b5a679a12a34c` | #972, merged, green | `b7cbd784e5ed2dd750a09f1656de611f13d4ca86` | closed; native receipt `9245f71230bcc51a20a767c16b10d2f1c105976eec6601e5b181001abb91db10`; worktree absent |
| #905 SPEC-RETEST | `0906d81d3c848bd69d89a32ba011ca27ba6cd036` | #1004, merged, green | `8a6fdb694f0e49bf72062d01830eb66ffa15d8e2` | closed; native receipt `0890d676b9077eb7929320ce236d3f3ccc8bdd3efb8f0bce5b0b488bd7f974db`; worktree absent |

All three merge commits are ancestors of the Sprint 6 closing candidate.

## Acceptance ledger

### #903 — PLAT-MLX

1. PASS — The registered MLX adapter consumes the canonical provider definition and dispatches through the production workflow with pinned Apple M4 Pro, MLX and model evidence.
2. PASS — The retained actual Metal workflow produced a nonempty response through the canonical Runtime path with immutable output and sidecar identities.
3. PASS — Invalid definitions, missing service/model behavior, timeout/cancellation, malformed output and unsupported-platform behavior are covered by focused negative proof without provider fallback.
4. PASS — Shared reload, profile, cost and credential boundaries remain intact; focused checks, strict Clippy, hosted CI and independent exact-head review passed.

Limit: this qualifies the bounded adapter and actual Metal route. It does not establish faster code review, a public benchmark, or general MLX superiority.

### #904 — PLAT-PAIR

1. PASS — PAIR v0.1.1, identical Phi-4 model bytes, Apple M4 Pro and RTX 3090 nodes, transport, corpus, concurrency and zero-cloud-cost bounds were pinned.
2. PASS — The same corpus ran through direct Ollama baseline, raw PAIR and canonical Runtime-through-PAIR routes. The retained matrix accounts for 72 requests and 72 correct outputs.
3. PASS — Concurrency 1 and 2, controlled RTX node loss, Mac failover and RTX recovery were executed and attributed to the selected node.
4. PASS — Independent review accepted the evidence-bound `REPAIR` decision. PAIR materially improved larger/concurrent work in this heterogeneous two-node experiment, while startup outliers and operational gaps remain.

Limit: PAIR distributes requests; it does not pool VRAM. The measured gain includes RTX hardware and does not isolate router overhead or establish production readiness.

### #905 — SPEC-RETEST

1. PASS — The current Runtime candidate used immutable same-blob Qwen3.5:9b aliases, identical tokenizer metadata, one Runtime identity, a fixed prompt and bounded local Apple M4 Pro resources.
2. PASS — Four counterbalanced blocks produced eight exact baseline/speculative output pairs. End-to-end speculative wins were 1/4, decode wins were 2/4, with median changes of -5.9553% and -11.3340% respectively.
3. PASS — Invalid draft configuration was rejected and healthy operator-selected ordinary generation recovered; alias ownership and cleanup failure paths have focused regression coverage.
4. PASS — Independent exact-head review of final revision `0906d81d3c848bd69d89a32ba011ca27ba6cd036` found no actionable issues, its typed receipt is retained in this packet, and hosted CI passed. The evidence-bound disposition is `repair_inconclusive`. The merged child SRP/SOR predate that final review and remain historical publication-time truth.

Limit: speculative decoding is not qualified as faster. A follow-on needs stabilization, a larger paired denominator and a robust confidence rule.

## Integration and release truth

Sprint 6 delivered three bounded qualification results through existing provider and Runtime contracts. No child changes the meaning of another child’s proof. The combined result supports the MLX adapter on the tested Apple path, PAIR as a repair-stage request-distribution experiment, and speculative decoding as an inconclusive repair candidate.

No retained limitation blocks closing Sprint 6 because each issue asked for an evidence-bound result, including negative or inconclusive outcomes. None of these results authorizes production rollout, public performance marketing, pooled-VRAM claims, or release approval.

## Evidence references

- `.csdlc/evidence/903/` and `.csdlc/issues/903/cards/sor.values.json`
- `.csdlc/evidence/904/` and `.csdlc/issues/904/cards/sor.values.json`
- `.csdlc/evidence/905/RUNTIME_RETEST.json`, `.csdlc/evidence/905/ATTEMPT_REGISTER.json`, and `.csdlc/issues/905/cards/sor.values.json`
- `.csdlc/evidence/932/child-905-exact-head-review-0906d81d.md` and `.csdlc/evidence/932/child-905-typed-review-receipt-0906d81d.json`
- Native terminal receipts retained under `.git/csdlc-v3/local/evidence/{903,904,905}/terminal-receipt.json`
- GitHub PRs #965, #972 and #1004, with their hosted CI results and merge commits recorded above
