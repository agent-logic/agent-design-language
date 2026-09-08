# v0.92.1 Release-tail Gap Analysis

Candidate: `af5f8036ab7a0619751ab55fa9bd4891f377cd9d`

Captured-input digest: `cb71568f0a14504e79bbe210c726f6a860b9adf44f00ac23efe75213c2663f7b`

Canonical projection digest: `a1b4647e3ad98ca68a643fc567f0d22fc6cae5d8e7d0df6eed71b99d5362a4f0`

## Findings

- **P2 consolidated-live-spec-sync-debt** — Live criteria for WP-01, GCP-E, HOT-01, OBS-B are equivalent or stronger expansions of the spec, except OBS-B moves backlog authority to canonical planning and adds no-mock proof; synchronize the records. Evidence: https://github.com/agent-logic/agent-design-language/issues/480#issue-body, f9a7235866ee1a2565fe18a7399b51b2704c37dae4ed875cfec987c8050e5532, 96ab626bad8ef94a33c1d7211a98d2786e94590998ee2cbad6180124b2f33cd5, https://github.com/agent-logic/agent-design-language/issues/494#issue-body, e4a5840ac93ada0212c84353af8527b00cc38d710ec5f0bb14bbc50cf6480851, 25aeef825468013dbd959e1ebd443f9c2552a1ef526814a42cc05c2f075a9286, https://github.com/agent-logic/agent-design-language/issues/510#issue-body, fe073f4c536507594f21ea5206be6ef683dcbcbb9034baa06d129b3453df06ca, 5a236113e049937ac0fd466b425d43acafabd891c2835cde80b197d55993a1e7, https://github.com/agent-logic/agent-design-language/issues/512#issue-body, 80be51e3a6ae692a23386c52cfcd8aed130b058c398c55659d1d724a79eaaa5d, 759236137cc551e220664b5528b0850882fddd4e5c1df4f1158b01d582ee02ba. Owner: release planning maintainers. Disposition: follow_up.
- **P1 issue-721-v3-standalone-parity-gap** — #721 is incorrectly closed by JSON-only PR #722, whose body says it does not fix the issue. At af5f8036 and current main, v3 cannot compile without csdlc-v2 and still reads the v2 selector. Full standalone v3 parity remains release-blocking. Evidence: https://github.com/agent-logic/agent-design-language/issues/721, 4ada86fd4285faf82c9931f369d5e2d9deeae392760649fd03b7a30994fc818d, https://github.com/agent-logic/agent-design-language/pull/722, 73b1821851fba12d3a70c0baec403b65fbd4eb358cb2511b0eb37e0850ad6455, 1735a38b2048021ec231a5e81e47e84adeca57c86526b4e3a563ed2e3aac6718, {"path"=>".csdlc/evidence/516/no-v2-canary-af5f8036.json", "sha256"=>"4bddf1edfc51c004fb0b4ac04ffed7f4d3ff133242d3ba31c25e8ccf90e48ad7"}, csdlc-v3/Cargo.toml, csdlc-v3/src/authority.rs, csdlc-v3/src/commands/remote/mod.rs. Owner: issue #721. Disposition: open_incorrectly_closed_owner.
- **P1 issue-487-semantic-implementation-gap** — AWS-D has 3 audit-confirmed unmet implementation criteria. Evidence: infra/aws/account-foundation/main.tf, .csdlc/issues/487/cards/sor.md. Owner: issue #487. Disposition: open.
- **P1 issue-489-semantic-implementation-gap** — AWS-F has 1 audit-confirmed unmet implementation criteria. Evidence: infra/aws/runtime/private-node/main.tf, .csdlc/issues/489/cards/sor.md. Owner: issue #489. Disposition: open.
- **P1 issue-491-semantic-implementation-gap** — GCP-B has 1 audit-confirmed unmet implementation criteria. Evidence: infra/gcp/bootstrap/main.tf, .csdlc/issues/491/cards/sor.md. Owner: issue #491. Disposition: open.
- **P1 issue-493-semantic-implementation-gap** — GCP-D has 1 audit-confirmed unmet implementation criteria. Evidence: infra/gcp/platform/main.tf, .csdlc/issues/493/cards/sor.md. Owner: issue #493. Disposition: open.
- **P2 semantic-criterion-proof-gaps** — Two GCP-B criteria lack live recovery/cleanup proof. Evidence: infra/gcp/bootstrap/main.tf, .csdlc/issues/491/cards/sor.md. Owner: issue #491. Disposition: follow_up.
- **P2 accepted-semantic-amendments** — Explicit live acceptance/absorption/sequencing amendments replace the listed original criteria. Evidence: github:issue-483:closed, github:issue-497:checked-live-acceptance, github:issue-624:closed-sidecar, github:issue-507:closed, github:issue-511:closed, github:issue-512:closed. Owner: release operator. Disposition: accepted.
- **P2 consolidated-retained-successor-proof-debt** — Some retained predecessors lack a successor with observed merged execution evidence. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/188/cards/stp.md, docs/milestones/v0.92.1/planned-issue-packets/issues/188/cards/stp.md, docs/milestones/v0.92.1/planned-issue-packets/issues/190/cards/stp.md, docs/milestones/v0.92.1/planned-issue-packets/issues/189/cards/stp.md. Owner: release evidence maintainers. Disposition: follow_up.
- **P2 consolidated-owned-path-resolution-proof-debt** — Shared paths lack explicit owner sign-off; no final-content requirement loss was demonstrated. Evidence: infra/aws/account-foundation, infra/aws/runtime, csdlc-v3/src/lib.rs. Owner: release evidence maintainers. Disposition: follow_up.

## Denominator

Execution roots: 35; release-tail stages: 11; retained predecessors: 39; backlog dispositions: 2; acceptance rows: 397.

| Planned ID | Issue | Head revision | Merge revision | Ancestry | Disposition |
|---|---:|---|---|---|---|
| WP-01 | #480 | 23856abbc7cde90dd9d1c6467dd6c61aca1bc274 | 001c270beda2b35b60e0be04f3c3bd331a156c48 | ancestor | observed_execution_evidence |
| CORP-A | #482 | 2070d1b4ff269c2571a2077ae00d9f7fbb0ac67c | e2c1d1649b0c930a5a1254575a07ef2a4496d48d | ancestor | observed_execution_evidence |
| CORP-B | #483 | a0fae2fca6d802ae3f7ab987cc81c0bd36dd5239 | 4a0b49c0071bacdaab19d6d9eb8c44380beb51be | ancestor | observed_execution_evidence |
| AWS-A | #484 | e9fdf5b07bdcbde235511c52c40fb8c626cc95a7 | e5f30c60c68a60d43f51c70b4615065197a34404 | ancestor | observed_execution_evidence |
| AWS-B | #485 | 2a5d25239853499b6ac73b37d968d2b97e75a586 | a71d699d52831b32bb68ed9c7c7e837925949de4 | ancestor | observed_execution_evidence |
| AWS-C | #486 | cfd5f0edabfde2e380a5534d619e7832f484bb8c | 1964b2e1f6e24a9dcb5788394502a2421300751a | ancestor | observed_execution_evidence |
| AWS-D | #487 | 79be6b5b0327be752817197738887d25335e71a9 | 1d31016a8df3cf07a4c3f2e6acd2694bd10570c2 | ancestor | observed_execution_evidence |
| AWS-E | #488 | 4e904b6629ff3060094dbef3613388e6e5245b8d | a6b404cd6e74d7528745325036ceb1a85fd47bd2 | ancestor | observed_execution_evidence |
| AWS-F | #489 | 485b4197908231bb2065e1e29c7c5013536e1975 | 69ba35e066d1389a9f194659acb066a7dca82a40 | ancestor | observed_execution_evidence |
| GCP-A | #490 | f0be8c8d1a2f12df8b2d8169997583dd66a9a521 | daa05de9332c82e3f9f2191975ef95b0ed4e211d | ancestor | observed_execution_evidence |
| GCP-B | #491 | 695ca0f6cec62357349390afda3952e39cd92337 | 75ee9e6b2888a81b355d1fb496b488329a4c7d30 | ancestor | observed_execution_evidence |
| GCP-C | #492 | 179fbc9fb2b1affc68577b8e94b66bc5ac5c49aa | b9a98710e2a0a50565c3835386f7f6a348a26eae | ancestor | observed_execution_evidence |
| GCP-D | #493 | d5b1584bb55e92974e6d3481b59d2e28a17db441 | c0bf217934508d6dbc70d78633e6a95d5ddd9d06 | ancestor | observed_execution_evidence |
| GCP-E | #494 | de959c6263f671fa8fe1df851ea6ae1d25686831 | dc08b5abf10682ed9ace5deefd0e1389ea6899b6 | ancestor | observed_execution_evidence |
| XCL-01 | #495 | 6177249dfb46fe3cf95fbcc996469517928f525d | c78c60f5a45a87a96159d4910a831b69b62b042c | ancestor | observed_execution_evidence |
| AWS-G | #496 | 59a3d0bd106f8bdd0def53dbe1564667ee6adac4 | 83077ca029d52c9d613ed5a373da30f1dd42d9b3 | ancestor | observed_execution_evidence |
| CORP-C | #497 | none | none | not_applicable_absorbed | satisfied_recordless_acceptance |
| CORP-D | #498 | dc17d8c9ccdef3b65f3d2f7371dd3c2b8f48c7c8 | c51c8c7a8b51395986af8185f6e6ca2edaf4f435 | ancestor | observed_execution_evidence |
| RUST-01 | #499 | 940c42d246be5d54f34b7b300526a644cc881580 | e986de6d06aacd385de93dd033def77a718c1581 | ancestor | observed_execution_evidence |
| V3-A | #500 | d02f90008acadcc10df048b7f089cc4b98ef608f | 1dddcce35d061bc128c2431b4f31cf09e0f4d435 | ancestor | observed_execution_evidence |
| V3-B | #501 | 9056f19245f93bc9efa3b55561671a8f002c6536 | 1972aa47bd7047b8594a03bf770fb92f7fb63d51 | ancestor | observed_execution_evidence |
| V3-C | #502 | ed6f01c1e33b8057142491fca3028641ce5efc74 | 76de907734ab69efe00b5bc0bf24f066002d0131 | ancestor | observed_execution_evidence |
| V3-D | #503 | 974abc520454690f0b392162b9ced783e8584017 | 5692d95ee6e4ee632833be348fa5601ddccbca1a | ancestor | observed_execution_evidence |
| V3-E | #504 | 9ef650bc174a81849ffb09ae4d21b699fee1368d | f68ca996541b8825090261fd70845bf1c406b410 | ancestor | observed_execution_evidence |
| V3-F | #505 | 74ddb31702482172eea4ba3d74700536eab32e49 | d3f98ed005cf44c359ac482e271e6b8634e93f23 | ancestor | observed_execution_evidence |
| DRT-A | #506 | 4676aef4189376b3f64d17efdb717732274e3240 | badcf9067da6eb46fc9f59e9da8b11a41e2f24f6 | ancestor | observed_execution_evidence |
| DRT-B | #507 | cca85d6fd4976e2b2358d0d130b8291a88cb1c95 | d022d6c198669bcbc10cd98bee4d7c8520f9c4d4 | ancestor | observed_execution_evidence |
| DRT-C | #508 | cca4f7f675241a0a473b38006c8be2eb95165028 | a1c440cc7b1e3708961680c802585d0b80e2263f | ancestor | observed_execution_evidence |
| DRT-D | #509 | c89a584e53f152ed499a5d56d479296590044714 | 5a1109ffa795d411e0e15cdfd29adf68e2c2d953 | ancestor | observed_execution_evidence |
| HOT-01 | #510 | fac5eaa63a82eaf50fe455df14cc22ebb08a2678 | 000fb7beb5fe4107e3e80d5de9183be224716a6d | ancestor | observed_execution_evidence |
| OBS-A | #511 | none | none | not_applicable_absorbed | satisfied_by_captured_absorption |
| OBS-B | #512 | e8a0e0b9bb6a18687a5a2dc9e85b8130bbb182b4 | af5f8036ab7a0619751ab55fa9bd4891f377cd9d | ancestor | observed_execution_evidence |
| DEC-01 | #513 | 489bc0f3af68b059e5c26ff22600d945b3d21db8 | 5bc84a0f27a522b6d500551d64f8d12dc2357427 | ancestor | observed_execution_evidence |
| PROV-A | #514 | 1b0fd87496bb09200a5cb1bbb0529e8730be1b20 | 18f1c76667dc6913c2553b53228e73e8de9d11c9 | ancestor | observed_execution_evidence |
| PROV-B | #515 | 9e6a8bd104d79f77edc4460ee5424cea83ef9cdc | 17b883e93abff7a155cd783a3d76f52ba2eabbf2 | ancestor | observed_execution_evidence |

### Release-tail lifecycle denominator

| Planned ID | Issue | Observed state | Expected lifecycle | Gate role |
|---|---:|---|---|---|
| INT-01 | #516 | open | active_admission_work | denominator_only_not_execution_root |
| TAIL-01 | #517 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-02 | #518 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-03 | #519 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-04 | #520 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-05 | #521 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-06 | #522 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-07 | #523 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-08 | #524 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-09 | #525 | open | future_serial_stage | denominator_only_not_execution_root |
| TAIL-10 | #526 | open | future_serial_stage | denominator_only_not_execution_root |

## Backlog and retained authority

- #84: `4c0b8a4cec3741f4ebb145864fcdf02cfccf62b352e6a8a900c8ebc97385b59a`
- #251: `4113989e3d613b8044e49c08e7ebc26be4be346fdb19db14695ec5f2b9fbcb40`
- Retained predecessor packets are indexed with SHA-256 digests in `gap_analysis_report.json`.

## Decision

**BLOCKED**

This is an admission decision only; it is not release approval.

## Revision history

- Historical result at `e68c666803185c348140547e243bc3922b6a571c`: 2 P1 product blockers (#497, #721), 4 consolidated P2 proof-debt findings, and 2 routed backlog entries. Superseded after direct #497 recordless-acceptance evidence and final review.
- Revised result: 5 P1 product blocker, 5 P2 proof-debt findings; backlog #84/#251 is excluded scope and is not counted as a finding.

## Complete acceptance projection

| Planned ID | Issue | Criterion ID | Status | Criterion |
|---|---:|---|---|---|
| WP-01 | #480 | WP-01-ac-1 | proven | Exactly 45 unique child issues are created from the reviewed package |
| WP-01 | #480 | WP-01-ac-2 | proven | Every child title and dependency matches canonical planning |
| WP-01 | #480 | WP-01-ac-3 | proven | Existing or conflicting children fail closed before mutation |
| WP-01 | #480 | WP-01-ac-4 | proven | Partial failure resumes without duplicate creation or renumbering |
| WP-01 | #480 | WP-01-ac-5 | proven | Final readback binds every planned ID to one issue number |
| CORP-A | #482 | CORP-A-ac-1 | proven | Every critical asset has an owner and disposition |
| CORP-A | #482 | CORP-A-ac-2 | proven | Private instruments remain outside Git |
| CORP-A | #482 | CORP-A-ac-3 | proven | Redacted receipts bind accepted instruments to the asset schedule |
| CORP-B | #483 | CORP-B-ac-1 | accepted_with_explicit_amendment | Every critical service has corporate administration and billing custody |
| CORP-B | #483 | CORP-B-ac-2 | accepted_with_explicit_amendment | Recovery does not depend on one personal factor |
| CORP-B | #483 | CORP-B-ac-3 | accepted_with_explicit_amendment | Break-glass use is bounded and audited |
| AWS-A | #484 | AWS-A-ac-1 | proven | The approved business account and regions are exact |
| AWS-A | #484 | AWS-A-ac-2 | proven | Every discovered resource has an owner or frozen-unknown disposition |
| AWS-A | #484 | AWS-A-ac-3 | proven | Website Terraform and issue evidence remain separately classified |
| AWS-A | #484 | AWS-A-ac-4 | proven | EBS and other retained assets are not inferred disposable |
| AWS-B | #485 | AWS-B-ac-1 | proven | Corporate recovery does not depend on one personal factor |
| AWS-B | #485 | AWS-B-ac-2 | proven | Human workload and agent-initiated identities are distinguishable |
| AWS-B | #485 | AWS-B-ac-3 | proven | Agent Toolkit for AWS is configured for the approved Codex path with AWS CLI 2.35 or newer |
| AWS-B | #485 | AWS-B-ac-4 | proven | IAM context policies bind agent actions with read-only default posture |
| AWS-B | #485 | AWS-B-ac-5 | proven | CloudWatch metrics and CloudTrail requests are attributable |
| AWS-B | #485 | AWS-B-ac-6 | proven | Billing and budget ownership is visible |
| AWS-B | #485 | AWS-B-ac-7 | proven | Existing administrator access remains until replacement is proven |
| AWS-C | #486 | AWS-C-ac-1 | proven | Existing website and DDNS states are inventoried first |
| AWS-C | #486 | AWS-C-ac-2 | proven | The new backend is encrypted versioned locked and recoverable |
| AWS-C | #486 | AWS-C-ac-3 | proven | Deployment identity is least privilege |
| AWS-C | #486 | AWS-C-ac-4 | proven | No existing state is copied or dual-owned |
| AWS-D | #487 | AWS-D-ac-1 | implementation_gap | Account changes are durably observable |
| AWS-D | #487 | AWS-D-ac-2 | implementation_gap | Security findings have an owner and destination |
| AWS-D | #487 | AWS-D-ac-3 | implementation_gap | Retention and encryption are explicit |
| AWS-D | #487 | AWS-D-ac-4 | proven | Sensitive values are excluded from retained proof |
| AWS-E | #488 | AWS-E-ac-1 | proven | Every durable resource has one management authority |
| AWS-E | #488 | AWS-E-ac-2 | proven | Website and historical evidence ownership is preserved |
| AWS-E | #488 | AWS-E-ac-3 | proven | Cleanup requires exact non-use retention recovery and deletion authority |
| AWS-E | #488 | AWS-E-ac-4 | proven | Live and declared state agree |
| AWS-F | #489 | AWS-F-ac-1 | proven | Runtime hosts have no direct public ingress |
| AWS-F | #489 | AWS-F-ac-2 | proven | Shared edge network build and node states remain separated |
| AWS-F | #489 | AWS-F-ac-3 | proven | Existing issue 122 owns public Route53 and ACM exposure |
| AWS-F | #489 | AWS-F-ac-4 | implementation_gap | Disposable deployment and cleanup bind the exact modules |
| GCP-A | #490 | GCP-A-ac-1 | proven | Organization folder project billing and region are exact |
| GCP-A | #490 | GCP-A-ac-2 | proven | POC and long-term ownership are explicit |
| GCP-A | #490 | GCP-A-ac-3 | proven | The first workload has a hard cost ceiling |
| GCP-A | #490 | GCP-A-ac-4 | proven | Quota is not treated as capacity |
| GCP-B | #491 | GCP-B-ac-1 | proof_gap | State is versioned private recoverable and auditable |
| GCP-B | #491 | GCP-B-ac-2 | implementation_gap | Deployment uses short-lived impersonation |
| GCP-B | #491 | GCP-B-ac-3 | proven | Provider and module versions are pinned |
| GCP-B | #491 | GCP-B-ac-4 | proof_gap | Local bootstrap state is removed recoverably |
| GCP-C | #492 | GCP-C-ac-1 | proven | Every new project has corporate group ownership and cost attribution |
| GCP-C | #492 | GCP-C-ac-2 | proven | Policies are scoped and impact-reviewed |
| GCP-C | #492 | GCP-C-ac-3 | proven | Billing export and budgets are observable |
| GCP-C | #492 | GCP-C-ac-4 | proven | Existing POC resources remain unchanged unless explicitly admitted |
| GCP-D | #493 | GCP-D-ac-1 | proven | No unintended public route address or ingress exists |
| GCP-D | #493 | GCP-D-ac-2 | proven | Human and workload identities are separate |
| GCP-D | #493 | GCP-D-ac-3 | proven | State artifacts models continuity evidence and logs have separate owners |
| GCP-D | #493 | GCP-D-ac-4 | implementation_gap | A disposable non-GPU workload is destroyed with zero residue |
| GCP-E | #494 | GCP-E-ac-1 | proven | Paid launch has separate authorization and a USD 20 ceiling |
| GCP-E | #494 | GCP-E-ac-2 | proven | Exact inputs and hardware are retained |
| GCP-E | #494 | GCP-E-ac-3 | proven | GPU inference and headroom are proven |
| GCP-E | #494 | GCP-E-ac-4 | proven | All owned resources are independently absent afterward |
| XCL-01 | #495 | XCL-01-ac-1 | proven | The portable workload contract is provider-neutral |
| XCL-01 | #495 | XCL-01-ac-2 | proven | AWS and GCP modules preserve the exact admitted template behavior |
| XCL-01 | #495 | XCL-01-ac-3 | proven | Provider identity and differences remain explicit |
| XCL-01 | #495 | XCL-01-ac-4 | proven | Neither provider silently substitutes for the other |
| XCL-01 | #495 | XCL-01-ac-5 | proven | Existing CloudFormation remains rollback authority until AWS-G |
| AWS-G | #496 | AWS-G-ac-1 | proven | Issue 194 and 268 templates are inventoried |
| AWS-G | #496 | AWS-G-ac-2 | proven | Every consumer and retained evidence path has a disposition |
| AWS-G | #496 | AWS-G-ac-3 | proven | Retirement requires proven Terraform parity and rollback |
| AWS-G | #496 | AWS-G-ac-4 | proven | No active stack is silently abandoned |
| CORP-C | #497 | CORP-C-ac-1 | accepted_recordless | Each control plane has corporate owner and rollback |
| CORP-C | #497 | CORP-C-ac-2 | accepted_recordless | AWS uses the approved business account |
| CORP-C | #497 | CORP-C-ac-3 | accepted_recordless | Terraform and CI authority are company-controlled |
| CORP-C | #497 | CORP-C-ac-4 | accepted_recordless | Availability and recovery readbacks pass |
| CORP-D | #498 | CORP-D-ac-1 | proven | Every CORP-A-C blocker has disposition |
| CORP-D | #498 | CORP-D-ac-2 | proven | Counsel-controlled judgments are recorded only as bounded receipts |
| CORP-D | #498 | CORP-D-ac-3 | proven | Corporate acceptance binds the exact diligence index |
| RUST-01 | #499 | RUST-01-ac-1 | proven | Supported resilience behavior and public API remain compatible |
| RUST-01 | #499 | RUST-01-ac-2 | proven | Each extracted module has one coherent owner |
| RUST-01 | #499 | RUST-01-ac-3 | proven | Tests remain behavior-focused and PVF-classified |
| RUST-01 | #499 | RUST-01-ac-4 | proven | Validation-impact change is measured exactly |
| RUST-01 | #499 | RUST-01-ac-5 | proven | No line-count reduction quota is imposed |
| V3-A | #500 | V3-A-ac-1 | proven | The v3 authority boundary and compatibility posture are explicit |
| V3-A | #500 | V3-A-ac-2 | proven | Requirements 161 through 163 are mapped exactly |
| V3-A | #500 | V3-A-ac-3 | proven | Construction and rollback decisions are reviewable |
| V3-B | #501 | V3-B-ac-1 | proven | State and projections are deterministic |
| V3-B | #501 | V3-B-ac-2 | proven | Repository context is explicit |
| V3-B | #501 | V3-B-ac-3 | proven | Requirements 164 through 167 have focused behavioral proof |
| V3-C | #502 | V3-C-ac-1 | proven | Transitions are capability-checked and atomic |
| V3-C | #502 | V3-C-ac-2 | proven | Recovery preserves audit provenance |
| V3-C | #502 | V3-C-ac-3 | proven | Requirements 168 through 170 pass failure-injection tests |
| V3-D | #503 | V3-D-ac-1 | proven | Commands consume typed contracts |
| V3-D | #503 | V3-D-ac-2 | proven | Bind enforces registered topology |
| V3-D | #503 | V3-D-ac-3 | proven | Cards render from the active registry |
| V3-D | #503 | V3-D-ac-4 | proven | Requirements 171 through 173 have CLI proof |
| V3-E | #504 | V3-E-ac-1 | proven | Review binds exact immutable scope |
| V3-E | #504 | V3-E-ac-2 | proven | Publication modes are explicit |
| V3-E | #504 | V3-E-ac-3 | proven | Finish derives terminal truth |
| V3-E | #504 | V3-E-ac-4 | proven | Requirements 174 through 178 have positive and refusal proof |
| V3-F | #505 | V3-F-ac-1 | proven | Requirements 179 and 180 are mapped |
| V3-F | #505 | V3-F-ac-2 | implementation_gap | v2-v3 parity is measured |
| V3-F | #505 | V3-F-ac-3 | proven | Canary rollback is exercised |
| V3-F | #505 | V3-F-ac-4 | proven | Cutover and retirement require operator approval |
| DRT-A | #506 | DRT-A-ac-1 | proven | Requirements 181 and 182 are mapped |
| DRT-A | #506 | DRT-A-ac-2 | proven | Identity and authority are deterministic |
| DRT-A | #506 | DRT-A-ac-3 | proven | Duplicate denial and replay receipts are exact |
| DRT-B | #507 | DRT-B-ac-1 | proven | Requirements 183 and 184 are mapped |
| DRT-B | #507 | DRT-B-ac-2 | proven | Six distinct residents complete assigned UTS work |
| DRT-B | #507 | DRT-B-ac-3 | proven | Dehydrate and restore preserve exact population |
| DRT-B | #507 | DRT-B-ac-4 | accepted_with_explicit_amendment | GPU evidence waits for reviewed merged 345 authority |
| DRT-C | #508 | DRT-C-ac-1 | proven | Requirements 185 through 187 are mapped |
| DRT-C | #508 | DRT-C-ac-2 | proven | Identity provider and transport failures fail closed |
| DRT-C | #508 | DRT-C-ac-3 | proven | Observatory evidence is authentic |
| DRT-C | #508 | DRT-C-ac-4 | proven | Soak cleanup and synthesis bind exact revisions |
| DRT-D | #509 | DRT-D-ac-1 | proven | Six identities and lineage remain exact |
| DRT-D | #509 | DRT-D-ac-2 | proven | GCP account project billing and credentials are separately governed |
| DRT-D | #509 | DRT-D-ac-3 | proven | Cost and cleanup receipts are retained |
| DRT-D | #509 | DRT-D-ac-4 | proven | No resources remain |
| HOT-01 | #510 | HOT-01-ac-1 | proven | Reads use atomically swappable state |
| HOT-01 | #510 | HOT-01-ac-2 | proven | Invalid updates preserve the last valid configuration |
| HOT-01 | #510 | HOT-01-ac-3 | proven | File events are debounced |
| HOT-01 | #510 | HOT-01-ac-4 | proven | Concurrent requests observe complete configurations only |
| OBS-A | #511 | OBS-A-ac-1 | accepted_with_explicit_amendment | Every view has a stable information contract |
| OBS-A | #511 | OBS-A-ac-2 | accepted_with_explicit_amendment | Empty degraded recovery and revoked states are designed |
| OBS-A | #511 | OBS-A-ac-3 | accepted_with_explicit_amendment | Keyboard and screen-reader flows are specified |
| OBS-A | #511 | OBS-A-ac-4 | accepted_with_explicit_amendment | No invented Runtime field is introduced |
| OBS-B | #512 | OBS-B-ac-1 | proven | OBS-A contracts are implemented |
| OBS-B | #512 | OBS-B-ac-2 | proven | Runtime projections are source-grounded |
| OBS-B | #512 | OBS-B-ac-3 | proven | Accessibility and recovery cases pass |
| OBS-B | #512 | OBS-B-ac-4 | accepted_with_explicit_amendment | Issues 84 and 251 remain visible operator-deferred backlog and do not gate this release |
| DEC-01 | #513 | DEC-01-ac-1 | proven | Every source and reverse reference has one owner and disposition |
| DEC-01 | #513 | DEC-01-ac-2 | proven | Supported behavior has compatibility proof |
| DEC-01 | #513 | DEC-01-ac-3 | proven | Rollback and migration are executable |
| DEC-01 | #513 | DEC-01-ac-4 | proven | Runtime v4 remains excluded |
| PROV-A | #514 | PROV-A-ac-1 | proven | Profiles bind provider model and bounded parameters |
| PROV-A | #514 | PROV-A-ac-2 | proven | Invalid profiles fail before activation |
| PROV-A | #514 | PROV-A-ac-3 | proven | Last-known-good state is retained |
| PROV-A | #514 | PROV-A-ac-4 | proven | Credentials prompts and private payloads are excluded |
| PROV-B | #515 | PROV-B-ac-1 | proven | Shadow and authority paths are distinguishable |
| PROV-B | #515 | PROV-B-ac-2 | proven | Inputs and comparison rules are exact |
| PROV-B | #515 | PROV-B-ac-3 | proven | Failures preserve the authoritative result |
| PROV-B | #515 | PROV-B-ac-4 | proven | Evidence is redacted |

## Complete retained projection

| Successor | Retained | Criterion ID | Status | Criterion |
|---|---:|---|---|---|
| CORP-A | #153 | retained-153-ac-1 | observed_in_merged_successor | Every asset class named by the promoted corporate source has at least one inventoried row or an explicit not-applicable disposition. |
| CORP-A | #153 | retained-153-ac-2 | observed_in_merged_successor | Each critical row identifies current control, target corporate control, transfer dependency, verification method, rollback posture, and evidence location. |
| CORP-A | #153 | retained-153-ac-3 | observed_in_merged_successor | The validator rejects duplicate identifiers, missing owners, missing recovery authority, unbounded secret fields, and unapproved exclusions. |
| CORP-A | #153 | retained-153-ac-4 | observed_in_merged_successor | No transfer or credential rotation occurs in this inventory issue. |
| CORP-A | #154 | retained-154-ac-1 | observed_in_merged_successor | Qualified counsel approves the instrument set before execution. |
| CORP-A | #154 | retained-154-ac-2 | observed_in_merged_successor | All required parties and corporate authorities execute or receive an explicit blocking disposition. |
| CORP-A | #154 | retained-154-ac-3 | observed_in_merged_successor | Private instruments remain outside the public repository and company-controlled custody is verified. |
| CORP-A | #154 | retained-154-ac-4 | observed_in_merged_successor | Redacted receipts bind each executed instrument to the asset schedule without exposing signatures, addresses, secrets, or privileged advice. |
| CORP-A | #155 | retained-155-ac-1 | observed_in_merged_successor | Every critical source, model, dataset, media, and brand asset has a provenance and use-rights disposition. |
| CORP-A | #155 | retained-155-ac-2 | observed_in_merged_successor | Dependency and license conclusions cite machine-readable manifests or authoritative source evidence. |
| CORP-A | #155 | retained-155-ac-3 | observed_in_merged_successor | Unresolved or restricted assets are excluded from transfer and release gates rather than silently accepted. |
| CORP-A | #155 | retained-155-ac-4 | observed_in_merged_successor | Trademark conclusions are explicitly bounded and routed to counsel where legal judgment is required. |
| CORP-B | #156 | retained-156-ac-1 | observed_in_merged_successor | Every critical service has a company-controlled administrator, billing owner, secure MFA, recovery route, and vault location. |
| CORP-B | #156 | retained-156-ac-2 | observed_in_merged_successor | Recovery is exercised without relying solely on a founder-owned phone, email, card, or device. |
| CORP-B | #156 | retained-156-ac-3 | observed_in_merged_successor | Break-glass access is bounded, audited, and distinct from routine credentials. |
| CORP-B | #156 | retained-156-ac-4 | observed_in_merged_successor | The repository records names and outcomes only; no credential material is retained. |
| CORP-C | #157 | retained-157-ac-1 | observed_in_merged_successor | Only the seven approved migration repositories move; asksifu and Horust remain unchanged. |
| CORP-C | #157 | retained-157-ac-2 | observed_in_merged_successor | Agent Design Language remains public and all other company repositories remain private unless separately authorized. |
| CORP-C | #157 | retained-157-ac-3 | observed_in_merged_successor | Founder-account repositories are copied or dispositioned without deletion or destructive history changes. |
| CORP-C | #157 | retained-157-ac-4 | observed_in_merged_successor | Domains, brands, Apps, webhooks, packages, Pages, OIDC, and repository references receive verified dispositions. |
| CORP-C | #158 | retained-158-ac-1 | observed_in_merged_successor | Every AWS operation verifies the approved Agent Logic business account and uses the permanent business profile. |
| CORP-C | #158 | retained-158-ac-2 | observed_in_merged_successor | Public TLS uses ACM or another publicly trusted issuer; production paths contain no self-signed certificate. |
| CORP-C | #158 | retained-158-ac-3 | observed_in_merged_successor | DNS, email, storage, CDN, workload, monitoring, backup, budget, and rollback checks pass from company authority. |
| CORP-C | #158 | retained-158-ac-4 | observed_in_merged_successor | Temporary resources are inventoried, tagged, bounded, and deleted with provider readback after each phase. |
| CORP-C | #159 | retained-159-ac-1 | observed_in_merged_successor | Terraform state, locks, plans, applies, and recovery operate under company custody. |
| CORP-C | #159 | retained-159-ac-2 | observed_in_merged_successor | CI uses company-controlled OIDC or equivalent short-lived identity and least privilege. |
| CORP-C | #159 | retained-159-ac-3 | observed_in_merged_successor | A clean deployment and rollback complete without founder-local credentials or unrecorded manual steps. |
| CORP-C | #159 | retained-159-ac-4 | observed_in_merged_successor | Runbooks name prerequisites, single commands, expected outputs, rollback, cleanup, and escalation without exposing secrets. |
| CORP-D | #160 | retained-160-ac-1 | observed_in_merged_successor | Every critical asset and service has a terminal transferred, retained, excluded, or blocked disposition. |
| CORP-D | #160 | retained-160-ac-2 | observed_in_merged_successor | All required counsel and corporate approvals are present and bound to exact evidence. |
| CORP-D | #160 | retained-160-ac-3 | observed_in_merged_successor | The public index is redacted and recomputable without exposing private instruments or credentials. |
| CORP-D | #160 | retained-160-ac-4 | observed_in_merged_successor | Any unresolved critical exception blocks the corporate release gate and is not downgraded to residual risk without explicit authority. |
| V3-A | #161 | retained-161-ac-1 | observed_in_merged_successor | Every public command and output mode has a versioned contract. |
| V3-A | #161 | retained-161-ac-2 | observed_in_merged_successor | Every retained v2 invariant maps to one owner issue and proof lane. |
| V3-A | #161 | retained-161-ac-3 | observed_in_merged_successor | Exact review, GitHub truth, topology ownership, atomic state, and cleanup boundaries cannot be weakened by later implementation choices. |
| V3-A | #161 | retained-161-ac-4 | observed_in_merged_successor | Unknown or intentionally changed v2 behavior is explicit and reviewed. |
| V3-A | #161 | retained-161-ac-5 | observed_in_merged_successor | The importer remains available until the later of all v2-origin issues reaching terminal state or the operator-approved rollback window expiring. |
| V3-A | #161 | retained-161-ac-6 | observed_in_merged_successor | Output filtering and templating have one approved in-process implementation boundary and cannot invoke a shell or external formatter. |
| V3-A | #161 | retained-161-ac-7 | observed_in_merged_successor | Reviewer independence is structurally checked where identity is bindable; policy-only identity cannot silently satisfy publication. |
| V3-A | #161 | retained-161-ac-8 | observed_in_merged_successor | Closing and non-closing publication are disjoint typed modes; `PartOf` cannot close or terminally complete its parent issue, and split-repository linkage is qualified in both modes. |
| V3-A | #161 | retained-161-ac-9 | observed_in_merged_successor | Every mutable authoritative field has exactly one matrix owner and at least one typed authoring path; every supported invalidation/recovery state has a valid typed next operation. Operator authority may gate that operation but cannot replace its command, transition, target state, or audit contract. |
| V3-A | #161 | retained-161-ac-10 | observed_in_merged_successor | Command help, kernel authorization, doctor findings, and tests are generated from or mechanically checked against the same capability matrix so scattered phase allowlists cannot silently diverge. |
| V3-A | #161 | retained-161-ac-11 | observed_in_merged_successor | The state-size warning precedes the mutation block, initial block capacity is at least ten times the largest deterministic v2 baseline bundle, warning is fixed at 80 percent of that block, and neither path silently drops audit evidence. |
| V3-A | #161 | retained-161-ac-12 | observed_in_merged_successor | V3-01 approval is blocked until the state-size artifact identifies the actual largest v2 bundle at `f1c01499`, records every measured blob and total, and passes the locked recomputation case; no unmeasured adequacy claim is allowed. |
| V3-A | #161 | retained-161-ac-13 | observed_in_merged_successor | If that measurement makes the 10x block impractical for atomic state or operator latency, V3-01 stops and returns to architecture review for a versioned retention/compaction decision; it may neither lower the factor nor proceed with an unbounded aggregate. |
| V3-A | #161 | retained-161-ac-14 | observed_in_merged_successor | The same gate proves the complete V3-16 review/recover/card-family canary fits below 50 percent of the block using maximum schema-valid event sizes, so embedded audit growth is represented rather than inferred from typical v2 history. |
| V3-A | #161 | retained-161-ac-15 | observed_in_merged_successor | `--jq` accepts only the frozen supported subset; unsupported syntax fails with a typed usage error rather than partial or external execution. |
| V3-A | #161 | retained-161-ac-16 | observed_in_merged_successor | The retained `adl.external_source_baseline.v1` manifest passes the VPP's repository-relative `upstream-source-baseline` lane before V3-02 can start; every cited blob must match the pinned `cli/cli` tree object exactly. |
| V3-A | #162 | retained-162-ac-1 | observed_in_merged_successor | The slice uses one binary and one library with the proposed four layers. |
| V3-A | #162 | retained-162-ac-2 | observed_in_merged_successor | Parsing initializes no repository, credentials, network, or child task. |
| V3-A | #162 | retained-162-ac-3 | observed_in_merged_successor | Fake adapters reject unexpected operations and support deterministic tests. |
| V3-A | #162 | retained-162-ac-4 | observed_in_merged_successor | Every required GitHub operation is classified as native typed Octocrab, reviewed raw request, or unsupported. More than three required raw-request operations trigger GitHub client dependency re-evaluation before V3-13. |
| V3-A | #162 | retained-162-ac-5 | observed_in_merged_successor | The slice completes one end-to-end recovery journey: exact review, typed recovery, capability-derived field correction, projection regeneration, audit readback, and fresh exact review, with no direct state or Markdown edit. |
| V3-A | #162 | retained-162-ac-6 | observed_in_merged_successor | Measurements either satisfy approved thresholds or trigger architecture revision before `V3-03`; a missing measurement or any threshold miss is a binding stop, not a discretionary finding. |
| V3-A | #162 | retained-162-ac-7 | observed_in_merged_successor | The spike identifies the exact Decision 11 record required next and proves that its recommendation alone cannot satisfy the V3-08 dependency gate. |
| V3-A | #163 | retained-163-ac-1 | observed_in_merged_successor | Every supported platform has a measured commit primitive and durability contract. |
| V3-A | #163 | retained-163-ac-2 | observed_in_merged_successor | Windows mutation is either equivalently proven or explicitly fail-closed read-only. |
| V3-A | #163 | retained-163-ac-3 | observed_in_merged_successor | The operator decision cites exact V3-02 evidence and cannot be inferred from recommendation text. |
| V3-A | #163 | retained-163-ac-4 | observed_in_merged_successor | V3-08 remains blocked until this issue is terminal. |
| V3-B | #164 | retained-164-ac-1 | observed_in_merged_successor | Every approved command is discoverable from `csdlc --help`. |
| V3-B | #164 | retained-164-ac-2 | observed_in_merged_successor | Cargo package `csdlc-v3` builds and installs exactly one binary named `csdlc`; generated docs, completions, provenance, and installer checks bind both immutable identities. |
| V3-B | #164 | retained-164-ac-3 | observed_in_merged_successor | Constructor and parser tests invoke no repository, network, or process adapter. |
| V3-B | #164 | retained-164-ac-4 | observed_in_merged_successor | Human and JSON output never mix machine payloads with diagnostics. |
| V3-B | #164 | retained-164-ac-5 | observed_in_merged_successor | JSON carries the V3-01 schema discriminant; `--jq` and `--template` parse, conflict, and operate only through the V3-01/V3-02 approved in-process path. |
| V3-B | #164 | retained-164-ac-6 | observed_in_merged_successor | `--jq` implements exactly the approved subset manifest, has golden compatibility tests for every supported form, and returns a typed usage error for unsupported jq syntax. |
| V3-B | #164 | retained-164-ac-7 | observed_in_merged_successor | Every command that supports structured `--input` rejects combining it with any direct field flag at the Clap parser boundary; positive and conflict parser tests are required for each such command. |
| V3-B | #164 | retained-164-ac-8 | observed_in_merged_successor | Dependency-policy CI rejects unapproved licenses, advisories, bans, and duplicate dependency families from this issue onward. |
| V3-B | #164 | retained-164-ac-9 | observed_in_merged_successor | The release build emits one provenance-bound executable. |
| V3-B | #165 | retained-165-ac-1 | observed_in_merged_successor | One `App` exists per invocation and no mutable global service locator exists. |
| V3-B | #165 | retained-165-ac-2 | observed_in_merged_successor | `Git`, `FileSystem`, and `ProcessRunner` signatures are reviewed and frozen at an explicit checkpoint before parallel V3-05 or V3-09 implementation begins. |
| V3-B | #165 | retained-165-ac-3 | observed_in_merged_successor | Expensive or credential-bearing services initialize only on demand. |
| V3-B | #165 | retained-165-ac-4 | observed_in_merged_successor | Sync lazy accessors initialize once without panic and propagate one cached typed result to concurrent callers. |
| V3-B | #165 | retained-165-ac-5 | observed_in_merged_successor | Async lazy accessors cache completed success/error results while cancelled initialization remains uninitialized and retryable. |
| V3-B | #165 | retained-165-ac-6 | observed_in_merged_successor | Cancelled async initialization remains single-flight on retry, applies the configured cooldown for localized cancellation/timeouts, and never retries after root cancellation. |
| V3-B | #165 | retained-165-ac-7 | observed_in_merged_successor | The selected Tokio release is exact-version pinned, and deterministic leader drop tests prove state reset, waiter notification, exactly one cooldown-governed retry, and absence of deadlock, leaked waiter, or retained initializer future. |
| V3-B | #165 | retained-165-ac-8 | observed_in_merged_successor | Sync initialization tests prove that one terminal error is cached for the invocation and is not changed by later filesystem mutation. |
| V3-B | #165 | retained-165-ac-9 | observed_in_merged_successor | Async adapter traits remain object-safe without infecting pure domain APIs. |
| V3-B | #165 | retained-165-ac-10 | observed_in_merged_successor | Supported OS and console interruption signals drive root cancellation and bounded child/task teardown before exit code 130. |
| V3-B | #165 | retained-165-ac-11 | observed_in_merged_successor | Machine output is stdout-only and diagnostics/tracing are stderr-only by default. |
| V3-B | #165 | retained-165-ac-12 | observed_in_merged_successor | Secrets and machine-local paths are absent from durable output. |
| V3-B | #166 | retained-166-ac-1 | observed_in_merged_successor | Resolution precedence is explicit and produces one canonical identity. |
| V3-B | #166 | retained-166-ac-2 | observed_in_merged_successor | Symlink, path escape, ambiguous remote, and ambiguous issue cases fail closed. |
| V3-B | #166 | retained-166-ac-3 | observed_in_merged_successor | Every unsupported v2 field is reported with record and field identity. |
| V3-B | #166 | retained-166-ac-4 | observed_in_merged_successor | Unsupported fields produce `ImportStatus::BlockedUnsupportedFields`; the record cannot enter a v3 mutation path until every field has a reviewed preserve, map, or explicit operator disposition. |
| V3-B | #166 | retained-166-ac-5 | observed_in_merged_successor | Import never writes v2 or v3 state and does not infer missing authority. |
| V3-B | #167 | retained-167-ac-1 | observed_in_merged_successor | `state.json` is the sole machine authority and every projection is reproducible from it plus declared immutable inputs. |
| V3-B | #167 | retained-167-ac-2 | observed_in_merged_successor | Unknown schema versions and enum values fail explicitly. |
| V3-B | #167 | retained-167-ac-3 | observed_in_merged_successor | All six cards preserve their distinct lifecycle semantics. |
| V3-B | #167 | retained-167-ac-4 | observed_in_merged_successor | Missing required fields fail with a typed error; optional unset fields render only the declared placeholder at each lifecycle phase. |
| V3-B | #167 | retained-167-ac-5 | observed_in_merged_successor | `audit.jsonl` is reproducible from embedded state events and has no separate mutation or integrity authority. |
| V3-B | #167 | retained-167-ac-6 | observed_in_merged_successor | Projection drift is diagnosable and repair never treats Markdown as authority. |
| V3-C | #168 | retained-168-ac-1 | observed_in_merged_successor | Every state/command pair has an explicit allowed or rejected outcome. |
| V3-C | #168 | retained-168-ac-2 | observed_in_merged_successor | The compiler enforces exhaustive closed-state handling. |
| V3-C | #168 | retained-168-ac-3 | observed_in_merged_successor | Branch/worktree topology is the only local ownership authority. |
| V3-C | #168 | retained-168-ac-4 | observed_in_merged_successor | Review staleness, publication gates, terminal truth, and cleanup eligibility remain fail-closed. |
| V3-C | #168 | retained-168-ac-5 | observed_in_merged_successor | Every accepted recovery transition preserves a reachable typed correction or typed terminal-disposition command; no supported state is a lifecycle dead end and no abstract operator-required sink satisfies reachability. |
| V3-C | #168 | retained-168-ac-6 | observed_in_merged_successor | The generated transition table accepts `review recover` only from `reviewed`, `published`, or `merge_ready`, returns to `implemented`, rejects `merged` and `closed_out`, and proves the matrix-declared atomic invalidations. |
| V3-C | #168 | retained-168-ac-7 | observed_in_merged_successor | Removing or changing any authorization predicate causes mutation/property tests to fail, including correction invalidation and stale-CAS predicates. |
| V3-C | #168 | retained-168-ac-8 | observed_in_merged_successor | Cleanup eligibility requires committed `closed_out` state and a retained terminal receipt; remote merge observation alone is insufficient. |
| V3-C | #169 | retained-169-ac-1 | observed_in_merged_successor | Only atomic replacement of `state.json` commits authority. |
| V3-C | #169 | retained-169-ac-2 | observed_in_merged_successor | State commits before projection replacement; post-commit projection failure is a specific repair-required result, never rollback or ambiguous authority. |
| V3-C | #169 | retained-169-ac-3 | observed_in_merged_successor | Cards, evidence indexes, and audit views are repairable projections. |
| V3-C | #169 | retained-169-ac-4 | observed_in_merged_successor | Stale generation/digest writers fail before commit. |
| V3-C | #169 | retained-169-ac-5 | observed_in_merged_successor | Every injected interruption converges to the prior or new valid state. |
| V3-C | #169 | retained-169-ac-6 | observed_in_merged_successor | A remote operation cannot begin before its typed intent and parent directory are durably synced; recovery resumes committed intents through exact readback. |
| V3-C | #169 | retained-169-ac-7 | observed_in_merged_successor | An unresolved intent is authoritative only as a pending-operation journal: it blocks competing mutation, contains no lifecycle/card state, and is consumed only after exact readback commits its outcome into `state.json`. |
| V3-C | #169 | retained-169-ac-8 | observed_in_merged_successor | Linux, macOS, and every mutation-enabled Windows filesystem have a named, documented, fault-tested commit primitive; unproven Windows mutation fails closed while compile and read-only support remain available. |
| V3-C | #169 | retained-169-ac-9 | observed_in_merged_successor | An injected platform-capability fixture proves the Windows fail-closed path and stable `unsupported_platform_mutation` error on every CI host; native Windows CI separately proves any mutation-enabled primitive. |
| V3-C | #169 | retained-169-ac-10 | observed_in_merged_successor | Locks protect transaction integrity without becoming lifecycle authority. |
| V3-C | #170 | retained-170-ac-1 | observed_in_merged_successor | Every Git/process invocation is argv-based and typed. |
| V3-C | #170 | retained-170-ac-2 | observed_in_merged_successor | Exit status, stdout, stderr, timeout, cancellation, and truncation remain distinguishable. |
| V3-C | #170 | retained-170-ac-3 | observed_in_merged_successor | Credentials exist only in the child/provider process scope that needs them. |
| V3-C | #170 | retained-170-ac-4 | observed_in_merged_successor | Branch-name observation alone never authorizes lifecycle work. |
| V3-D | #171 | retained-171-ac-1 | observed_in_merged_successor | Common paths use direct flags while `--input` provides typed automation. |
| V3-D | #171 | retained-171-ac-2 | observed_in_merged_successor | Bind verifies actual canonical branch/worktree topology and rejects every same-issue, cross-issue, main-branch, missing, dirty-policy, and drift case. |
| V3-D | #171 | retained-171-ac-3 | observed_in_merged_successor | Issue commands remain idempotent and never infer ownership from branch names alone. |
| V3-D | #171 | retained-171-ac-4 | observed_in_merged_successor | Human and JSON results preserve the same typed outcome. |
| V3-D | #172 | retained-172-ac-1 | observed_in_merged_successor | Card edits mutate semantic values and regenerate all affected projections. |
| V3-D | #172 | retained-172-ac-2 | observed_in_merged_successor | Rendered Markdown and stale projections never become input authority. |
| V3-D | #172 | retained-172-ac-3 | observed_in_merged_successor | Doctor is read-only, specific, and identifies the next valid operation. |
| V3-D | #172 | retained-172-ac-4 | observed_in_merged_successor | Doctor reports a dedicated invariant failure when a wrong or stale acceptance-bearing field has no authorized correction path; ordinary healthy states always receive a capability-derived next operation. |
| V3-D | #172 | retained-172-ac-5 | observed_in_merged_successor | Projection drift, invalid schema, unsupported import fields, and topology blockers remain distinguishable. |
| V3-D | #172 | retained-172-ac-6 | observed_in_merged_successor | `card show`, `card edit`, and doctor enforce the V3-06 per-phase required and optional field table and its one declared placeholder. |
| V3-D | #173 | retained-173-ac-1 | observed_in_merged_successor | Every lane declares proof role, determinism, resource profile, gate posture, command, timeout, dependencies, and evidence destination. |
| V3-D | #173 | retained-173-ac-2 | observed_in_merged_successor | Pending, deferred, blocked, failed, skipped, and passed cannot be conflated. |
| V3-D | #173 | retained-173-ac-3 | observed_in_merged_successor | Cycles, duplicate ownership, missing acceptance coverage, and hidden routing policy fail before execution. |
| V3-D | #173 | retained-173-ac-4 | observed_in_merged_successor | Planning has no process, network, clock, or filesystem side effects beyond declared input loading. |
| V3-E | #174 | retained-174-ac-1 | observed_in_merged_successor | Parallel tasks are bounded and every Tokio task is awaited after cancellation. |
| V3-E | #174 | retained-174-ac-2 | observed_in_merged_successor | Every OS child is registered with root cancellation; Unix termination uses bounded `SIGTERM`/kill escalation and Windows uses the reviewed termination primitive, followed by handle wait and output drain. |
| V3-E | #174 | retained-174-ac-3 | observed_in_merged_successor | Every sleep and network/process await participates in `tokio::select!` with cancellation. |
| V3-E | #174 | retained-174-ac-4 | observed_in_merged_successor | Incomplete, cancelled, timed-out, or tampered evidence cannot appear passed. |
| V3-E | #174 | retained-174-ac-5 | observed_in_merged_successor | Each captured stream records `truncated`, `captured_bytes`, and `original_bytes_if_known`; human and JSON output distinguish an enforced cap from naturally short process output. |
| V3-E | #174 | retained-174-ac-6 | observed_in_merged_successor | Passing validation cannot authorize review, publication, or merge. |
| V3-E | #175 | retained-175-ac-1 | observed_in_merged_successor | Review names exact revision, scope, reviewer, findings, and dispositions. |
| V3-E | #175 | retained-175-ac-2 | observed_in_merged_successor | Substantive head changes stale review; non-substantive exceptions require deterministic proof. |
| V3-E | #175 | retained-175-ac-3 | observed_in_merged_successor | `review recover` is accepted only from `reviewed`, `published`, or `merge_ready`; it is rejected from `merged` and `closed_out`. It requires actor/reason and stale-truth provenance, returns to `implemented`, and atomically clears every dependent review, publication, readiness, and terminal field declared by the capability row before a card correction can proceed. |
| V3-E | #175 | retained-175-ac-4 | observed_in_merged_successor | Recovery followed by a semantic card correction and fresh review is a complete executable path; direct state/card edits and abstract operator dispositions cannot satisfy it. |
| V3-E | #175 | retained-175-ac-5 | observed_in_merged_successor | Both linkage modes prove the full review journey: review, publish, recover, semantic correction, re-review, and republish preserve the exact normalized target and invalidate the superseded mode-bound authorization. |
| V3-E | #175 | retained-175-ac-6 | observed_in_merged_successor | Publication fails closed on missing, stale, blocked, or actionable review. |
| V3-E | #175 | retained-175-ac-7 | observed_in_merged_successor | Model/provider output is evidence input, never direct lifecycle authority. |
| V3-E | #175 | retained-175-ac-8 | observed_in_merged_successor | Same-principal implementation/review/publication is rejected; policy-only identity cannot pass the publication gate without a named typed override. |
| V3-E | #175 | retained-175-ac-9 | observed_in_merged_successor | Human-review publication remains fail-closed until a concrete authenticated principal observer implements the V3-04 interface; V3-12 proves this with a fake and does not depend on the V3-13 GitHub implementation. |
| V3-E | #175 | retained-175-ac-10 | observed_in_merged_successor | Authorization consumes the V3-01 `PublicationLinkage` value, binds it to the exact reviewed revision and target issue, and rejects absent, mixed, ambiguous, or wrong-repository linkage. |
| V3-E | #175 | retained-175-ac-11 | observed_in_merged_successor | `PartOf` rejects a closing keyword for its target and `Closing` rejects a non-closing-only relation. |
| V3-E | #176 | retained-176-ac-1 | observed_in_merged_successor | Domain modules depend only on normalized GitHub observations. |
| V3-E | #176 | retained-176-ac-2 | observed_in_merged_successor | Pagination, rate limits, authentication, missing resources, and unknown mergeability remain distinct. |
| V3-E | #176 | retained-176-ac-3 | observed_in_merged_successor | Required checks bind to exact head SHA and terminal conclusions. |
| V3-E | #176 | retained-176-ac-4 | observed_in_merged_successor | `IssueObservation` is populated from the typed REST issue endpoint and preserves qualified identity, `state`, `state_reason`, `updated_at`, and observation time; missing or ambiguous fields cannot be normalized to open. |
| V3-E | #176 | retained-176-ac-5 | observed_in_merged_successor | REST fixtures separately prove `state: null`, HTTP 404, and `state: closed` with `state_reason: completed`; none can normalize to an open checkpoint target. |
| V3-E | #176 | retained-176-ac-6 | observed_in_merged_successor | Every raw-request endpoint names its GitHub API reference and has typed request/response structures plus transport-level fixtures. |
| V3-E | #176 | retained-176-ac-7 | observed_in_merged_successor | Read-only commands perform no remote or local lifecycle mutation. |
| V3-E | #176 | retained-176-ac-8 | observed_in_merged_successor | Authenticated human-principal observation is typed and activates no publication authority until V3-12 independently evaluates it. |
| V3-E | #177 | retained-177-ac-1 | observed_in_merged_successor | No remote mutation begins before its durable intent commit. |
| V3-E | #177 | retained-177-ac-2 | observed_in_merged_successor | Every mutation is idempotent and verified by exact remote readback. |
| V3-E | #177 | retained-177-ac-3 | observed_in_merged_successor | `closing` requires the exact closing relation; `part_of` requires the exact non-closing relation and proves the target issue remains open after PR publication and checkpoint merge observation. |
| V3-E | #177 | retained-177-ac-4 | observed_in_merged_successor | Same-repository shorthand normalizes to a qualified identity, while split repositories reject unqualified linkage in either mode. |
| V3-E | #177 | retained-177-ac-5 | observed_in_merged_successor | `pr watch` is foreground, cancellable by root signals, bounded, and leaves no persistent job or unjoined task. |
| V3-E | #177 | retained-177-ac-6 | observed_in_merged_successor | Fake-adapter tests prove that a `part_of` watch cannot report checkpoint-ready unless exact REST issue readback still observes the qualified target issue open; closed, missing, stale, or contradictory observations produce reconciliation-required. |
| V3-E | #177 | retained-177-ac-7 | observed_in_merged_successor | Every watch sleep and network await is selected against root cancellation; cancellation drains and joins the watch scope before exit 130. |
| V3-E | #177 | retained-177-ac-8 | observed_in_merged_successor | Default and overridden timeout/poll values remain within the V3-01 bounds and timeout exits without a persistent job or unjoined task. |
| V3-E | #177 | retained-177-ac-9 | observed_in_merged_successor | If `now + max(poll_interval, retry_after)` exceeds the fixed deadline, watch exits immediately without sleeping past the deadline. |
| V3-E | #177 | retained-177-ac-10 | observed_in_merged_successor | Merge occurs only when the approved explicit policy and operator authority are both present. |
| V3-E | #178 | retained-178-ac-1 | observed_in_merged_successor | Finish derives terminal truth from exact GitHub state and never creates or selects an ambiguous second PR. |
| V3-E | #178 | retained-178-ac-2 | observed_in_merged_successor | A merged `part_of` publication records checkpoint completion without closing or terminally completing the parent issue; only a matching `closing` publication or explicit no-PR outcome can do so. |
| V3-E | #178 | retained-178-ac-3 | observed_in_merged_successor | Successful checkpoint finish transitions `published / merge_ready` through `checkpoint_completed` to `implemented`, retains checkpoint evidence, and invalidates the prior review/publication authorization before another slice. |
| V3-E | #178 | retained-178-ac-4 | observed_in_merged_successor | A complete acceptance journey merges multiple `part_of` checkpoints for one issue, preserves the open parent after each, then processes a later independently reviewed `closing` publication through finish and closes that exact parent without selecting any checkpoint PR as terminal authority. |
| V3-E | #178 | retained-178-ac-5 | observed_in_merged_successor | A merged `part_of` checkpoint whose parent later closes returns `operator_required`; the separately authorized external-parent-close disposition records distinct causes and reaches terminal truth without crediting the checkpoint PR or requiring remote rollback. |
| V3-E | #178 | retained-178-ac-6 | observed_in_merged_successor | Cleanup is a separate command after finish and defaults to preview. |
| V3-E | #178 | retained-178-ac-7 | observed_in_merged_successor | Cleanup requires canonical candidate-path equality with the verified Git worktree root; prefix and relative matches are rejected. |
| V3-E | #178 | retained-178-ac-8 | observed_in_merged_successor | Live, dirty, mismatched, absent, unregistered, and already-removed worktrees have distinct outcomes. |
| V3-E | #178 | retained-178-ac-9 | observed_in_merged_successor | Build/cache directories from any other worktree are never deletion targets. |
| V3-E | #178 | retained-178-ac-10 | observed_in_merged_successor | Cleanup requires committed `closed_out` state and its terminal receipt; a GitHub merge without local terminal reconciliation remains ineligible. |
| V3-F | #179 | retained-179-ac-1 | observed_in_merged_successor | Normalized parity covers cards, lifecycle, validation, review, both publication linkage modes, linkage-aware finish, and cleanup with no unexplained mismatch. |
| V3-F | #179 | retained-179-ac-2 | observed_in_merged_successor | Every imported record reports unsupported fields before mutation. |
| V3-F | #179 | retained-179-ac-3 | observed_in_merged_successor | At least the approved canary cohort completes end to end on v3-only authority. |
| V3-F | #179 | retained-179-ac-4 | observed_in_merged_successor | The canary cohort includes normal authoring and post-review correction for every card family, plus the issue #73 STP-denominator recovery journey; doctor must identify a valid next operation at each intermediate state. |
| V3-F | #179 | retained-179-ac-5 | observed_in_merged_successor | Every known v2 tooling defect in the retained register has a passing v3 positive or negative regression, or a reviewed explicit non-parity decision. |
| V3-F | #179 | retained-179-ac-6 | observed_in_merged_successor | Each migrated issue receives an archived exact v2 snapshot and a durable writer fence; the canonical v2 index is absent before v3 mutation begins. |
| V3-F | #179 | retained-179-ac-7 | observed_in_merged_successor | Supported v2 tools and repository guards reject fenced issue mutation and any reintroduced v2 index or post-fence v2 state. |
| V3-F | #179 | retained-179-ac-8 | observed_in_merged_successor | No issue is writable by supported v2 and v3 authorities simultaneously. |
| V3-F | #179 | retained-179-ac-9 | observed_in_merged_successor | The final delta precedes authority switch; source archival follows cutover. |
| V3-F | #179 | retained-179-ac-10 | observed_in_merged_successor | Cutover requires exact independent review and explicit operator approval. |
| V3-F | #179 | retained-179-ac-11 | observed_in_merged_successor | V2 remains available only as the time-bounded read-only importer/rollback surface defined by policy. |
| V3-F | #180 | retained-180-ac-1 | observed_in_merged_successor | Every deletion target is classified before mutation. |
| V3-F | #180 | retained-180-ac-2 | observed_in_merged_successor | Historical Gate and migration evidence remains readable and immutable. |
| V3-F | #180 | retained-180-ac-3 | observed_in_merged_successor | No v2 executable, operator skill, selector route, or writable state authority remains after removal. |
| V3-F | #180 | retained-180-ac-4 | observed_in_merged_successor | V3 can install, validate, review, publish, finish, and clean from a fresh checkout without v2 artifacts. |
| DRT-A | #181 | retained-181-ac-1 | observed_in_merged_successor | The contract names exactly three voters, three governed agents, one non-voting Shepherd, and one quorum-leased Observatory. |
| DRT-A | #181 | retained-181-ac-2 | observed_in_merged_successor | Every node has distinct identity, credential, port, state root, storage, and failure-domain placement. |
| DRT-A | #181 | retained-181-ac-3 | observed_in_merged_successor | Each scenario has setup, action, expected commit/election/fence behavior, timeout, receipt fields, cleanup, and fail-closed outcome. |
| DRT-A | #181 | retained-181-ac-4 | observed_in_merged_successor | The contract distinguishes production proof from harness orchestration and forbids in-process substitutes or hard-coded success counts. |
| DRT-A | #182 | retained-182-ac-1 | observed_in_merged_successor | Canonical encode-decode-reencode is byte-stable for every supported message family. |
| DRT-A | #182 | retained-182-ac-2 | observed_in_merged_successor | Identity, authority, permit, causation, correlation, sequence, term, and polis bindings reject every declared mutation. |
| DRT-A | #182 | retained-182-ac-3 | observed_in_merged_successor | Duplicate, reordered, stale, malformed, unsigned, wrong-domain, and cross-polis messages produce typed deterministic outcomes. |
| DRT-A | #182 | retained-182-ac-4 | observed_in_merged_successor | Independent replay from retained inputs reproduces the exact committed outcome and digest. |
| DRT-B | #183 | retained-183-ac-1 | observed_in_merged_successor | The exact #142 merge SHA is ancestral to the tested revision and its retained Guardian/API/WSS/WP-04.16 proof passes. |
| DRT-B | #183 | retained-183-ac-2 | observed_in_merged_successor | Three independently started voters commit governed work; two voters preserve quorum; one voter cannot mutate. |
| DRT-B | #183 | retained-183-ac-3 | observed_in_merged_successor | The old Observatory lease expires before successor binding and stale-owner writes are denied. |
| DRT-B | #183 | retained-183-ac-4 | observed_in_merged_successor | Snapshot restore, voter restart, agent continuity, replay, and cleanup pass without shared state roots or direct executor bypass. |
| DRT-B | #184 | retained-184-ac-1 | observed_in_merged_successor | AWS identity resolves to the approved Agent Logic business account before provisioning. |
| DRT-B | #184 | retained-184-ac-2 | observed_in_merged_successor | AWS voters use separate AZs, private authenticated transport, distinct state and independently materialized snapshots. |
| DRT-B | #184 | retained-184-ac-3 | observed_in_merged_successor | Isolating Wuji preserves AWS-only quorum continuity while the isolated stale voter cannot mutate; loss of quorum halts mutation. |
| DRT-B | #184 | retained-184-ac-4 | observed_in_merged_successor | Healing converges term, commit index, state digest, fence, and Observatory ownership before traffic resumes; every phase cleans up. |
| DRT-C | #185 | retained-185-ac-1 | observed_in_merged_successor | Voting, agent, Shepherd, operator, and Observatory identities use separated keys and roles; Shepherd cannot vote. |
| DRT-C | #185 | retained-185-ac-2 | observed_in_merged_successor | Production TLS chains to an approved trust anchor and no self-signed certificate appears on a production path. |
| DRT-C | #185 | retained-185-ac-3 | observed_in_merged_successor | Forged, stale, wrong-domain, missing-capability, cross-polis, malformed, and pre-auth disclosure attempts are denied with typed receipts. |
| DRT-C | #185 | retained-185-ac-4 | observed_in_merged_successor | Provider timeout, stall, malformed output, and partial failure preserve state and authority invariants. |
| DRT-C | #186 | retained-186-ac-1 | observed_in_merged_successor | Exactly one Observatory owns the quorum lease at any instant and successor binding follows old-lease expiry. |
| DRT-C | #186 | retained-186-ac-2 | observed_in_merged_successor | Every displayed operation correlates agent, node, polis, identity, authority, trace, term, commit index, and state revision. |
| DRT-C | #186 | retained-186-ac-3 | observed_in_merged_successor | Partitions and leadership changes cannot present stale authority as current or combine an incoherent cut. |
| DRT-C | #186 | retained-186-ac-4 | observed_in_merged_successor | Secrets, credentials, private legal data, and unredacted provider payloads never appear in retained or visible evidence. |
| DRT-C | #187 | retained-187-ac-1 | observed_in_merged_successor | Both soak durations complete under declared workload, fault, resource, and error thresholds. |
| DRT-C | #187 | retained-187-ac-2 | observed_in_merged_successor | Receipts bind exact commands, terms, committed indexes, envelopes, source revisions, model digests, clocks, and cleanup outcomes. |
| DRT-C | #187 | retained-187-ac-3 | observed_in_merged_successor | Independent replay reproduces the declared deterministic outcomes without live-provider dependence. |
| DRT-C | #187 | retained-187-ac-4 | observed_in_merged_successor | Provider and process readback proves cleanup after normal completion and every injected or unexpected failure phase. |
| TAIL-01 | #188 | retained-188-ac-1 | consolidated_successor_uncertainty | CORP-08, V3-16, and DRT-07 are terminal and exact revisions are ancestral to the review revision. |
| TAIL-01 | #188 | retained-188-ac-2 | consolidated_successor_uncertainty | Every required lane artifact and quality gate is independently recomputed or explicitly rejected. |
| TAIL-01 | #188 | retained-188-ac-3 | consolidated_successor_uncertainty | All P1/P2 findings receive verified terminal dispositions before recommendation. |
| TAIL-01 | #188 | retained-188-ac-4 | consolidated_successor_uncertainty | The review does not treat one lane's success as evidence for another lane. |
| TAIL-06 | #188 | retained-188-ac-1 | consolidated_successor_uncertainty | CORP-08, V3-16, and DRT-07 are terminal and exact revisions are ancestral to the review revision. |
| TAIL-06 | #188 | retained-188-ac-2 | consolidated_successor_uncertainty | Every required lane artifact and quality gate is independently recomputed or explicitly rejected. |
| TAIL-06 | #188 | retained-188-ac-3 | consolidated_successor_uncertainty | All P1/P2 findings receive verified terminal dispositions before recommendation. |
| TAIL-06 | #188 | retained-188-ac-4 | consolidated_successor_uncertainty | The review does not treat one lane's success as evidence for another lane. |
| TAIL-07 | #190 | retained-190-ac-1 | consolidated_successor_uncertainty | The handoff cites exact terminal release evidence and every accepted residual risk. |
| TAIL-07 | #190 | retained-190-ac-2 | consolidated_successor_uncertainty | Deferred work retains owners, dependencies, proof requirements, and routing without being presented as complete. |
| TAIL-07 | #190 | retained-190-ac-3 | consolidated_successor_uncertainty | V3-R01 remains ineligible until rollback expiry, stability metrics, historical readability, and explicit operator approval all pass. |
| TAIL-07 | #190 | retained-190-ac-4 | consolidated_successor_uncertainty | The downstream milestone can consume the packet without relying on chat or machine-local state. |
| TAIL-10 | #189 | retained-189-ac-1 | consolidated_successor_uncertainty | Every release artifact, source revision, schema, evidence bundle, and external dependency is pinned. |
| TAIL-10 | #189 | retained-189-ac-2 | consolidated_successor_uncertainty | Rollback is rehearsed from the candidate without data loss, dual authority, or hidden personal credentials. |
| TAIL-10 | #189 | retained-189-ac-3 | consolidated_successor_uncertainty | Named company authority, observers, timing, abort triggers, communication, and post-release verification are recorded. |
| TAIL-10 | #189 | retained-189-ac-4 | consolidated_successor_uncertainty | No release action occurs until the final operator authorization. |

## Complete collision projection

| Path | Owners | Status |
|---|---|---|
| infra/aws/account-foundation | 485,487 | unresolved |
| infra/aws/runtime | 489,495 | unresolved |
| csdlc-v3/src/lib.rs | 500,503 | unresolved |

## Complete backlog projection

| Issue | Disposition | Authority digest |
|---|---|---|
| #84 | routed_to_backlog | 4c0b8a4cec3741f4ebb145864fcdf02cfccf62b352e6a8a900c8ebc97385b59a |
| #251 | routed_to_backlog | 4113989e3d613b8044e49c08e7ebc26be4be346fdb19db14695ec5f2b9fbcb40 |
