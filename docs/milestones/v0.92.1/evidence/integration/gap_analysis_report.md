# v0.92.1 Release-tail Gap Analysis

Candidate: `b8e1b4483b1c345e42ec7817bfde8214469867bb`

Captured-input digest: `ebe7fe5d2d2a55df068e920b9517e5d7b1956d8688aeabdd300178b58c41163f`

Canonical projection digest: `bb81ed42f8c0b1b4ac107007d483cb9a212f9b98184ca4847c48001eeb63711a`

## Findings

- **P1 issue-482-review-or-terminal-gap** — CORP-A / #482 has a current exact-head semantic review gap. Evidence: https://github.com/agent-logic/agent-design-language/issues/482, .csdlc/issues/482/cards/srp.md, .csdlc/issues/482/cards/sor.md. Owner: issue #482. Disposition: open.
- **P1 issue-497-review-or-terminal-gap** — CORP-C / #497 has a current exact-head semantic review gap. Evidence: https://github.com/agent-logic/agent-design-language/issues/497, .csdlc/issues/497/cards/srp.md, .csdlc/issues/497/cards/sor.md. Owner: issue #497. Disposition: open.
- **P1 issue-505-review-or-terminal-gap** — V3-F / #505 has a current exact-head semantic review gap. Evidence: https://github.com/agent-logic/agent-design-language/issues/505. Owner: issue #505. Disposition: open.
- **P1 issue-511-review-or-terminal-gap** — OBS-A / #511 has a current exact-head semantic review gap. Evidence: https://github.com/agent-logic/agent-design-language/issues/511, .csdlc/issues/511/cards/srp.md, .csdlc/issues/511/cards/sor.md. Owner: issue #511. Disposition: open.
- **P1 issue-512-review-or-terminal-gap** — OBS-B / #512 lacks reviewed merged ancestral authority. Evidence: https://github.com/agent-logic/agent-design-language/issues/512, .csdlc/issues/512/cards/srp.md, .csdlc/issues/512/cards/sor.md. Owner: issue #512. Disposition: open.
- **P1 issue-660-review-or-terminal-gap** — PODCAST-EXPOSURE-REPAIR / #660 has a current exact-head semantic review gap. Evidence: https://github.com/agent-logic/agent-design-language/issues/660, .csdlc/issues/660/cards/srp.md, .csdlc/issues/660/cards/sor.md. Owner: issue #660. Disposition: open.
- **P1 issue-662-review-or-terminal-gap** — A2A-INITIATION / #662 has a current exact-head semantic review gap. Evidence: https://github.com/agent-logic/agent-design-language/issues/662, .csdlc/issues/662/cards/srp.md, .csdlc/issues/662/cards/sor.md. Owner: issue #662. Disposition: open.
- **P1 issue-480-spec-ac-drift** — WP-01 live acceptance criteria do not cover the exact execution specification. Evidence: https://github.com/agent-logic/agent-design-language/issues/480#issue-body, f9a7235866ee1a2565fe18a7399b51b2704c37dae4ed875cfec987c8050e5532. Owner: issue #480. Disposition: open.
- **P1 issue-497-spec-ac-drift** — CORP-C live acceptance criteria do not cover the exact execution specification. Evidence: https://github.com/agent-logic/agent-design-language/issues/497#issue-body, 92168c2666207956df37ed6fc00dcb78ef339a8be0284bd5e9683db1b95bc43f. Owner: issue #497. Disposition: open.
- **P1 issue-512-spec-ac-drift** — OBS-B live acceptance criteria do not cover the exact execution specification. Evidence: https://github.com/agent-logic/agent-design-language/issues/512#issue-body, 80be51e3a6ae692a23386c52cfcd8aed130b058c398c55659d1d724a79eaaa5d. Owner: issue #512. Disposition: open.
- **P1 retained-153-observed-gap** — Retained predecessor #153 is not covered by a reviewed-green successor. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/153/cards/stp.md. Owner: issue #482. Disposition: open.
- **P1 retained-154-observed-gap** — Retained predecessor #154 is not covered by a reviewed-green successor. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/154/cards/stp.md. Owner: issue #482. Disposition: open.
- **P1 retained-155-observed-gap** — Retained predecessor #155 is not covered by a reviewed-green successor. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/155/cards/stp.md. Owner: issue #482. Disposition: open.
- **P1 retained-157-observed-gap** — Retained predecessor #157 is not covered by a reviewed-green successor. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/157/cards/stp.md. Owner: issue #497. Disposition: open.
- **P1 retained-158-observed-gap** — Retained predecessor #158 is not covered by a reviewed-green successor. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/158/cards/stp.md. Owner: issue #497. Disposition: open.
- **P1 retained-159-observed-gap** — Retained predecessor #159 is not covered by a reviewed-green successor. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/159/cards/stp.md. Owner: issue #497. Disposition: open.
- **P1 retained-179-observed-gap** — Retained predecessor #179 is not covered by a reviewed-green successor. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/179/cards/stp.md. Owner: issue #505. Disposition: open.
- **P1 retained-180-observed-gap** — Retained predecessor #180 is not covered by a reviewed-green successor. Evidence: docs/milestones/v0.92.1/planned-issue-packets/issues/180/cards/stp.md. Owner: issue #505. Disposition: open.
- **P2 issue-84-operator-deferred** — [backlog][Observatory] Complete live Unity Observatory Runtime v3 integration is explicitly routed outside the release gate. Evidence: {"source"=>"github_label_and_canonical_plan", "label"=>"track:backlog", "issue_body_sha256"=>"044f67590bac43d355059a0704cb19d617955345d4b6af57df044a0b18a2f6f7", "candidate"=>"b8e1b4483b1c345e42ec7817bfde8214469867bb"}. Owner: issue #84. Disposition: routed_to_backlog.
- **P2 issue-251-operator-deferred** — [backlog][Runtime] Support TLS 1.2 on public Axum HTTPS/WSS for Unity is explicitly routed outside the release gate. Evidence: {"source"=>"github_label_and_canonical_plan", "label"=>"track:backlog", "issue_body_sha256"=>"9a922be2175e34d96720e2e247ed943e01d301617eb594341197c09d340186ef", "candidate"=>"b8e1b4483b1c345e42ec7817bfde8214469867bb"}. Owner: issue #251. Disposition: routed_to_backlog.

## Denominator

Execution roots: 58; release-tail stages: 11; retained predecessors: 36; backlog dispositions: 2; acceptance rows: 558.

| Planned ID | Issue | Head revision | Merge revision | Ancestry | Disposition |
|---|---:|---|---|---|---|
| POD-COORD | #51 | none | none | not_applicable_absorbed | satisfied_by_explicit_no_pr_closure |
| OBS-PUBLIC | #122 | c6189b927198726787683a0fa8b45a644d1e01ff | 49e20d099353b6bc57795c2597682e23f5251b97 | ancestor | satisfied |
| POD-51A | #261 | 78af9095a16886f8c0876e139113620dca806984 | 7f04298f87e2bde5b90eb174d9d7758067d348d0 | ancestor | satisfied |
| POD-51B | #262 | 842b7d57d09b66a6f9ea9a43a4e5d02be43aa3bb | 6e01e2bbe40915e814e43e54f84dfb61c84601e3 | ancestor | satisfied |
| POD-51C | #263 | a77e8e6c8e6e3c59330a5c15ce45924985735b7c | e13b5db0b49f9bc6772ca2765634a852abdf1ed2 | ancestor | satisfied |
| POD-51D | #264 | a285a690b86f95f6fc3ea3dd150ded76b32c469b | bdbf8aa32620da0e277bf3e2ed5f272354021744 | ancestor | satisfied |
| POD-STUDIO | #342 | 822c81e9d0ad15e479960de542d419e64c80e1f9 | b381edce8020567c4ac5af03f5df062157f55a16 | ancestor | satisfied |
| AWS-GPU-SIDECAR | #345 | ae702043682e18783a07234673150e3f1fd692f5 | 663792105abd977b7f9225d981200a8ed5749adb | ancestor | satisfied |
| WP-01 | #480 | 23856abbc7cde90dd9d1c6467dd6c61aca1bc274 | 001c270beda2b35b60e0be04f3c3bd331a156c48 | ancestor | satisfied |
| CORP-A | #482 | 2070d1b4ff269c2571a2077ae00d9f7fbb0ac67c | e2c1d1649b0c930a5a1254575a07ef2a4496d48d | ancestor | release_blocker |
| CORP-B | #483 | a0fae2fca6d802ae3f7ab987cc81c0bd36dd5239 | 4a0b49c0071bacdaab19d6d9eb8c44380beb51be | ancestor | satisfied |
| AWS-A | #484 | e9fdf5b07bdcbde235511c52c40fb8c626cc95a7 | e5f30c60c68a60d43f51c70b4615065197a34404 | ancestor | satisfied |
| AWS-B | #485 | 2a5d25239853499b6ac73b37d968d2b97e75a586 | a71d699d52831b32bb68ed9c7c7e837925949de4 | ancestor | satisfied |
| AWS-C | #486 | cfd5f0edabfde2e380a5534d619e7832f484bb8c | 1964b2e1f6e24a9dcb5788394502a2421300751a | ancestor | satisfied |
| AWS-D | #487 | 79be6b5b0327be752817197738887d25335e71a9 | 1d31016a8df3cf07a4c3f2e6acd2694bd10570c2 | ancestor | satisfied |
| AWS-E | #488 | 4e904b6629ff3060094dbef3613388e6e5245b8d | a6b404cd6e74d7528745325036ceb1a85fd47bd2 | ancestor | satisfied |
| AWS-F | #489 | 485b4197908231bb2065e1e29c7c5013536e1975 | 69ba35e066d1389a9f194659acb066a7dca82a40 | ancestor | satisfied |
| GCP-A | #490 | f0be8c8d1a2f12df8b2d8169997583dd66a9a521 | daa05de9332c82e3f9f2191975ef95b0ed4e211d | ancestor | satisfied |
| GCP-B | #491 | 695ca0f6cec62357349390afda3952e39cd92337 | 75ee9e6b2888a81b355d1fb496b488329a4c7d30 | ancestor | satisfied |
| GCP-C | #492 | 179fbc9fb2b1affc68577b8e94b66bc5ac5c49aa | b9a98710e2a0a50565c3835386f7f6a348a26eae | ancestor | satisfied |
| GCP-D | #493 | d5b1584bb55e92974e6d3481b59d2e28a17db441 | c0bf217934508d6dbc70d78633e6a95d5ddd9d06 | ancestor | satisfied |
| GCP-E | #494 | de959c6263f671fa8fe1df851ea6ae1d25686831 | dc08b5abf10682ed9ace5deefd0e1389ea6899b6 | ancestor | satisfied |
| XCL-01 | #495 | 6177249dfb46fe3cf95fbcc996469517928f525d | c78c60f5a45a87a96159d4910a831b69b62b042c | ancestor | satisfied |
| AWS-G | #496 | 59a3d0bd106f8bdd0def53dbe1564667ee6adac4 | 83077ca029d52c9d613ed5a373da30f1dd42d9b3 | ancestor | satisfied |
| CORP-C | #497 | none | none | not_applicable_absorbed | release_blocker |
| CORP-D | #498 | dc17d8c9ccdef3b65f3d2f7371dd3c2b8f48c7c8 | c51c8c7a8b51395986af8185f6e6ca2edaf4f435 | ancestor | satisfied |
| RUST-01 | #499 | 940c42d246be5d54f34b7b300526a644cc881580 | e986de6d06aacd385de93dd033def77a718c1581 | ancestor | satisfied |
| V3-A | #500 | d02f90008acadcc10df048b7f089cc4b98ef608f | 1dddcce35d061bc128c2431b4f31cf09e0f4d435 | ancestor | satisfied |
| V3-B | #501 | 9056f19245f93bc9efa3b55561671a8f002c6536 | 1972aa47bd7047b8594a03bf770fb92f7fb63d51 | ancestor | satisfied |
| V3-C | #502 | ed6f01c1e33b8057142491fca3028641ce5efc74 | 76de907734ab69efe00b5bc0bf24f066002d0131 | ancestor | satisfied |
| V3-D | #503 | 974abc520454690f0b392162b9ced783e8584017 | 5692d95ee6e4ee632833be348fa5601ddccbca1a | ancestor | satisfied |
| V3-E | #504 | 9ef650bc174a81849ffb09ae4d21b699fee1368d | f68ca996541b8825090261fd70845bf1c406b410 | ancestor | satisfied |
| V3-F | #505 | 74ddb31702482172eea4ba3d74700536eab32e49 | d3f98ed005cf44c359ac482e271e6b8634e93f23 | ancestor | release_blocker |
| DRT-A | #506 | 4676aef4189376b3f64d17efdb717732274e3240 | badcf9067da6eb46fc9f59e9da8b11a41e2f24f6 | ancestor | satisfied |
| DRT-B | #507 | cca85d6fd4976e2b2358d0d130b8291a88cb1c95 | d022d6c198669bcbc10cd98bee4d7c8520f9c4d4 | ancestor | satisfied |
| DRT-C | #508 | cca4f7f675241a0a473b38006c8be2eb95165028 | a1c440cc7b1e3708961680c802585d0b80e2263f | ancestor | satisfied |
| DRT-D | #509 | c89a584e53f152ed499a5d56d479296590044714 | 5a1109ffa795d411e0e15cdfd29adf68e2c2d953 | ancestor | satisfied |
| HOT-01 | #510 | fac5eaa63a82eaf50fe455df14cc22ebb08a2678 | 000fb7beb5fe4107e3e80d5de9183be224716a6d | ancestor | satisfied |
| OBS-A | #511 | none | none | not_applicable_absorbed | release_blocker |
| OBS-B | #512 | none | none | not_proven | release_blocker |
| DEC-01 | #513 | 489bc0f3af68b059e5c26ff22600d945b3d21db8 | 5bc84a0f27a522b6d500551d64f8d12dc2357427 | ancestor | satisfied |
| PROV-A | #514 | 1b0fd87496bb09200a5cb1bbb0529e8730be1b20 | 18f1c76667dc6913c2553b53228e73e8de9d11c9 | ancestor | satisfied |
| PROV-B | #515 | 9e6a8bd104d79f77edc4460ee5424cea83ef9cdc | 17b883e93abff7a155cd783a3d76f52ba2eabbf2 | ancestor | satisfied |
| PROV-C | #528 | 502626325d377042ee096fff31675998c3bcbd56 | edbc3ebc9b4e7c0862595345eebff8e04c9d5260 | ancestor | satisfied |
| CSDLC-BOOTSTRAP-DEFECT | #544 | 46e9d67378b2ca799326aac6000553a27300cffd | 5f0aa6eea5dd4aed6b9087255cad9b9d8ca4d8fc | ancestor | satisfied |
| LEARNER-COVERAGE-GATE | #558 | 6df008e183decf5829be5bdb8ef057c8bbb5a1e1 | c06b7d691eb6eb59867db87bbccbcdd1c74bebad | ancestor | satisfied |
| RUNTIME-COVERAGE-GATE | #560 | c58326d8796c4963ed4defa4bae4269667f332ee | 6a2a6f1d0b595797022eb291528a3c4c8c5541e9 | ancestor | satisfied |
| CSDLC-STALE-BINARY-GUARD | #563 | e8f5c952521bae9822b7d668da4572f7aa0f0b77 | 282ced247168e9d49c938ce194df92b97ebe7f65 | ancestor | satisfied |
| GCP-PROVIDER-CONFIG | #592 | 5e8165934330676d51be20ae3240b5eadb239f8e | 3e44cf33ec840443bf3c4639b6a4c106298cf405 | ancestor | satisfied |
| CANONICAL-AGENT-NAMES | #617 | 5b53a153ae9720d45a867409b08924459946c27c | c7f01661d00f7eec0e0a2c9266b18064e43d7133 | ancestor | satisfied |
| RUNTIME-ATOMIC-GENERATION | #656 | 05e93e7173fcde3cb6a55c2dcbde8d2f59742c78 | fef0e4f4657dadca389dc0c3ed315a88a15905f4 | ancestor | satisfied |
| RUNTIME-CONVERGENCE-DEADLINE | #659 | 30716d4806f350d4fd6aeb6c2e58b4cb9ce9d6f2 | db476f118ab11729d35d47ce20d4b59cc904fc01 | ancestor | satisfied |
| PODCAST-EXPOSURE-REPAIR | #660 | fdc3356e5777addf067bbdd61a01ee3f1ccbe5e1 | 549f1b6a51ee644a6563c9d02ad63cf49af0d3b6 | ancestor | release_blocker |
| SHEPHERD-PROVIDER-REPLY | #661 | f7211e3bba9d6759d7cae18511e397dd33e8ecee | 5fe80903ac847b08d82aabd0d30ac168b2e0bf70 | ancestor | satisfied |
| A2A-INITIATION | #662 | f2d09fa64efed868b043809387efe573eee54941 | cea5219f6e74b34d930d0dc39b6a607bc6303acb | ancestor | release_blocker |
| CSDLC-EMERGENCY-RECOVERY | #665 | 8e3648f0268107e8d842887b623d6972587c8cb5 | 0d6423644e7b1ee77e5729ec86d016203c9732fd | ancestor | satisfied |
| A2A-ACTION-RELIABILITY | #693 | 1946974be9839a13df6c4736b0f3ea34f0d1debc | 0e460810e5ac78ce655283f35e8167337eb33160 | ancestor | satisfied |
| POLIS-WELCOME-PACKAGE | #708 | 7b753133251ea8f7d01a6471ede2d40ca39d4813 | 3b3e406119568cc31a4587aa43fb5d94ce54368d | ancestor | satisfied |

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

- #84: `5c94385acfba95f93cd9673b30745a78a5f49f17c02f9606a9f7f220bc1af899`
- #251: `23c4be414ab0bb57a63b0a7a8e783bc72a5523b9b0565c4250905f9fd12c2749`
- Retained predecessor packets are indexed with SHA-256 digests in `gap_analysis_report.json`.

## Decision

**BLOCKED**

This is an admission decision only; it is not release approval.
