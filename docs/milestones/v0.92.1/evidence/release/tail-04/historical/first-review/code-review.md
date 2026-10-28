# Issue #520 Code Correctness Specialist Review

## Review identity

- Candidate: `c24f8fa65ce445b03ce6cd69007307291d78b60c`
- Base: `f0a011a5c59d46c763d669f69a10308b3f870ba4`
- Role: code correctness specialist
- Disposition: **changes requested**
- Review mode: read-only exact-candidate review; no reviewed-checkout or GitHub mutation

## Findings

### P1 — Failed agent removal can durably erase recoverable state while leaving the agent resident

**Evidence:** `adl-runtime-kernel/src/control.rs:3723-3777`; `adl-runtime-kernel/src/control.rs:10743-10788`; `adl-runtime-kernel/src/agent_partial_checkpoint.rs:495-564`; `adl-runtime-kernel/src/agent_partial_checkpoint.rs:1523-1562`; `adl-runtime-kernel/src/agent_partial_checkpoint.rs:1922-1935`

`remove_agent` writes the authoritative checkpoint tombstone before it persists the reduced dynamic-agent roster. The tombstone is durable at `agent_partial_checkpoint.rs:536`; subsequent spool/degraded-state work can fail, and `persist_dynamic_agents` can independently fail because an orientation is missing or because its atomic file write fails. Any failure after the local tombstone has landed returns an error before `*agents = next` and before the in-memory population is removed. The latest-record selector intentionally treats the later tombstone as authoritative, and the existing reopen test proves that the earlier partial is then omitted.

**Trigger scenario:** disk pressure, permissions, an interrupted or failed directory sync, a missing orientation record, or an archive-spool error occurs after the local tombstone write but before the roster commit.

**Impact:** the API reports removal failure and the dynamic roster still contains the agent, but restart/recovery suppresses its most recent checkpoint. This creates cross-store split-brain and silent resident-agent state loss.

**Required correction:** make roster removal and checkpoint tombstoning one recoverable transaction. A durable intent/journal with idempotent recovery is acceptable; simple reordering is acceptable only if failure injection proves that neither boundary can leave an admitted agent with a committed tombstone or a removed agent capable of resurrection. Add failure-injection tests for every persistence boundary and reopen recovery.

**Residual risk:** even after the two named files are coordinated, orientation, population, conversation-session, and migration cleanup remain separate stores. Recovery must specify which durable record is authoritative and reconcile every derived in-memory projection.

### P1 — Admission acknowledges success before the autonomous greeting has durable ownership

**Evidence:** `adl-runtime-kernel/src/control.rs:2002-2051`; `adl-runtime-kernel/src/control.rs:3661-3721`; `adl-runtime-kernel/src/control.rs:5033-5063`

The admission handler persists the agent and returns HTTP success, then launches health refresh and Beacon's greeting in an untracked `tokio::spawn`. The spawned result is discarded. A provider refusal, task panic, process shutdown, or restart after admission therefore loses the greeting. Replaying the same admission cannot recover it because `admit_agent` returns `already_present`, while the handler schedules work only for `status == "admitted"`.

**Trigger scenario:** the runtime is restarted or the provider is temporarily unavailable between the successful admission response and completion of the spawned greeting.

**Impact:** the agent remains correctly admitted but the self-starting agent-to-agent conversation never occurs. The transport can be healthy while the autonomous demo and product behavior stall permanently with no retryable state or surfaced failure.

**Required correction:** persist a greeting/outbox intent as part of admission, drive it with an idempotent retrying worker, and clear it only after a terminal recorded result. Startup and repeated admission must resume pending work. Tests must cover provider failure, task interruption, process reopen, replay, and duplicate suppression.

**Residual risk:** a durable outbox still needs bounded retry/backoff, operator-visible health, and a stable idempotency key shared with the conversation ledger to avoid duplicate welcomes after uncertain completion.

### P2 — One failed health task aborts the remainder of the dynamic-agent health sweep

**Evidence:** `adl-runtime-kernel/src/control.rs:3600-3659`

The join loop uses `while let Some(Ok(...)) = checks.join_next().await`. The first `JoinError` makes the pattern fail and exits the loop. Dropping the still-populated `JoinSet` aborts the remaining checks rather than recording one failed check and continuing.

**Trigger scenario:** any per-agent task panics or is cancelled during a multi-agent refresh.

**Impact:** unrelated agents retain stale health/readiness state; a single model-specific fault cascades across the health sweep and can misroute later work.

**Required correction:** match `Ok`, `Err`, and end-of-set explicitly; record the failed agent/check and continue draining the set. Add a deterministic multi-agent test in which one task fails and all other health projections still update.

**Residual risk:** the task result currently carries no stable agent identifier on join failure. The implementation may need a wrapper that preserves identity even when the inner future panics.

### P1 — The repository-review packet builder can silently assign no product code or tests to specialists

**Evidence:** `adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py:276-332`; `docs/milestones/v0.92.1/evidence/release/tail-04/repo-packet/specialist_assignments.json`

`build_evidence` applies one global score and truncates the repository to 120 entries before deriving lanes. Manifests score 50 while executable code and tests score 20; `assignments_from_evidence` then caps each already-starved lane to 30. The exact #520 packet consequently assigns the same 30 `.csdlc/locks/*.lock` files to code, architecture, security, dependencies, and diagrams, and assigns **zero files** to tests.

**Trigger scenario:** any large repository with more high-scoring manifests/lock-like files than the global evidence cap.

**Impact:** a mandatory internal review can produce apparently complete specialist artifacts without any specialist receiving the changed product implementation. This is a false-negative release gate, not merely a low-quality recommendation.

**Required correction:** classify the complete inventory into lanes before applying per-lane caps; reject a packet when a materially present required lane has an empty or category-invalid assignment; retain each lane's source denominator, selected denominator, exclusions, and digest. Add the exact #520 shape as a regression fixture.

**Residual risk:** any cap remains sampling unless the packet distinguishes full deterministic scans from manually inspected priority subsets. Synthesis must not convert a sampled lane into a whole-repository claim.

### P2 — Proof/install routes bind mutable work to the binary's checkout instead of the invoking worktree

**Evidence:** `csdlc-v3/src/main.rs:290-303`; `csdlc-v3/src/main.rs:566-583`; `csdlc-v3/src/commands/proof.rs:275-310`; `csdlc-v3/src/commands/proof.rs:617-625`; `csdlc-v3/src/commands/proof.rs:1279-1334`; `csdlc-v3/src/commands/proof.rs:1447-1457`

`run_proof_route` discovers the repository from `current_exe`, not from the current/bound worktree or an explicitly validated repository argument. `common_findings` requires the request's `evidence_root` to equal that binary checkout, and the proof/install implementations then write receipts or install the binary below that root. The stable operational binary is intentionally installed in the primary checkout, so invoking one of these routes from an issue worktree either fails against the valid worktree root or directs mutations into the primary checkout.

**Trigger scenario:** run `proof`, `shadow`, `soak`, or `install` from a bound FastWork worktree using the stable `.adl/bin/native-v3/csdlc` binary installed in the primary checkout.

**Impact:** valid issue work is rejected, or evidence/install mutations target the inspection-only primary checkout. The behavior violates worktree ownership and can validate the wrong exact head.

**Required correction:** resolve the invoking repository from the bound operational request/current checkout, then verify branch, worktree registration, exact head, common Git directory, and containment. The executable's path may identify provenance but must not select the mutation root. Add integration tests using one stable binary across a primary checkout and a linked worktree.

**Residual risk:** the proof routes still present themselves as pre-cutover construction surfaces. Remediation must decide whether they are current operational routes or immutable historical compatibility code; leaving a callable mutating half-state is unsafe.

## Complete changed-path denominator

The supplied specialist assignment was unusable, so this review rebuilt the denominator directly from Git using the immutable base and candidate. `git diff --name-only <base>..<candidate> | LC_ALL=C sort` produced 5,481 changed paths with SHA-256 `59c4c5de57d5aac07549a97bf508c00cbc2b5234e63f984a2eece6a8e07acb`.

The production-code denominator was every changed path under `adl/src`, `adl/tools`, `adl-runtime/src`, `adl-runtime-kernel/src`, `csdlc-v2/src`, `csdlc-v3/src`, `tools/aws_remote_validation`, and `infra`. It contains 392 paths with sorted-list SHA-256 `73d624e8e2b094e57473f19d2cefc82357d4f331e5bad8fc971cf1c6f9584fc3`. Group counts are:

| Root | Changed paths |
|---|---:|
| `adl/src` | 41 |
| `adl/tools` | 44 |
| `adl-runtime/src` | 6 |
| `adl-runtime-kernel/src` | 21 |
| `csdlc-v2/src` | 24 |
| `csdlc-v3/src` | 17 |
| `infra/aws` | 133 |
| `infra/gcp` | 102 |
| other selected production roots | 4 |

Every path in that denominator received deterministic path/category classification and complete diff-hazard scans. The 109 changed Rust source paths were scanned for panic/poisoning, asynchronous task, subprocess, filesystem-mutation, synchronization, and unsafe-code boundaries; the 44 `adl/tools` paths were scanned for subprocess, shell, deletion, network, permission, and write boundaries; and all 235 AWS/GCP paths were scanned for public exposure, SSH/TLS/runtime ports, IAM wildcard/pass-role, destructive lifecycle, snapshot/disk, metadata, startup, and secret boundaries. Manual semantic tracing then followed every matching high-risk boundary through its callers, durable state, recovery behavior, and corroborating tests. Tests and manifests were used as corroborating evidence rather than treated as proof by existence. Generated lifecycle/evidence records outside those roots were intentionally left to the records, documentation, and synthesis lanes.

## Validation performed

- `cargo check --locked --all-targets` passed for `adl/Cargo.toml`, `adl-runtime/Cargo.toml`, `adl-runtime-kernel/Cargo.toml`, `csdlc-v3/Cargo.toml`, and `csdlc-v2/Cargo.toml`, using an external review target directory.
- The exact packet assignment was decoded and compared with the complete Git denominator.
- Runtime persistence order, restart selection, task ownership, retry conditions, and existing tests were traced end to end.
- C-SDLC repository-root discovery was traced from CLI dispatch through containment validation and each mutating proof/install operation.

Passing compilation does not resolve any finding above; all are behavioral, failure-boundary, or review-authority defects.

## Assumptions and out-of-scope boundaries

- The candidate and base SHAs are the immutable review authority supplied for #520.
- No live cloud deployment, provider credential, destructive failure injection, or GitHub mutation was performed.
- Dependency provenance, AppSec, documentation prose, test sufficiency, redaction, and release synthesis have separate specialist owners. Cross-lane evidence is cited here only where needed to establish a code behavior.
