# Sprint 2 execution preparation — ADL #1299

> Current execution and acceptance status is in [EXECUTION_ACCEPTANCE.md](EXECUTION_ACCEPTANCE.md). The preparation sections below preserve the earlier planning record and are not current delivery status.

Prepared by Worker #29 on 2026-10-19 at Daniel's request: “get sprint 2 ready to execute, get #12 to review when done”. Preparation and Planning #12 review are complete. The operator subsequently authorized execution with Worker #30; CodeFriend #120 assessment is now in progress.

## Readiness decision

**READY FOR SEQUENTIAL EXECUTION. Planning #12 final preparation review: PASS.** Both child issues are bound to the selected FastWork worktrees using verified isolated package `12237907988`, which includes delivered #299 and #304. Reviewed SIP/STP/SPP/VPP/SOR amendments are applied and native validation passes. Read `evidence/native-final-readback.json` for exact checkouts, heads and active `.csdlc/v3/issues/.../cards` paths. Older `.csdlc/issues` compatibility copies are not the current semantic projections.

Both SRPs have concrete issue-specific prompt fields and `review_results: not_run`; the generated SRP templates retain identity/link and future result placeholders listed in the readback. Planning #12 accepted these as a documented nonblocking template follow-up. This preparation review does not establish product acceptance. Assessment execution status is recorded below.

The operator has authorized working with Worker #30 to execute Sprint 2 after preparation is ready. Dispatch is authorized after the completed final review; security #9 remains sequentially dependent on accepted #120 outputs. See [LAUNCH.md](LAUNCH.md) and [VALIDATION_CONTRACT.md](VALIDATION_CONTRACT.md).

## Source and baseline

Use the reviewed ADL #1296 package at commit `1c5f164ca8d2eac35b9d1b16a98e136f5021f4b7` and merge `74c5431062974e410e41d315088beb68b18a63d1`, retained in the #1296 worktree. Its EXECUTION_PLAN, issue index, reorganization and feature specifications define this packet. Live GitHub issue bodies for ADL #1299, CodeFriend #120 and enterprise-security #9 were read on 2026-10-19. C-SDLC #299 was OPEN at that observation.

The native binder initially selected primary HEAD `2f26936b9c0ec582471a3f8fbebcce31b471512e`. After Planning #12 identified the stale base, this issue worktree was fast-forwarded to observed origin/main `74c5431062974e410e41d315088beb68b18a63d1` and native bind was refreshed successfully. The primary checkout was not changed. Native publication metadata now explicitly describes final Sprint 2 outcome acceptance and prohibits publication for preparation alone.

## Sprint Execution Packet

Mode: **sequential**, with one accountable execution owner per issue. Planning #13 (formerly Worker #29) coordinates the umbrella. Worker #30 is the sole execution owner for the children and is actively drafting the #120 assessment. Planning #12 completed preparation review. Security #9 remains dependent on accepted #120 inputs.

| Order | Issue | Outcome | Required entry evidence | Write set |
| --- | --- | --- | --- | --- |
| 1 | agent-logic/codefriend#120 / CF-B1-01 | Representative actual outputs and journeys produce an operator-agreed Beta 1 denominator and prioritized gap register | Accepted ADL #1296; delivered C-SDLC #299; current admitted tooling; selected candidate and corpus agreed during assessment | CodeFriend assessment documents and evidence; no product fixes |
| 2 | agent-logic/agent-logic-enterprise-security#9 / WP-S1 | Define and enforce CodeFriend trust boundaries against supported Runtime interfaces | Accepted #120 outputs; retained RD-09 provenance; authenticated Runtime interface artifact and consumer identities | Enterprise adapter source, tests, conformance and contract docs only |

Security #9 is not a docs-only contract: its acceptance explicitly requires enforcement and actual installed consumer cases. Missing Runtime primitives become bounded prerequisites. Runtime v4, GHB and broad governance stay in unactivated v0.93.3.

### CodeFriend #120 execution brief

1. Verify #299 delivered and its admitted commands/manuals before relying on the lifecycle simplifications. Bind one issue worktree at the operator-selected destination; assign its sole owner and create an issue goal naming Sprint #1299 and CodeFriend #120.
2. Inventory reusable actual report/export artifacts and end-to-end journey evidence. Record agent, gateway and website candidate identities separately, repository revisions, platform and configuration. Agree the representative corpus and measurable scenario denominator with Daniel; existing fixture or extraction proof cannot replace this decision.
3. Assess readable reports/exports, login, repository consent, pairing, first review, results and recovery. Each scenario row records expected/actual behavior, provenance, evidence and pass/fail/blocked/skipped state. Do not invent defects where evidence is absent.
4. Produce a prioritized gap register with reproduction, impact, priority, acceptance measure and mapping to existing #121/#123/#125/#126. This assessment does not implement those fixes or create duplicate issues.
5. Obtain operator acceptance of the denominator/priorities and independent exact-head review. Retain failed and unexercised cases. Publish only when actual issue acceptance is supported.

Suggested artifact set: `docs/codefriend/beta1/assessment-120/` containing candidate/corpus inventory, scenario evidence matrix, gap register and acceptance decision. Specific filenames may be chosen by the assigned owner; preserve these four distinct outcomes.

### Enterprise-security #9 execution brief

1. Consume the accepted #120 denominator and exact consumer identities. Verify the retained RD-09 producer/consumer contract and authenticated `adl-runtime-policy-interface` artifact, without rebuilding or changing Runtime.
2. Map principal, tenant, repository, service and paired-agent boundaries to actual enforcement points. Runtime owns final enforcement; the optional enterprise adapter can restrict but cannot add baseline grants.
3. Preserve least privilege, per-message checks, revocation, secret redaction, audit integrity, tenant isolation, deletion and fail-closed consumer behavior. Explicit negative cases include implicit internal trust and treating a connection as permission.
4. Implement only needed adapter-side changes. Missing supported Runtime functionality is a bounded dependency to route, never permission to widen to Runtime v4 or silently drop an invariant.
5. Retain exact source/artifact/configuration identities, actual scenario counts, failures and review dispositions. Independently review changed code and proof before publication.

## Validation plan and budgets

- Preparation: deterministic, small local JSON/identity/card checks and native validate/status; no product build or provider run. Planning #12 reviews completeness, dependency truth and launch boundaries.
- #120: live issue specifies `planning_contract`; the planning graph labels its evidence `product_behavior`. Use planning-contract checks for the assessment register and actual product-behavior evidence for every exercised journey claim. These are distinct proof roles, not permission to replace actual outputs with documents. Estimated validation: 1,800 seconds / 6,000 tokens, low confidence, excluding unavailable inputs and human review. No goal budget is set by this estimate.
- #9: `installed_integration`, deterministic local adapter tests and bounded independently installed consumer scenarios against authenticated artifacts. Start from `python3 tools/validate_rd09.py --runtime-interface ARCHIVE --sha256 DIGEST`, whose existing denominator is four unit tests and six conformance cases. Add only required #9 cases; the prior denominator is a starting baseline, not proof of all new invariants. Estimated validation: 1,800 seconds / 6,000 tokens, low confidence. Replan after the accepted #120 contract defines the actual denominator.
- Required unrun/failed cases block acceptance; no source or fixture PASS establishes installed or release qualification. Reuse valid proof and avoid broad builds, repeated reviews, API loops and paid execution.

Declared native validators currently contain only `git diff --check`, explicitly labeled hygiene, not acceptance proof. [VALIDATION_CONTRACT.md](VALIDATION_CONTRACT.md) specifies exact entry inputs, commands, checks, platform restrictions and evidence. Planning #12 accepted registration as an explicit execution setup step before native proof/publication; new wrapper code is not required during preparation. No native product proof or publication has been invoked.

## Remaining gates

- Final Planning #12 review passed in turn `01a157e3-f5c9-7ea1-9fdf-23a89418b996`; no blocking findings.
- Worker #30 has been dispatched. The operator approved the nine assessment areas and continuing work; the fixed 27-case proposal is superseded. Environment checks and repository checks are recorded separately. Retained evidence can be assessed now; exact current-candidate acceptance still requires identified candidate and corpus revisions.
- New templates for all 25 document types are being designed. Continue assessment and findings now. Current-template output observations are provisional; before launch, identify the final template versions and repeat affected document/output checks. Template design is not a blocker to the current assessment.
- Security #9 starts after accepted #120 outputs and verified RD-09/interface inputs. Its retained installed artifact is Linux x86_64; macOS source proof cannot replace those installed scenarios.
- Register concrete validators during execution setup before native proof/publication, as accepted by #12. The current git-diff validator is hygiene only.
- #304 is merged at `7429dbacf6a5442d92762f291c0c27f273175edb`, native finish completed, and the isolated owner is installed and verified. FastWork is a caller-selected destination, not a placement policy. The operator-approved one-time bootstrap and installation receipts are retained; shared binaries were not replaced.

## Readiness evidence and review instructions

`evidence/` contains exact preparation, rebuild and validation results, source plans, complete-values proposals, dependency decisions and local readiness findings. The child native plans are retained under each primary repository's resolved Git metadata. This packet's copies are review evidence, not replacement lifecycle authority.

Final review is limited to the admitted-tool identity, native application/validation readbacks, correct active projection paths, and the explicitly retained SRP placeholders. Prior plan and validation-contract content review is already accepted. Return actionable findings without rerunning product proof.

Review path: `REVIEW.md` (actual Planning #12 findings and dispositions). Activity path: `ACTIVITY.md`. Sprint-end implementation review remains separate: review changed code, tests and security behavior, then synthesize coverage and residual findings. This preparation does not close #1299, #120 or #9 and must not use an umbrella-closing PR as a preparation checkpoint.

## Resumption contract

After final review, use the retained admitted executable and paired manual with the existing bound child checkouts. Assign one owner, confirm current state, create its issue-bound goal, and perform the execution setup in VALIDATION_CONTRACT.md before implementation proof. Record exact candidate and accepted inputs. Do not replay the defective uncertain-issue-creation recovery route; no new issue is needed for these three identities.

No deployment, provider/cloud spend, Runtime restart, shared binary replacement, release acceptance or v0.93.3 activation is authorized here. Milestone-assignment administration remains outside this preparation scope.
