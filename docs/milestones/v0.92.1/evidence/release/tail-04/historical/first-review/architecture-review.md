# Issue #520 Architecture Specialist Review

## Review identity

- Candidate: `c24f8fa65ce445b03ce6cd69007307291d78b60c`
- Base: `f0a011a5c59d46c763d669f69a10308b3f870ba4`
- Role: architecture specialist
- Disposition: **changes requested**
- Review mode: read-only exact-candidate review; no reviewed-checkout or GitHub mutation

## Findings

### P1 — Dynamic-agent removal has no atomic authority boundary across roster and checkpoint stores

**Evidence:** `adl-runtime-kernel/src/control.rs:3723-3791`; `adl-runtime-kernel/src/control.rs:10743-10788`; `adl-runtime-kernel/src/agent_partial_checkpoint.rs:495-564`; `adl-runtime-kernel/src/agent_partial_checkpoint.rs:1523-1562`

The roster file and checkpoint/tombstone store independently encode whether an agent exists, but there is no transaction record or recovery reconciliation between them. Removal commits the tombstone first and the roster second. The tombstone is explicitly authoritative for preventing resurrection; the roster remains authoritative for admission and reconstruction. A crash or second-store failure between those commits therefore produces two incompatible durable truths.

**Scenario and impact:** after the tombstone lands, roster persistence fails. The operation reports failure and the agent remains in the roster, while checkpoint recovery treats it as removed and discards prior state. The runtime can restart into a resident identity with irrecoverably suppressed continuity.

**Required correction:** define one authoritative dynamic-agent lifecycle state machine and a durable transaction/recovery protocol spanning declaration, orientation, checkpoint/tombstone, and derived in-memory projections. Prove both crash directions and idempotent restart reconciliation.

**Residual risk:** merely reversing writes changes the failure from state loss to resurrection. Atomicity/recovery, not preferred ordering alone, is the architectural requirement.

### P1 — Admission and autonomous A2A initiation cross an unowned asynchronous boundary

**Evidence:** `adl-runtime-kernel/src/control.rs:2002-2051`; `adl-runtime-kernel/src/control.rs:3661-3721`; `adl-runtime-kernel/src/control.rs:5033-5063`

Admission is a durable state transition, but the required Beacon greeting is an ephemeral spawned future whose result is discarded. There is no outbox, pending state, startup reconciliation, retry owner, or failure projection. Repeated admission returns `already_present` and cannot re-enter the scheduling branch.

**Scenario and impact:** a restart or transient provider fault after admission acknowledgement permanently suppresses the expected autonomous initiation. The transport remains functional on explicit operator command, but the self-running system fails as an integrated architecture.

**Required correction:** treat admission-triggered conversation as durable workflow state with an idempotent outbox, retry policy, terminal receipt, startup recovery, and health projection.

**Residual risk:** exactly-once delivery is unrealistic across provider boundaries. The contract should promise durable at-least-once initiation with ledger-level deduplication and observable terminal disposition.

### P1 — Current operator guidance contradicts the completed C-SDLC v3 authority cutover

**Evidence:** `AGENTS.md:3-33`; `csdlc-v3/AGENTS.md:1-19`; `docs/csdlc-v3/TOOLING_CHANGEOVER_NOTICE.md:3-17`; `csdlc-v3/src/main.rs:290-294`; `csdlc-v3/src/main.rs:586-601`; `csdlc-v3/src/lib.rs:1-8`; `csdlc-v3/src/commands/local/mod.rs:1-6`; `csdlc-v3/Cargo.toml:1-6`; `csdlc-v3/tests/command_manifest.rs:219-249`

The root and nested agent contracts say PR #591 completed the atomic cutover and v3 is now operational authority. The same nested contract sends operators to a changeover notice that says v2 remains live and v3 must not bind, publish, finish, clean, or mutate GitHub. CLI help for remote and terminal commands repeats the pre-cutover authority denial, package/module documentation still calls v3 non-authoritative, and tests require the obsolete help string. Root `AGENTS.md:28-33` also retains a live conditional instruction to keep using v2 if pre-cutover proof is incomplete despite the preceding declaration that the cutover is complete.

**Scenario and impact:** an operator or agent follows the referenced notice or CLI help rather than the opening paragraph. It routes new work to retired v2 surfaces, refuses valid v3 work, or attempts an unauthorized transition workaround. The authority architecture no longer has one coherent entrypoint.

**Required correction:** atomically update current operator docs, CLI help, package/module contracts, manifest metadata, and assertions to the post-cutover state. Preserve pre-cutover records only under an unmistakably historical evidence label and remove current cross-links that present them as routing instructions.

**Residual risk:** prose repair alone cannot prevent recurrence. A fitness check must compare the canonical selector/cutover fact with every current operator-facing help and guidance surface.

### P1 — The internal-review architecture is non-proving because assignment is globally truncated before lane routing

**Evidence:** `adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py:276-332`; `docs/milestones/v0.92.1/evidence/release/tail-04/repo-packet/repo_inventory.json`; `docs/milestones/v0.92.1/evidence/release/tail-04/repo-packet/specialist_assignments.json`

The packet architecture computes one score over the whole repository, takes the first 120 entries, derives specialist lanes only afterward, and then caps each lane. For #520 this assigned identical lock records to five unrelated specialist roles and no files at all to tests, despite a 29,155-file repository inventory and material code/test surfaces.

**Scenario and impact:** the release process can aggregate multiple polished specialist reports whose underlying assignments never covered the product. This creates a structurally false review-completeness claim and allows material defects to survive a mandatory gate.

**Required correction:** make full inventory and lane classification the first-class authority; produce per-lane denominators and digests; validate category compatibility; fail closed on materially empty lanes; distinguish deterministic full scans from explicitly sampled manual review.

**Residual risk:** even valid automatic classification cannot establish semantic coverage. Review artifacts must retain exclusions and a reviewer-owned expansion path for cross-cutting files.

### P2 — Proof/install checkout identity is coupled to executable location rather than issue ownership

**Evidence:** `csdlc-v3/src/main.rs:290-303`; `csdlc-v3/src/main.rs:566-583`; `csdlc-v3/src/commands/proof.rs:275-310`; `csdlc-v3/src/commands/proof.rs:1279-1334`; `csdlc-v3/src/commands/proof.rs:1391-1409`

Proof routing discovers a repository by walking upward from the running binary. The stable binary is installed in the primary checkout, while issue authority belongs to a bound worktree. Proof validation then requires the request root to match the binary checkout and exact-head validation runs there. This makes deployment topology, rather than typed issue ownership, choose the state and mutation boundary.

**Scenario and impact:** the same stable binary is invoked from a linked issue worktree. The legitimate worktree is rejected or proof/install artifacts are evaluated and written against the primary checkout's head. This breaks the clean-main/issue-worktree isolation invariant.

**Required correction:** select the repository through authenticated bound-worktree identity and use executable location only for binary provenance. Validate the shared Git directory plus exact branch/worktree registration before any write.

**Residual risk:** construction-only and operational responsibilities are interleaved in one callable binary. The routes need an explicit current lifecycle disposition so dormant transition code cannot remain an accidental mutation surface.

## Architecture map and ownership boundaries

| Component | Owned state / responsibility | Boundary reviewed |
|---|---|---|
| Runtime control service | admission/removal orchestration, HTTP/WS control, health refresh | durable roster, asynchronous workers, failure propagation |
| Dynamic-agent store | declarations plus orientation snapshots | atomic file replacement, recovery authority |
| Partial checkpoint store | partials, tombstones, local and S3-spool projections | removal authority, restart selection, cross-store ordering |
| Conversation/A2A subsystem | initiation acceptance, dispatch, terminal events | idempotency, retry ownership, autonomous triggers |
| Provider/Ollama boundary | model verification and inference | per-agent isolation, failure containment |
| C-SDLC v3 CLI | typed local/remote/proof/terminal routing | authority selection, checkout identity, mutating root |
| Review packet builder | repository inventory and specialist assignment | completeness denominator, lane selection, release-gate truth |
| AWS/GCP infrastructure | runtime/guardian and GPU/model deployment | lifecycle, identity, networking, persistence; no new code finding beyond other specialist lanes |

## Complete architecture denominator

The packet's architecture assignment contained only 30 `.csdlc/locks/*.lock` paths and was rejected as category-invalid. The replacement denominator was rebuilt from the immutable Git diff.

- Complete changed tree: 5,481 sorted paths; SHA-256 `59c4c5de57d5aac07549a97bf508c00cbc2b5234e63f984a2eece6a8e07acb`.
- Architecture denominator: every changed path under `adl/src`, `adl/tools`, `adl-runtime`, `adl-runtime-kernel`, `csdlc-v2`, `csdlc-v3`, `tools/aws_remote_validation`, `infra`, `.github/workflows`, `docs/csdlc-v3`, plus root `AGENTS.md`; 482 paths.
- Production-code subset: 392 paths; SHA-256 `73d624e8e2b094e57473f19d2cefc82357d4f331e5bad8fc971cf1c6f9584fc3`.

All 482 paths were deterministically classified by subsystem and boundary and received complete category-specific diff scans. The 109 changed Rust source paths were scanned for state ownership, synchronization, task/process creation, failure propagation, persistence, and mutation boundaries; the 235 cloud paths were scanned for resource lifecycle, networking/exposure, IAM, disk/snapshot, bootstrap, metadata, and secret boundaries; scripts and workflows were scanned for subprocess, shell, deletion, network, write, permission, and release-routing boundaries. Manual semantic tracing then followed every matching high-risk boundary through its callers, state owners, recovery behavior, tests, and operator contracts. Generated lifecycle/evidence records were excluded from architecture ownership except where they were the direct output proving the packet-builder defect.

## Candidate ADRs

1. **Dynamic-agent lifecycle transaction and recovery authority.** Define one lifecycle state machine, durable intent format, commit order, reconciliation owner, and crash semantics across roster, orientation, checkpoint/tombstone, and in-memory projections.
2. **Durable runtime outbox for autonomous agent work.** Record at-least-once scheduling, idempotency, retry/backoff, startup recovery, and terminal projection for admission-triggered A2A initiation.
3. **Repository identity is derived from bound issue context.** Prohibit executable-location-derived mutation roots; reserve binary path for provenance only.

## Candidate diagram tasks

1. Diagram the admission path from authenticated request through roster/orientation commit, durable greeting outbox, Beacon dispatch, conversation ledger, provider, terminal receipt, and retry/startup recovery.
2. Diagram dynamic-agent removal as a state machine spanning transaction intent, tombstone, roster/orientation update, derived population/session cleanup, crash points, and reconciliation.
3. Diagram the internal-review evidence flow from complete Git inventory through per-lane denominators, specialist outputs, synthesis, finding-to-issue routing, and release decision.

## Candidate architecture fitness functions

1. Failure-inject every durable removal boundary and assert that reopen yields exactly one coherent state: resident with recoverable checkpoint, or absent with authoritative tombstone.
2. Interrupt admission after acknowledgement and at each greeting phase; assert restart resumes one idempotent initiation to terminal disposition.
3. Force one health task failure among multiple agents and assert all other checks drain and update.
4. Reject current operator/help text that identifies v2 as default or v3 as pre-cutover when the canonical selector proves post-cutover authority.
5. Reject review packets when a materially present required lane is empty, category-invalid, lacks a full denominator/digest, or conflates sampling with complete coverage.
6. Invoke one stable C-SDLC binary from primary and linked issue worktrees and assert all writes bind to the authenticated issue worktree and exact head.

## Validation performed

- Compiled all targets under the five Rust manifests (`adl`, `adl-runtime`, `adl-runtime-kernel`, `csdlc-v3`, and `csdlc-v2`) with locked dependencies and an external target directory; all passed.
- Traced durable write order and restart selection through the roster, orientation, checkpoint, tombstone, and derived population implementations.
- Traced autonomous initiation from HTTP admission to spawned work, provider dispatch, replay behavior, and terminal event emission.
- Compared canonical post-cutover contracts with every cited current CLI/help/module/test surface.
- Recomputed packet coverage directly from the exact Git pair and compared it with the emitted assignments.

Compilation success is non-proving for crash consistency, durable asynchronous ownership, authority coherence, and review-denominator completeness.

## Assumptions and residual review risk

- The supplied base and candidate are the exact review authority.
- No cloud deployment, destructive crash test, credentialed provider action, or GitHub mutation was performed.
- Infrastructure was reviewed statically. Live AWS/GCP quota, timing, billing, and eventual-consistency behavior remain for the bounded proof lanes.
- The candidate is a very large release-tail diff. Deterministic inventory, full source-module coverage, boundary tracing, and targeted high-risk review reduce omission risk but cannot replace the missing valid packet assignment; synthesis must record that packet defect rather than claiming the generated assignments proved completeness.
