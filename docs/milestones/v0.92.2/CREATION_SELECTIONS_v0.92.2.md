# v0.92.2 Creation Selections

These are concrete scope selections for issue creation under WP-01. They do not claim implemented features, successful execution, deployment or publication. Implementation resolves current authority, ownership and exact candidate again. The adopted local-product boundary remains authoritative.

## CodeFriend command and source ownership

Select `adl codefriend` as the local product entrypoint in this repository. Add `adl/src/cli/codefriend_cmd.rs` through the existing module/dispatch pattern in `adl/src/cli/mod.rs`, with help in `adl/src/cli/usage.rs` and cohesive production modules under new `adl/src/codefriend/`. Register library modules through `adl/src/lib.rs` as needed. These are selected new paths, not existing functionality. Split issue owners own their named behavior beneath that namespace; agree exact submodule ownership before parallel edits. Reuse shared Runtime/provider and Memory Palace boundaries, not the historical `runtime_v2/codefriend_adapter_obligations.rs` as a substitute product.

Select macOS and Linux as the qualification environments. Existing Rust/native-memory CI includes both (`.github/workflows/wp11-native-memory-palace.yml`); this establishes an available platform pattern, not CodeFriend support proof. The installed product must pass its applicable actual success/failure journey on both before claiming support. Record OS, architecture, toolchain, installed digest and candidate revision with execution. Windows is outside this selected qualification claim.

## Provider route

Select the existing OpenAI HTTP adapter route in `adl/src/provider/http_family.rs`, which resolves `OPENAI_API_KEY` and the OpenAI Responses endpoint. Operator supplies the approved credential reference/environment; this selection neither inspects credentials nor makes a provider call. Use the shared registered adapter and capability contract, never a CodeFriend-specific client.

The exact registered model, endpoint/profile digest, Runtime/provider revision and environment are execution-bound values: pin them in the run manifest before the first proving call. Do not use a floating model identity in accepted proof. Actual qualification requires a nonzero real generated review run and declared failure evidence; fixture-only execution, absent credentials or a configured-but-unused adapter cannot satisfy it. Paid/live execution retains its separate authorization boundary. This CodeFriend route choice does not reduce #855's five-route lifecycle matrix.

## External Rust fixture

Select `https://github.com/vectordotdev/vector.git` at immutable revision `410da89a0ed42c523143da89fffeb7f6402833e0`. A preexisting local checkout was inspected read-only: origin matched, HEAD matched and `git status --short` was empty. Read Git blobs at the pinned revision, leaving the source checkout unchanged; no download or fixture execution occurred.

The analysis scope is exactly these seven tracked files:

| Path | Blob bytes |
|---|---:|
| `lib/dnsmsg-parser/Cargo.toml` | 388 |
| `lib/dnsmsg-parser/LICENSE` | 1080 |
| `lib/dnsmsg-parser/benches/benches.rs` | 3634 |
| `lib/dnsmsg-parser/src/dns_message.rs` | 3368 |
| `lib/dnsmsg-parser/src/dns_message_parser.rs` | 89969 |
| `lib/dnsmsg-parser/src/ede.rs` | 2957 |
| `lib/dnsmsg-parser/src/lib.rs` | 249 |

Include only root `Cargo.toml` (52,534 bytes), `Cargo.lock` (350,676 bytes) and `LICENSE` (15,922 bytes) as additional dependency/license context, not additional analyzed implementation. The crate declares MIT and includes its own license; root LICENSE is MPL-2.0. Preserve both notices and per-scope provenance rather than claiming one blanket license. Total input ceiling: ten files, 600 KiB; per-file ceiling: 400 KiB. Reject changed revision, extra files or exceeded bounds. Dependencies outside this scope are explicitly external/unknown; no whole-Vector architecture claim is allowed.

Do not build Vector, run its scripts, fetch dependencies or recursively expand the workspace for ingestion/review. Provider token and elapsed limits are declared in the execution run profile before calls, with truncation/partial coverage explicit. Longitudinal proof compares repeated compatible scoped runs; a changed-source scenario must record its separate controlled revision instead of silently modifying this pin.

## PLAT-UTS package

Select a new in-repository Rust library crate `adl-uts/`, package version `0.1.0`, consumed by `adl/Cargo.toml`. No external registry publication or repository extraction is implied. Existing `adl/src/uts.rs` contains `uts.v1` and `uts.v1.1` schema constants/types and uses standalone serde/schemars/collection dependencies; `adl/src/tool_registry.rs` and `adl/src/resident_tool_execution.rs` import its contract. Preserve existing public imports through compatibility reexports where required and retain all actually supported serialization and schema assets. Package version is distinct from UTS schema version. Installation means packaged consumption by the named production Runtime tool-dispatch path; type presence does not establish every proposed v1.1 behavior, and schema compatibility never grants ACC execution authority.

## PLAT-RUST responsibility and ownership hold

Select process-status argument parsing and validation in `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, and `validate_loopback_host`. The production caller is `real_process_status`. The baseline file has 482 lines; parsing holds four separate optional targets, counts them and then reconstructs a target using a repeated conditional chain ending in `expect`.

Complete a cohesive parser under `adl/src/cli/process_cmd/args.rs` and simplify redundant target-selection representation/branching while preserving accepted/rejected argv, repeated-option behavior, error text/order, default host, zero PID/port denial and loopback restrictions. Prove through `adl/tests/cli_smoke/process_status.rs` and focused parser negatives. Preserve syscall/network/status output behavior. Measure before/after parser control flow, duplicated selection logic and recursive source totals. A shorter facade, moved lines or added tests alone is not measurable simplification; demonstrate reduced duplicated target-selection logic without changed observable semantics.

Ownership is **not clear for execution**: the registered worktree ending `.worktrees/adl-process-status-fanout` has a tracked modification to this exact file. A bounded read-only scan checked the selected paths across 332 registered worktrees and found that collision. Preserve its bytes and reconcile its owner before binding/implementation; do not overwrite or assume it abandoned. No live issue identity was inferred from historical source inventories. If reconciliation changes this responsibility, revise the selection explicitly before execution.

## PUB-MEDIUM named brief

Select **“What CodeFriend Beta 1 Is Designed to Prove”**, a complete unpublished article for technically literate skeptical readers. Use the tone/bounded-claims brief in `demos/fixtures/medium_article_writing/v0-89-medium-article-brief.md`, accepted ADR 0025, and current adopted design/atomic completion contracts. Explain evidence-linked findings, isolated perspectives, human-controlled reports, required ADL/external demonstrations and explicit limitations. Describe unexecuted features as design and acceptance obligations; do not imply working Beta delivery.

Deliver complete prose, source/claim check and editorial review under a named issue-local manuscript path. This design-and-limits article has no added CF-PROOF dependency and makes no empirical performance claims. External publication remains excluded. Changing the article into a launch/success report would require current delivery evidence and an explicit scope update.

## PUB-CSDLC author-feedback revision

PUB-CSDLC is an ordinary backlog issue for iterative author-feedback revisions. The operator will provide feedback as the work develops and expects the paper to be rewritten repeatedly; create the issue now and track those revisions normally.

Select one complete author-feedback revision of **The Cognitive Software Development Lifecycle** in its existing canonical private manuscript repository: `COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_DRAFT.md`, `tex/main.tex`, and the claim/citation packet with `refs/sources.bib` remain synchronized where the requested corrections affect them. Do not substitute an empirical update, optional related-work expansion or generic paper packet. The private content is not copied into this public planning document.

Track the current manuscript revision, evolving feedback and explicit dispositions for each revision. Begin the substantive revision when actionable feedback is available, and update the issue-local plan as feedback changes; no immutable upfront author checklist is required. Completion requires the milestone's complete reviewed revised manuscript with synchronized representations/citations and clear disposition of its feedback. An outline, feedback intake record or future-work list cannot close it. Later rewrites remain expected and do not make the completed milestone revision half-work. Submission and external publication remain separate from manuscript revision.
