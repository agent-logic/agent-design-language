# Fresh-Session Standard SRP Review

The standard SRP remains the sole review prompt and result authority. After the
complete substantive commit, the implementation session sends the current SRP,
worktree path, and exact SHA to a fresh external review session. The reviewer is
read-only and reports findings first. The implementation session resolves every
actionable finding and repeats the review after substantive changes.

No new packet type, synthesis engine, daemon, scheduler, registry, claim,
parallel review record, provider abstraction, or lifecycle phase is introduced.
