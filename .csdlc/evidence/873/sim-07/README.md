# Issue #873 independent qualification

This packet qualifies candidate `f31b1b710074fdb51f41400b32b79c633bc3336e` with the frozen issue-specific installed binary. It used isolated fixture repositories, synthetic remote transports, and both primary and genuine linked-worktree entrypoints. It did not replace the shared owner binary, activate writers, call live providers, or mutate GitHub.

The two exact installed-binary journeys passed with 35 retained attempts. The 27 declared integration targets and five exact library regressions contributed 252 unique passing tests, with no unexpected test failure. All 33 emitted result envelopes have correlation IDs; two deliberately interrupted attempts have attempt IDs and no result envelope. The warm-host prepared-start samples were 3.808 s, 3.746 s, and 3.698 s, all below the three-minute fixture target. The measurement does not establish OS-cold performance or human authoring time.

The retained #872 conversion evidence validator passed. The separately required ignored old-owner release gate is not proven: after receiving the exact accepted #868 binary, the harness refused to run without `ISSUE872_OLD_STATE`. That old state is unavailable and was not guessed or reconstructed. This blocker prevents an overall passing decision.

`scorecard.json` and `qualification-decision.json` contain the adjudication. Raw commands, stdout, stderr, result records, exact installed attempt ledgers, and timing evidence are retained alongside them.
