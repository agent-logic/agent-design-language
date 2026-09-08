# v0.92.1 Release-tail Gap Analysis

Generated at `2026-09-08T03:36:12Z` against `b8e1b4483b1c345e42ec7817bfde8214469867bb`.

## Findings

- **P2 issue-84-operator-deferred** — [backlog][Observatory] Complete live Unity Observatory Runtime v3 integration is explicitly deferred to backlog and excluded from the v0.92.1 release gate. Owner: issue #84. Disposition: routed_to_backlog.
- **P2 issue-251-operator-deferred** — [backlog][Runtime] Support TLS 1.2 on public Axum HTTPS/WSS for Unity is explicitly deferred to backlog and excluded from the v0.92.1 release gate. Owner: issue #251. Disposition: routed_to_backlog.
- **P1 issue-512-not-terminal** — [v0.92.1][OBS-B] Observatory redesign implementation is not yet closed by a merged PR. Owner: issue #512. Disposition: open.

## Denominator

60 milestone execution issues were evaluated. Sprint umbrellas and release-tail children are coordination or downstream outputs and are excluded from the admission input denominator. Explicit backlog issues remain visible.

| Issue | State | Revision | Merge ancestry | Disposition |
|---:|---|---|---|---|
| #51 | closed | `none` | not_proven | satisfied |
| #84 | open | `none` | not_applicable_operator_deferred | operator_deferred_backlog |
| #122 | closed | `c6189b927198` | ancestor_of_candidate:49e20d099353b6bc57795c2597682e23f5251b97 | satisfied |
| #251 | open | `none` | not_applicable_operator_deferred | operator_deferred_backlog |
| #261 | closed | `78af9095a168` | ancestor_of_candidate:7f04298f87e2bde5b90eb174d9d7758067d348d0 | satisfied |
| #262 | closed | `842b7d57d09b` | ancestor_of_candidate:6e01e2bbe40915e814e43e54f84dfb61c84601e3 | satisfied |
| #263 | closed | `a77e8e6c8e6e` | ancestor_of_candidate:e13b5db0b49f9bc6772ca2765634a852abdf1ed2 | satisfied |
| #264 | closed | `a285a690b86f` | ancestor_of_candidate:bdbf8aa32620da0e277bf3e2ed5f272354021744 | satisfied |
| #342 | closed | `822c81e9d0ad` | ancestor_of_candidate:b381edce8020567c4ac5af03f5df062157f55a16 | satisfied |
| #345 | closed | `ae702043682e` | ancestor_of_candidate:663792105abd977b7f9225d981200a8ed5749adb | satisfied |
| #480 | closed | `23856abbc7cd` | ancestor_of_candidate:001c270beda2b35b60e0be04f3c3bd331a156c48 | satisfied |
| #482 | closed | `2070d1b4ff26` | ancestor_of_candidate:e2c1d1649b0c930a5a1254575a07ef2a4496d48d | satisfied |
| #483 | closed | `a0fae2fca6d8` | ancestor_of_candidate:4a0b49c0071bacdaab19d6d9eb8c44380beb51be | satisfied |
| #484 | closed | `e9fdf5b07bdc` | ancestor_of_candidate:e5f30c60c68a60d43f51c70b4615065197a34404 | satisfied |
| #485 | closed | `2a5d25239853` | ancestor_of_candidate:a71d699d52831b32bb68ed9c7c7e837925949de4 | satisfied |
| #486 | closed | `cfd5f0edabfd` | ancestor_of_candidate:1964b2e1f6e24a9dcb5788394502a2421300751a | satisfied |
| #487 | closed | `79be6b5b0327` | ancestor_of_candidate:1d31016a8df3cf07a4c3f2e6acd2694bd10570c2 | satisfied |
| #488 | closed | `4e904b6629ff` | ancestor_of_candidate:a6b404cd6e74d7528745325036ceb1a85fd47bd2 | satisfied |
| #489 | closed | `485b41979082` | ancestor_of_candidate:69ba35e066d1389a9f194659acb066a7dca82a40 | satisfied |
| #490 | closed | `f0be8c8d1a2f` | ancestor_of_candidate:daa05de9332c82e3f9f2191975ef95b0ed4e211d | satisfied |
| #491 | closed | `695ca0f6cec6` | ancestor_of_candidate:75ee9e6b2888a81b355d1fb496b488329a4c7d30 | satisfied |
| #492 | closed | `179fbc9fb2b1` | ancestor_of_candidate:b9a98710e2a0a50565c3835386f7f6a348a26eae | satisfied |
| #493 | closed | `d5b1584bb55e` | ancestor_of_candidate:c0bf217934508d6dbc70d78633e6a95d5ddd9d06 | satisfied |
| #494 | closed | `de959c6263f6` | ancestor_of_candidate:dc08b5abf10682ed9ace5deefd0e1389ea6899b6 | satisfied |
| #495 | closed | `6177249dfb46` | ancestor_of_candidate:c78c60f5a45a87a96159d4910a831b69b62b042c | satisfied |
| #496 | closed | `59a3d0bd106f` | ancestor_of_candidate:83077ca029d52c9d613ed5a373da30f1dd42d9b3 | satisfied |
| #497 | closed | `none` | not_proven | satisfied |
| #498 | closed | `dc17d8c9ccde` | ancestor_of_candidate:c51c8c7a8b51395986af8185f6e6ca2edaf4f435 | satisfied |
| #499 | closed | `940c42d246be` | ancestor_of_candidate:e986de6d06aacd385de93dd033def77a718c1581 | satisfied |
| #500 | closed | `d02f90008aca` | ancestor_of_candidate:1dddcce35d061bc128c2431b4f31cf09e0f4d435 | satisfied |
| #501 | closed | `9056f19245f9` | ancestor_of_candidate:1972aa47bd7047b8594a03bf770fb92f7fb63d51 | satisfied |
| #502 | closed | `ed6f01c1e33b` | ancestor_of_candidate:76de907734ab69efe00b5bc0bf24f066002d0131 | satisfied |
| #503 | closed | `974abc520454` | ancestor_of_candidate:5692d95ee6e4ee632833be348fa5601ddccbca1a | satisfied |
| #504 | closed | `9ef650bc174a` | ancestor_of_candidate:f68ca996541b8825090261fd70845bf1c406b410 | satisfied |
| #505 | closed | `74ddb3170248` | ancestor_of_candidate:d3f98ed005cf44c359ac482e271e6b8634e93f23 | satisfied |
| #506 | closed | `4676aef41893` | ancestor_of_candidate:badcf9067da6eb46fc9f59e9da8b11a41e2f24f6 | satisfied |
| #507 | closed | `cca85d6fd497` | ancestor_of_candidate:d022d6c198669bcbc10cd98bee4d7c8520f9c4d4 | satisfied |
| #508 | closed | `cca4f7f67524` | ancestor_of_candidate:a1c440cc7b1e3708961680c802585d0b80e2263f | satisfied |
| #509 | closed | `c89a584e53f1` | ancestor_of_candidate:5a1109ffa795d411e0e15cdfd29adf68e2c2d953 | satisfied |
| #510 | closed | `fac5eaa63a82` | ancestor_of_candidate:000fb7beb5fe4107e3e80d5de9183be224716a6d | satisfied |
| #511 | closed | `none` | not_proven | satisfied |
| #512 | open | `e8a0e0b9bb6a` | not_proven | release_blocker |
| #513 | closed | `489bc0f3af68` | ancestor_of_candidate:5bc84a0f27a522b6d500551d64f8d12dc2357427 | satisfied |
| #514 | closed | `1b0fd87496bb` | ancestor_of_candidate:18f1c76667dc6913c2553b53228e73e8de9d11c9 | satisfied |
| #515 | closed | `9e6a8bd104d7` | ancestor_of_candidate:17b883e93abff7a155cd783a3d76f52ba2eabbf2 | satisfied |
| #528 | closed | `502626325d37` | ancestor_of_candidate:edbc3ebc9b4e7c0862595345eebff8e04c9d5260 | satisfied |
| #544 | closed | `46e9d67378b2` | ancestor_of_candidate:5f0aa6eea5dd4aed6b9087255cad9b9d8ca4d8fc | satisfied |
| #558 | closed | `6df008e183de` | ancestor_of_candidate:c06b7d691eb6eb59867db87bbccbcdd1c74bebad | satisfied |
| #560 | closed | `c58326d8796c` | ancestor_of_candidate:6a2a6f1d0b595797022eb291528a3c4c8c5541e9 | satisfied |
| #563 | closed | `e8f5c952521b` | ancestor_of_candidate:282ced247168e9d49c938ce194df92b97ebe7f65 | satisfied |
| #592 | closed | `5e8165934330` | ancestor_of_candidate:3e44cf33ec840443bf3c4639b6a4c106298cf405 | satisfied |
| #617 | closed | `5b53a153ae97` | ancestor_of_candidate:c7f01661d00f7eec0e0a2c9266b18064e43d7133 | satisfied |
| #656 | closed | `05e93e7173fc` | ancestor_of_candidate:fef0e4f4657dadca389dc0c3ed315a88a15905f4 | satisfied |
| #659 | closed | `30716d4806f3` | ancestor_of_candidate:db476f118ab11729d35d47ce20d4b59cc904fc01 | satisfied |
| #660 | closed | `fdc3356e5777` | ancestor_of_candidate:549f1b6a51ee644a6563c9d02ad63cf49af0d3b6 | satisfied |
| #661 | closed | `f7211e3bba9d` | ancestor_of_candidate:5fe80903ac847b08d82aabd0d30ac168b2e0bf70 | satisfied |
| #662 | closed | `f2d09fa64efe` | ancestor_of_candidate:cea5219f6e74b34d930d0dc39b6a607bc6303acb | satisfied |
| #665 | closed | `8e3648f02681` | ancestor_of_candidate:0d6423644e7b1ee77e5729ec86d016203c9732fd | satisfied |
| #693 | closed | `1946974be983` | ancestor_of_candidate:0e460810e5ac78ce655283f35e8167337eb33160 | satisfied |
| #708 | closed | `7b753133251e` | ancestor_of_candidate:3b3e406119568cc31a4587aa43fb5d94ce54368d | satisfied |

## Limitations

- This pass consumes existing issue, PR, lifecycle, and retained proof; it does not repeat paid or disruptive execution.
- #84 and #251 remain operator-deferred backlog work and do not gate v0.92.1.

## Decision

**BLOCKED**
